# Technology Stack

## Language: Rust (100%)

### Decision
Implement the entire MCP server in Rust, including embedding generation at runtime.

### Rationale
| Aspect | Rust | Python Alternative |
|--------|------|-------------------|
| Distribution | Single static binary | Requires Python runtime + dependencies |
| Startup Time | <10ms | 100-300ms |
| Memory | ~50-200MB | ~200-500MB+ |
| Performance | Native speed | Interpreted |
| Integration | Natural with vim_rs | Foreign function calls |
| Cross-platform | Easy cross-compilation | Virtual env complexity |
| Dependencies | Zero runtime deps | pip, venv, system Python |

### Implementation
```rust
// Stack overview
├── MCP Protocol: Official Rust MCP SDK
├── Vector Search: Qdrant or LanceDB
├── Text Search: Tantivy
├── Embeddings: ort (ONNX Runtime) or candle
├── PDF Processing: pdf-extract
├── OpenAPI: serde_json
├── vCenter: vim_rs
└── HTTP: reqwest
```

## MCP Protocol Implementation

### Technology: Official Rust MCP SDK

**Repository:** https://github.com/modelcontextprotocol/rust-sdk

### Why Official SDK
- ✅ Handles JSON-RPC over stdio automatically
- ✅ Type-safe tool definitions
- ✅ Standard error handling
- ✅ Future protocol updates
- ✅ Official support

### Usage Pattern
```rust
use mcp_server::{Server, Tool};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new("vim-rs-helper", "1.0.0");

    server.add_tool(
        "search_vim_apis",
        "Search vSphere APIs",
        schema,
        handler
    );

    server.run_stdio().await
}
```

## Vector Database

### Primary Option: LanceDB

**Why LanceDB:**
- Written in Rust
- Embedded mode (no server)
- Designed for ML workloads
- Handles vectors + metadata together
- Good performance at our scale (50k vectors)
- Active development

**Usage:**
```rust
use lancedb::{Connection, Table};

let db = Connection::open("vim_api_vectors").await?;
let table = db.open_table("apis").await?;

// Search
let results = table
    .vector_search(query_vector)
    .limit(10)
    .execute()
    .await?;
```

### Alternative: Qdrant (Client Library)

**Why Qdrant:**
- Rust-native
- Production-grade vector search
- HNSW algorithm (fast)
- Can run in-process
- Excellent filtering capabilities

**Usage:**
```rust
use qdrant_client::prelude::*;

let client = QdrantClient::from_url("file://./vectors").build()?;
let results = client.search_points(&SearchPoints {
    collection_name: "vim_apis".to_string(),
    vector: query_vector,
    limit: 10,
    ..Default::default()
}).await?;
```

### Decision Criteria
- **LanceDB**: Simpler integration, single dependency
- **Qdrant**: More features, battle-tested, better for >100k vectors

**Recommendation:** Start with LanceDB, migrate to Qdrant if needed

## Full-Text Search

### Technology: Tantivy

**Why Tantivy:**
- Pure Rust, fast, mature
- Lucene-like capabilities
- Excellent for technical documentation
- Supports structured queries
- Used in production (Quickwit, etc.)

**Usage:**
```rust
use tantivy::{Index, IndexWriter, doc, schema::*};

// Build index
let mut schema_builder = Schema::builder();
schema_builder.add_text_field("api_name", TEXT | STORED);
schema_builder.add_text_field("description", TEXT);
let schema = schema_builder.build();

let index = Index::create_in_dir("text_index", schema)?;

// Search
let searcher = index.reader()?.searcher();
let results = searcher.search(&query, &TopDocs::with_limit(10))?;
```

**Features we use:**
- Full-text search on API names and descriptions
- Phrase queries ("create snapshot")
- Boolean queries (AND, OR, NOT)
- Fuzzy matching for typos
- Field-specific search

## Embedding Models

### Runtime: ONNX Runtime (ort crate)

