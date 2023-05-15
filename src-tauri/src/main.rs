// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crypto::digest::Digest;
use crypto::md5::Md5;
use reqwest::header;
use reqwest::Client as re_client;
use tiebaSign_tauri::{FavoriteRes, Result, Tbs};

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

async fn get_favorite(bduss: &str) -> Result<Vec<String>> {
    let client = get_client(bduss)?;
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

async fn client_sign(bduss: &str, tbs: &str, kw: &str) -> Result<()> {
    let mut md5 = Md5::new();
    let sign = format!("kw={}tbs={}{}", kw, tbs, SIGN_KEY);
    md5.input_str(&sign);
    let md5_sign = md5.result_str();
    let post_body = format!("kw={}&tbs={}&sign={}", kw, tbs, md5_sign);
    let client = get_client(bduss)?;
    let _res = client
        .post(SIGN_URL)
        .body(post_body)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await?;
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn sign_tb(bduss: String) -> Result<()> {
    let tbs = get_tbs(&bduss).await?;
    let favorite = get_favorite(&bduss).await;
    let favorite = match favorite {
        Ok(favorite) => favorite,
        Err(e) => {
            return Err(e);
        }
    };
    for i in favorite {
        let bduss = bduss.to_owned();
        let tbs_data = tbs.tbs.to_string();
        client_sign(&bduss, &tbs_data, &i).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, sign_tb])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
