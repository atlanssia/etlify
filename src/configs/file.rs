use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config {
    source: SourceConfig,
    sink: SinkConfig,
    transform: TransformConfig, // 可选的转换逻辑配置
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "config")]
enum SourceConfig {
    Kafka(KafkaSourceConfig),
    PostgreSQL(PostgreSQLSourceConfig),
    // ...其他数据源类型及其对应的配置
}

// 同样定义SinkConfig枚举结构

// 根据配置文件中的类型标签，使用工厂方法或Box<dyn DataSource>、Box<dyn DataSink>动态创建相应的数据源和目标实例：
fn create_data_source(config: &SourceConfig) -> Result<Box<dyn DataSource>, Box<dyn Error>> {
    match config {
        SourceConfig::Kafka(kafka_config) => Ok(Box::new(KafkaSource::from_config(kafka_config)?)),
        SourceConfig::PostgreSQL(pg_config) => {
            Ok(Box::new(PostgreSQLSource::from_config(pg_config)?))
        }
        // ... 对每种数据源添加类似处理
    }
}

// 同理，定义create_data_sink函数

// 最后，设计主程序结构，读取配置文件，创建数据源和目标，执行数据抽取、转换（如果有的话）和加载过程。
fn run_etl(config_path: &str) -> Result<(), Box<dyn Error>> {
    let config: Config = load_config(config_path)?;

    let source = create_data_source(&config.source)?;
    let sink = create_data_sink(&config.sink)?;

    // 连接数据源和目标
    source.connect()?;
    sink.connect()?;

    // 抽取数据
    let records = source.read_records()?;

    // 转换数据（如果有此步骤）
    let transformed_records = transform_records(records, &config.transform)?;

    // 加载数据
    sink.write_records(transformed_records)?;

    // 关闭连接
    source.disconnect()?;
    sink.disconnect()?;

    Ok(())
}
