mod handlers;
mod repositories;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use handlers::{find_all, create_todo, delete_todo, find_todo, update_todo};
use std::net::SocketAddr;
use std::{env, sync::Arc};


#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("into".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127,0,0,1],3000));
    tracing::debug!("listening on {}",addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

fn create_app<T:TodoRepository> (repository:T) -> Router{
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>).get(find_all::<T>))
        .route("/todos/:id", 
            get(find_todo::<T>)
                    .patch(update_todo::<T>)
                    .delete(delete_todo::<T>),)
        .layer(Extension(Arc::new(repository)))
}




async fn root() -> &'static str{
    "Hello world"
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{body::Body,http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world(){
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body:String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body,"Hello world");
    }
}