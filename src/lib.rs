//! Rust client for Waldur MasterMind.
//!
//! All types and client methods live under [`generated`], produced by
//! `openapi-to-rust` from the Waldur OpenAPI schema (see
//! `openapi-to-rust.toml` for the exact operation surface). Do not hand-edit
//! that module; re-run generation instead.

mod generated;
pub use generated::*;
