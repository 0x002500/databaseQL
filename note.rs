use async_graphql::*;

/// 第三级：字段（Column）
#[derive(SimpleObject)]
struct Column {
    id: i32,
    name: String,
}

/// 第二级：表（Table）
#[derive(SimpleObject)]
struct Table {
    id: i32,
    name: String,
}

#[Object]
impl Table {
    /// 动态查询表中的字段列表
    async fn columns(&self, ctx: &Context<'_>) -> Vec<Column> {
        // 例如：可以根据 ctx.look_ahead() 判断客户端是否请求了 columns 字段，
        // 从而决定是否发起额外查询，这里仅返回模拟数据
        vec![
            Column { id: 1, name: "column1".to_string() },
            Column { id: 2, name: "column2".to_string() },
        ]
    }
}

/// 第一级：数据库（Database）
#[derive(SimpleObject)]
struct Database {
    id: i32,
    name: String,
}

#[Object]
impl Database {
    /// 动态查询数据库中的表列表
    async fn tables(&self, ctx: &Context<'_>) -> Vec<Table> {
        // 根据实际情况，比如查询数据库或调用其他服务，这里返回模拟数据
        vec![
            Table { id: 1, name: "table1".to_string() },
            Table { id: 2, name: "table2".to_string() },
        ]
    }
}

/// 根查询类型
struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 顶层查询返回一个 Database 对象，
    /// 通过传入 id 或其他标识符可以加载指定数据库
    async fn db(&self, ctx: &Context<'_>, id: i32) -> Database {
        // 实际应用中可以根据 id 查询对应数据库信息
        Database { id, name: "my_database".to_string() }
    }
}

fn main() {
    // 构建 schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    // 此处你可以将 schema 嵌入 HTTP 服务（例如 Actix-web 或 Tide）来启动 GraphQL 服务器
}
