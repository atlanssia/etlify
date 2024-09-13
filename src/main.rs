use std::env;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

    match 1 {
            num @ (1 | 2) => {
                println!("{}", num);
            }
            _ => {}
        }

    // let c = config::load_config();

    // server::server();

    // 使用配置创建服务
    // let nats_service = NatsService::new(&config);

    // 其他操作...

    // Ok(())
}
