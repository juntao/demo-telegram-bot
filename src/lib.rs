use serde_json::Value;
use tg_flows::{listen_to_update, Telegram, Update, UpdateKind};
use flowsnet_platform_sdk::logger;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    logger::init();
    let telegram_token = std::env::var("telegram_token").unwrap();
    let placeholder_text = std::env::var("placeholder").unwrap_or("Typing ...".to_string());
    let help_mesg = std::env::var("help_mesg").unwrap_or("Type command /top to see what's hot!".to_string());

    listen_to_update(&telegram_token, |update| {
        let tele = Telegram::new(telegram_token.to_string());
        handler(tele, &placeholder_text, &help_mesg, update)
    }).await;

    Ok(())
}

async fn handler(tele: Telegram, _placeholder_text: &str, help_mesg: &str, update: Update) {
    if let UpdateKind::Message(msg) = update.kind {
        let chat_id = msg.chat.id;
        log::info!("Received message from {}", chat_id);

        let text = msg.text().unwrap_or("");
        if text.eq_ignore_ascii_case("/help") {
            _ = tele.send_message(chat_id, help_mesg);

        } else if text.eq_ignore_ascii_case("/top") {
            // let placeholder = tele
            //     .send_message(chat_id, placeholder_text)
            //     .expect("Error occurs when sending Message to Telegram");

            let fetch_uri: Uri = Uri::try_from("https://gamefidash.com/api/v2/projects?page=1&page_size=10&date_range=24h&sort_dim=volume").unwrap();
            let mut bytes = Vec::new();
            Request::new(&fetch_uri)
                .method(Method::GET)
                .send(&mut bytes).unwrap();
            let json_str = String::from_utf8(bytes).unwrap();
            log::info!("Received from web service : {}", json_str);

            // data.data[0].name
            // data.data[0].main_token_address
            // data.data[0].tokens[0].price
            let c: Value = serde_json::from_str(&json_str).unwrap();
            let data_arr = c["data"]["data"].as_array().unwrap();
            for d in data_arr {
                let mut resp_str = String::new();
                resp_str.push_str("Name: ");
                resp_str.push_str(d["name"].as_str().unwrap());
                resp_str.push_str("\n");
                resp_str.push_str("Address: ");
                resp_str.push_str(d["main_token_address"].as_str().unwrap());
                resp_str.push_str("\n");

                let token_arr = d["tokens"].as_array().unwrap();
                for t in token_arr {
                    if let Some(p) = t["price"].as_f64() {
                        resp_str.push_str("Price: ");
                        resp_str.push_str(&p.to_string());
                        resp_str.push_str("\n");
                        break;
                    }
                }
                _ = tele.send_message(chat_id, &resp_str);
            }

        } else {
            // Do nothing
        }

        return;

    }
}
