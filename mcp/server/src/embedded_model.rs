//! Embedded BGE-small-en-v1.5 model for self-contained deployment.
//!
//! This module is only compiled when the `embed-model` feature is enabled.
//! The model files are located by build.rs and embedded at compile time.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use vim_mcp_server::embedded_model::create_embedded_model;
//! use fastembed::TextEmbedding;
//!
//! let model = create_embedded_model();
//! let embedding = TextEmbedding::try_new_from_user_defined(model, Default::default())?;
//! ```

use fastembed::{TokenizerFiles, UserDefinedEmbeddingModel};

/// ONNX model file (~127MB)
pub static ONNX_MODEL: &[u8] = include_bytes!(env!("EMBED_MODEL_ONNX"));

/// Tokenizer vocabulary and configuration (~695KB)  
pub static TOKENIZER: &[u8] = include_bytes!(env!("EMBED_MODEL_TOKENIZER"));

/// Model configuration
pub static CONFIG: &[u8] = include_bytes!(env!("EMBED_MODEL_CONFIG"));

/// Special tokens mapping
pub static SPECIAL_TOKENS: &[u8] = include_bytes!(env!("EMBED_MODEL_SPECIAL_TOKENS"));

/// Tokenizer configuration
pub static TOKENIZER_CONFIG: &[u8] = include_bytes!(env!("EMBED_MODEL_TOKENIZER_CONFIG"));

/// Creates the user-defined embedding model from embedded bytes.
///
/// This returns a `UserDefinedEmbeddingModel` that can be passed to
/// `TextEmbedding::try_new_from_user_defined()`.
pub fn create_embedded_model() -> UserDefinedEmbeddingModel {
    UserDefinedEmbeddingModel::new(
        ONNX_MODEL.to_vec(),
        TokenizerFiles {
            tokenizer_file: TOKENIZER.to_vec(),
            config_file: CONFIG.to_vec(),
            special_tokens_map_file: SPECIAL_TOKENS.to_vec(),
            tokenizer_config_file: TOKENIZER_CONFIG.to_vec(),
        },
    )
}

