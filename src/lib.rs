mod api;
mod auth;
mod db;
mod error;
mod types;
mod user;

use anyhow::Context;
use std::sync::Arc;
use tower::ServiceBuilder;

pub struct Config {
    pub jwt_signing_key: hmac::Hmac<sha2::Sha384>,
}

#[derive(Clone)]
pub struct App {
    pub config: Arc<Config>,
}

pub async fn serve(app: App) -> anyhow::Result<()> {
    let app = api::api_router().layer(
        ServiceBuilder::new()
            .layer(axum::extract::Extension(app))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(tower_http::trace::TraceLayer::new_for_http()),
    );

    // We use 8080 as our default HTTP server port, it's pretty easy to remember.
    //
    // Note that any port below 1024 needs superuser privileges to bind on Linux,
    // so 80 isn't usually used as a default for that reason.
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}
