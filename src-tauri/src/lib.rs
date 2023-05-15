use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tbs {
    pub tbs: String,
    is_login: i32,
}

#[serde_as]
#[derive(Debug, Serialize, thiserror::Error)]
#[serde(tag = "type", content = "message")]
pub enum Error {
    #[error("http error")]
    Http(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        reqwest::Error,
    ),
    #[error("serde error")]
    Serde(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        serde_json::Error,
    ),
    #[error("Infallible error")]
    Infallible(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        std::convert::Infallible,
    ),
    #[error("SystemTimeError")]
    SystemTimeError(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        std::time::SystemTimeError,
    ),
}

// 自定义错误类型
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize)]
pub struct FollowResLike {
    pub like_forum: Vec<FavoriteResLikeName>,
}

#[derive(Deserialize)]
pub struct FavoriteResLikeName {
    pub forum_name: String,
}

#[derive(Deserialize)]
pub struct FavoriteRes {
    pub data: FollowResLike,
}
