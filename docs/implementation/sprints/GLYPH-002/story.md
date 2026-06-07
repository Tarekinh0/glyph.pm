# GLYPH-002: Contrôles négatifs zero-data (anti-régression)

**Status:** DONE
**Stack:** 100% Rust (Axum)
**CI/CD:** GitHub Actions

## Context & Objectives
Glyph is a zero-data infrastructure. Before coding business features, we must establish the "load-bearing walls" that prevent future drift. This sprint creates CI tests and Rust middlewares that physically forbid the addition of databases, tracking, or user logs.

## Acceptance Criteria (DoD)
1. **CI Linter/Scanner (GitHub Actions):** A workflow that scans Rust source code (`Cargo.toml` and `.rs`) and fails if database crates (e.g., `sqlx`, `diesel`, `postgres`) or tracking tools are detected.
2. **Axum Redaction Middleware:** A global Rust/Axum middleware that intercepts `panic!` or HTTP 500 errors. It must guarantee no raw financial data (numbers, dates) leaks into `stderr`. Errors must be replaced by `[REDACTED]`.
3. **Dummy Unit Tests:** Rust tests (`cargo test`) using 100% synthetic data to prove the redaction middleware works (e.g., forcing a panic with a fake IBAN and verifying the output is censored).

## Final verdict
PASS
