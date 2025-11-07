# Design Overview

## Problem Statement

The vim_rs project provides Rust bindings for VMware vSphere API with:
- **2,195 API endpoints** across various managed object types
- **9,738 data structures** (schemas)
- Complex inheritance hierarchies and relationships
- Extensive documentation across multiple guides (8,000+ pages)

Working with such a large API surface is challenging:
- Finding the right API for a task
- Understanding workflows that span multiple APIs
- Knowing required privileges and error conditions
- Tying API syntax to operational workflows

## Solution: MCP Server with RAG

Build an MCP (Model Context Protocol) server that provides AI coding assistants with:

1. **Semantic Search**: Find APIs by intent/task description
2. **Documentation Context**: Combine API specs with admin/developer guides
3. **Live Environment Access**: Optional vCenter connection for testing
4. **Code Generation**: Generate vim_rs-specific code examples

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  AI Coding Assistant (Claude, GPT-4, etc.)                  │
│  - Receives user questions about vSphere APIs               │
│  - Calls MCP tools to search/query                          │
└─────────────────────┬───────────────────────────────────────┘
                      │ MCP Protocol (JSON-RPC over stdio)
┌─────────────────────▼───────────────────────────────────────┐
│  vim_rs MCP Server (Rust)                                   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ MCP Tools                                           │   │
│  │ - search_vim_apis                                   │   │
│  │ - get_api_details                                   │   │
│  │ - find_workflow                                     │   │
│  │ - list_inventory (if connected)                     │   │
│  │ - test_api_call (if connected)                      │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Search Engine                                       │   │
│  │ - Hybrid search (vector + full-text)                │   │
│  │ - Semantic similarity via embeddings                │   │
│  │ - Keyword search via Tantivy                        │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Indexes (Pre-built, shipped with binary)           │   │
│  │ - Vector DB: API embeddings                         │   │
│  │ - Text Index: Full-text search                      │   │
│  │ - Metadata DB: Structured API info                  │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Optional: vim_rs Client                             │   │
│  │ - Connect to vCenter                                │   │
│  │ - Query live inventory                              │   │
│  │ - Test API calls                                    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. MCP Server (stdio)
- Implements MCP protocol using official Rust SDK
- Exposes tools for API search, documentation, and live queries
- Runs as subprocess launched by Claude Desktop or other MCP clients
- Fast startup, low memory footprint

### 2. Search Engine
**Hybrid Search Strategy:**
- **Vector Search**: Semantic similarity for intent-based queries
  - "How do I backup VMs?" → finds snapshot and scheduled task APIs
- **Full-text Search**: Keyword and structured queries
  - "VirtualMachine snapshot" → exact matches
- **Metadata Filtering**: Filter by privileges, object type, etc.

### 3. Index Structure
**Three-tier index system:**

1. **Vector Index** (Qdrant/LanceDB)
   - Embeddings of API descriptions
   - Embeddings of documentation chunks
   - Fast semantic similarity search

2. **Text Index** (Tantivy)
   - Full-text search on API names, descriptions
   - Structured filtering capabilities
   - Fast keyword search

3. **Metadata DB** (SQLite or JSON)
   - Structured API metadata
   - Cross-references (API → docs, docs → APIs)
   - Privilege requirements, error types, etc.

### 4. vim_rs Integration
**Index Structure Decision: Use vim_rs names, not OpenAPI**

Why? Developers use vim_rs, not raw OpenAPI:

```rust
// vim_rs (what developers write):
let snapshot = vm.create_snapshot(
    "backup-2024",
    Some("Daily backup"),
    true,  // memory
    true   // quiesce
).await?;

// OpenAPI (internal):
POST /VirtualMachine/{moId}/CreateSnapshot
Body: CreateSnapshotRequestType { ... }
```

**Index vim_rs structure:**
- Module: `vim_rs::mo::VirtualMachine`
- Method: `create_snapshot`
- Signature: `async fn create_snapshot(&self, name: String, ...)`
- Rust-style parameter names and types

**Data source:** Parse generated vim_rs code or extend vim_build to output metadata

## Data Flow

### Build Time (CI/CD)
```
1. Parse OpenAPI specs
   ├─> Extract API metadata
   └─> Map to vim_rs structure (via vim_build)

2. Process documentation PDF
   ├─> Extract text (pdf-extract)
   ├─> Chunk semantically (512 tokens)
   └─> Link to relevant APIs

3. Generate embeddings
   ├─> Use embedding model (all-MiniLM-L6-v2)
   ├─> Embed all API descriptions
   └─> Embed all doc chunks

4. Build indexes
   ├─> Vector index (Qdrant/Lance)
   ├─> Text index (Tantivy)
   └─> Metadata DB (SQLite)

5. Package for distribution
   ├─> Binary + data directory
   ├─> Include ONNX embedding model
   └─> tar.gz per platform
```

### Runtime (User's Machine)
```
1. MCP client launches vim_rs_mcp

2. Load indexes (instant, pre-built)

3. Receive MCP tool call
   ├─> Example: search_vim_apis("backup VMs")

4. Process query
   ├─> Embed query (ONNX model)
   ├─> Vector search (find similar APIs)
   ├─> Text search (keyword matching)
   └─> Combine and rank results

5. Optional: Query vCenter
   ├─> If connected, fetch live data
   └─> Enhance results with real object IDs

6. Return results to MCP client
   ├─> Formatted API documentation
   ├─> Code examples
   └─> Related workflows
```

