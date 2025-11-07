# Implementation Plan

Phased development approach with incremental, testable deliverables.

## Phase 0: Research & Prototyping (1 week)

### Goals
- Validate PDF extraction quality from VCF documentation
- Test embedding generation approaches (ONNX vs Candle)
- Prototype MCP stdio protocol in Rust using official SDK
- Evaluate LanceDB vs Qdrant for our scale
- Make final technology decisions

### Tasks
1. **PDF Extraction Validation**
   - Extract sample pages from VCF 9.0 PDF
   - Assess text quality
   - Test pdf-extract vs poppler-rs
   - Identify any issues (formatting, special characters, etc.)

2. **Embedding Experiments**
   - Test ONNX Runtime with all-MiniLM-L6-v2
   - Test Candle with same model
   - Benchmark performance (CPU inference time)
   - Compare quality on sample API descriptions

3. **MCP Hello World**
   - Create minimal MCP server using Rust SDK
   - Implement single tool: `echo`
   - Test with Claude Desktop
   - Validate stdio communication

4. **Vector DB Evaluation**
   - Test LanceDB: Insert/search 1000 vectors
   - Test Qdrant: Insert/search 1000 vectors
   - Benchmark search performance
   - Compare API ergonomics

### Deliverables
- [ ] PDF extraction quality report
- [ ] Embedding model benchmark results
- [ ] Working MCP hello-world server
- [ ] Vector DB comparison document
- [ ] Final technology stack decision

### Success Criteria
- PDF text extraction is >90% accurate
- Embedding inference <50ms per query
- MCP server responds to Claude Desktop
- Vector search <20ms for 1000 vectors

---

## Phase 1: Core Search (2-3 weeks)

### Goals
- Parse OpenAPI spec into searchable index
- Implement basic keyword search (Tantivy)
- Build MCP server with search tool
- Package as distributable binary

### Tasks

#### 1. OpenAPI Parser
```rust
// Parse vim_rs generated code or OpenAPI directly
struct ApiIndex {
    apis: HashMap<String, ApiInfo>,
    schemas: HashMap<String, SchemaInfo>,
}

struct ApiInfo {
    module: String,         // "vim_rs::mo::VirtualMachine"
    method: String,         // "create_snapshot"
    signature: String,      // Full Rust signature
    description: String,
    privileges: Vec<String>,
    parameters: Vec<ParameterInfo>,
    returns: String,
    errors: Vec<String>,
}
```

**Options:**
1. Parse generated `.rs` files using `syn` crate
2. Extend vim_build to output JSON index
3. Parse OpenAPI and map to vim_rs names

**Recommendation:** Option 2 - extend vim_build

#### 2. Tantivy Text Index
```rust
// Build full-text search index
- Index API names (module::method)
- Index descriptions
- Index parameter names
- Support boolean queries (AND, OR, NOT)
- Support phrase queries
- Support fuzzy matching
```

#### 3. MCP Server Foundation
```rust
// Implement core MCP server
use mcp_server::{Server, Tool};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::new("vim-rs-helper", "0.1.0");

    // Load indexes
    let text_index = load_text_index("data/text_index")?;

    // Register tool
    server.add_tool(
        "search_vim_apis",
        search_vim_apis_schema(),
        move |params| {
            let results = text_index.search(&params.query)?;
            Ok(format_results(results))
        }
    );

    server.run_stdio().await
}
```

#### 4. Search Tool Implementation
```rust
// Tool: search_vim_apis
Input: {
    query: String,
    max_results: Option<usize>,
    filter: Option<SearchFilter>
}

Output: {
    results: Vec<ApiSearchResult>,
    total_found: usize
}

struct ApiSearchResult {
    api: String,              // "VirtualMachine::create_snapshot"
    module: String,           // "vim_rs::mo::VirtualMachine"
    description: String,
    signature: String,
    example: Option<String>,
    privileges: Vec<String>,
    relevance_score: f32
}
```

### Deliverables
- [ ] API index builder (extend vim_build or standalone)
- [ ] Tantivy text search implementation
- [ ] MCP server with `search_vim_apis` tool
- [ ] Binary distribution for Linux x86_64
- [ ] Basic documentation

