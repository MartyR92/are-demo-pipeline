# Changelog

All notable changes to the `are-demo-pipeline` architecture repository will be documented in this file.

## [1.0.2] — 2026-03-05
### Fixed
-   Updated `pyo3` dependency from `0.21` to `0.23` in `Cargo.toml`.
-   Resolved build failure: "the configured Python interpreter version (3.13) is newer than PyO3's maximum supported version (3.12)".
-   Verified `privacy_filter` example functionality with Python 3.13.

## [1.0.1] — 2026-03-05
### Fixed
-   Standardized all diagram extensions to `.png` for consistency with the specification.
-   Added a structural placeholder for the missing `pipeline-flow.png`.
-   Updated `ARCHITECTURE.md` to include both `system-overview.png` and `pipeline-flow.png`.
-   Verified all internal links and English-first language policy.

## [1.0.0] — 2025-Q4
### Initial Release
-   **Architecture:** Core Documentation (README, ARCHITECTURE, CASE_STUDY).
-   **GDPR:** Defined the "Compile-Time GDPR enforcement" mechanism using Rust traits.
-   **ADR:** Accepted ADR 001 for Rust-First Architecture with isolated Python bridge.
-   **Examples:** Added `privacy_filter.rs` (Rust), `context_builder.py` (Python), and `schema.sql` (SQL).
-   **CI:** Added GitHub Action for Rust `clippy` and linting.
-   **Flows:** Defined step-by-step `compliance-flow.md` and `gdpr-architecture.md`.
