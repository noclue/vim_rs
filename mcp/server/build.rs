//! Build script for vim_mcp_server
//!
//! This script performs two main functions:
//!
//! 1. **Database validation** (always runs):
//!    - Checks if `api_database.bin` exists
//!    - Checks if source files (OpenAPI spec, examples) are newer than the database
//!    - Fails with instructions if database is missing or stale
//!
//! 2. **Model embedding** (when `embed-model` feature is enabled):
//!    - Locates or downloads the BGE-small-en-v1.5 model
//!    - Sets environment variables for include_bytes!

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mcp_root = Path::new(&manifest_dir).parent().unwrap();
    let workspace_root = mcp_root.parent().unwrap();

    // Always validate the API database
    validate_api_database(mcp_root, workspace_root);

    // Embed model when feature is enabled
    if env::var("CARGO_FEATURE_EMBED_MODEL").is_ok() {
        embed_model(mcp_root);
    }
}

// ============================================================================
// API Database Validation
// ============================================================================

fn validate_api_database(mcp_root: &Path, workspace_root: &Path) {
    let database_path = mcp_root.join("data/api_database.bin");
    
    // Check if database exists
    if !database_path.exists() {
        panic!(
            "\n\n\
            ╔══════════════════════════════════════════════════════════════════════╗\n\
            ║  ERROR: api_database.bin not found!                                  ║\n\
            ╠══════════════════════════════════════════════════════════════════════╣\n\
            ║                                                                      ║\n\
            ║  The API database must be built before compiling the server.         ║\n\
            ║                                                                      ║\n\
            ║  Run the data transformer:                                           ║\n\
            ║                                                                      ║\n\
            ║    cargo run -p data-transformer --release                           ║\n\
            ║                                                                      ║\n\
            ║  Expected path: {:<43} ║\n\
            ╚══════════════════════════════════════════════════════════════════════╝\n\n",
            database_path.display()
        );
    }

    // Get database modification time
    let db_modified = fs::metadata(&database_path)
        .and_then(|m| m.modified())
        .expect("Failed to get database modification time");

    // Track sources and check for staleness
    let mut stale_sources: Vec<String> = Vec::new();

    // Check OpenAPI spec
    let openapi_spec = workspace_root.join("vim_build/data/vi_json_openapi_specification_v9_0_0_0_24798170.json");
    if openapi_spec.exists() {
        println!("cargo:rerun-if-changed={}", openapi_spec.display());
        if is_newer_than(&openapi_spec, db_modified) {
            stale_sources.push(format!("  • {}", openapi_spec.display()));
        }
    }

    // Check example source files (excluding target directories and build artifacts)
    let examples_dir = workspace_root.join("examples");
    if examples_dir.exists() {
        for source in find_rust_sources(&examples_dir) {
            println!("cargo:rerun-if-changed={}", source.display());
            if is_newer_than(&source, db_modified) {
                stale_sources.push(format!("  • {}", source.display()));
            }
        }
    }

    // Also rerun if database itself changes
    println!("cargo:rerun-if-changed={}", database_path.display());

    // Report staleness
    if !stale_sources.is_empty() {
        // Limit display to first 10 files
        let display_sources: Vec<_> = stale_sources.iter().take(10).collect();
        let remaining = stale_sources.len().saturating_sub(10);
        let more_msg = if remaining > 0 {
            format!("  ... and {} more files\n", remaining)
        } else {
            String::new()
        };

        panic!(
            "\n\n\
            ╔══════════════════════════════════════════════════════════════════════╗\n\
            ║  ERROR: api_database.bin is out of date!                             ║\n\
            ╠══════════════════════════════════════════════════════════════════════╣\n\
            ║                                                                      ║\n\
            ║  The following source files have been modified:                      ║\n\
            ║                                                                      ║\n\
{}\
{}\
            ║                                                                      ║\n\
            ║  Rebuild the database:                                               ║\n\
            ║                                                                      ║\n\
            ║    cargo run -p data-transformer --release                           ║\n\
            ║                                                                      ║\n\
            ╚══════════════════════════════════════════════════════════════════════╝\n\n",
            display_sources.iter().map(|s| format!("║  {:<66} ║\n", s)).collect::<String>(),
            if !more_msg.is_empty() { format!("║  {:<66} ║\n", more_msg.trim()) } else { String::new() }
        );
    }

    eprintln!("✓ api_database.bin is up to date");
}

/// Check if a file is newer than the reference time
fn is_newer_than(path: &Path, reference: SystemTime) -> bool {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|t| t > reference)
        .unwrap_or(false)
}

