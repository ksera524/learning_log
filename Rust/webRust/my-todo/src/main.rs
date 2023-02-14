use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get,post},
    Router,Json,
};
use std::net::SocketAddr;
use std::env;
use serde::{Deserialize,Serialize};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("into".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/",get(root))
        .route("/users", post(create_user));
    let addr = SocketAddr::from(([127,0,0,1],3000));
    tracing::debug!("listening on {}",addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}


async fn root() -> &'static str{
    "Hello world"
}

async fn create_user(Json(payload):Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id:1337,
        username:payload.username,
    };

    (StatusCode::CREATED,Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username:String,
}

#[derive(Serialize)]
struct User {
    id:u64,
    username:String,
}