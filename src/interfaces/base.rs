// 抽象的数据源接口
trait DataSource {
    fn connect(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn read_records(&mut self) -> Result<Vec<Record>, Box<dyn std::error::Error>>;
    // ... 其他方法，如关闭连接等
}

// 抽象的目标接口
trait DataSink {
    fn connect(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn write_records(&mut self, records: Vec<Record>) -> Result<(), Box<dyn std::error::Error>>;
    // ... 其他方法，如关闭连接等
}

// 定义通用的数据记录结构体
#[derive(Debug, Clone)]
struct Record {
    // 根据实际需求定义字段
}
