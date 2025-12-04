pub mod model;
pub mod resolver;
pub mod field_data;
pub mod property_collector;

#[cfg(feature = "embed-model")]
pub mod embedded_model;

pub use model::*;

/// The embedding model used for semantic search.
pub use fastembed::EmbeddingModel;
pub const EMBEDDING_MODEL: EmbeddingModel = EmbeddingModel::BGESmallENV15;