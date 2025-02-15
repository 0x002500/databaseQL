use async_graphql::*;
use sqlx::{database, Any, Pool, Row};

#[derive(SimpleObject)]
struct Column {
    pub id: String,
}

#[derive(SimpleObject)]
pub struct Table {
    pub table_name: String,
}

/// Database 类型包含数据库名称，提供动态查询指定表数据的能力
pub struct Database {
    pub db_name: String,
}

#[Object]
impl Table {
    async fn columns<'ctx>(&self, ctx: &Context<'_>, db_name: String) -> Vec<Column> {
        let pool = ctx.data::<Pool<Any>>().expect("unable to get pool");
        let mut conn = pool.acquire().await.unwrap();
        let query = format!("SELECT column_name FROM information_schema.columns WHERE table_name = '{}'", self.table_name);
        let rows = sqlx::query(&query).fetch_all(&mut conn).await.unwrap();
        let column: Vec<String> = sqlx::query_as(
            r#"
            SELECT id, name, email FROM users WHERE name = ?;
            "#
        )
        .bind("Alice")
        .fetch_one(&mut conn)
        .await?;
        columns
    }
}
