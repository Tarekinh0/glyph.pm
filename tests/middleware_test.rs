use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    routing::get,
};
use glyph::middleware::{RedactingPanicHandler, redact_errors_middleware};
use std::{
    io::{self, Read, Write},
    os::fd::{FromRawFd, RawFd},
    sync::{Arc, Mutex},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower::ServiceExt;
use tower_http::catch_panic::ResponseForPanic;
use tracing_subscriber::fmt::MakeWriter;

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone)]
struct BufferWriter(Arc<Mutex<Vec<u8>>>);

struct BufferGuard(Arc<Mutex<Vec<u8>>>);

impl<'a> MakeWriter<'a> for BufferWriter {
    type Writer = BufferGuard;

    fn make_writer(&'a self) -> Self::Writer {
        BufferGuard(self.0.clone())
    }
}

impl Write for BufferGuard {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn make_log_capture() -> (tracing::Dispatch, Arc<Mutex<Vec<u8>>>) {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let writer = BufferWriter(buffer.clone());
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_level(true)
        .with_writer(writer)
        .finish();

    (tracing::Dispatch::new(subscriber), buffer)
}

fn logs_as_string(buffer: &Arc<Mutex<Vec<u8>>>) -> String {
    String::from_utf8(buffer.lock().unwrap().clone()).unwrap()
}

fn synthetic_panic_message() -> &'static str {
    "Synthetic failure for IBAN FR76 3000 6000 0112 3456 7890 123\nline 2: from john.doe@example.com on 2026-06-07\nline 3: malformed {\"amount\": 150.00€, \"card\": 4532 1234 5678 9012"
}

fn synthetic_edge_case_payload() -> String {
    format!(
        "line 1: Synthetic failure for IBAN FR76 3000 6000 0112 3456 7890 123\nline 2: {{\"user\": \"John Doe\", \"date\": \"2026-06-07\", \"card\": \"4532 1234 5678 9012\"}}\nline 3: body chunk {}\nline 4: malformed trailing braces: {{{{\nline 5: amount 150.00€ from 127.0.0.1",
        "payload".repeat(128)
    )
}

#[cfg(unix)]
mod stderr_capture {
    use super::*;

    const STDERR_FD: i32 = 2;

    unsafe extern "C" {
        fn dup(oldfd: i32) -> i32;
        fn dup2(oldfd: i32, newfd: i32) -> i32;
        fn pipe(fds: *mut i32) -> i32;
        fn close(fd: i32) -> i32;
    }

    pub struct Capture {
        saved_fd: RawFd,
        read_fd: RawFd,
        active: bool,
    }

    impl Capture {
        pub fn start() -> io::Result<Self> {
            unsafe {
                let mut fds = [0_i32; 2];
                if pipe(fds.as_mut_ptr()) == -1 {
                    return Err(io::Error::last_os_error());
                }

                let saved_fd = dup(STDERR_FD);
                if saved_fd == -1 {
                    let _ = close(fds[0]);
                    let _ = close(fds[1]);
                    return Err(io::Error::last_os_error());
                }

                if dup2(fds[1], STDERR_FD) == -1 {
                    let err = io::Error::last_os_error();
                    let _ = close(saved_fd);
                    let _ = close(fds[0]);
                    let _ = close(fds[1]);
                    return Err(err);
                }

                let _ = close(fds[1]);

                Ok(Self {
                    saved_fd,
                    read_fd: fds[0],
                    active: true,
                })
            }
        }

        pub fn finish(mut self) -> io::Result<String> {
            self.restore()?;

            let read_fd = std::mem::replace(&mut self.read_fd, -1);
            let mut file = unsafe { std::fs::File::from_raw_fd(read_fd) };
            let mut output = String::new();
            file.read_to_string(&mut output)?;
            Ok(output)
        }

        fn restore(&mut self) -> io::Result<()> {
            if !self.active {
                return Ok(());
            }

            unsafe {
                if dup2(self.saved_fd, STDERR_FD) == -1 {
                    return Err(io::Error::last_os_error());
                }

                let _ = close(self.saved_fd);
            }

            self.active = false;
            Ok(())
        }
    }

    impl Drop for Capture {
        fn drop(&mut self) {
            let _ = self.restore();

            if self.read_fd != -1 {
                unsafe {
                    let _ = close(self.read_fd);
                }
                self.read_fd = -1;
            }
        }
    }
}

#[test]
fn test_panic_response_is_redacted() {
    let mut handler = RedactingPanicHandler;
    let response = handler.response_for_panic(Box::new(synthetic_panic_message().to_owned()));

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let body = rt.block_on(async { to_bytes(response.into_body(), usize::MAX).await.unwrap() });
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body_str, "[REDACTED]");
}

async fn error_handler() -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        synthetic_edge_case_payload(),
    )
}

#[tokio::test(flavor = "current_thread")]
async fn test_500_redaction() {
    let _lock = TEST_LOCK.lock().unwrap();
    let (dispatch, buffer) = make_log_capture();
    let _subscriber = tracing::dispatcher::set_default(&dispatch);

    let app = Router::new()
        .route("/error", get(error_handler))
        .layer(axum::middleware::from_fn(redact_errors_middleware));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/error")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body_str, "[REDACTED]");

    let logs = logs_as_string(&buffer);
    assert!(logs.contains("server error redacted"));
    assert!(logs.contains("[REDACTED]"));
    assert!(!logs.contains("FR76 3000 6000 0112 3456 7890 123"));
    assert!(!logs.contains("John Doe"));
    assert!(!logs.contains("4532 1234 5678 9012"));
    assert!(!logs.contains("150.00€"));
}

#[cfg(unix)]
#[tokio::test(flavor = "current_thread")]
async fn test_panic_path_redacts_stderr_end_to_end() {
    let _lock = TEST_LOCK.lock().unwrap();
    let _panic_hook = glyph::middleware::install_redacting_panic_hook();
    let capture = stderr_capture::Capture::start().unwrap();

    async fn panic_handler() -> &'static str {
        panic!("{}", synthetic_edge_case_payload());
    }

    let app = Router::new()
        .route("/panic", get(panic_handler))
        .layer(axum::middleware::from_fn(redact_errors_middleware))
        .layer(tower_http::catch_panic::CatchPanicLayer::custom(
            RedactingPanicHandler,
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    stream
        .write_all(b"GET /panic HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut raw_response = Vec::new();
    stream.read_to_end(&mut raw_response).await.unwrap();

    server.abort();
    let _ = server.await;
    let stderr = capture.finish().unwrap();

    let response = String::from_utf8(raw_response).unwrap();
    let (headers, body) = response.split_once("\r\n\r\n").unwrap();

    assert!(headers.starts_with("HTTP/1.1 500"), "{headers}");
    assert_eq!(body, "[REDACTED]");
    assert!(stderr.contains("panic redacted at"), "{stderr}");
    assert!(stderr.contains("[REDACTED]"), "{stderr}");
    assert!(
        !stderr.contains("FR76 3000 6000 0112 3456 7890 123"),
        "{stderr}"
    );
    assert!(!stderr.contains("John Doe"), "{stderr}");
    assert!(!stderr.contains("150.00€"), "{stderr}");
}