### Testing
```rust
// Test cases
1. search("VirtualMachine") → Returns VM-related APIs
2. search("create snapshot") → Returns create_snapshot_* methods
3. search("backup") → Returns snapshot + scheduled task APIs
4. search("alarm") → Returns Alarm and AlarmManager APIs
5. Filter by privileges → Returns only matching APIs
```

### Success Criteria
- [ ] Search 2,195 APIs in <10ms
- [ ] Keyword search returns relevant results
- [ ] Works with Claude Desktop via MCP
- [ ] Binary runs on Linux without dependencies

---

## Phase 2: Semantic Search (2-3 weeks)

### Goals
- Integrate embedding model (ONNX)
- Build vector index from API descriptions
- Implement hybrid search (semantic + keyword)
- Improve result relevance

### Tasks

#### 1. Embedding Pipeline (Build Time)
```rust
// tools/build_index.rs
async fn build_vector_index() -> Result<()> {
    // 1. Load ONNX model
    let model = load_embedding_model("models/minilm-l6-v2.onnx")?;

    // 2. Load API descriptions
    let apis = load_api_index("data/api_index.json")?;

    // 3. Generate embeddings
    for api in apis {
        let text = format_api_for_embedding(&api);
        let embedding = model.embed(&text).await?;

        vector_db.insert(VectorRecord {
            id: api.id,
            vector: embedding,
            text: api.description,
            metadata: api.metadata,
        })?;
    }

    // 4. Save vector index
    vector_db.save("data/vector_index")?;
    Ok(())
}
```

#### 2. Runtime Embedding
```rust
// src/embeddings.rs
struct EmbeddingModel {
    session: ort::Session,
    tokenizer: Tokenizer,
}

impl EmbeddingModel {
    fn embed(&self, text: &str) -> Result<Vec<f32>> {
        // 1. Tokenize
        let tokens = self.tokenizer.encode(text)?;

        // 2. Run ONNX inference
        let input = ort::Value::from_array(&tokens)?;
        let outputs = self.session.run(vec![input])?;

        // 3. Extract embeddings
        let embeddings = outputs[0].extract_tensor()?;
        Ok(embeddings.to_vec())
    }
}
```

#### 3. Vector Search Integration
```rust
// src/search/vector_search.rs
struct VectorSearch {
    db: LanceDB,
    embedding_model: EmbeddingModel,
}

impl VectorSearch {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // 1. Embed query
        let query_vector = self.embedding_model.embed(query)?;

        // 2. Search vector DB
        let results = self.db.search_vectors(
            &query_vector,
            limit,
            0.7, // min similarity threshold
        ).await?;

        Ok(results)
    }
}
```

#### 4. Hybrid Search Strategy
```rust
// src/search/hybrid.rs
async fn hybrid_search(
    query: &str,
    vector_search: &VectorSearch,
    text_search: &TextSearch,
) -> Result<Vec<SearchResult>> {
    // 1. Run both searches in parallel
    let (vector_results, text_results) = tokio::join!(
        vector_search.search(query, 20),
        text_search.search(query, 20),
    );

    // 2. Merge results
    let mut combined = merge_results(vector_results?, text_results?);

    // 3. Re-rank using hybrid score
    combined.sort_by(|a, b| {
        let score_a = 0.6 * a.vector_score + 0.4 * a.text_score;
        let score_b = 0.6 * b.vector_score + 0.4 * b.text_score;
        score_b.partial_cmp(&score_a).unwrap()
    });

    Ok(combined)
}
```

#### 5. Enhanced Search Tool
```rust
// Update search_vim_apis to use hybrid search
// Add new tool: find_by_task

Tool: find_by_task
Description: "Find APIs for a specific task using semantic understanding"
Input: {
    task: String,  // e.g., "provision a new virtual machine"
    context: Option<String>  // Optional additional context
}
Output: {
    relevant_apis: Vec<ApiInfo>,
    suggested_workflow: Option<String>
}
```

### Deliverables
- [ ] ONNX embedding model integration
- [ ] Vector index builder tool
- [ ] Hybrid search implementation
- [ ] Enhanced `search_vim_apis` tool
- [ ] New `find_by_task` tool
- [ ] Updated binary distribution

