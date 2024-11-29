// TiebaClient 实现
pub struct TiebaClient;

use crate::Result;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::{header, Client};

const LIKE_URL: &str = "https://tieba.baidu.com/mo/q/newmoindex";
const TBS_URL: &str = "http://tieba.baidu.com/dc/common/tbs";
const SIGN_URL: &str = "http://c.tieba.baidu.com/c/c/forum/sign";
const SIGN_KEY: &str = "tiebaclient!!!";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tbs {
    pub tbs: String,
    is_login: i32,
}

#[derive(Deserialize)]
pub struct FavoriteRes {
    pub data: FollowResLike,
}

#[derive(Deserialize)]
pub struct FollowResLike {
    pub like_forum: Vec<FavoriteResLikeName>,
}

#[derive(Deserialize)]
pub struct FavoriteResLikeName {
    pub forum_name: String,
}

impl TiebaClient {
    // 创建 HTTP 客户端
    pub fn create_client(bduss: &str) -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Host", header::HeaderValue::from_static("tieba.baidu.com"));
        headers.insert(
            "User-Agent",
            header::HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36",
            ),
        );
        headers.insert(
            "COOKIE",
            header::HeaderValue::from_str(&("BDUSS=".to_owned() + bduss)).unwrap(),
        );

        Ok(Client::builder().default_headers(headers).build()?)
    }

    // 获取 TBS
    pub async fn get_tbs(client: &Client) -> Result<Tbs> {
        let req = client
            .post(TBS_URL)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await?;
        let body = req.text().await?;
        Ok(serde_json::from_str(&body)?)
    }

    // 签到实现
    pub async fn sign(client: &Client, kw: &str, tbs: &str) -> Result<String> {
        let mut hasher = Md5::new();
        hasher.update(format!("kw={}tbs={}{}", kw, tbs, SIGN_KEY));
        let md5_sign = hasher.finalize();

        client
            .post(SIGN_URL)
            .body(format!("kw={}&tbs={}&sign={:x}", kw, tbs, md5_sign))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;

        Ok(kw.to_string())
    }

    // 获取收藏列表，收藏贴吧列表是否已签到未实现
    pub async fn get_favorites(client: &Client) -> Result<Vec<String>> {
        let req: FavoriteRes = client
            .get(LIKE_URL)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?
            .json()
            .await?;

        Ok(req
            .data
            .like_forum
            .into_iter()
            .map(|x| x.forum_name)
            .collect())
    }
}
