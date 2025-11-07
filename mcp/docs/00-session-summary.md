# Session Summary - vim_rs MCP Design Discussion

**Date:** 2025-01-07
**Participants:** User (noclue), Claude

## Context

The vim_rs project provides Rust bindings for VMware vSphere API with extensive coverage:
- 2,195 API endpoints
- 9,738 data structures/schemas
- Complex object-oriented design mapped to Rust

**Challenge:** This large API surface is difficult to navigate and discover. Developers need help finding the right APIs for their tasks and understanding how to use them.

**Solution:** Build an MCP (Model Context Protocol) server that enables AI coding assistants to intelligently search and recommend vSphere APIs.

## Key Decisions Made

### 1. Distribution Model
**Decision:** Single static binary + data directory, no containers

**Rationale:**
- MCP servers run via stdio (stdin/stdout communication)
- Containers add latency (100-500ms startup overhead)
- Static binary is simpler: download, extract, configure
- No runtime dependencies needed

**Distribution:**
- Binary per platform (x86_64 + aarch64, Linux/Mac/Windows)
- Tarball with binary + pre-built indexes + embedding model
- Size: ~100-200 MB compressed

### 2. Technology Stack
**Decision:** 100% Rust implementation

**Stack:**
- MCP Protocol: Official Rust MCP SDK
- Vector DB: LanceDB (embedded) or Qdrant
- Text Search: Tantivy
- Embeddings: ONNX Runtime with all-MiniLM-L6-v2
- PDF Processing: pdf-extract
- vCenter Integration: vim_rs

**Why Rust over Python:**
- Single binary (no Python runtime needed)
- Fast startup: <10ms vs 100-300ms
- Natural integration with vim_rs
- Easy cross-compilation
- Professional performance

### 3. Index Structure
**Decision:** Use vim_rs API structure, not raw OpenAPI

**Rationale:**
- Developers use vim_rs module/method names in their code
- Better for code generation (copy-paste ready)
- Examples show actual vim_rs usage patterns

**Example:**
```rust
// What we index:
Module: vim_rs::mo::VirtualMachine
Method: create_snapshot
Signature: async fn create_snapshot(&self, name: String, ...) -> Result<...>

// Not just:
Path: /VirtualMachine/{moId}/CreateSnapshot
```

### 4. Search Strategy
**Decision:** Hybrid search (vector + full-text)

**Components:**
1. **Vector Search:** Semantic similarity via embeddings
   - Find APIs by intent: "How do I backup VMs?" → snapshot APIs
   - Model: all-MiniLM-L6-v2 (384 dimensions, 90 MB)

2. **Text Search:** Keyword matching via Tantivy
   - Find APIs by exact names: "VirtualMachine"
   - Support structured queries and filters

3. **Hybrid:** Combine and re-rank results
   - Best of both worlds
   - Weighted scoring (60% vector, 40% text)

### 5. Documentation Integration
**Decision:** Process and index VCF documentation PDF (8,000 pages)

**Approach:**
- Extract text using pdf-extract crate
- Semantic chunking (512 tokens, 100 overlap)
- Generate embeddings for all chunks
- Link documentation to relevant APIs
- Enable workflow-based queries

**Benefits:**
- Combine API syntax with operational guidance
- Provide step-by-step workflows
- Link procedures to specific APIs

### 6. Live vCenter Integration
**Decision:** Optional two-mode operation

**Mode 1: Offline (default)**
- Documentation search only
- No vCenter required
- Fully offline operation

**Mode 2: Connected (optional)**
- Connect to vCenter via vim_rs
- Query live inventory
- Test APIs against real environment
- Generate code with actual object IDs
- Read-only by default

### 7. Offline-First Architecture
**Decision:** Pre-build all indexes, ship with binary

**Build Time (CI/CD):**
- Parse OpenAPI specs
- Process PDF documentation
- Generate embeddings for everything
- Build vector + text indexes
- Package with binary

