use etlify::configs::config;
use std::env;

fn main() {
    println!("Hello, world!");
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let c = config::load_config(&config_path)?;

    // 使用配置创建服务
    let nats_service = NatsService::new(&config);

    // 其他操作...

    Ok(())
}