/// Find all Rust source files, excluding target directories and build artifacts
fn find_rust_sources(dir: &Path) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    find_rust_sources_recursive(dir, &mut sources);
    sources
}

fn find_rust_sources_recursive(dir: &Path, sources: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip build artifacts and hidden directories
        if file_name == "target" 
            || file_name == ".git"
            || file_name.starts_with('.')
            || file_name == "Cargo.lock"
        {
            continue;
        }

        if path.is_dir() {
            find_rust_sources_recursive(&path, sources);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            sources.push(path);
        }
    }
}

// ============================================================================
// Model Embedding
// ============================================================================

fn embed_model(mcp_root: &Path) {
    let model_cache = mcp_root.join("data/model_cache");
    
    // Ensure cache directory exists
    if !model_cache.exists() {
        fs::create_dir_all(&model_cache).expect("Failed to create model cache directory");
    }
    
    // Find or download the BGE model
    let model_dir = model_cache.join("models--Xenova--bge-small-en-v1.5");
    
    if !model_dir.exists() {
        download_model(&model_cache);
    }
    
    // Find the snapshots directory and get the snapshot
    let snapshots_dir = model_dir.join("snapshots");
    
    // If snapshots don't exist, the download may have failed or be incomplete
    if !snapshots_dir.exists() {
        download_model(&model_cache);
    }
    
    let snapshot_dir = find_snapshot_dir(&snapshots_dir);
    
    // Verify all required files exist
    let onnx_file = snapshot_dir.join("onnx/model.onnx");
    let tokenizer_file = snapshot_dir.join("tokenizer.json");
    let config_file = snapshot_dir.join("config.json");
    let special_tokens_file = snapshot_dir.join("special_tokens_map.json");
    let tokenizer_config_file = snapshot_dir.join("tokenizer_config.json");
    
    for (name, path) in [
        ("ONNX model", &onnx_file),
        ("tokenizer.json", &tokenizer_file),
        ("config.json", &config_file),
        ("special_tokens_map.json", &special_tokens_file),
        ("tokenizer_config.json", &tokenizer_config_file),
    ] {
        if !path.exists() {
            panic!("Missing required model file: {} at {}", name, path.display());
        }
    }
    
    // Output the paths as environment variables for the main build
    println!("cargo:rustc-env=EMBED_MODEL_ONNX={}", onnx_file.display());
    println!("cargo:rustc-env=EMBED_MODEL_TOKENIZER={}", tokenizer_file.display());
    println!("cargo:rustc-env=EMBED_MODEL_CONFIG={}", config_file.display());
    println!("cargo:rustc-env=EMBED_MODEL_SPECIAL_TOKENS={}", special_tokens_file.display());
    println!("cargo:rustc-env=EMBED_MODEL_TOKENIZER_CONFIG={}", tokenizer_config_file.display());
    
    // Re-run if the model cache changes
    println!("cargo:rerun-if-changed={}", model_dir.display());
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_EMBED_MODEL");
    
    eprintln!("✓ Embedding model from: {}", snapshot_dir.display());
}

fn download_model(cache_dir: &Path) {
    use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
    
    eprintln!("⬇ Downloading BGE-small-en-v1.5 model to cache...");
    eprintln!("  Cache directory: {}", cache_dir.display());
    
    // Create TextEmbedding which triggers the download
    let init_options = InitOptions::new(EmbeddingModel::BGESmallENV15)
        .with_cache_dir(cache_dir.to_path_buf())
        .with_show_download_progress(true);
    
    match TextEmbedding::try_new(init_options) {
        Ok(_) => {
            eprintln!("✓ Model downloaded successfully");
        }
        Err(e) => {
            panic!(
                "\n\n\
                ╔══════════════════════════════════════════════════════════════════╗\n\
                ║  ERROR: Failed to download embedding model!                       ║\n\
                ╠══════════════════════════════════════════════════════════════════╣\n\
                ║  {}  \n\
                ║                                                                   ║\n\
                ║  Please check your internet connection and try again.            ║\n\
                ╚══════════════════════════════════════════════════════════════════╝\n\n",
                e
            );
        }
    }
}

fn find_snapshot_dir(snapshots_dir: &Path) -> PathBuf {
    if !snapshots_dir.exists() {
        panic!("Snapshots directory not found: {}", snapshots_dir.display());
    }
    
    let entries: Vec<_> = fs::read_dir(snapshots_dir)
        .expect("Failed to read snapshots directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    
    if entries.is_empty() {
        panic!("No snapshot found in: {}", snapshots_dir.display());
    }
    
    if entries.len() > 1 {
        eprintln!(
            "Warning: Multiple snapshots found, using first: {}",
            entries[0].path().display()
        );
    }
    
    entries[0].path()
}