**Model:** sentence-transformers/all-MiniLM-L6-v2
- Size: ~90 MB (ONNX format)
- Dimensions: 384
- Quality: Excellent for technical text
- Speed: 10-20ms per query on CPU

**Why ONNX:**
- Mature, well-tested
- Easy model conversion from PyTorch/TF
- Fast inference on CPU
- Rust bindings (ort crate)

**Usage:**
```rust
use ort::{Session, Value};

// Load model (shipped with MCP)
let session = Session::builder()?
    .with_model_from_file("models/minilm-l6-v2.onnx")?;

// Embed text
let tokens = tokenizer.encode(text)?;
let input = Value::from_array(session.allocator(), &tokens)?;
let outputs = session.run(vec![input])?;
let embeddings = outputs[0].extract_tensor::<f32>()?;
```

### Alternative: Candle

**Why Candle:**
- Pure Rust implementation
- Hugging Face models
- No C++ dependencies
- Emerging but promising

**Current Status:**
- ⚠️ Newer, less mature than ONNX
- ✅ Native Rust, easier to embed
- ✅ Growing model support

**Decision:** ONNX for v1 (stable), consider Candle for v2

### Build Time Embedding

For building the index, can use any tool:
- Python + sentence-transformers
- OpenAI API (text-embedding-3-small)
- Local ONNX/Candle

**Recommendation:** Python + sentence-transformers (fastest to implement)

## PDF Processing

### Technology: pdf-extract crate

**Why pdf-extract:**
- Pure Rust
- Good text extraction quality
- Simple API
- No system dependencies

**Usage:**
```rust
use pdf_extract::extract_text;

let text = extract_text("vcf-9-0.pdf")?;
```

### Alternative: poppler-rs

**If text quality is poor:**
- More robust extraction
- Better handling of complex PDFs
- Requires libpoppler system dependency

**Decision:** Start with pdf-extract, switch to poppler if needed

## Data Storage

### Vector DB: LanceDB files
- Format: Apache Arrow / Parquet
- Location: `data/vector_index/`
- Size: ~100-150 MB

### Text Index: Tantivy files
- Format: Tantivy segment files
- Location: `data/text_index/`
- Size: ~20-50 MB

### Metadata: SQLite or JSON

**Option 1: SQLite**
```rust
use rusqlite::{Connection, params};

let conn = Connection::open("data/metadata.db")?;
```

**Option 2: JSON files**
```rust
use serde_json;

let metadata: HashMap<String, ApiMetadata> =
    serde_json::from_str(&fs::read_to_string("data/metadata.json")?)?;
```

**Decision:** JSON for v1 (simpler), SQLite if we need complex queries

## HTTP Client (Optional vCenter Connection)

### Technology: reqwest

Already used by vim_rs, proven integration.

```rust
use reqwest::Client;
use vim_rs::ClientBuilder;

let client = ClientBuilder::new("https://vcenter.example.com")
    .basic_authn(user, pass)
    .insecure(true)
    .build()
    .await?;
```

## Async Runtime

### Technology: tokio

Standard choice for async Rust, required by vim_rs.

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // MCP server runs on tokio runtime
}
```

## Distribution

### Build System: cargo

Standard Rust toolchain.

### Cross-Compilation: cross

```bash
# Install cross
cargo install cross

# Build for all platforms
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

### CI/CD: GitHub Actions

```yaml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-gnu
          - aarch64-unknown-linux-gnu
          - x86_64-apple-darwin
          - aarch64-apple-darwin
          - x86_64-pc-windows-msvc
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cross build --release --target ${{ matrix.target }}
      - name: Package
        run: |
          tar czf vim-rs-mcp-${{ matrix.target }}.tar.gz \
            target/${{ matrix.target }}/release/vim-rs-mcp \
            data/
```

### Package Format

```
vim-rs-mcp-v1.0.0-linux-x86_64.tar.gz
├── vim-rs-mcp                    (binary)
├── data/
│   ├── vector_index/             (LanceDB files)
│   ├── text_index/               (Tantivy files)
│   ├── metadata.json             (API metadata)
│   └── models/
│       └── minilm-l6-v2.onnx    (embedding model)
└── README.md
```

