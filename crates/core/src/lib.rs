// user -> wgl
// datetime -> 2025/4/7-22:00
// project -> squirrel

use std::collections::HashMap;
use std::time::SystemTime;
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;

///
/// 列的类型定义。
///
#[derive(Debug, Clone)]
pub enum ColumnType {
    Int,
    Float,
    String,
    Bool,
    Timestamp,
    Decimal,
    Text,
    Json,
}

///
/// 列的结构定义，包含列名、类型、是否主键/可为空。
///
#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub name: String,  // 列名
    pub data_type: ColumnType, // 数据类型
    pub is_nullable: bool, // 是否允许为NULL
    pub is_primary: bool, // 是否是主键
    pub is_indexed: bool, // 是否需要全文索引（例如Text字段）
}

///
/// 表的结构定义，包含表名和所有列的信息。
///
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,  // 表名
    pub columns: Vec<ColumnDef>, // 表中所有列的定义
}

///
/// 元数据服务的 trait，负责表的定义、获取 schema 信息等。
///
pub trait MetaService {
    /// 创建一个新表 （注册schema）
    fn create_table(&mut self, schema: TableSchema) -> Result<(), String>;

    /// 根据表名获取表的 schema 信息
    fn get_table(&self, name: &str) -> Option<&TableSchema>;
}

///
/// 表示单元格中的值，用于插入和查询数据。
///
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Timestamp(SystemTime),
    Decimal(Decimal),
    Text(String),
    Json(JsonValue),
    Null,
}

///
/// 存储引擎的 Trait，定义插入/查询的基本接口
/// 实现类调用 column-store 模块进行数据落盘
///
pub trait StorageEngine {
    /// 插入一条数据 （按列落盘）
    fn insert_row(&mut self, table: &str, row: HashMap<String, Value>) -> Result<(), String>;
    /// 扫描整个表，返回所有数据（全表查询扫描）
    fn scan_table(&self, table: &str) -> Vec<HashMap<String, Value>>;
}

///
/// 搜索引擎 trait, 用于全文检索、关键字匹配等
///
pub trait SearchEngine {
    /// 将一行数据加入索引，（热key数据）
    fn index_row(&mut self, table: &str, row_id: u64, row: &HashMap<String, Value>) -> Result<u64, String>;
    /// 搜索关键词，返回符合条件的 row_id
    fn search(&self, table: &str, keyword: &str) -> Vec<u64>;
}