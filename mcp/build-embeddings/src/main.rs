use anyhow::{Context, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};
use vim_mcp_server::model::ApiData;

use arrow_array::{RecordBatch, RecordBatchIterator, StringArray, Float32Array, FixedSizeListArray};
use arrow_schema::{DataType, Field, Schema};
use lancedb::{connect, Connection};

#[derive(Debug, Clone)]
struct EmbeddingRecord {
    text: String,
    item_type: String,
    object_name: String,
    item_name: String,
    rust_name: String,
    rust_module: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting embedding generation for vim_rs MCP server");

    // Paths
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data");
    let embeddings_db_path = data_dir.join("embeddings.lancedb");

    info!("Loading API data from {}", data_dir.display());
    let api_data = ApiData::load_from_dir(&data_dir)
        .context("Failed to load API data from JSON files")?;

    // Step 1: Create text chunks for embedding
    info!("Creating text chunks for embedding...");
    let mut records = Vec::new();

    // Process methods
    for mo in &api_data.managed_objects {
        for method in &mo.methods {
            let text = format!(
                "{}.{} - {}",
                mo.name,
                method.name,
                method.description.as_deref().unwrap_or("No description")
            );
            records.push(EmbeddingRecord {
                text,
                item_type: "method".to_string(),
                object_name: mo.name.clone(),
                item_name: method.name.clone(),
                rust_name: method.rust_name.clone(),
                rust_module: mo.rust_module.clone(),
            });
        }
    }

    // Process data structures
    for structure in &api_data.data_structures {
        let text = format!(
            "{} - {}",
            structure.name,
            structure.description.as_deref().unwrap_or("No description")
        );
        records.push(EmbeddingRecord {
            text,
            item_type: "structure".to_string(),
            object_name: "".to_string(),
            item_name: structure.name.clone(),
            rust_name: structure.rust_name.clone(),
            rust_module: structure.rust_module.clone(),
        });
    }

    // Process enumerations
    for enumeration in &api_data.enumerations {
        // Include variant names in the text for better search
        let variant_names: Vec<&str> = enumeration.variants.iter()
            .map(|v| v.name.as_str())
            .collect();
        let text = format!(
            "{} - {}. Variants: {}",
            enumeration.name,
            enumeration.description.as_deref().unwrap_or("No description"),
            variant_names.join(", ")
        );
        records.push(EmbeddingRecord {
            text,
            item_type: "enum".to_string(),
            object_name: "".to_string(),
            item_name: enumeration.name.clone(),
            rust_name: enumeration.rust_name.clone(),
            rust_module: enumeration.rust_module.clone(),
        });
    }

    info!("Created {} text chunks", records.len());

    // Step 2: Initialize embedding model
    info!("Initializing embedding model (all-MiniLM-L6-v2)...");
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_progress: true,
        ..Default::default()
    })
    .context("Failed to initialize embedding model")?;

    info!("Model initialized successfully");

    // Step 3: Generate embeddings
    info!("Generating embeddings for {} items...", records.len());
    let texts: Vec<String> = records.iter().map(|r| r.text.clone()).collect();

    let embeddings = model
        .embed(texts, None)
        .context("Failed to generate embeddings")?;

    info!("Generated {} embeddings", embeddings.len());

    // Step 4: Store in LanceDB
    info!("Creating LanceDB at {}", embeddings_db_path.display());

    // Remove existing database if it exists
    if embeddings_db_path.exists() {
        warn!("Removing existing embeddings database");
        std::fs::remove_dir_all(&embeddings_db_path)?;
    }

    let db = connect(&embeddings_db_path.to_string_lossy())
        .execute()
        .await
        .context("Failed to connect to LanceDB")?;

    // Prepare data for LanceDB
    let schema = create_schema();
    let batch = create_record_batch(&records, &embeddings, &schema)?;

    // Create table
    info!("Creating 'vim_api' table with {} records", records.len());
    let batches = RecordBatchIterator::new(vec![Ok(batch)], schema);

    db.create_table("vim_api", Box::new(batches))
        .execute()
        .await
        .context("Failed to create LanceDB table")?;

    info!("✓ Successfully generated and stored embeddings");
    info!("  Database: {}", embeddings_db_path.display());
    info!("  Records: {}", records.len());
    info!("  Dimensions: 384");

    Ok(())
}

fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("text", DataType::Utf8, false),
        Field::new("item_type", DataType::Utf8, false),
        Field::new("object_name", DataType::Utf8, false),
        Field::new("item_name", DataType::Utf8, false),
        Field::new("rust_name", DataType::Utf8, false),
        Field::new("rust_module", DataType::Utf8, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                384,
            ),
            false,
        ),
    ]))
}

fn create_record_batch(
    records: &[EmbeddingRecord],
    embeddings: &[Vec<f32>],
    schema: &Arc<Schema>,
) -> Result<RecordBatch> {
    let text_array = StringArray::from(
        records.iter().map(|r| r.text.as_str()).collect::<Vec<_>>()
    );
    let item_type_array = StringArray::from(
        records.iter().map(|r| r.item_type.as_str()).collect::<Vec<_>>()
    );
    let object_name_array = StringArray::from(
        records.iter().map(|r| r.object_name.as_str()).collect::<Vec<_>>()
    );
    let item_name_array = StringArray::from(
        records.iter().map(|r| r.item_name.as_str()).collect::<Vec<_>>()
    );
    let rust_name_array = StringArray::from(
        records.iter().map(|r| r.rust_name.as_str()).collect::<Vec<_>>()
    );
    let rust_module_array = StringArray::from(
        records.iter().map(|r| r.rust_module.as_str()).collect::<Vec<_>>()
    );

    // Flatten embeddings for FixedSizeListArray
    let flat_embeddings: Vec<f32> = embeddings.iter().flat_map(|v| v.iter().copied()).collect();
    let values = Float32Array::from(flat_embeddings);
    let vector_array = FixedSizeListArray::try_new_from_values(values, 384)?;

    RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(text_array),
            Arc::new(item_type_array),
            Arc::new(object_name_array),
            Arc::new(item_name_array),
            Arc::new(rust_name_array),
            Arc::new(rust_module_array),
            Arc::new(vector_array),
        ],
    )
    .context("Failed to create RecordBatch")
}
