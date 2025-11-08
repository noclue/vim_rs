/// TOC parsing using glyph width clustering

use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;

use crate::glyph_widths::calculate_width;

#[derive(Debug, Clone)]
pub struct TocEntry {
    pub title: String,
    pub page: u32,
    pub markdown_level: u8,
}

/// Parse TOC from first 73 pages of text
pub fn parse_toc(lines: &[String]) -> Result<HashMap<u32, Vec<TocEntry>>> {
    let toc_pattern = Regex::new(r"^(.+?)(\.{3,})(\s*)(\d+)$")?;

    let mut toc_lines = Vec::new();

    // Extract TOC entries (first 73 pages worth of lines)
    // Assuming ~50 lines per page = ~3650 lines
    let toc_limit = lines.len().min(4000);

    for line in &lines[..toc_limit] {
        if let Some(caps) = toc_pattern.captures(line) {
            let title = caps[1].trim().to_string();
            let page: u32 = caps[4].parse()?;

            // Calculate total visual width
            let total_width = calculate_width(line);

            toc_lines.push((title, page, total_width));
        }
    }

    println!("Found {} TOC entries", toc_lines.len());

    // Cluster by width
    let hierarchy_levels = cluster_by_width(&toc_lines)?;

    // Build page -> entries map
    let mut toc_map: HashMap<u32, Vec<TocEntry>> = HashMap::new();

    for ((title, page, _width), level) in toc_lines.iter().zip(hierarchy_levels.iter()) {
        let markdown_level = level + 1; // +1 because # is for doc title

        toc_map.entry(*page).or_insert_with(Vec::new).push(TocEntry {
            title: title.clone(),
            page: *page,
            markdown_level: *markdown_level,
        });
    }

    Ok(toc_map)
}

/// Cluster TOC entries by visual width to determine hierarchy
fn cluster_by_width(entries: &[(String, u32, f32)]) -> Result<Vec<u8>> {
    let tolerance = 15.0;
    let mut width_groups: HashMap<i32, Vec<usize>> = HashMap::new();

    // Group entries by similar widths
    for (idx, (_title, _page, width)) in entries.iter().enumerate() {
        let width_int = *width as i32;

        // Find existing cluster within tolerance
        let mut found_cluster = None;
        for cluster_width in width_groups.keys() {
            if (width_int - cluster_width).abs() <= tolerance as i32 {
                found_cluster = Some(*cluster_width);
                break;
            }
        }

        if let Some(cluster) = found_cluster {
            width_groups.get_mut(&cluster).unwrap().push(idx);
        } else {
            width_groups.insert(width_int, vec![idx]);
        }
    }

    // Sort clusters by width (ascending - narrowest = level 1)
    let mut clusters: Vec<_> = width_groups.into_iter().collect();
    clusters.sort_by_key(|(width, _)| *width);

    println!("Found {} hierarchy levels:", clusters.len());
    for (level, (width, indices)) in clusters.iter().enumerate() {
        println!("  Level {}: width ~{} ({} entries)", level + 1, width, indices.len());
    }

    // Assign levels to each entry
    let mut levels = vec![0u8; entries.len()];
    for (level, (_width, indices)) in clusters.iter().enumerate() {
        for &idx in indices {
            levels[idx] = (level + 1) as u8;
        }
    }

    Ok(levels)
}
