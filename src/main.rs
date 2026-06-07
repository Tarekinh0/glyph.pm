use axum::{Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use glyph::middleware::{
    RedactingPanicHandler, install_redacting_panic_hook, redact_errors_middleware,
};

async fn root_handler() -> &'static str {
    "Hello, World!"
}

async fn panic_handler() -> &'static str {
    panic!("Synthetic failure for IBAN FR76 3000 6000 0112 3456 7890 123 on 2026-06-07");
}

#[tokio::main]
async fn main() {
    let _panic_hook = install_redacting_panic_hook();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "glyph=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/panic", get(panic_handler))
        .layer(axum::middleware::from_fn(redact_errors_middleware))
        .layer(CatchPanicLayer::custom(RedactingPanicHandler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
