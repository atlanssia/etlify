pub mod database;
pub mod kafka;
pub mod nats;

pub trait DataSource {
    type Item;
    fn connect(&mut self) -> anyhow::Result<()>;
    fn fetch_data(&mut self) -> Box<dyn Stream<Item = Self::Item> + Send>;
}

// 示例：数据库源
mod database {
    use super::*;
    use sqlx::{postgres::PgPoolOptions, Executor, Row};
    use tokio_stream::StreamExt;

    pub struct DatabaseSource {
        // ...
    }

    impl DataSource for DatabaseSource {
        // ...
    }
}
