use std::fs;
use crate::Result;
use crate::Error;

// 配置管理实现
pub struct ConfigManager;

impl ConfigManager {
    // 保存 BDUSS
    pub async fn save_bduss(config_path: &std::path::Path, bduss: &str) -> Result<()> {
        fs::create_dir_all(config_path.parent().unwrap())?;
        let config = serde_json::json!({ "bduss": bduss });
        fs::write(
            config_path,
            serde_json::to_string_pretty(&config)?,
        )?;
        Ok(())
    }

    // 读取 BDUSS
    pub async fn get_bduss(config_path: &std::path::Path) -> Result<String> {
        let config: serde_json::Value = serde_json::from_str(&fs::read_to_string(config_path)?)?;
        Ok(config["bduss"]
            .as_str()
            .ok_or_else(|| Error::Error("bduss not found".to_string()))?
            .to_string())
    }
}