use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {}

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {err}"),
        )
            .into_response(),
    }
}

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("assets");

    let app = Router::new()
        .route("/", get(hello))
        .nest_service("/assets", serve_dir.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
