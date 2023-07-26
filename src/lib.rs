use serde_json::Value;
use tg_flows::{listen_to_update, Telegram, Update, UpdateKind, InputFile};
use store_flows::{get, set};
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
    let api_key = std::env::var("miaoshouai_key").unwrap();
    let placeholder_text = std::env::var("placeholder").unwrap_or("Generating your image ...".to_string());
    let help_mesg = std::env::var("help_mesg").unwrap_or("Type command `/model model_name` to select a model. Available choices for `model_name` are: redshift-diffusion, samdoesarts-ultmerge,  midjourney-v4, inkpunk".to_string());

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
        if text.eq_ignore_ascii_case("/help") {
            _ = tele.send_message(chat_id, help_mesg);

        if text.starts_with("/model ") {
            let model_id = text.strip_prefix("/model ").unwrap_or("");
            if model_id.is_empty() {
                _ = tele.send_message(chat_id, "Sorry the model ID is not recognized.");
                return;
            }
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
            
            let prompts: Vec<&str> = text.split_whitespace().collect();
            let mut positive_prompt = String::new();
            let mut negative_prompt = String::new();
            for p in prompts {
                if p.starts_with("-") {
                    negative_prompt.push_str(p.strip_prefix("-").unwrap_or(""));
                } else {
                    positive_prompt.push_str(p);
                }
            }

            let json_template = r#"
{
  "apiKey": "{}",
  "batch_size": "1",
  "height": 512,
  "width": 512,
  "model_id": "{}",
  "prompt": "{}",
  "negative_prompt": "{}",
  "num_inference_steps": 20,
  "samplers": "EulerAncestralDiscreteScheduler",
  "seed": 0,
  "enhance_prompt": "no",
  "cfg_scale": 7.5
}"#;
            let json_request = format!(json_template, api_key, model_id, positive_prompt, positive_prompt);
            let api_uri: Uri = Uri::try_from("https://miaoshouai.com/playground/translation/produce/do/text2img").unwrap();
            let mut bytes = Vec::new();
            Request::new(&api_uri)
                .method(Method::POST)
                .body(&json_request)
                .send(&mut bytes).unwrap();
            let json_response = String::from_utf8(bytes).unwrap();
            log::info!("Received from web service : {}", json_response);

            // wait for 15 sec!
            
            let c: Value = serde_json::from_str(&json_response).unwrap();
            let fetch_key = c["data"]["fetchKey"].as_str().unwrap();
            let fetch_url = format!("https://miaoshouai.com/playground/translation/produce/get/fetchResult?fetchKey={}", fetch_key);
            _ = tele.send_photo(chat_id, InputFile::url(&fetch_url));

        } else {
            // Do nothing
        }

        return;

    }
}
