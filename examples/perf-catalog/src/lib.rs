//! Read-only performance catalog dump and query probes.
//!
//! See `docs/vim_rs-perf-poc.md`. These tools do not generate `vim_rs::perf`
//! and they do not modify the server.

pub mod catalog;
pub mod checks;
pub mod discover;
pub mod ledger;
pub mod query;
pub mod session;
