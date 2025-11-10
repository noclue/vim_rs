# build_guides

Chunks vSphere/VCF markdown documentation into 200-800 word semantic sections for embedding.

## Directory Structure

```
mcp/data/
├── guides/
│   ├── md/                          # Input: cleaned markdown files
│   │   ├── vmware-vsphere-9-0.md
│   │   ├── vmware-vcf-9-0.md
│   │   └── vmware-webservices-api.md
│   │
│   └── *.json                       # Output: chunked guide JSON (goes HERE!)
│       ├── vmware-vsphere-9-0.json
│       ├── vmware-vcf-9-0.json
│       └── vmware-webservices-api.json
│
└── api_definitions/                 # NOT for guides! API data only
    ├── managed_objects.json
    ├── data_structures.json
    └── enumerations.json
```

## Pipeline Flow

```
1. Extract PDF text → raw text files
   (pdf_parser or external tool)

2. Clean raw text → markdown
   (text_processor: TOC parsing, header removal, bullets, lists)

3. Chunk markdown → JSON
   (THIS TOOL: build_guides)
   Input:  mcp/data/guides/md/*.md
   Output: mcp/data/guides/*.json

4. Generate embeddings
   (build_embeddings: loads guides from mcp/data/guides/)

5. Run MCP server
   (loads guides via ApiData::load_from_dir())
```

## Usage

```bash
cargo build --release

# Process vSphere guide
./target/release/build_guides \
  ../../data/guides/md/vmware-vsphere-9-0.md \
  ../../data/guides/vmware-vsphere-9-0.json

# Process VCF guide
./target/release/build_guides \
  ../../data/guides/md/vmware-vcf-9-0.md \
  ../../data/guides/vmware-vcf-9-0.json

# Process WebServices API guide
./target/release/build_guides \
  ../../data/guides/md/vmware-webservices-api.md \
  ../../data/guides/vmware-webservices-api.json
```

## Chunking Strategy

- **Primary split:** H3 section boundaries (natural semantic units)
- **Target size:** 200-800 words per chunk
- **Oversized handling:** Split on paragraph breaks at 800 words, hard limit at 1200 words
- **Never split:** Complete lists, tables, Important/Note boxes stay together
- **Filters out:** Empty sections, "Related Links", TOC, legal boilerplate

## Output Format

```json
[
  {
    "heading_h2": "Installing ESX Using vSphere Auto Deploy",
    "heading_h3": "Understanding vSphere Auto Deploy",
    "sub_section": null,  // or "Part 1" if split
    "content": "Auto Deploy provisions ESX hosts...",
    "word_count": 427,
    "source_file": "vmware-vsphere-9-0",
    "chunk_id": "installing-esx-understanding-vsphere-auto-deploy",
    "topics": ["installing", "vsphere", "auto", "deploy", "host", "profiles"]
  },
  ...
]
```

## Common Issues

**Issue:** Output goes to wrong directory (e.g., `api_definitions/`)
**Fix:** Output MUST be `mcp/data/guides/*.json` - MCP server only loads from there

**Issue:** Empty chunks (min=0 words)
**Fix:** v0.1.1+ filters out empty sections automatically

**Issue:** Oversized chunks (max>1200 words)
**Fix:** v0.1.1+ forces splits at 1200-word hard limit even in long lists
