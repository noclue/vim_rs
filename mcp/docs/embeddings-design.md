# Embeddings Design

## Overview

Semantic search over vSphere API using vector embeddings. This enables natural language queries like "how do I power on a VM?" instead of keyword searches.

## Architecture

### Two-Stage Pipeline

```
vim_build → JSON files → build-embeddings → Vector DB → MCP Server
```

1. **Stage 1**: `vim_build` generates JSON files (existing, v0.2.0)
2. **Stage 2**: `build-embeddings` processes JSON into vector database (new, v0.3.0)

### Why Separate Tools?

- **Clean separation**: Code generation vs. search optimization
- **Different dependencies**: vim_build stays lean, embeddings adds ML deps
- **Iteration speed**: Can regenerate embeddings without rebuilding code
- **Optional**: Text search works without embeddings

## Components

### Embedding Model

**Model**: `all-MiniLM-L6-v2` via `fastembed-rs`
- **Dimensions**: 384
- **Size**: ~25MB
- **Quality**: Good for code/API documentation
- **Speed**: Fast inference, pure Rust
- **Crate**: `fastembed = "5.2.0"`

### Vector Database

**Database**: LanceDB
- **Type**: Embedded (no server needed)
- **Storage**: Single file/directory
- **Features**: Fast ANN search, metadata filtering
- **Deployment**: Ships with MCP server binary
- **Crate**: `lancedb = "0.22.3"`

## What Gets Embedded

### Text Chunks

Each API item becomes a searchable embedding:

**Methods** (~2,195 embeddings):
```
"VirtualMachine.PowerOnVM - Powers on this virtual machine. If the virtual machine is suspended, this method resumes execution from the suspend point."
```

**Data Structures** (~3,890 embeddings):
```
"VirtualMachineConfigInfo - Configuration settings for a virtual machine. Contains all configuration data including hardware, software, and resource settings."
```

**Enumerations** (~623 embeddings):
```
"VirtualMachinePowerState - Defines the power state of a virtual machine: poweredOn, poweredOff, suspended"
```

**Total**: ~6,708 embeddings

### Metadata

Each embedding stores metadata for retrieval:
```rust
{
    "item_type": "method" | "structure" | "enum",
    "object_name": "VirtualMachine",      // for methods
    "item_name": "PowerOnVM",
    "rust_name": "power_on_vm",
    "rust_module": "vim_rs::mo::VirtualMachine",
}
```

## Implementation

### Build Tool: `mcp/build-embeddings`

```
mcp/
├── build-embeddings/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── data/
    ├── managed_objects.json      (input)
    ├── data_structures.json      (input)
    ├── enumerations.json         (input)
    └── embeddings.lancedb/       (output)
```

### Workflow

```bash
# Generate JSON (if needed)
cd vim_build && cargo run --release

# Generate embeddings
cd mcp/build-embeddings && cargo run --release
```

### MCP Server Integration

- Load LanceDB at startup (optional, falls back to text search)
- New tool: `semantic_search` with type filter
- Query embedding → vector similarity search → top-k results
- Format results same as text search tools

## Search Tools Design

### Unified Tool with Filter

```rust
semantic_search(query: String, limit: usize, filter: Option<String>)
```

**Filter values**:
- `"all"` (default): Search everything
- `"methods"`: Only search methods
- `"types"`: Only search data structures
- `"enums"`: Only search enumerations

**Example queries**:
- "how do I power on a VM?" → finds `VirtualMachine.PowerOnVM`
- "virtual machine configuration" → finds `VirtualMachineConfigInfo`
- "VM power states" + filter=enums → finds `VirtualMachinePowerState`

### Search Strategy

1. **Semantic search**: Use for natural language queries
2. **Text search**: Use for exact name matching
3. **Hybrid** (future v0.4.0): Combine both with re-ranking

## Data Flow

```
User Query: "how to clone a VM?"
    ↓
Embed query with all-MiniLM-L6-v2 → [0.123, -0.456, ...]
    ↓
LanceDB vector similarity search (cosine distance)
    ↓
Top 10 most similar embeddings
    ↓
Retrieve metadata + full details from ApiData
    ↓
Format as markdown with Rust signatures
    ↓
Return to user via MCP
```

## Performance

- **Embedding generation**: ~1 minute for 6,708 items (one-time, at build)
- **Database size**: ~10MB (6,708 × 384-dim vectors + metadata)
- **Query latency**: <100ms (vector search + formatting)
- **Startup time**: ~500ms (load LanceDB)

## Future Enhancements (v0.4.0+)

- Hybrid search (semantic + keyword)
- Re-ranking with cross-encoder
- Query understanding (extract Rust type names)
- Usage examples in embeddings
- Update mechanisms for API changes

## Key Decisions

1. **Separate build tool**: Cleaner than extending vim_build
2. **LanceDB over Qdrant**: Simpler deployment, no server needed
3. **Single semantic_search tool**: More flexible than per-type tools
4. **Text + semantic**: Keep both, don't replace text search
5. **Build-time embedding**: Faster runtime, consistent results
6. **384-dim model**: Good quality/size tradeoff

## Dependencies

```toml
# mcp/build-embeddings/Cargo.toml
[dependencies]
fastembed = "5.2.0"
lancedb = "0.22.3"
vim_mcp_server = { path = ".." }
tokio = { version = "1.42", features = ["full"] }
anyhow = "1.0"
```

## Deployment

The MCP server ships with:
- Binary: `vim_mcp_server`
- Data: `mcp/data/embeddings.lancedb/` (10MB)
- JSON: `mcp/data/*.json` (12MB)

Total size: ~22MB + binary (~5MB) = ~27MB
