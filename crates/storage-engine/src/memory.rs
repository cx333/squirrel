// user -> wgl
// datetime -> 2025/4/8-18:52
// project -> squirrel
// InMemoryStorage 实现

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use meta_service::{InMemoryMetaService, MetaService};
// 从 core create 引入 Value 和 StorageEngine trait
use squirrel_core::{Value, StorageEngine};

///
/// 内存存储引擎实现，使用hashmap存储每张表的数据
/// 每张表对应一个 vec<row>，每行是一个hashmap<列名，值>
/// 为了线程安全，使用 Arc + Mutex 包装。
///
pub struct InMemoryStorage {
    data: Arc<Mutex<HashMap<String, Vec<HashMap<String, Value>>>>>,
}

impl InMemoryStorage {
    /// 创建一个新的内存存储引擎实例
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl StorageEngine for InMemoryStorage {
    /// 插入一行数据到指定表中
    fn insert_row(&mut self, table: &str, row: HashMap<String, Value>) -> Result<(), String> {
        let arc = Arc::clone(&self.data);
        let mut guard = (*arc).lock().unwrap();
        let entry = guard.entry(table.to_string()).or_default();
        entry.push(row);
        Ok(())
    }

    /// 扫描指定表，返回所有数据
    fn scan_table(&self, table: &str) -> Vec<HashMap<String, Value>> {
        let arc = Arc::clone(&self.data);
        let guard = (*arc).lock().unwrap();
        guard.get(table).cloned().unwrap_or_default()
    }
}


#[cfg(test)]
mod tests {
    use super::InMemoryStorage;
    use squirrel_core::{Value, StorageEngine, TableSchema, ColumnDef, ColumnType};
    use std::collections::HashMap;
    use meta_service::{InMemoryMetaService, MetaService};

    #[test]
    fn create_table() {
        let mut meta_service = InMemoryMetaService::new();
        let schema = TableSchema {
            name: "users".to_string(),
            columns: vec![
                ColumnDef {
                    name: "id".to_string(),
                    data_type: ColumnType::Int,
                    is_nullable: false,
                    is_primary: true,
                    is_indexed: false,
                }, ColumnDef {
                    name: "name".to_string(),
                    data_type: ColumnType::String,
                    is_nullable: false,
                    is_primary: false,
                    is_indexed: false,
                }, ColumnDef {
                    name: "passwd".to_string(),
                    data_type: ColumnType::String,
                    is_nullable: false,
                    is_primary: false,
                    is_indexed: false,
                }
            ],
        };
        meta_service.create_table(schema).unwrap();
        let tale_info = meta_service.get_table("users");
        println!("{:?}", tale_info);
    }

    #[test]
    fn test_insert_and_scan() {
        let mut engine = InMemoryStorage::new();

        // 构造一行数据
        let mut row = HashMap::new();
        row.insert("id".to_string(), Value::Int(1));
        row.insert("name".to_string(), Value::String("wgl".to_string()));

        // 插入
        engine.insert_row("users", row.clone()).unwrap();

        // 查询
        let result = engine.scan_table("users");
        println!("{:?}", result);

        // 验证结果
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("id"), Some(&Value::Int(1)));
        assert_eq!(result[0].get("name"), Some(&Value::String("wgl".to_string())));
    }
}

