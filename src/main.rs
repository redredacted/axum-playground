use axum::{
    routing::get,
    Router, extract::{Path, Query}, http::StatusCode,
};

use serde::Deserialize;

#[derive(Deserialize)]
struct Pizza {
    flavor: String,
}

impl Default for Pizza {
    fn default() -> Self {
        Self { flavor: "default".to_owned() }
    }
}

async fn print_userid(Path(username): Path<u32>, pizza: Option<Query<Pizza>>) -> Result<String, (StatusCode, String)> {
    if username == 8 {
        return Err((StatusCode::IM_A_TEAPOT, "you made the server spontaneosly turn into a teapot".to_string()));
    }

    let Query(pizza) = pizza.unwrap_or_default();

    Ok(format!("user id: {}, favorite pizza flavor: {}", username, pizza.flavor))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(|| async { "Hello, Foo!" }))
        .route("/thing/:username", get(print_userid));
    
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
