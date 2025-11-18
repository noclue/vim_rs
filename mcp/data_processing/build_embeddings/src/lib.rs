use anyhow::{Context, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::path::PathBuf;
use tracing::{info, warn};
use vim_mcp_server::model::{ApiData, EmbeddingRecord, EmbeddingDatabase};
use std::fs::File;
use std::io::BufWriter;

// Conditional imports for CUDA GPU acceleration
#[cfg(feature = "cuda")]
use ort::execution_providers::CUDAExecutionProvider;

pub async fn build_embeddings(api_data: &ApiData, embeddings_path: &PathBuf, model_cache_dir: PathBuf) -> Result<()> {
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
        warn!("⚠️  WARNING: No guide chunks loaded! Check that guides are available.");
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
