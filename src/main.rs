use axum::{Router, response::Html, response::Json, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Clone, Deserialize, Serialize)]
struct Model {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/models", get(models))
        .nest_service("/model", ServeDir::new("model"))
        .fallback_service(ServeDir::new("html"));

    let listener = tokio::net::TcpListener::bind("localhost:80").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1><a href='/'>Home</a>")
}

async fn models() -> Json<Vec<Model>> {
    let models = vec![
        Model {
            name: "hello.stl".to_string(),
        },
        Model {
            name: "hello.stl".to_string(),
        },
        Model {
            name: "hello.stl".to_string(),
        },
    ];
    Json(models)
}