### Testing
```rust
// Semantic search tests
1. "backup virtual machines" → snapshot APIs (even without exact words)
2. "schedule automated tasks" → ScheduledTaskManager APIs
3. "VM provisioning" → createVM, configureVM, deployVM
4. "How do I monitor host health?" → HostSystem monitoring APIs
5. Hybrid: "VirtualMachine backup" → Both exact matches + semantic
```

### Success Criteria
- [ ] Semantic search understands intent (not just keywords)
- [ ] Query embedding <20ms
- [ ] Vector search <20ms for 10k+ vectors
- [ ] Hybrid results more relevant than text-only

---

## Phase 3: Documentation Integration (2-3 weeks)

### Goals
- Process VCF PDF documentation (8,000 pages)
- Chunk and embed documentation
- Link docs to APIs
- Implement workflow guide tool

### Tasks

#### 1. PDF Processing Pipeline
```rust
// tools/process_pdf.rs
async fn process_vcf_pdf() -> Result<()> {
    // 1. Extract text
    let full_text = pdf_extract::extract_text("vcf-9-0.pdf")?;

    // 2. Detect structure (TOC, chapters, sections)
    let structure = detect_document_structure(&full_text)?;

    // 3. Segment by chapters
    let chapters = segment_by_structure(&full_text, &structure)?;

    // 4. Clean text
    for chapter in &mut chapters {
        chapter.text = clean_pdf_artifacts(&chapter.text);
    }

    // 5. Semantic chunking
    let chunks = chunk_chapters(chapters, ChunkConfig {
        max_tokens: 512,
        overlap: 100,
        preserve_sections: true,
    })?;

    // 6. Link to APIs
    for chunk in &mut chunks {
        chunk.related_apis = find_mentioned_apis(&chunk.text)?;
    }

    // 7. Embed and index
    embed_and_index_chunks(chunks).await?;

    Ok(())
}
```

#### 2. Documentation Index
```rust
struct DocChunk {
    id: String,
    text: String,
    embedding: Vec<f32>,
    metadata: DocMetadata,
}

struct DocMetadata {
    source: String,           // "VCF 9.0 Admin Guide"
    chapter: String,          // "Chapter 5: Virtual Machine Management"
    section: String,          // "5.3 Snapshot Management"
    page_range: (usize, usize),
    related_apis: Vec<String>,
    keywords: Vec<String>,
}
```

#### 3. API-Doc Cross-Referencing
```rust
// Build bidirectional links
struct CrossReference {
    api_to_docs: HashMap<String, Vec<String>>,  // API → doc chunks
    doc_to_apis: HashMap<String, Vec<String>>,  // doc chunk → APIs
}

// Link strategies:
1. Explicit mentions: "VirtualMachine.createSnapshot"
2. Natural language: "create a VM snapshot" → link to snapshot APIs
3. Workflow patterns: Multi-step procedures → sequence of APIs
```

#### 4. Workflow Guide Tool
```rust
Tool: get_workflow_guide
Description: "Get step-by-step guide for vSphere workflows"
Input: {
    workflow: String,  // "VM migration", "backup strategy", etc.
}
Output: {
    workflow_name: String,
    steps: Vec<WorkflowStep>,
    documentation_reference: String,
    complete_example: Option<String>
}

struct WorkflowStep {
    step_number: usize,
    description: String,
    apis: Vec<String>,
    code_example: Option<String>,
    required_privileges: Vec<String>,
    notes: Vec<String>
}
```

#### 5. Enhanced Search with Docs
```rust
// Update search_vim_apis to include doc context
Output: {
    results: Vec<ApiSearchResult>,
    related_documentation: Vec<DocChunk>,
    workflow_suggestions: Vec<String>
}
```

### Deliverables
- [ ] PDF processing tool
- [ ] Documentation chunk index
- [ ] API-doc cross-reference database
- [ ] `get_workflow_guide` tool
- [ ] Enhanced search results with doc context
- [ ] Complete data package

### Testing
```rust
// Documentation integration tests
1. search("backup") → Returns APIs + relevant doc sections
2. get_workflow_guide("VM migration") → Step-by-step with APIs
3. get_api_details("VirtualMachine::create_snapshot") → Includes admin guide context
4. Search combines API reference + operational guidance
```