## Key Design Decisions

### Use vim_rs Structure Instead of OpenAPI
**Decision:** Index using vim_rs module/method names, not OpenAPI paths

**Rationale:**
- Developers use vim_rs API, not HTTP endpoints
- Better code generation (copy-paste ready)
- Natural language query → Rust code mapping
- Examples show actual vim_rs usage

### Offline-First Architecture
**Decision:** All indexes and models ship with binary

**Rationale:**
- No API calls or cloud dependencies
- Fast (no network latency)
- Privacy (no data sent externally)
- Reliable (works without internet)

### Hybrid Search Strategy
**Decision:** Combine vector and full-text search

**Rationale:**
- Vector search: Great for semantic queries ("how to backup")
- Text search: Great for specific names ("VirtualMachine")
- Hybrid: Best of both worlds
- Metadata filters: Narrow by privileges, types, etc.

### Optional vCenter Connection
**Decision:** Two modes - offline (docs only) and connected (live data)

**Rationale:**
- Documentation search works standalone
- Live connection adds huge value for testing
- Generate code with actual object IDs
- Validate API calls before running

### Rust Implementation
**Decision:** 100% Rust, including embeddings (ONNX)

**Rationale:**
- Single binary distribution
- Fast startup (<10ms vs Python's 100-300ms)
- Natural integration with vim_rs
- Professional-grade performance
- No runtime dependencies

### Embedded Vector DB
**Decision:** Ship vector DB files with binary, no separate server

**Rationale:**
- Simpler deployment (no database server)
- Faster (no network/IPC overhead)
- Smaller footprint for local use
- Qdrant and LanceDB support embedded mode

## MCP Tools Design

### Core Tools (Always Available)

#### 1. search_vim_apis
```json
{
  "name": "search_vim_apis",
  "description": "Search vSphere APIs by keyword or task description",
  "parameters": {
    "query": "string",
    "max_results": "number (default: 10)",
    "filter": {
      "object_type": "optional VirtualMachine|HostSystem|...",
      "privileges": "optional array of required privileges"
    }
  }
}
```

#### 2. get_api_details
```json
{
  "name": "get_api_details",
  "description": "Get comprehensive details for a specific API",
  "parameters": {
    "api_path": "string (e.g., 'VirtualMachine::create_snapshot')"
  }
}
```

#### 3. find_workflow
```json
{
  "name": "find_workflow",
  "description": "Get step-by-step workflow for common vSphere tasks",
  "parameters": {
    "task": "string (e.g., 'VM provisioning', 'backup workflow')"
  }
}
```

### Connected Mode Tools (Require vCenter)

#### 4. list_inventory
```json
{
  "name": "list_inventory",
  "description": "List inventory objects from connected vCenter",
  "parameters": {
    "object_type": "VirtualMachine|HostSystem|Datastore|...",
    "filter": "optional filter expression",
    "properties": "optional array of properties to fetch"
  }
}
```

#### 5. get_object_properties
```json
{
  "name": "get_object_properties",
  "description": "Fetch properties of a specific managed object",
  "parameters": {
    "mo_id": "string (e.g., 'vm-123')",
    "properties": "array of property paths"
  }
}
```

#### 6. test_api_call
```json
{
  "name": "test_api_call",
  "description": "Test an API call against vCenter (read-only by default)",
  "parameters": {
    "api": "string (API method)",
    "params": "object (parameters)",
    "dry_run": "boolean (default: true)"
  }
}
```

## Scalability Considerations

### Current Scale
- 2,195 API endpoints
- 9,738 schemas
- ~15,000-30,000 documentation chunks (from 8,000 page PDF)
- Total vectors: ~45,000-60,000

### Performance Targets
- Startup time: <100ms
- Query latency: <50ms for vector + text search
- Memory usage: <500MB including loaded models
- Binary size: <50MB
- Data size: <200MB (compressed)

### Optimization Strategies
- Pre-built indexes (no build step at runtime)
- Embedded models (no API calls)
- Efficient vector quantization if needed
- Lazy loading of documentation chunks
- Result caching for common queries

## Security Considerations

### API Keys and Credentials
- Never store vCenter passwords in config
- Support secure credential sources:
  - Environment variables
  - OS keychain integration
  - Prompt on first connection

### Read-Only by Default
- Live vCenter tools default to read-only
- Write operations require explicit flag
- Dry-run mode for testing destructive operations

### Data Privacy
- All processing happens locally
- No telemetry or external API calls
- Documentation stays on user's machine

## Success Metrics

### User Experience
- Time to find relevant API: <10 seconds
- Search result relevance: >80% accuracy
- Code examples work without modification

### Technical Performance
- Startup latency: <100ms
- Search latency: <50ms
- Memory footprint: <500MB
- Binary size: <50MB per platform

### Adoption
- Used by vim_rs developers
- Integrated into development workflows
- Positive feedback on API discoverability

## Next Steps

See [Implementation Plan](04-implementation-plan.md) for detailed development roadmap.
