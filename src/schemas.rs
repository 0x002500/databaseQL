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
        let rows = sqlx::query(&query).fetch_all(&mut conn).await.unwrap();
        let column: Vec<String> = sqlx::query_as(SELECT * FROM ? WHERE id = ?;)
        .bind(&self.table_name)
        .bind(id)
        .fetch_one(&mut conn)
        .await?;
        columns
    }
}

#[Object]
impl Database {
    async fn tables<'ctx>(&self, ctx: &Context<'_>) -> Vec<Table> {
        let pool = ctx.data::<Pool<Any>>().expect("unable to get pool");
        let mut conn = pool.acquire().await.unwrap();
        let rows = sqlx::query("SELECT * FROM ?").bind(ctx.table_name).fetch_all(&mut conn).await.unwrap();
        let tables: Vec<Table> = rows.iter().map(|row| Table { table_name: row.get(0) }).collect();
        tables
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn db<'ctx>(&self, ctx: &Context<'_>, db_name: String) -> Database {
        Database { db_name }
    }
}
