use anyhow::{Context, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};
use vim_mcp_server::model::ApiData;

use arrow_array::{RecordBatch, RecordBatchIterator, StringArray, Float32Array, FixedSizeListArray};
use arrow_schema::{DataType, Field, Schema};
use lancedb::connect;
use lancedb::query::{QueryBase, ExecutableQuery};

// Conditional imports for CUDA GPU acceleration
#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;

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

    // Paths - navigate to mcp/data/
    let mcp_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data");
    let api_definitions_dir = mcp_data_dir.join("api_definitions");
    let embeddings_db_path = mcp_data_dir.join("embeddings.lancedb");
    let model_cache_dir = mcp_data_dir.join("model_cache");

    // Create cache directory if it doesn't exist
    if !model_cache_dir.exists() {
        std::fs::create_dir_all(&model_cache_dir)
            .context("Failed to create model cache directory")?;
    }

    info!("Loading API data from {}", api_definitions_dir.display());
    info!("Loading guides from {}", mcp_data_dir.join("guides").display());
    info!("Using model cache directory: {}", model_cache_dir.display());

    // ApiData::load_from_dir expects the parent data directory, not api_definitions
    // because it looks for data_dir/guides/ subdirectory
    let api_data = ApiData::load_from_dir(&mcp_data_dir)
        .context("Failed to load API data from JSON files")?;

    // Step 1: Create text chunks for embedding
    info!("Creating text chunks for embedding...");
    let mut records = Vec::new();

    // Process managed objects
    for mo in &api_data.managed_objects {
        let text = format!(
            "{} - {}",
            mo.name,
            mo.description.as_deref().unwrap_or("No description")
        );
        records.push(EmbeddingRecord {
            text,
            item_type: "managed_object".to_string(),
            object_name: mo.name.clone(),
            item_name: mo.name.clone(),
            rust_name: mo.rust_struct.clone(),
            rust_module: mo.rust_module.clone(),
        });
    }

    let managed_object_count = api_data.managed_objects.len();

    // Verification: Ensure managed objects were loaded
    if managed_object_count == 0 {
        warn!("⚠️  WARNING: No managed objects loaded! Check that managed_objects.json is in {}", api_definitions_dir.display());
    } else {
        info!("✓ Loaded {} managed objects", managed_object_count);
    }

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

    // Process code examples
    for example in &api_data.examples {
        let text = format!(
            "{} - {} (Category: {}). {}",
            example.name,
            example.title,
            example.category,
            example.description
        );
        records.push(EmbeddingRecord {
            text,
            item_type: "example".to_string(),
            object_name: example.category.clone(),
            item_name: example.name.clone(),
            rust_name: example.title.clone(),
            rust_module: "examples".to_string(),
        });
    }

    // Process guide chunks
    for guide in &api_data.guides {
        // Include headings, topics, and content summary for search
        let topics_str = if guide.topics.is_empty() {
            String::new()
        } else {
            format!(" Topics: {}.", guide.topics.join(", "))
        };

        let sub_section_str = if let Some(ref sub) = guide.sub_section {
            format!(" ({})", sub)
        } else {
            String::new()
        };

        // Truncate content to first 200 chars for embedding text (full content in chunk)
        let content_preview = if guide.content.len() > 200 {
            format!("{}...", &guide.content[..200])
        } else {
            guide.content.clone()
        };

        let text = format!(
            "{} > {} > {}{}.{} {}",
            guide.heading_h1,
            guide.heading_h2,
            guide.heading_h3,
            sub_section_str,
            topics_str,
            content_preview
        );

        records.push(EmbeddingRecord {
            text,
            item_type: "guide".to_string(),
            object_name: guide.source_file.clone(),
            item_name: guide.chunk_id.clone(),
            rust_name: format!("{} > {} > {}", guide.heading_h1, guide.heading_h2, guide.heading_h3),
            rust_module: "guides".to_string(),
        });
    }

    let guide_count = api_data.guides.len();
    info!("Created {} text chunks ({} methods, {} structures, {} enums, {} examples, {} guides)",
        records.len(),
        api_data.managed_objects.iter().map(|mo| mo.methods.len()).sum::<usize>(),
        api_data.data_structures.len(),
        api_data.enumerations.len(),
        api_data.examples.len(),
        guide_count
    );

    // Verification: Ensure guides were loaded
    if guide_count == 0 {
        warn!("⚠️  WARNING: No guide chunks loaded! Check that guides are in {}", mcp_data_dir.join("guides").display());
    } else {
        info!("✓ Loaded {} guide chunks", guide_count);
        
        // Verify guide records were created
        let guide_records_count = records.iter().filter(|r| r.item_type == "guide").count();
        if guide_records_count != guide_count {
            warn!("⚠️  WARNING: Mismatch between loaded guides ({}) and guide records ({})", guide_count, guide_records_count);
        } else {
            info!("✓ Created {} guide embedding records", guide_records_count);
        }
    }

    // Step 2: Initialize embedding model
    info!("Initializing embedding model (all-MiniLM-L6-v2)...");

    // Configure execution providers: CUDA if available, fallback to CPU
    #[cfg(feature = "cuda")]
    let init_options = {
        info!("CUDA feature enabled - using GPU acceleration for embeddings");
        InitOptions::new(EmbeddingModel::AllMiniLML6V2)
            .with_cache_dir(model_cache_dir)
            .with_show_download_progress(true)
            .with_execution_providers(vec![
                CUDAExecutionProvider::default().build()
            ])
    };

    #[cfg(not(feature = "cuda"))]
    let init_options = {
        info!("CUDA feature disabled - using CPU acceleration for embeddings");
        InitOptions::new(EmbeddingModel::AllMiniLML6V2)
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

    // Verification: Test that guides are searchable in the database
    if guide_count > 0 {
        info!("Verifying guide embeddings in database...");
        let table = db.open_table("vim_api")
            .execute()
            .await
            .context("Failed to open embeddings table for verification")?;

        // Search for a guide record to verify it's in the database
        let guide_records: Vec<_> = records.iter()
            .enumerate()
            .filter(|(_, r)| r.item_type == "guide")
            .take(5)
            .collect();

        if !guide_records.is_empty() {
            let (idx, guide_record) = guide_records[0];
            info!("  Testing search for guide: {}", guide_record.item_name);
            
            // Get the embedding for this guide
            let guide_embedding = &embeddings[idx];
            
            // Perform a vector search
            let mut query = table
                .vector_search(guide_embedding.clone())
                .map_err(|e| anyhow::anyhow!("Vector search failed: {}", e))?
                .limit(5);

            query = query.only_if("item_type = 'guide'");
            
            let results = query.execute().await
                .map_err(|e| anyhow::anyhow!("Failed to execute verification search: {}", e))?;

            // Collect results (simplified verification - just check that we can query)
            let mut found_count = 0;
            use futures::stream::TryStreamExt;
            use arrow_array::cast::AsArray;
            let mut results_stream = results;
            while let Some(batch_result) = results_stream.try_next().await
                .map_err(|e| anyhow::anyhow!("Failed to read verification results: {}", e))? {
                let batch = batch_result;
                let item_type_array = batch.column_by_name("item_type").unwrap().as_string::<i32>();
                for i in 0..batch.num_rows() {
                    if item_type_array.value(i) == "guide" {
                        found_count += 1;
                    }
                }
            }

            if found_count > 0 {
                info!("  ✓ Verified: Found {} guide(s) in database search", found_count);
            } else {
                warn!("  ⚠️  WARNING: No guides found in database search - embeddings may not be working correctly");
            }
        }
    }

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
    use arrow_schema::FieldRef;

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
    let values = Arc::new(Float32Array::from(flat_embeddings));

    // Create field for the inner array
    let field: FieldRef = Arc::new(Field::new("item", DataType::Float32, true));
    let vector_array = FixedSizeListArray::try_new(field, 384, values, None)?;

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
