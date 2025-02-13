mod schemas;

use actix_web::{web, App, HttpResponse, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schemas::{MySchema, QueryRoot};
use sqlx::{Any, Pool};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init();

    // 从环境变量中获取数据库连接 URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // 建立数据库连接池，使用 sqlx 的 Any 类型支持多种数据库
    let pool = Pool::<Any>::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // 构建 GraphQL schema，并将数据库连接池注入到上下文中
    let schema = MySchema::build(QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription)
        .data(pool)
        .finish();

    println!("GraphQL server running on http://localhost:8000/graphql");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            // POST 请求处理 GraphQL 查询
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
            // GET 请求展示 Playground 调试页面
            .service(web::resource("/graphql").route(web::get().to(graphql_playground)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

/// 处理 GraphQL 请求
async fn graphql_handler(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// 返回 GraphQL Playground 页面，方便调试和测试
async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            async_graphql::http::playground_source(
                async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
            ),
        )
}