**Runtime (User's Machine):**
- Load pre-built indexes (instant)
- No embedding generation needed for docs
- Only embed user queries (fast, <20ms)
- Fully offline, no API calls

## Embedding Workflow (Simplified)

### What Are Embeddings?
Convert text into vectors (arrays of numbers) that capture semantic meaning.

**Example:**
```
"create VM snapshot" → [0.23, -0.45, 0.67, ..., 0.12] (384 floats)
"take VM backup"     → [0.21, -0.43, 0.69, ..., 0.14] (similar!)
"delete alarm"       → [0.89, 0.12, -0.34, ..., 0.56] (different)
```

### Build Time Process
1. **Collect text:** API docs + PDF documentation
2. **Chunk:** Split into 512-token semantic chunks
3. **Tokenize:** Convert words to token IDs
4. **Embed:** Run through neural network → vectors
5. **Index:** Store in vector DB with fast search index
6. **Package:** Ship everything pre-built

### Runtime Process
1. **User query:** "How do I backup VMs?"
2. **Embed query:** Same model → query vector
3. **Search:** Find similar vectors (cosine similarity)
4. **Return:** Matching API docs + code examples
5. **Fast:** <50ms total latency

### Key Points
- **Chunk size:** 200-512 tokens (~2-3 paragraphs)
- **Dimensions:** 384 (all-MiniLM-L6-v2)
- **Model size:** 90 MB ONNX file
- **Client needs:** Embedding model (NOT a chat LLM!)
- **Fully offline:** No API calls, runs locally

## Architecture Overview

```
┌─────────────────────────────────────────┐
│  AI Assistant (Claude, GPT-4, etc.)    │
│  - User asks about vSphere APIs         │
│  - Calls MCP tools                      │
└──────────────┬──────────────────────────┘
               │ MCP (JSON-RPC over stdio)
┌──────────────▼──────────────────────────┐
│  vim_rs MCP Server (Rust)               │
│  ┌────────────────────────────────────┐ │
│  │ Tools                              │ │
│  │ - search_vim_apis                  │ │
│  │ - get_api_details                  │ │
│  │ - find_workflow                    │ │
│  │ - list_inventory (connected)       │ │
│  │ - test_api_call (connected)        │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │ Search Engine                      │ │
│  │ - Vector search (semantic)         │ │
│  │ - Text search (keywords)           │ │
│  │ - Hybrid ranking                   │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │ Pre-built Indexes (shipped)        │ │
│  │ - Vector DB: 50k+ embeddings       │ │
│  │ - Text index: Full-text search     │ │
│  │ - Metadata: API details            │ │
│  └────────────────────────────────────┘ │
│  ┌────────────────────────────────────┐ │
│  │ Optional: vim_rs Client            │ │
│  │ - vCenter connection               │ │
│  │ - Live queries                     │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

## MCP Tools Design

### Core Tools (Always Available)

1. **search_vim_apis**
   - Search APIs by keyword or task description
   - Hybrid search (vector + text)
   - Returns: Ranked API results with examples

2. **get_api_details**
   - Get comprehensive details for specific API
   - Includes: signature, privileges, errors, examples
   - Links to related documentation

3. **find_workflow**
   - Get step-by-step workflow for common tasks
   - Returns: Ordered steps with relevant APIs
   - Includes: Code examples, privilege requirements

### Connected Mode Tools (Require vCenter)

4. **list_inventory**
   - Query live vCenter inventory
   - Filter by type, properties
   - Returns: Real object IDs and names

5. **get_object_properties**
   - Fetch properties of managed objects
   - Use real object IDs
   - Validate API compatibility

6. **test_api_call**
   - Test API calls (dry-run by default)
   - Validate parameters
   - Generate working code with real IDs

## Implementation Plan

### Phase 0: Research & Prototyping (1 week)
- Validate PDF extraction quality
- Test ONNX vs Candle for embeddings
- Prototype MCP server with Rust SDK
- Evaluate LanceDB vs Qdrant
- Make final tech decisions

### Phase 1: Core Search (2-3 weeks)
- Parse OpenAPI → API index
- Build Tantivy text search
- Implement MCP server with search_vim_apis
- Binary distribution (Linux x86_64)

### Phase 2: Semantic Search (2-3 weeks)
- Integrate ONNX embedding model
- Build vector index from API descriptions
- Implement hybrid search
- Add find_by_task tool

### Phase 3: Documentation Integration (2-3 weeks)
- Process VCF PDF (8,000 pages)
- Chunk and embed documentation
- Link docs to APIs
- Implement workflow guide tool

### Phase 4: Live VCF Integration (2-3 weeks)
- vim_rs client integration
- Inventory query tools
- API testing capability
- Code generation with real IDs

### Phase 5: Polish & Distribution (1-2 weeks)
- Cross-platform builds (5 targets)
- CI/CD pipeline
- Documentation
- Performance optimization

**Total Timeline:** 9-14 weeks to production-ready release

## Key Metrics & Goals

### Performance Targets
- Startup time: <100ms
- Query latency: <50ms (p99)
- Memory usage: <500MB active
- Binary size: <50MB

### Scale
- APIs indexed: 2,195
- Schemas indexed: 9,738
- Documentation chunks: 15,000-30,000
- Total vectors: ~50,000

### Quality Targets
- Search relevance: >80% accuracy
- Code examples: Copy-paste ready
- Workflow completeness: End-to-end coverage

## Questions Answered

### Why not containers?
- Stdio overhead (extra process boundary)
- Slower startup (100-500ms vs <10ms)
- Requires Docker/Podman
- Binary is simpler for local tools

### Why not Python?
- Requires Python runtime
- Slower startup (100-300ms)
- Distribution complexity (pip, venv)
- Rust is better for this use case

### Why not ChromaDB?
- Python-based (conflicts with Rust)
- Qdrant and LanceDB are Rust-native
- Can embed directly in binary

### Does vector DB work with all LLMs?
- Yes! MCP is LLM-agnostic
- We return JSON/text results
- Claude, GPT-4, Qwen all work identically
- Vector DB is internal implementation detail

### What about PDF quality?
- VCF PDF has selectable text (good!)
- Will validate in Phase 0
- Fallback to poppler if needed
- Build-time processing only

### Do we need GPU for embeddings?
- No! CPU inference is fast enough
- ONNX model runs at ~10-20ms per query
- Pre-built indexes don't need runtime embedding
- Only user queries need embedding (lightweight)

## Next Steps

1. **Create GitHub Repository**
   - Repository: noclue/vim_rs_mcp (private)
   - Initialize with documentation
   - Set up project structure

2. **Phase 0: Research & Prototyping**
   - Test PDF extraction
   - Benchmark embedding models
   - Prototype MCP server
   - Evaluate vector DBs

3. **Begin Development**
   - Follow phased implementation plan
   - Incremental, testable deliverables
   - Regular milestones

## References

- [vim_rs](https://github.com/noclue/vim_rs) - Base project
- [MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk) - Official SDK
- [Model Context Protocol](https://modelcontextprotocol.io) - Protocol spec
- OpenAPI Spec: `vim_build/data/vi_json_openapi_specification_v9_0_0_0_24798170.json`
- VCF Documentation: https://techdocs.broadcom.com/...vmware-cloud-foundation-9-0.pdf

## Open Questions for Next Session

1. Should we parse vim_rs generated code or extend vim_build for API index?
2. LanceDB vs Qdrant - final decision after benchmarking
3. PDF extraction quality - need to test with actual VCF doc
4. ONNX vs Candle - performance comparison needed
5. Chunk size optimization - may need tuning for technical content

## File Organization

```
vim_rs_mcp/
├── README.md                      # Project overview
├── docs/
│   ├── 00-session-summary.md     # This file
│   ├── 01-design-overview.md     # Architecture & design
│   ├── 02-technology-stack.md    # Detailed tech choices
│   ├── 03-embedding-workflow.md  # Embeddings explained
│   └── 04-implementation-plan.md # Phased development plan
└── [Future: src/, tools/, data/, etc.]
```

## Summary

We've designed a comprehensive MCP server for vim_rs that will:
- Make 2,195+ vSphere APIs discoverable via AI assistants
- Combine semantic search with full-text search for best results
- Integrate 8,000 pages of documentation with API reference
- Optionally connect to live vCenter for testing and code generation
- Ship as a single binary with all data pre-built
- Support all major platforms (Linux, Mac, Windows, x86_64 + ARM64)
- Provide fully offline operation with professional performance

Next session can begin Phase 0 prototyping and validation.
