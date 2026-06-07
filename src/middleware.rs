use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::any::Any;
use std::io::{self, Write};
use std::panic::{self, PanicHookInfo};
use tower_http::catch_panic::ResponseForPanic;
use tracing::error;

#[derive(Clone)]
pub struct RedactingPanicHandler;

impl ResponseForPanic for RedactingPanicHandler {
    type ResponseBody = Body;

    fn response_for_panic(
        &mut self,
        _err: Box<dyn Any + Send + 'static>,
    ) -> axum::http::Response<Self::ResponseBody> {
        (StatusCode::INTERNAL_SERVER_ERROR, "[REDACTED]").into_response()
    }
}

pub struct PanicHookGuard(Option<Box<dyn Fn(&PanicHookInfo<'_>) + Send + Sync + 'static>>);

pub fn install_redacting_panic_hook() -> PanicHookGuard {
    let previous = panic::take_hook();

    panic::set_hook(Box::new(|panic_info| {
        let location = panic_info
            .location()
            .map(|location| format!("{}:{}", location.file(), location.line()))
            .unwrap_or_else(|| "unknown".to_string());

        let mut stderr = io::stderr().lock();
        let _ = writeln!(stderr, "panic redacted at {location}: [REDACTED]");
    }));

    PanicHookGuard(Some(previous))
}

impl Drop for PanicHookGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.0.take() {
            panic::set_hook(previous);
        }
    }
}

pub async fn redact_errors_middleware(req: Request<Body>, next: Next) -> Response {
    let response = next.run(req).await;

    if response.status().is_server_error() {
        error!(
            kind = "http_5xx",
            status = response.status().as_u16(),
            payload = "[REDACTED]",
            "server error redacted"
        );

        return (response.status(), "[REDACTED]").into_response();
    }

    response
}
