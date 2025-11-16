/// Mark headings using exact TOC match

use anyhow::{bail, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use tracing::{info, warn};

use crate::toc_parser::TocEntry;

pub fn mark_headings(
    lines: &[String],
    toc_map: &HashMap<u32, Vec<TocEntry>>,
) -> Result<Vec<String>> {
    let footer_pattern = Regex::new(r"^VMware by Broadcom\s+\d{1,4}$")?;

    let mut output = Vec::new();
    let mut current_page = 1u32;
    let mut found_titles: HashSet<(u32, String)> = HashSet::new();
    let mut toc_entries: Option<&Vec<TocEntry>> = None;

    for line in lines {
        // Track page via footer
        if let Some(caps) = footer_pattern.captures(line) {
            // Extract page number from footer
            let page_str = caps[0].split_whitespace().last().unwrap();
            current_page = page_str.parse()?;
            current_page += 1u32;
            toc_entries = toc_map.get(&current_page);
        }

        // Check if this line matches a TOC title for current page
        let mut matched = false;
        if let Some(entries) = toc_entries {
            for entry in entries {
                if line.trim() == entry.title {
                    // Exact match found!
                    let heading = format!(
                        "{} {}",
                        "#".repeat(entry.markdown_level as usize),
                        entry.title
                    );
                    output.push(heading);
                    found_titles.insert((current_page, entry.title.clone()));
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            output.push(line.clone());
        }
    }

    // Validate: Error if any TOC entry not found
    for (page, entries) in toc_map {
        for entry in entries {
            if !found_titles.contains(&(*page, entry.title.clone())) {
                warn!(
                    "TOC entry '{}' not found on page {} (expected after previous title)",
                    entry.title,
                    page
                );
            }
        }
    }

    info!("✓ All {} TOC entries found and marked", found_titles.len());

    Ok(output)
}
