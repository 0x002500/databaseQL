use async_graphql::*;
use sqlx::{Any, Pool, Row};

#[derive(SimpleObject)]
pub struct Table {
    pub id: i32,
}

/// Database 类型包含数据库名称，提供动态查询指定表数据的能力
pub struct Database {
    pub db_name: String,
}

#[Object]
impl Database {
    /// 根据传入的 table_name 动态查询对应数据库中的表数据（示例中仅查询 id 字段）
    async fn table(&self, ctx: &Context<'_>, table_name: String) -> Result<Table> {
        // 从上下文中取出数据库连接池
        let pool = ctx.data_unchecked::<Pool<Any>>();
        // 构造动态 SQL 查询（注意：实际生产环境下应防范 SQL 注入）
        let query = format!("SELECT id FROM {}.{} LIMIT 1", self.db_name, table_name);
        let row = sqlx::query(&query)
            .fetch_one(pool)
            .await?;
        // 从查询结果中获取 id 字段
        let id: i32 = row.try_get("id")?;
        Ok(Table { id })
    }
}

/// 根查询类型，通过 db 字段获取对应 Database 对象
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn db(&self, db_name: String) -> Database {
        Database { db_name }
    }
}

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
