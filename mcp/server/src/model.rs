//! Model types for the vim_rs MCP server.
//!
//! This module re-exports types from the `api_database` crate and provides
//! embedded resources (database and guides) that are compiled into the binary.

use anyhow::Result;

// Re-export all types from api_database
pub use api_database::*;

// ============================================================================
// Embedded Resources
// ============================================================================

/// The vim_rs starter guide, embedded at compile time.
pub const STARTER_GUIDE: &str = include_str!("../guides/VIM_RS_STARTER_GUIDE.md");

/// Load the embedded database from the compiled binary.
/// The database file is embedded at compile time using include_bytes!
pub fn load_embedded_database() -> Result<ApiDatabase> {
    const DATA: &[u8] = include_bytes!("../../data/api_database.bin");
    let (database, _) = bincode::serde::decode_from_slice(DATA, bincode::config::standard())
        .map_err(|e| anyhow::anyhow!("Failed to deserialize embedded database: {}", e))?;
    Ok(database)
}