### Success Criteria
- [ ] Successfully extract 8,000 pages
- [ ] Generate 15,000-30,000 doc chunks
- [ ] API-doc links are accurate
- [ ] Workflow guides are comprehensive and actionable

---

## Phase 4: Live VCF Integration (2-3 weeks)

### Goals
- Integrate vim_rs for vCenter connection
- Implement live inventory query tools
- Add API testing capability
- Generate code with real object IDs

### Tasks

#### 1. Connection Management
```rust
// src/vcenter/connection.rs
enum OperationMode {
    Offline,
    Connected {
        client: Arc<vim_rs::Client>,
        config: ConnectionConfig,
        read_only: bool,
    }
}

struct ConnectionConfig {
    vcenter_url: String,
    username: String,
    password_source: PasswordSource,  // Env, keychain, prompt
}

enum PasswordSource {
    Environment(String),    // VCENTER_PASSWORD
    Command(String),        // security find-generic-password...
    Prompt,
}
```

#### 2. Inventory Query Tool
```rust
Tool: list_inventory
Description: "List inventory objects from connected vCenter"
Input: {
    object_type: ObjectType,  // VirtualMachine, HostSystem, etc.
    filter: Option<String>,
    properties: Vec<String>
}
Output: {
    objects: Vec<InventoryObject>,
    total_count: usize
}

struct InventoryObject {
    mo_id: String,
    name: String,
    type_: String,
    properties: HashMap<String, Value>,
}
```

#### 3. Property Query Tool
```rust
Tool: get_object_properties
Description: "Fetch specific properties of a managed object"
Input: {
    mo_id: String,
    properties: Vec<String>
}
Output: {
    mo_id: String,
    object_type: String,
    properties: HashMap<String, Value>
}
```

#### 4. API Testing Tool
```rust
Tool: test_api_call
Description: "Test an API call (dry-run by default)"
Input: {
    api: String,           // "VirtualMachine::create_snapshot"
    object_id: String,     // "vm-123"
    parameters: Value,
    dry_run: bool,         // default: true
}
Output: {
    success: bool,
    result: Option<Value>,
    error: Option<String>,
    would_succeed: bool,   // If dry_run
    warnings: Vec<String>
}
```

#### 5. Code Generation with Real Data
```rust
Tool: generate_code
Description: "Generate vim_rs code for a task using real object IDs"
Input: {
    task: String,
    target_objects: Vec<String>,  // ["vm-123", "vm-456"]
}
Output: {
    code: String,
    explanation: String,
    required_imports: Vec<String>
}

Example output:
```rust
use vim_rs::mo::VirtualMachine;

async fn backup_vms(client: Arc<Client>) -> Result<()> {
    // VM: "prod-web-01" (vm-123)
    let vm1 = VirtualMachine::new(client.clone(), "vm-123");
    let snapshot1 = vm1.create_snapshot(
        "backup-2024-01-15",
        Some("Daily backup"),
        true,
        true
    ).await?;

    // VM: "prod-db-01" (vm-456)
    let vm2 = VirtualMachine::new(client.clone(), "vm-456");
    let snapshot2 = vm2.create_snapshot(
        "backup-2024-01-15",
        Some("Daily backup"),
        true,
        true
    ).await?;

    Ok(())
}
```
```

### Deliverables
- [ ] vCenter connection management
- [ ] `list_inventory` tool
- [ ] `get_object_properties` tool
- [ ] `test_api_call` tool
- [ ] `generate_code` tool
- [ ] Configuration examples

### Testing
```rust
// Live integration tests (requires test vCenter)
1. Connect to vCenter successfully
2. List VMs, Hosts, Datastores
3. Query VM properties
4. Test API call (dry-run)
5. Generate code with real object IDs
6. Validate generated code compiles
```

### Success Criteria
- [ ] Successfully connect to vCenter
- [ ] Query inventory objects
- [ ] Generate working vim_rs code
- [ ] Read-only mode enforced by default

---

## Phase 5: Polish & Distribution (1-2 weeks)

### Goals
- Cross-platform builds (5 targets)
- CI/CD pipeline
- Documentation
- Performance optimization
- Release automation

