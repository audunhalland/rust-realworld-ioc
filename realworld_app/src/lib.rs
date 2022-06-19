pub mod app;
pub mod config;

mod api;
mod auth;
mod password;
mod user;

#[cfg(test)]
mod test_util;

use anyhow::Context;
use implementation::Impl;
use tower::ServiceBuilder;

pub async fn serve(app: app::App) -> anyhow::Result<()> {
    let router = api::api_router().layer(
        ServiceBuilder::new()
            .layer(axum::extract::Extension(Impl::new(app)))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(tower_http::trace::TraceLayer::new_for_http()),
    );

    // We use 8080 as our default HTTP server port, it's pretty easy to remember.
    //
    // Note that any port below 1024 needs superuser privileges to bind on Linux,
    // so 80 isn't usually used as a default for that reason.
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(router.into_make_service())
        .await
        .context("error running HTTP server")
}
