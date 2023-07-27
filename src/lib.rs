use serde_json::json;
use serde_json::Value;
use tg_flows::{listen_to_update, Telegram, Update, UpdateKind, InputFile};
use store_flows::{get, set};
use flowsnet_platform_sdk::logger;
use url::Url;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use tokio::time::{sleep, Duration};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    logger::init();
    let telegram_token = std::env::var("telegram_token").unwrap();
    let api_key = std::env::var("miaoshouai_key").unwrap();
    let placeholder_text = std::env::var("placeholder").unwrap_or("Generating your image ...".to_string());
    let help_mesg = std::env::var("help_mesg").unwrap_or("Type command `/model_id` to select a model. Available choices are: /redshift-diffusion, /samdoesarts-ultmerge,  /midjourney-v4, /inkpunk".to_string());

    listen_to_update(&telegram_token, |update| {
        let tele = Telegram::new(telegram_token.to_string());
        handler(tele, &api_key, &placeholder_text, &help_mesg, update)
    }).await;

    Ok(())
}

async fn handler(tele: Telegram, api_key: &str, placeholder_text: &str, help_mesg: &str, update: Update) {
    if let UpdateKind::Message(msg) = update.kind {
        let chat_id = msg.chat.id;
        log::info!("Received message from {}", chat_id);

        let text = msg.text().unwrap_or("");
        if text.eq_ignore_ascii_case("/help") || text.eq_ignore_ascii_case("/start") {
            _ = tele.send_message(chat_id, help_mesg);

        } else if text.eq_ignore_ascii_case("/redshift-diffusion") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/samdoesarts-ultmerge") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/midjourney-v4") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/inkpunk") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/synthwave-diffusion") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/analog-diffusion") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/elldreths-vi") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/dreamlike") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/dream-shaper-8797") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/counterfeit-v20") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/deliberate") {
            let model_id = text.strip_prefix("/").unwrap();
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else {
            let placeholder = tele
                .send_message(chat_id, placeholder_text)
                .expect("Error occurs when sending Message to Telegram");
            
            let model_id = match get("model_id") {
                Some(v) => v.as_str().unwrap_or_default().to_owned(),
                None => "midjourney-v4".to_owned(),
            };
            
            let prompts: Vec<&str> = text.split(",").collect();
            let mut positive_prompt = String::new();
            let mut negative_prompt = String::new();
            for p in prompts {
                let cp = p.trim();
                if cp.starts_with("-") {
                    negative_prompt.push_str(cp.strip_prefix("-").unwrap_or(""));
                    negative_prompt.push_str(", ");
                } else {
                    positive_prompt.push_str(cp);
                    positive_prompt.push_str(", ");
                }
            }

            let json_request = format!(include_str!("request_template.json"), api_key, model_id, positive_prompt.trim(), negative_prompt.trim());
            let api_uri: Uri = Uri::try_from("https://miaoshouai.com/playground/translation/produce/do/text2img").unwrap();
            let mut bytes = Vec::new();
            Request::new(&api_uri)
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(json_request.as_bytes())
                .send(&mut bytes).unwrap();
            let json_response = String::from_utf8(bytes).unwrap();
            log::info!("Received from api service : {}", json_response);

            // wait for 20 sec just to be safe!
            sleep(Duration::from_millis(20000)).await;
            
            let c: Value = serde_json::from_str(&json_response).unwrap();
            let fetch_key = c["data"]["fetchKey"].as_str().unwrap();
            let fetch_url = format!("https://miaoshouai.com/playground/translation/produce/get/fetchResult?fetchKey={}", fetch_key);
            let fetch_uri: Uri = Uri::try_from(fetch_url.as_str()).unwrap();
            let mut bytes = Vec::new();
            Request::new(&fetch_uri)
                .method(Method::GET)
                .send(&mut bytes).unwrap();
            let json_response = String::from_utf8(bytes).unwrap();
            log::info!("Received from fetch service : {}", json_response);


            let c: Value = serde_json::from_str(&json_response).unwrap();
            let pic_url = c["data"]["picUrl"].as_str().unwrap();
            _ = tele.send_photo(chat_id, InputFile::url(Url::parse(pic_url).unwrap()));
            _ = tele.edit_message_text(chat_id, placeholder.id, "");
        }

        return;
    }
}
