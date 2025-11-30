pub mod model;
pub mod resolver;
pub mod field_data;
pub mod property_collector;

pub use model::*;

/// The embedding model used for semantic search.
/// Centralized here so both the server and build-embeddings use the same model.
#[cfg(feature = "embeddings")]
pub use fastembed::EmbeddingModel;

#[cfg(feature = "embeddings")]
pub const EMBEDDING_MODEL: EmbeddingModel = EmbeddingModel::BGESmallENV15;