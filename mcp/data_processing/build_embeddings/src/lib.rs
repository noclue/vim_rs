use anyhow::{Context, Result};
use fastembed::{InitOptions, TextEmbedding, EmbeddingModel};
use std::path::Path;
use std::time::Instant;
use tracing::info;
use api_database::{ApiData, ApiItem};

/// The embedding model used for semantic search.
pub const EMBEDDING_MODEL: EmbeddingModel = EmbeddingModel::BGESmallENV15;

#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;
#[cfg(feature = "coreml")]
use ort::execution_providers::{CoreMLExecutionProvider, coreml::{ModelFormat, ComputeUnits}};

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

    #[cfg(any(feature = "cuda", feature = "coreml"))]
    let init_options = {
        let mut providers = Vec::new();

        #[cfg(feature = "cuda")]
        {
            info!("CUDA feature enabled - using GPU acceleration");
            providers.push(CUDAExecutionProvider::default().build());
        }

        #[cfg(feature = "coreml")]
        {
            info!("CoreML feature enabled - using Apple Neural Engine acceleration");
            let coreml_cache = model_cache_dir.join("coreml_cache");
            std::fs::create_dir_all(&coreml_cache).ok();
            // MLProgram format for broader op coverage; static input shapes to
            // avoid segfaults from CoreML's dynamic-shape handling in transformer
            // models; CPU+GPU to sidestep ANE tensor shape restrictions.
            providers.push(
                CoreMLExecutionProvider::default()
                    .with_model_format(ModelFormat::MLProgram)
                    .with_static_input_shapes(true)
                    .with_compute_units(ComputeUnits::CPUAndGPU)
                    .with_model_cache_dir(coreml_cache.display().to_string())
                    .build()
            );
        }

        InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(model_cache_dir.to_path_buf())
            .with_show_download_progress(true)
            .with_execution_providers(providers)
    };

    #[cfg(not(any(feature = "cuda", feature = "coreml")))]
    let init_options = {
        info!("Using CPU for embeddings");
        InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(model_cache_dir.to_path_buf())
            .with_show_download_progress(true)
    };

    let mut model = TextEmbedding::try_new(init_options)
        .context("Failed to initialize embedding model")?;

    info!("Model initialized successfully");

    // Step 3: Generate embeddings in batches to bound memory and show progress.
    // CoreML needs smaller batches: the CPU-fallback BFC arena allocates
    // exponentially and OOM-kills the process at batch size 256.
    #[cfg(feature = "coreml")]
    const BATCH_SIZE: usize = 256;
    #[cfg(not(feature = "coreml"))]
    const BATCH_SIZE: usize = 4096;
    let total = texts.len();
    let num_batches = total.div_ceil(BATCH_SIZE);
    info!("Generating embeddings for {total} items in {num_batches} batches (batch size: {BATCH_SIZE})...");

    let start = Instant::now();
    let mut embeddings: Vec<Vec<f32>> = Vec::with_capacity(total);

    for (batch_idx, chunk) in texts.chunks(BATCH_SIZE).enumerate() {
        let batch_start = Instant::now();
        let batch_embeddings = model
            .embed(chunk.to_vec(), None)
            .with_context(|| format!("Failed to generate embeddings for batch {}/{num_batches}", batch_idx + 1))?;

        embeddings.extend(batch_embeddings);

        let done = embeddings.len();
        let pct = (done as f64 / total as f64) * 100.0;
        let elapsed = start.elapsed().as_secs_f64();
        let items_per_sec = done as f64 / elapsed;
        let eta = (total - done) as f64 / items_per_sec;
        info!(
            "  Batch {}/{num_batches}: {} items in {:.1}s  [{done}/{total} {pct:.0}%  {items_per_sec:.0} items/s  ETA {eta:.0}s]",
            batch_idx + 1,
            chunk.len(),
            batch_start.elapsed().as_secs_f64(),
        );
    }

    let elapsed = start.elapsed();
    info!(
        "✓ Generated {} embeddings (384 dimensions each) in {:.1}s ({:.0} items/s)",
        embeddings.len(),
        elapsed.as_secs_f64(),
        embeddings.len() as f64 / elapsed.as_secs_f64(),
    );

    Ok(embeddings)
}