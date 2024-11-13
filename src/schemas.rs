use async_graphql::{Object, SimpleObject};

pub struct QueryRoot;

#[derive(SimpleObject)]
struct User {
    a: i32,
}

#[Object]
impl QueryRoot {
    // It works on foreign types without extensions as before
    async fn user(&self, username: String) -> &str {
        // Look up users from the database
        "a"
    }
}
