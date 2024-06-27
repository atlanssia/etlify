// src/configs/config.rs
use std::{
    env,
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use serde_derive::Deserialize;
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub settings: Settings,
    pub source: SourceConfig,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub nats: Option<NatsConfig>,
    pub kafka: Option<KafkaConfig>,
    pub pgsql: Option<PgsqlConfig>,
}

#[derive(Debug, Deserialize)]
pub struct NatsConfig {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct KafkaConfig {
    pub endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct PgsqlConfig {
    pub address: String,
}

impl AppConfig {
    pub fn load_from_file<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        from_str(&contents).map_err(|e| e.into())
    }
}

// 公开一个便捷的加载配置函数
pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    AppConfig::load_from_file(PathBuf::from(config_path))
}
