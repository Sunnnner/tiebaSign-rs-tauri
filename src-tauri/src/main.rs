// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;

use crypto::digest::Digest;
use crypto::md5::Md5;
use reqwest::header;
use reqwest::Client as re_client;
use BaiDu::{FavoriteRes, Result, Tbs, Error};

const LIKE_URL: &str = "https://tieba.baidu.com/mo/q/newmoindex";
const TBS_URL: &str = "http://tieba.baidu.com/dc/common/tbs";
const SIGN_URL: &str = "http://c.tieba.baidu.com/c/c/forum/sign";
const SIGN_KEY: &str = "tiebaclient!!!";

fn get_client(bduss: &str) -> Result<re_client> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Host", header::HeaderValue::from_static("tieba.baidu.com"));
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36"));
    headers.insert(
        "COOKIE",
        header::HeaderValue::from_str(&("BDUSS=".to_owned() + bduss)).unwrap(),
    );
    let client = re_client::builder().default_headers(headers).build()?;
    Ok(client)
}

async fn get_tbs(bduss: &str) -> Result<Tbs> {
    let client = get_client(bduss)?;
    let req = client
        .post(TBS_URL)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await?;
    let body = req.text().await?;
    let tbs: Tbs = serde_json::from_str(&body)?;
    Ok(tbs)
}


async fn save_bduss_os(bduss: &str) -> Result<()>{
    let home_dir =
        dirs::home_dir().ok_or_else(|| Error::Error("home dir not found".to_string()))?;
    let config_path = home_dir.join(".config").join("bd");
    fs::create_dir_all(&config_path)?;
    let config = serde_json::json!({
        "bduss": bduss,
    });
    let config_str = serde_json::to_string_pretty(&config)?;
    let bd_config_path = config_path.join("config.json");
    fs::write(bd_config_path, config_str)?;
    Ok(())
}

#[tauri::command]
async fn get_bduss_os() -> Result<String>{
    let home_dir =
        dirs::home_dir().ok_or_else(|| Error::Error("home dir not found".to_string()))?;
    let config_path = home_dir.join(".config").join("bd");
    let bd_config_path = config_path.join("config.json");
    let config_str = fs::read_to_string(bd_config_path)?;
    let config: serde_json::Value = serde_json::from_str(&config_str)?;
    let bduss = config["bduss"].as_str().ok_or_else(|| Error::Error("bduss not found".to_string()))?;
    Ok(bduss.to_string())
}



#[tauri::command]
async fn client_sign(kw: &str) -> Result<String> {
    let bduss = get_bduss_os().await?;
    let tbs_obj = get_tbs(&bduss).await?;
    let tbs = tbs_obj.tbs.to_string();
    let mut md5 = Md5::new();
    let sign = format!("kw={}tbs={}{}", kw, tbs, SIGN_KEY);
    md5.input_str(&sign);
    let md5_sign = md5.result_str();
    let post_body = format!("kw={}&tbs={}&sign={}", kw, tbs, md5_sign);
    let client = get_client(&bduss)?;
    let _res = client
        .post(SIGN_URL)
        .body(post_body)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?;
    Ok(kw.to_string())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_favorite_name(bduss: &str) -> Result<Vec<String>> {
    save_bduss_os(bduss).await?;
    let client: re_client = get_client(bduss)?;
    let req: FavoriteRes = client
        .get(LIKE_URL)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?
        .json()
        .await?;
    let favorite_list = req
        .data
        .like_forum
        .into_iter()
        .map(|x| x.forum_name)
        .collect();

    Ok(favorite_list)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, get_favorite_name, client_sign, get_bduss_os
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
