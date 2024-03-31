pub mod aggregation;
pub mod filter;
pub mod json_to_csv;

pub trait DataProcessor {
    type Input;
    type Output;
    async fn process(&mut self, input: Self::Input) -> anyhow::Result<Self::Output>;
}

// 示例：JSON 转 CSV 处理器
mod json_to_csv {
    use super::*;
    use serde_json::Value;
    use std::io::Write;

    pub struct JsonToCsvProcessor {
        // ...
    }

    impl DataProcessor for JsonToCsvProcessor {
        // ...
    }
}
