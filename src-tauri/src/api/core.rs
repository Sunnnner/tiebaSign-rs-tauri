use std::sync::Arc;

use crate::{AppState, Result};

use super::{client::TiebaClient, config::ConfigManager};

// Tauri 命令实现
#[tauri::command]
pub async fn get_bduss_os(state: tauri::State<'_, Arc<AppState>>) -> Result<String> {
    ConfigManager::get_bduss(state.get_config_path()).await
}

#[tauri::command]
pub async fn client_sign(
    state: tauri::State<'_, Arc<AppState>>,
    kw: &str
) -> Result<String> {
    let config_path = state.get_config_path();
    let bduss = ConfigManager::get_bduss(config_path).await?;
    let client = TiebaClient::create_client(&bduss)?;
    let tbs = TiebaClient::get_tbs(&client).await?.tbs;
    TiebaClient::sign(&client, kw, &tbs).await
}

#[tauri::command]
pub async fn get_favorite_name(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<String>> {
    let config_path = state.get_config_path();
    let bduss = ConfigManager::get_bduss(config_path).await?;
    let client = TiebaClient::create_client(&bduss)?;
    ConfigManager::save_bduss(config_path, &bduss).await?;
    TiebaClient::get_favorites(&client).await
}
