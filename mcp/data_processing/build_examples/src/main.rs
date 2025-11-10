use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodeExample {
    name: String,
    title: String,
    description: String,
    category: String,
    source_code: String,
    file_path: String,
    dependencies: String,
}

#[derive(Debug, Serialize)]
struct ExamplesOutput {
    examples: Vec<CodeExample>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting example indexing for vim_rs MCP server");

    // Navigate to the examples directory
    let examples_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples");

    if !examples_dir.exists() {
        anyhow::bail!("Examples directory not found at {}", examples_dir.display());
    }

    info!("Scanning examples directory: {}", examples_dir.display());

    let mut examples = Vec::new();

    // Dependency template for new projects
    let dependencies_template = r#"[dependencies]
vim_rs = "0.2"
vim_macros = "0.2"
tokio = { version = "1.44", features = ["macros"] }
anyhow = "1.0"
log = "0.4"
env_logger = "0.11"

# Optional: for utils (connection helper)
# utils = { path = "path/to/utils" }
"#;

    // Scan for all .rs files in examples subdirectories
    for entry in WalkDir::new(&examples_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();

        // Skip the utils lib and vtui (it's a large TUI app)
        if path.to_string_lossy().contains("/utils/")
            || path.to_string_lossy().contains("/vtui/") {
            continue;
        }

        info!("Processing: {}", path.display());

        let source_code = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        // Extract module doc comments (lines starting with //!)
        let doc_lines: Vec<&str> = source_code
            .lines()
            .take_while(|line| line.trim().starts_with("//!") || line.trim().is_empty())
            .filter(|line| line.trim().starts_with("//!"))
            .map(|line| line.trim_start_matches("//!").trim())
            .collect();

        // First line is the title, rest is description
        let title = doc_lines.first()
            .unwrap_or(&"Example")
            .trim_start_matches('#')
            .trim()
            .to_string();

        let description = if doc_lines.len() > 1 {
            doc_lines[1..].join("\n").trim().to_string()
        } else {
            "No description available".to_string()
        };

        // Extract file name as the example name
        let name = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        // Determine category based on path and content
        let category = if path.to_string_lossy().contains("macro_examples") {
            "macro_usage"
        } else if name.contains("property_collector") || source_code.contains("PropertyCollector") {
            "property_collector"
        } else if name.contains("event") {
            "events"
        } else if name.contains("perf") || name.contains("metric") {
            "performance"
        } else if name == "root_objects" || source_code.contains("ClientBuilder") {
            "connection"
        } else {
            "general"
        };

        let relative_path = path.strip_prefix(&examples_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        examples.push(CodeExample {
            name: name.clone(),
            title: if title.is_empty() { name.clone() } else { title },
            description,
            category: category.to_string(),
            source_code,
            file_path: relative_path,
            dependencies: dependencies_template.to_string(),
        });
    }

    info!("Found {} examples", examples.len());

    // Sort by category then name
    examples.sort_by(|a, b| {
        a.category.cmp(&b.category)
            .then(a.name.cmp(&b.name))
    });

    // Write to JSON - output to mcp/data/api_definitions/examples.json
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("api_definitions");

    fs::create_dir_all(&output_dir)?;

    let output_path = output_dir.join("examples.json");
    let output = ExamplesOutput { examples };
    let json = serde_json::to_string_pretty(&output)?;
    fs::write(&output_path, json)?;

    info!("Wrote examples to {}", output_path.display());
    info!("Example indexing complete!");

    Ok(())
}
