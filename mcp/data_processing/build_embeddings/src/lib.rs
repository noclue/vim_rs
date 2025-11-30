use anyhow::{Context, Result};
use fastembed::{InitOptions, TextEmbedding};
use std::path::PathBuf;
use tracing::info;
use vim_mcp_server::model::{ApiData, ApiItem, EmbeddingRecord, EmbeddingDatabase};
use vim_mcp_server::EMBEDDING_MODEL;
use std::fs::File;
use std::io::BufWriter;

// Conditional imports for CUDA GPU acceleration
#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;

pub async fn build_embeddings(api_data: &ApiData, embeddings_path: &PathBuf, model_cache_dir: PathBuf) -> Result<()> {
    // Step 1: Create embedding records from unified items index
    info!("Creating embedding records from unified items...");
    
    let records: Vec<EmbeddingRecord> = api_data.items.iter()
        .map(|(id, item)| EmbeddingRecord {
            text: item.embedding_text(),
            item_type: item.item_type().to_string(),
            id: id.clone(),
        })
        .collect();

    // Count items by type for logging
    let managed_object_count = records.iter().filter(|r| r.item_type == "managed_object").count();
    let method_count = records.iter().filter(|r| r.item_type == "method").count();
    let structure_count = records.iter().filter(|r| r.item_type == "structure").count();
    let field_count = records.iter().filter(|r| r.item_type == "field").count();
    let enum_count = records.iter().filter(|r| r.item_type == "enum").count();
    let trait_count = records.iter().filter(|r| r.item_type == "trait").count();
    let example_count = records.iter().filter(|r| r.item_type == "example").count();

    info!("Created {} embedding records:", records.len());
    info!("  - {} managed objects", managed_object_count);
    info!("  - {} methods", method_count);
    info!("  - {} structures", structure_count);
    info!("  - {} fields", field_count);
    info!("  - {} enums", enum_count);
    info!("  - {} traits", trait_count);
    info!("  - {} examples", example_count);

    // Assert the VirtualHardware::device field is in the embeddings
    let virtual_hardware_device_field = records.iter()
        .find(|r| r.id == "VirtualHardware::device");
    if virtual_hardware_device_field.is_none() {
        panic!("VirtualHardware::device field not found in embeddings");
    }
    info!("✓ VirtualHardware::device field found in embeddings");
    
    // Step 2: Initialize embedding model
    info!("Initializing embedding model (all-MiniLM-L6-v2)...");

    // Configure execution providers: CUDA if available, fallback to CPU
    #[cfg(feature = "cuda")]
    let init_options = {
        info!("CUDA feature enabled - using GPU acceleration for embeddings");
        InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(model_cache_dir)
            .with_show_download_progress(true)
            .with_execution_providers(vec![
                CUDAExecutionProvider::default().build()
            ])
    };

    #[cfg(not(feature = "cuda"))]
    let init_options = {
        info!("CUDA feature disabled - using CPU acceleration for embeddings");
        InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(model_cache_dir)
            .with_show_download_progress(true)
    };

    let mut model = TextEmbedding::try_new(init_options)
        .context("Failed to initialize embedding model")?;

    info!("Model initialized successfully");


    // Step 3: Generate embeddings
    info!("Generating embeddings for {} items...", records.len());
    let texts: Vec<String> = records.iter().map(|r| r.text.clone()).collect();

    let embeddings = model
        .embed(texts, None)
        .context("Failed to generate embeddings")?;

    info!("Generated {} embeddings", embeddings.len());

    // Step 4: Store in binary file
    info!("Saving embeddings to {}", embeddings_path.display());

    let database = EmbeddingDatabase {
        records,
        vectors: embeddings,
    };

    let file = File::create(embeddings_path)
        .context("Failed to create embeddings file")?;
    let writer = BufWriter::new(file);

    bincode::serialize_into(writer, &database)
        .context("Failed to serialize embeddings database")?;

    info!("✓ Successfully generated and stored embeddings");
    info!("  File: {}", embeddings_path.display());
    info!("  Records: {}", database.records.len());
    info!("  Dimensions: 384");

    Ok(())
}