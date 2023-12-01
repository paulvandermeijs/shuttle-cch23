mod challenge_1;

use axum::{http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route(
            "/-1/error",
            get(|| async { (StatusCode::INTERNAL_SERVER_ERROR, "") }),
        )
        .nest("/1", challenge_1::router());

    Ok(router.into())
}
