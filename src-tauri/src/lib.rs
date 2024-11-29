pub mod api;
pub mod utils;

use std::sync::Arc;

use serde_json::json;
use tauri::Manager;
use utils::error::Error;

// AppState 结构体定义
pub struct AppState {
    config_path: std::path::PathBuf,
}

impl AppState {
    // 创建新的 AppState 实例
    pub fn new(config_path: std::path::PathBuf) -> Self {
        Self {
            config_path,
        }
    }

    // 获取配置文件路径
    pub fn get_config_path(&self) -> &std::path::PathBuf {
        &self.config_path
    }

}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            api::core::get_favorite_name,
            api::core::client_sign,
            api::core::get_bduss_os 
        ])
        .setup(|app| {
            let home_dir = app.path().home_dir().expect("home dir error");
            let config_path = home_dir.join(".config").join("tb.json");
            if !config_path.exists() {
                std::fs::write(&config_path, json!({"bduss":""}).to_string()).expect("write config error");
            }
            let app_state = Arc::new(AppState::new(config_path));

            app.manage(app_state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
