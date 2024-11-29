use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, thiserror::Error)]
#[serde(tag = "type", content = "message")]
pub enum Error {
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
    #[error("std error")]
    Std(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        std::io::Error,
    ),
    #[error("error: {0}")]
    Error(String),
    #[error("http error")]
    Http(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tauri_plugin_http::Error,
    ),
    #[error("reqwest error")]
    Reqwest(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tauri_plugin_http::reqwest::Error,
    ),
    #[error("reqwest header error")]
    ReqwestHeader(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tauri_plugin_http::reqwest::header::ToStrError,
    ),
    #[error("lock error")]
    Lock,
}
