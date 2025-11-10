/// TOC parsing using glyph width clustering

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

use crate::glyph_widths::calculate_width;

#[derive(Debug, Clone)]
pub struct TocEntry {
    pub title: String,
    pub page: u32,
    pub total_width: f32,
    pub markdown_level: u8,
}

/// Parse TOC and export directly to markdown format
pub fn parse_and_export_toc(lines: &[String]) -> Result<String> {
    let toc_map = parse_toc(lines)?;
        

    println!("Found {} TOC entries", toc_map.len());

    Ok(toc_to_markdown(&toc_map))
}

/// Convert TOC map to markdown format
fn toc_to_markdown(toc_map: &HashMap<u32, Vec<TocEntry>>) -> String {
    // Sort entries by total width ascending
    let mut entries: Vec<&TocEntry> = toc_map.values().flatten().collect();
    entries.sort_by_key(|e| e.page);

    let mut output = String::from("# Table of Contents\n\n");
    for entry in entries {
        let indent = "    ".repeat((entry.markdown_level - 1) as usize);
        output.push_str(&format!("{}* {} (page {}, width {:.1})\n", 
                                indent, entry.title, entry.page, entry.total_width));
    }
    output
}

/// Parse TOC from first 73 pages of text
pub fn parse_toc(lines: &[String]) -> Result<HashMap<u32, Vec<TocEntry>>> {
    let toc_pattern = Regex::new(r"^(.+?)(\.{3,})(\s*)(\d+)$")?;

    // Build page -> entries map
    let mut toc_map: HashMap<u32, Vec<TocEntry>> = HashMap::new();

    // Extract TOC entries (first 73 pages worth of lines)
    // Assuming ~100 lines per page = ~7300 lines
    let toc_limit = lines.len().min(7300);

    for line in &lines[..toc_limit] {
        if let Some(caps) = toc_pattern.captures(line) {
            let title = caps[1].trim().to_string();
            let page: u32 = caps[4].parse()?;

            // Calculate total visual width
            let total_width = calculate_width(line);

            //toc_lines.push((title, page, total_width));
            toc_map.entry(page).or_insert_with(Vec::new).push(TocEntry {
                title: title,
                page: page,
                markdown_level: level_from_width(total_width),
                total_width,
            });
        }
    }

    println!("Found {} TOC entries", toc_map.len());

    Ok(toc_map)
}

/// Determine hierarchy level based on visual width
/// No clustering needed - direct mapping from width to level:
/// - Level 1: width < 740 (large font titles)
/// - Level 2: width > 795 (small font - merges visual levels 2&3)
/// - Level 3: width 741-794 (all indented entries)
fn level_from_width(width: f32) -> u8 {
    if width < 740.0 {
        1
    } else if width > 795.0 {
        2
    } else {
        3
    }
}