## Dependencies Summary

### Core Dependencies
```toml
[dependencies]
# MCP Protocol
mcp-server = "0.1"  # Official Rust SDK

# Vector Search
lancedb = "0.6"     # or qdrant-client = "1.7"

# Text Search
tantivy = "0.21"

# Embeddings
ort = "1.16"        # ONNX Runtime

# PDF Processing
pdf-extract = "0.7"

# vim_rs Integration
vim_rs = { path = "../vim_rs" }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async
tokio = { version = "1.35", features = ["full"] }

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# HTTP (for vCenter)
reqwest = { version = "0.11", features = ["json"] }
```

### Build Dependencies
```toml
[build-dependencies]
# None needed - data built separately
```

### Dev Dependencies
```toml
[dev-dependencies]
criterion = "0.5"  # Benchmarking
```

## Development Tools

### Testing
- `cargo test` - Unit and integration tests
- `cargo bench` - Performance benchmarks

### Profiling
- `cargo flamegraph` - CPU profiling
- `valgrind` - Memory profiling

### Documentation
- `cargo doc` - Generate API docs
- `mdbook` - User documentation

## Platform Support

### Primary Targets
- ✅ x86_64-unknown-linux-gnu (Linux 64-bit Intel/AMD)
- ✅ aarch64-unknown-linux-gnu (Linux 64-bit ARM)
- ✅ x86_64-apple-darwin (macOS Intel)
- ✅ aarch64-apple-darwin (macOS Apple Silicon)
- ✅ x86_64-pc-windows-msvc (Windows 64-bit)

### Minimum Versions
- Rust: 1.75+ (for ONNX Runtime)
- Linux: kernel 3.10+ (RHEL 7, Ubuntu 18.04+)
- macOS: 10.15+ (Catalina)
- Windows: 10+

## Size Estimates

### Binary
- Debug: ~50-100 MB
- Release: ~10-20 MB (with LTO, strip)

### Data
- Vector index: ~100-150 MB
- Text index: ~20-50 MB
- Metadata: ~5-10 MB
- ONNX model: ~90 MB
- **Total: ~220-320 MB**

### Compressed Distribution
- tar.gz: ~100-150 MB per platform

## Performance Targets

### Startup Time
- Cold start: <100ms
- Load indexes: <50ms
- Initialize ONNX: <50ms

### Query Performance
- Vector search: <20ms
- Text search: <10ms
- Hybrid search: <30ms
- Total latency: <50ms (99th percentile)

### Resource Usage
- Memory (idle): <100 MB
- Memory (active): <500 MB
- CPU (idle): <1%
- CPU (searching): <20% per core

## Why Not Python?

For comparison, Python stack would be:

```
Language:       Python 3.11+
MCP:            Official Python SDK
Vector DB:      ChromaDB or Qdrant
Text Search:    Whoosh or Elasticsearch
Embeddings:     sentence-transformers
PDF:            pypdf2 or pdfplumber

Downsides:
- Requires Python runtime (100-500 MB)
- Slower startup (100-300ms)
- Dependency hell (pip, venv)
- Distribution complexity
- Larger memory footprint
```

**Rust is objectively better for this use case.**

## Technology Maturity Assessment

| Component | Maturity | Risk | Mitigation |
|-----------|----------|------|------------|
| Rust MCP SDK | New (2024) | Medium | Official, active development |
| LanceDB | Young (2023) | Low-Medium | Stable API, good support |
| Tantivy | Mature (2018) | Low | Battle-tested, stable |
| ONNX Runtime | Mature (2018) | Low | Industry standard |
| pdf-extract | Mature | Low | Simple, works well |
| vim_rs | In-house | Low | Full control |

**Overall Risk: Low** - Mostly mature technologies, acceptable risk for new components.
