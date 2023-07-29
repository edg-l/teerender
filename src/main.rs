#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}

async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/:skin", get(render));

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        std::env::var("TEERENDER_LISTEN_PORT")
            .map(|x| x.parse().unwrap())
            .unwrap_or(3000),
    ));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
struct RenderParams {}

// basic handler that responds with a static string
async fn render(Path(skin): Path<String>, Query(params): Query<RenderParams>) -> &'static str {
    todo!()
}
