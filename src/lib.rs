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
    let help_mesg = std::env::var("help_mesg").unwrap_or("Select a model. Available choices are:\n/redshift_diffusion\n/samdoesarts_ultmerge\n/midjourney_v4\n/inkpunk\n/counterfeit_v20".to_string());

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

        } else if text.eq_ignore_ascii_case("/redshift_diffusion") {
            let model_id = "redshift-diffusion";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/samdoesarts_ultmerge") {
            let model_id = "samdoesarts-ultmerge";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/midjourney_v4") {
            let model_id = "midjourney-v4";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/inkpunk") {
            let model_id = "inkpunk";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/synthwave_diffusion") {
            let model_id = "synthwave-diffusion";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/analog_diffusion") {
            let model_id = "analog-diffusion";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/elldreths_vi") {
            let model_id = "elldreths-vi";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/dreamlike") {
            let model_id = "dreamlike";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/dream_shaper_8797") {
            let model_id = "dream-shaper-8797";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/counterfeit_v20") {
            let model_id = "counterfeit-v20";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.eq_ignore_ascii_case("/deliberate") {
            let model_id = "deliberate";
            set("model_id", json!(model_id), None);
            _ = tele.send_message(chat_id, &format!("The model has been set to {}", model_id));

        } else if text.starts_with("/") {
            _ = tele.send_message(chat_id, "Sorry, I do not recognize the model name.");
            _ = tele.send_message(chat_id, help_mesg);

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

            let json_request = format!(include_str!("request_template.json"), api_key, model_id, positive_prompt.trim().trim_matches(','), negative_prompt.trim().trim_matches(','));
            log::info!("api request : {}", json_request);
            let api_uri: Uri = Uri::try_from("https://miaoshouai.com/playground/translation/produce/do/text2img").unwrap();
            let mut bytes = Vec::new();
            let mut req = Request::new(&api_uri);
            req.method(Method::POST)
                .header("User-Agent", "flows/1")
                .header("Accept", "*/*")
                .header("Content-Type", "application/json")
                .header("Content-Length", &json_request.as_bytes().len())
                .body(json_request.as_bytes());
            log::info!("api request : {:?}", req);
            let resp = req.send(&mut bytes).unwrap();
            log::info!("api response : {:?}", resp);
            let json_response = String::from_utf8(bytes).unwrap();
            log::info!("Received from api service : {}", json_response);

            let c: Value = serde_json::from_str(&json_response).unwrap();
            let fetch_key = c["data"]["fetchKey"].as_str().unwrap();
            let fetch_url = format!("https://miaoshouai.com/playground/translation/produce/get/fetchResult?fetchKey={}", fetch_key);
            log::info!("fetch request : {}", fetch_url);
            let fetch_uri: Uri = Uri::try_from(fetch_url.as_str()).unwrap();
            
            // Wait for a max of 120s
            for _ in 0..11 {
                // Wait for 10 sec
                sleep(Duration::from_millis(10000)).await;
                
                let mut bytes = Vec::new();
                Request::new(&fetch_uri)
                    .method(Method::GET)
                    .send(&mut bytes).unwrap();
                let json_response = String::from_utf8(bytes).unwrap();
                log::info!("Received from fetch service : {}", json_response);

                let c: Value = serde_json::from_str(&json_response).unwrap();
                let pic_url = match c["data"]["picUrl"].as_str() {
                    Some(v) => v,
                    None => "",
                };
                if !pic_url.is_empty() {
                    log::info!("pic request : {}", pic_url);
                    _ = tele.send_photo(chat_id, InputFile::url(Url::parse(pic_url).unwrap()));
                    _ = tele.edit_message_text(chat_id, placeholder.id, "");
                    break;
                }
            }
        }
        return;
    }
}
