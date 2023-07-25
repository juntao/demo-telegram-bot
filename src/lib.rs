// use serde::{Deserialize, Serialize};
use serde_json::Value;
use tg_flows::{listen_to_update, Telegram, Update, UpdateKind};
use flowsnet_platform_sdk::logger;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
// use std::collections::HashMap;

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

async fn handler(tele: Telegram, placeholder_text: &str, help_mesg: &str, update: Update) {
    if let UpdateKind::Message(msg) = update.kind {
        let chat_id = msg.chat.id;
        log::info!("Received message from {}", chat_id);

        let text = msg.text().unwrap_or("");
        if text.eq_ignore_ascii_case("/help") {
            _ = tele.send_message(chat_id, help_mesg);

        } else if text.eq_ignore_ascii_case("/top") {
            let placeholder = tele
                .send_message(chat_id, placeholder_text)
                .expect("Error occurs when sending Message to Telegram");

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

            /*
            let mut resp_str = String::new();
            let c: Collection = serde_json::from_str(&json_str).unwrap();
            log::info!("Parsed JSON!");
            for d in &c.data.data {
                log::info!("Construct the resp : {}", resp_str);
                resp_str.push_str("Name: ");
                resp_str.push_str(&d.name);
                resp_str.push_str("\n");
                resp_str.push_str("Address: ");
                resp_str.push_str(&d.main_token_address);
                resp_str.push_str("\n");

                for t in &d.tokens {
                    if let Some(p) = t.price {
                        resp_str.push_str("Price: ");
                        resp_str.push_str(&p.to_string());
                        resp_str.push_str("\n");
                        break;
                    }
                }
            }
            _ = tele.edit_message_text(chat_id, placeholder.id, &resp_str);
            */
            
            let mut resp_str = String::new();
            let c: Value = serde_json::from_str(&json_str).unwrap();
            let data_arr = c["data"]["data"].as_array().unwrap();
            for d in data_arr {
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
                    }
                }
            }
            _ = tele.edit_message_text(chat_id, placeholder.id, &resp_str);

        } else {
            // Do nothing
        }

        return;

    }
}

