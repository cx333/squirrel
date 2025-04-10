// user -> wgl
// datetime -> 2025/4/10-20:55
// project -> squirrel
// 表结构、元数据具体实现

use std::collections::HashMap;
use squirrel_core::TableSchema;

mod create_table;

pub trait MetaService {
    /// 创建表
    fn create_table(&mut self, schema: TableSchema) -> Result<(), String>;

    /// 删除表
    fn drop_table(&mut self, name: &str) -> Result<(), String>;

    /// 获取表结构信息
    fn get_table(&mut self, name: &str) -> Option<&TableSchema>;

    /// 修改表结构
    fn alter_table(&mut self, name: &str, new_schema: TableSchema) -> Result<(), String>;
}

/// 存储
pub struct InMemoryMetaService {
    tables: HashMap<String, TableSchema>,
}

impl InMemoryMetaService {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new()
        }
    }
}

impl MetaService for InMemoryMetaService {
    fn create_table(&mut self, schema: TableSchema) -> Result<(), String> {
        if self.tables.contains_key(&schema.name) {
            Err("table err".to_string())
        } else {
            self.tables.insert(schema.name.clone(), schema);
            Ok(())
        }
    }

    fn drop_table(&mut self, name: &str) -> Result<(), String> {
        todo!()
    }

    fn get_table(&mut self, name: &str) -> Option<&TableSchema> {
        self.tables.get(name)
    }

    fn alter_table(&mut self, name: &str, new_schema: TableSchema) -> Result<(), String> {
        todo!()
    }
}