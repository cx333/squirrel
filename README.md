# squirrel
多模型数据库
📌 第一步目标：创建表 & 定义 Schema


✅ 第 1 步：core 中定义接口
ColumnType, ColumnDef, Schema, TableMeta
Trait：StorageEngine、MetaService

✅ 第 2 步：meta-service 实现表注册、查询 Schema 的服务
可以存到内存或简单文件中
提供 create_table() / get_schema(table) 方法

✅ 第 3 步：cli 通过命令创建表
支持：xdb create-table --name users --columns id:int,name:str

✅ 第 4 步：column-store 实现按列写入磁盘，后续添加压缩、分块