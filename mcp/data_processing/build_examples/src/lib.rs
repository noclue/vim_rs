use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::Path;
use tracing::info;
use walkdir::WalkDir;

// Re-export CodeExample for external use
pub use api_database::CodeExample;

#[derive(Debug, Serialize)]
struct ExamplesOutput {
    examples: Vec<CodeExample>,
}

/// Collect examples from the examples directory and return them in memory (no file I/O).
/// This is the primary function for the unified binary pipeline.
pub fn collect_examples(examples_dir: &Path) -> Result<Vec<CodeExample>> {
    info!("Scanning examples directory: {}", examples_dir.display());

    let mut examples = Vec::new();

    // Scan for all .rs files in examples subdirectories
    for entry in WalkDir::new(examples_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();

        // Skip snippets library root (connection helper only), vtui (large TUI app), build artifacts
        let p = path.to_string_lossy().replace('\\', "/");
        if p.contains("/snippets/src/lib.rs")
            || p.contains("/vtui/")
            || p.contains("/perf-catalog/")
            || p.contains("/target/")
        {
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
        let category = if matches!(
            name.as_str(),
            "retrieve_host_info"
                | "retrieve_ds_hosts"
                | "print_vm_addresses"
                | "retrieve_recent_task"
                | "vm_events"
        ) {
            "macro_usage"
        } else if name.contains("perf") || name.contains("metric") {
            "performance"
        } else if name.contains("property_collector") || source_code.contains("PropertyCollector") {
            "property_collector"
        } else if name.contains("event") {
            "events"
        } else if name == "root_objects" || source_code.contains("ClientBuilder") {
            "connection"
        } else {
            "general"
        };

        let uses_perf = source_code
            .lines()
            .any(|line| line.trim().starts_with("use vim_rs::perf"));
        let vim_rs_dep = if uses_perf {
            "vim_rs = { version = \"0.7\", features = [\"perf\"] }"
        } else {
            "vim_rs = \"0.7\""
        };
        let dependencies_template = format!(
            r#"[dependencies]
{vim_rs_dep}
tokio = {{ version = "1.44", features = ["macros", "rt-multi-thread", "time"] }}
anyhow = "1.0"
log = "0.4"
env_logger = "0.11"

# vim_retrievable! / vim_updatable! are re-exported by vim_rs
# Optional: share connect() via a local library crate (see examples/snippets `lib.rs`)
"#
        );

        let relative_path = path.strip_prefix(examples_dir)
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

    Ok(examples)
}

/// Collect examples and write to JSON file (for debugging).
pub fn build_examples(examples_dir: &Path, output_dir: &Path) -> Result<()> {
    let examples = collect_examples(examples_dir)?;

    fs::create_dir_all(output_dir)?;

    let output_path = output_dir.join("examples.json");
    let output = ExamplesOutput { examples };
    let json = serde_json::to_string_pretty(&output)?;
    fs::write(&output_path, json)?;

    info!("Wrote examples to {}", output_path.display());
    info!("Example indexing complete!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn indexes_examples_without_unindexed_trees() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let previous = std::env::current_dir().unwrap();
        std::env::set_current_dir(&repo).unwrap();
        let examples = collect_examples(Path::new("examples")).unwrap();
        std::env::set_current_dir(previous).unwrap();

        assert!(examples.iter().all(|example| {
            let path = example.file_path.replace('\\', "/");
            !path.contains("perf-catalog/") && !path.contains("vtui/")
        }));

        let by_name: BTreeMap<_, _> = examples
            .iter()
            .map(|example| (example.name.as_str(), example))
            .collect();
        for name in [
            "perf_metrics",
            "perf_metrics_manual",
            "perf_vm_usage",
            "perf_historical",
            "perf_export",
            "perf_host_disks",
            "perf_memory",
        ] {
            assert_eq!(by_name[name].category, "performance", "{name}");
        }
        for name in [
            "perf_metrics",
            "perf_vm_usage",
            "perf_historical",
            "perf_export",
            "perf_host_disks",
            "perf_memory",
        ] {
            assert!(
                by_name[name].dependencies.contains("features = [\"perf\"]"),
                "{name}"
            );
        }
        assert!(!by_name["perf_metrics_manual"]
            .dependencies
            .contains("features = [\"perf\"]"));

        let expected = [
            ("retrieve_host_info", "macro_usage"),
            ("retrieve_ds_hosts", "macro_usage"),
            ("print_vm_addresses", "macro_usage"),
            ("retrieve_recent_task", "macro_usage"),
            ("vm_events", "macro_usage"),
            ("property_collector", "property_collector"),
            ("inventory_path", "property_collector"),
            ("mac_monitor", "property_collector"),
            ("env_browser", "property_collector"),
            ("vm_toggle_wol", "property_collector"),
            ("vm_disabled_method_len", "property_collector"),
            ("eventster", "events"),
            ("root_objects", "connection"),
            ("vm_ip", "general"),
            ("vm_rename", "general"),
            ("dynamic_property_fetch", "general"),
        ];
        assert_eq!(expected.len(), 16);
        for (name, category) in expected {
            assert_eq!(by_name[name].category, category, "{name}");
            assert!(
                !by_name[name].dependencies.contains("features = [\"perf\"]"),
                "{name}"
            );
        }
    }
}