/*
#[derive(Serialize, Deserialize)]
pub struct Collection {
    #[serde(rename = "code")]
    code: i64,

    #[serde(rename = "msg")]
    msg: String,

    #[serde(rename = "ms")]
    ms: i64,

    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "data")]
    data: Vec<Datum>,

    #[serde(rename = "page")]
    page: i64,

    #[serde(rename = "page_size")]
    page_size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "pid")]
    pid: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "main_chain")]
    main_chain: Chain,

    #[serde(rename = "main_token_address")]
    main_token_address: String,

    #[serde(rename = "chains")]
    chains: String,

    #[serde(rename = "icon")]
    icon: String,

    #[serde(rename = "gameTypes")]
    game_types: Vec<GameType>,

    #[serde(rename = "logo")]
    logo: String,

    #[serde(rename = "homepage")]
    homepage: String,

    #[serde(rename = "whitepaper_link")]
    whitepaper_link: String,

    #[serde(rename = "telegram_link")]
    telegram_link: String,

    #[serde(rename = "discord_link")]
    discord_link: String,

    #[serde(rename = "twitter_link")]
    twitter_link: String,

    #[serde(rename = "desktop_web_supported")]
    desktop_web_supported: bool,

    #[serde(rename = "mobile_web_supported")]
    mobile_web_supported: bool,

    #[serde(rename = "pc_supported")]
    pc_supported: bool,

    #[serde(rename = "mac_supported")]
    mac_supported: bool,

    #[serde(rename = "android_supported")]
    android_supported: bool,

    #[serde(rename = "ios_supported")]
    ios_supported: bool,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "added_timestamp")]
    added_timestamp: f64,

    #[serde(rename = "tags")]
    tags: Vec<Tag>,

    #[serde(rename = "weth_usd_price")]
    weth_usd_price: f64,

    #[serde(rename = "tokens")]
    tokens: Vec<Token>,

    #[serde(rename = "ft_stats")]
    ft_stats: Vec<FtStat>,

    #[serde(rename = "nfts")]
    nfts: Vec<Nft>,

    #[serde(rename = "volume_24h")]
    volume_24_h: f64,

    #[serde(rename = "volume_change_24h")]
    volume_change_24_h: f64,

    #[serde(rename = "volume_7d")]
    volume_7_d: f64,

    #[serde(rename = "volume_change_7d")]
    volume_change_7_d: f64,

    #[serde(rename = "volume_30d")]
    volume_30_d: f64,

    #[serde(rename = "volume_change_30d")]
    volume_change_30_d: f64,

    #[serde(rename = "users_24h")]
    users_24_h: i64,

    #[serde(rename = "users_change_24h")]
    users_change_24_h: f64,

    #[serde(rename = "users_7d")]
    users_7_d: i64,

    #[serde(rename = "users_change_7d")]
    users_change_7_d: f64,

    #[serde(rename = "users_30d")]
    users_30_d: i64,

    #[serde(rename = "users_change_30d")]
    users_change_30_d: f64,

    #[serde(rename = "market_cap")]
    market_cap: f64,

    #[serde(rename = "market_cap_change_24h")]
    market_cap_change_24_h: Option<f64>,

    #[serde(rename = "market_cap_change_7d")]
    market_cap_change_7_d: Option<f64>,

    #[serde(rename = "market_cap_change_30d")]
    market_cap_change_30_d: Option<f64>,

    #[serde(rename = "stats")]
    stats: Stats,
}

#[derive(Serialize, Deserialize)]
pub struct FtStat {
    #[serde(rename = "token_address")]
    token_address: String,

    #[serde(rename = "chain")]
    chain: Chain,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "symbol")]
    symbol: String,

    #[serde(rename = "icon")]
    icon: String,

    #[serde(rename = "users_24h")]
    users_24_h: i64,

    #[serde(rename = "users_7d")]
    users_7_d: i64,

    #[serde(rename = "users_30d")]
    users_30_d: i64,

    #[serde(rename = "users_change_24h")]
    users_change_24_h: f64,

    #[serde(rename = "users_change_7d")]
    users_change_7_d: f64,

    #[serde(rename = "users_change_30d")]
    users_change_30_d: f64,

    #[serde(rename = "volume_24h")]
    volume_24_h: Option<f64>,

    #[serde(rename = "volume_7d")]
    volume_7_d: Option<f64>,

    #[serde(rename = "volume_30d")]
    volume_30_d: Option<f64>,

    #[serde(rename = "volume_change_24h")]
    volume_change_24_h: Option<f64>,

    #[serde(rename = "volume_change_7d")]
    volume_change_7_d: Option<f64>,

    #[serde(rename = "volume_change_30d")]
    volume_change_30_d: Option<f64>,

    #[serde(rename = "transactions_24h")]
    transactions_24_h: i64,

    #[serde(rename = "transactions_7d")]
    transactions_7_d: i64,

    #[serde(rename = "transactions_30d")]
    transactions_30_d: i64,

    #[serde(rename = "transactions_change_24h")]
    transactions_change_24_h: f64,

    #[serde(rename = "transactions_change_7d")]
    transactions_change_7_d: f64,

    #[serde(rename = "transactions_change_30d")]
    transactions_change_30_d: f64,

    #[serde(rename = "market_cap")]
    market_cap: Option<f64>,

    #[serde(rename = "market_cap_change_24h")]
    market_cap_change_24_h: Option<f64>,

    #[serde(rename = "market_cap_change_7d")]
    market_cap_change_7_d: Option<f64>,

    #[serde(rename = "market_cap_change_30d")]
    market_cap_change_30_d: Option<f64>,

    #[serde(rename = "price")]
    price: Option<f64>,

    #[serde(rename = "price_change_24h")]
    price_change_24_h: Option<f64>,

    #[serde(rename = "price_change_7d")]
    price_change_7_d: Option<f64>,

    #[serde(rename = "price_change_30d")]
    price_change_30_d: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct GameType {
    #[serde(rename = "type")]
    game_type_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Nft {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "symbol")]
    symbol: String,

    #[serde(rename = "icon")]
    icon: String,

    #[serde(rename = "token_address")]
    token_address: String,

    #[serde(rename = "chain")]
    chain: Chain,

    #[serde(rename = "users_24h")]
    users_24_h: i64,

    #[serde(rename = "users_7d")]
    users_7_d: i64,

    #[serde(rename = "users_30d")]
    users_30_d: i64,

    #[serde(rename = "users_change_24h")]
    users_change_24_h: Option<f64>,

    #[serde(rename = "users_change_7d")]
    users_change_7_d: Option<f64>,

    #[serde(rename = "users_change_30d")]
    users_change_30_d: Option<f64>,

    #[serde(rename = "volume_24h")]
    volume_24_h: Option<f64>,

    #[serde(rename = "volume_7d")]
    volume_7_d: Option<f64>,

    #[serde(rename = "volume_30d")]
    volume_30_d: Option<f64>,

    #[serde(rename = "volume_change_24h")]
    volume_change_24_h: Option<f64>,

    #[serde(rename = "volume_change_7d")]
    volume_change_7_d: Option<f64>,

    #[serde(rename = "volume_change_30d")]
    volume_change_30_d: Option<f64>,

    #[serde(rename = "sales_24h")]
    sales_24_h: i64,

    #[serde(rename = "sales_7d")]
    sales_7_d: i64,

    #[serde(rename = "sales_30d")]
    sales_30_d: i64,

    #[serde(rename = "sales_change_24h")]
    sales_change_24_h: Option<f64>,

    #[serde(rename = "sales_change_7d")]
    sales_change_7_d: Option<f64>,

    #[serde(rename = "sales_change_30d")]
    sales_change_30_d: Option<f64>,

    #[serde(rename = "floor_price")]
    floor_price: f64,

    #[serde(rename = "floor_price_change_24h")]
    floor_price_change_24_h: f64,

    #[serde(rename = "floor_price_change_7d")]
    floor_price_change_7_d: f64,

    #[serde(rename = "floor_price_change_30d")]
    floor_price_change_30_d: f64,

    #[serde(rename = "market_cap")]
    market_cap: f64,

    #[serde(rename = "market_cap_change_24h")]
    market_cap_change_24_h: MarketCapChange,

    #[serde(rename = "market_cap_change_7d")]
    market_cap_change_7_d: MarketCapChange,

    #[serde(rename = "market_cap_change_30d")]
    market_cap_change_30_d: i64,

    #[serde(rename = "avg_price_24h")]
    avg_price_24_h: Option<f64>,

    #[serde(rename = "avg_price_7d")]
    avg_price_7_d: Option<serde_json::Value>,

    #[serde(rename = "avg_price_30d")]
    avg_price_30_d: Option<serde_json::Value>,

    #[serde(rename = "avg_price_change_24h")]
    avg_price_change_24_h: Option<f64>,

    #[serde(rename = "avg_price_change_7d")]
    avg_price_change_7_d: Option<f64>,

    #[serde(rename = "avg_price_change_30d")]
    avg_price_change_30_d: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "prices")]
    prices: HashMap<String, Option<f64>>,

    #[serde(rename = "transactions")]
    transactions: HashMap<String, Option<f64>>,

    #[serde(rename = "users")]
    users: HashMap<String, Option<f64>>,

    #[serde(rename = "volumes")]
    volumes: HashMap<String, Option<f64>>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "symbol")]
    symbol: Option<String>,

    #[serde(rename = "icon")]
    icon: String,

    #[serde(rename = "chain")]
    chain: Chain,

    #[serde(rename = "token_address")]
    token_address: String,

    #[serde(rename = "price")]
    price: Option<f64>,

    #[serde(rename = "price_change_24h")]
    price_change_24_h: Option<f64>,

    #[serde(rename = "price_change_abs_24h")]
    price_change_abs_24_h: Option<f64>,

    #[serde(rename = "price_change_7d")]
    price_change_7_d: Option<f64>,

    #[serde(rename = "price_change_abs_7d")]
    price_change_abs_7_d: Option<f64>,

    #[serde(rename = "price_change_30d")]
    price_change_30_d: Option<f64>,

    #[serde(rename = "price_change_abs_30d")]
    price_change_abs_30_d: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketCapChange {
    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
pub enum Chain {
    #[serde(rename = "BSC")]
    Bsc,

    #[serde(rename = "Ethereum")]
    Ethereum,

    #[serde(rename = "Harmony")]
    Harmony,

    #[serde(rename = "Polygon")]
    Polygon,

    #[serde(rename = "Solana")]
    Solana,
}
*/
