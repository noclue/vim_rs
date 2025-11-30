use anyhow::{Context, Result};
use fastembed::{InitOptions, TextEmbedding, EmbeddingModel};
use std::path::Path;
use tracing::info;
use api_database::{ApiData, ApiItem};

/// The embedding model used for semantic search.
pub const EMBEDDING_MODEL: EmbeddingModel = EmbeddingModel::BGESmallENV15;

// Conditional imports for CUDA GPU acceleration
#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;

/// Generate embeddings for all items in ApiData.
/// Returns a vector of embedding vectors, aligned with the items in ApiData.
/// (embeddings[i] corresponds to api_data.items[i])
///
/// This function does not perform any file I/O - the caller is responsible
/// for storing the embeddings alongside the ApiData.
pub fn generate_embeddings(api_data: &ApiData, model_cache_dir: &Path) -> Result<Vec<Vec<f32>>> {
    // Step 1: Extract embedding texts from items
    info!("Creating embedding texts from {} items...", api_data.items.len());
    
    let texts: Vec<String> = api_data.items.iter()
        .map(|(_, item)| item.embedding_text())
        .collect();

    // Log item counts by type
    info!("Items by type:");
    info!("  - {} managed objects", api_data.count_by_type("managed_object"));
    info!("  - {} methods", api_data.count_by_type("method"));
    info!("  - {} structures", api_data.count_by_type("structure"));
    info!("  - {} fields", api_data.count_by_type("field"));
    info!("  - {} enums", api_data.count_by_type("enum"));
    info!("  - {} traits", api_data.count_by_type("trait"));
    info!("  - {} examples", api_data.count_by_type("example"));

    // Verify VirtualHardware::device field is present
    if api_data.get("VirtualHardware::device").is_none() {
        panic!("VirtualHardware::device field not found in API data");
    }
    info!("✓ VirtualHardware::device field found");
    
    // Step 2: Initialize embedding model
    info!("Initializing embedding model...");

    // Configure execution providers: CUDA if available, fallback to CPU
    #[cfg(feature = "cuda")]
    let init_options = {
        info!("CUDA feature enabled - using GPU acceleration");
        InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(model_cache_dir.to_path_buf())
            .with_show_download_progress(true)
            .with_execution_providers(vec![
                CUDAExecutionProvider::default().build()
            ])
    };

    #[cfg(not(feature = "cuda"))]
    let init_options = {
        info!("Using CPU for embeddings");
        InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(model_cache_dir.to_path_buf())
            .with_show_download_progress(true)
    };

    let mut model = TextEmbedding::try_new(init_options)
        .context("Failed to initialize embedding model")?;

    info!("Model initialized successfully");

    // Step 3: Generate embeddings
    info!("Generating embeddings for {} items...", texts.len());
    
    let embeddings = model
        .embed(texts, None)
        .context("Failed to generate embeddings")?;

    info!("✓ Generated {} embeddings (384 dimensions each)", embeddings.len());

    Ok(embeddings)
}