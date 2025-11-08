# Text Processor for vSphere Documentation

Converts raw PDF-extracted text into clean markdown with proper heading hierarchy.

## Features

**5-pass processing pipeline:**
1. **TOC Parsing** - Extract hierarchy using Arial glyph width clustering
2. **Heading Marking** - Exact-match TOC titles and convert to markdown headings
3. **Page Marker Removal** - Strip headers, footers, and URLs
4. **Bullet Fixing** - Convert `•` and `–` to markdown bullets
5. **List Merging** - Join broken list items across lines

## Usage

```bash
# Basic usage (output to input_clean.md)
cargo run --release -- ../guides/vsphere.txt

# Custom output path
cargo run --release -- ../guides/vsphere.txt ../guides/vsphere_clean.md
```

## Algorithm

### TOC Hierarchy Detection

Uses **glyph advance widths** from Arial font to cluster TOC entries:

```
Level 1 (top):    width ~711  (stddev 3.41)
Level 2:          width ~776  (stddev 3.34)
Level 3:          width ~803  (stddev 0.71)
Level 4:          width ~811  (stddev 4.53)
Level 5 (nested): width ~838  (stddev 1.92)
```

Markdown levels are TOC level + 1 (since `#` is reserved for doc title).

### Heading Injection

- Exact title match required (errors if missing)
- Validates all TOC entries found
- No fuzzy matching to ensure accuracy

### Page Marker Patterns

**Footer:**
```
VMware by Broadcom  {page_number}
```

**Header:**
```
 VMware vSphere 9.0
 VMware Cloud Foundation 9.0
```

**URLs (skipped):**
```
https://techdocs.broadcom.com/...
```

### Bullet Conversion

```
• First level  →  * First level
– Second level →      * Second level
```

**Note:** Em-dashes in text (like `local – The first...`) are preserved.

## Output

Clean markdown with:
- ✅ Proper heading hierarchy (`##` through `######`)
- ✅ No page markers or footers
- ✅ Markdown-style bullets
- ✅ Merged list items (no line breaks mid-item)
- ✅ Validated against TOC (errors if mismatch)

## Testing

```bash
# Run unit tests
cargo test

# Process sample file
cargo run -- ../guides/vsphere.txt
```

## Architecture

```
src/
├── main.rs              # CLI orchestrator
├── glyph_widths.rs      # Arial font metrics
├── toc_parser.rs        # Width-based TOC parsing
├── heading_marker.rs    # Exact-match heading injection
├── cleanup.rs           # Header/footer removal
├── bullet_fixer.rs      # Bullet conversion
└── list_merger.rs       # List item merging
```
