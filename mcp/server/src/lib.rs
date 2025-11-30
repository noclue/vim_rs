pub mod model;
pub mod resolver;
pub mod field_data;
pub mod property_collector;

pub use model::*;

/// The embedding model used for semantic search.
pub use fastembed::EmbeddingModel;
pub const EMBEDDING_MODEL: EmbeddingModel = EmbeddingModel::BGESmallENV15;