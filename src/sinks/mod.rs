pub mod database;
pub mod kafka;
pub mod nats;

pub trait DataSink {
    type Item;
    fn connect(&mut self) -> anyhow::Result<()>;
    async fn write_data(&mut self, item: Self::Item) -> anyhow::Result<()>;
}

// 示例：Kafka 目标
mod kafka {
    use super::*;
    use rdkafka::producer::{FutureProducer, FutureRecord};

    pub struct KafkaSink {
        // ...
    }

    impl DataSink for KafkaSink {
        // ...
    }
}