### Tasks

#### 1. Cross-Platform Builds
```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
      - name: Package
        run: |
          tar czf vim-rs-mcp-${{ matrix.target }}.tar.gz \
            target/${{ matrix.target }}/release/vim-rs-mcp \
            data/
      - name: Upload Release
        uses: softprops/action-gh-release@v1
        with:
          files: vim-rs-mcp-${{ matrix.target }}.tar.gz
```

#### 2. Performance Optimization
```rust
// Optimization checklist
- [ ] Enable LTO (Link Time Optimization)
- [ ] Strip debug symbols
- [ ] Profile hot paths
- [ ] Optimize vector search
- [ ] Cache common queries
- [ ] Lazy-load documentation chunks
```

#### 3. Documentation
```markdown
User Documentation:
- [ ] Installation guide
- [ ] Configuration guide
- [ ] Tool reference
- [ ] Usage examples
- [ ] Troubleshooting

Developer Documentation:
- [ ] Architecture overview
- [ ] API documentation (rustdoc)
- [ ] Contributing guide
- [ ] Build instructions
```

#### 4. Testing & Quality
```rust
// Test coverage goals
- [ ] Unit tests for all modules
- [ ] Integration tests for MCP tools
- [ ] Performance benchmarks
- [ ] E2E tests with Claude Desktop
- [ ] Load tests (search performance)
```

### Deliverables
- [ ] Binaries for all 5 platforms
- [ ] Complete documentation
- [ ] CI/CD pipeline
- [ ] Release automation
- [ ] Performance benchmarks

### Success Criteria
- [ ] All platforms build successfully
- [ ] Binary size <50 MB
- [ ] Startup time <100ms
- [ ] Search latency <50ms (p99)
- [ ] Documentation complete

---

## Timeline Summary

| Phase | Duration | Deliverable |
|-------|----------|-------------|
| Phase 0: Research | 1 week | Tech decisions, prototypes |
| Phase 1: Core Search | 2-3 weeks | Working MCP with text search |
| Phase 2: Semantic Search | 2-3 weeks | Vector search, hybrid results |
| Phase 3: Documentation | 2-3 weeks | PDF integration, workflows |
| Phase 4: Live Integration | 2-3 weeks | vCenter connection, testing |
| Phase 5: Polish | 1-2 weeks | Multi-platform release |
| **Total** | **9-14 weeks** | Production-ready MCP server |

## Milestones

### M1: Basic Search (End of Phase 1)
- ✅ Search 2,195 APIs by keyword
- ✅ Working MCP server with Claude Desktop
- ✅ Linux binary distribution

### M2: Semantic Search (End of Phase 2)
- ✅ Intent-based API discovery
- ✅ Hybrid search results
- ✅ Improved relevance

### M3: Documentation (End of Phase 3)
- ✅ 8,000 pages indexed
- ✅ Workflow guides
- ✅ API-doc cross-references

### M4: Live Integration (End of Phase 4)
- ✅ vCenter connection
- ✅ Code generation with real IDs
- ✅ API testing capability

### M5: Release (End of Phase 5)
- ✅ Multi-platform support
- ✅ Production quality
- ✅ Complete documentation
- ✅ Public release

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| PDF extraction quality poor | Medium | Test early (Phase 0), fallback to manual processing |
| Vector search too slow | Medium | Benchmark early, optimize or use smaller model |
| MCP SDK immature | Low | SDK is official, worst case implement protocol directly |
| vCenter API changes | Low | vim_rs already handles versioning |
| Scale issues (50k+ vectors) | Low | Architecture supports optimization (quantization, etc.) |

## Post-Release Roadmap

### Future Enhancements
1. **Web UI** - Optional web interface for browsing APIs
2. **VS Code Extension** - Integrate with VS Code
3. **Custom Training** - Fine-tune embedding model on vSphere docs
4. **Multi-version Support** - Index multiple vSphere versions
5. **Community Workflows** - User-contributed workflow guides
6. **Analytics** - Track common searches, improve results

### Maintenance
- Monthly: Update OpenAPI specs for new vSphere releases
- Quarterly: Retrain embeddings with improved models
- As needed: Bug fixes, performance improvements
