use axum::{routing::get, Router, extract::{TypedHeader}, headers::UserAgent};

async fn index(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    String::from(user_agent.as_str())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
