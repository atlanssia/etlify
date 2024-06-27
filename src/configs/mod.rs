// src/configs/mod.rs
pub mod config;
// pub mod source;
// pub use source::{NatsConfig, PgsqlConfig};

// 加载配置的公共函数（若有需要）
pub use crate::configs::config::load_config;
