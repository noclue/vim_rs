# Embedding Workflow Explained

This document explains how text embeddings work in the vim_rs MCP server, simplified for understanding.

## What Are Embeddings?

**Simple Definition:** Embeddings convert text into numbers (vectors) that capture semantic meaning.

**Example:**
```
Text: "create VM snapshot"
Vector: [0.23, -0.45, 0.67, 0.12, ..., 0.89]  (384 numbers)

Text: "take VM backup"
Vector: [0.21, -0.43, 0.69, 0.14, ..., 0.87]  (similar numbers!)

Text: "delete alarm"
Vector: [0.89, 0.12, -0.34, 0.56, ..., 0.23]  (different numbers!)
```

**Key Insight:** Similar meanings → similar vectors → can find them mathematically!

## The Complete Workflow

### Phase 1: Build Time (Once, Before Shipping)

This happens on CI/CD, user never sees it.

```
┌─────────────────────────────────────────────────────────────┐
│ Step 1: Collect Text                                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Input sources:                                              │
│ 1. OpenAPI spec → 2,195 API descriptions                   │
│ 2. vim_rs code → Generated method docs                     │
│ 3. PDF docs → 8,000 pages of guides                        │
│                                                             │
│ Raw text collected: ~50-100 MB                             │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 2: Chunk Text                                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Split into semantic chunks:                                 │
│                                                             │
│ Chunk size: 200-512 tokens (~150-400 words)                │
│ Overlap: 50-100 tokens (preserve context)                  │
│                                                             │
│ Example chunk:                                              │
│ ┌─────────────────────────────────────────────────────┐   │
│ │ VirtualMachine::create_snapshot creates a snapshot  │   │
│ │ of the VM's current state. It captures memory if    │   │
│ │ the memory parameter is true. The quiesce parameter │   │
│ │ ensures file system consistency by flushing         │   │
│ │ pending writes...                                   │   │
│ │                                                     │   │
│ │ Required privileges:                                │   │
│ │ - VirtualMachine.State.CreateSnapshot              │   │
│ │                                                     │   │
│ │ Returns: Task<VirtualMachineSnapshot>              │   │
│ └─────────────────────────────────────────────────────┘   │
│ (~250 tokens)                                               │
│                                                             │
│ Total chunks created: ~15,000-30,000                        │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 3: Tokenize                                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Convert words to token IDs:                                 │
│                                                             │
│ "VirtualMachine" → [9182, 29008]                           │
│ "create"         → [3264]                                   │
│ "snapshot"       → [16040]                                  │
│                                                             │
│ Each chunk → array of ~200-512 integers                    │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 4: Generate Embeddings (THE MAGIC!)                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Use embedding model:                                        │
│ Model: sentence-transformers/all-MiniLM-L6-v2              │
│ Size: ~90 MB                                                │
│                                                             │
│ Process:                                                    │
│ Token IDs → Neural Network → Dense Vector                  │
│                                                             │
│ Input:  [9182, 29008, 3264, 16040, ...]                    │
│         ↓ (neural network does magic)                       │
│ Output: [0.23, -0.45, 0.67, ..., 0.12]                     │
│         (384 floating-point numbers)                        │
│                                                             │
│ What happens inside:                                        │
│ - Model learned from millions of text examples             │
│ - Understands that "create" ≈ "make" ≈ "generate"         │
│ - Understands that "snapshot" ≈ "backup" ≈ "copy"         │
│ - Places similar concepts close in 384-dimensional space   │
│                                                             │
│ Run for all chunks: ~15,000-30,000 embeddings generated    │
│ Time: ~30-60 minutes on CPU                                 │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 5: Build Vector Index                                 │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Store in vector database (LanceDB/Qdrant):                 │
│                                                             │
│ Record structure:                                           │
│ {                                                           │
│   id: "VirtualMachine::create_snapshot",                   │
│   vector: [0.23, -0.45, 0.67, ..., 0.12],  (384 floats)   │
│   text: "VirtualMachine::create_snapshot creates...",      │
│   metadata: {                                               │
│     module: "vim_rs::mo::VirtualMachine",                  │
│     method: "create_snapshot",                             │
│     privileges: ["VirtualMachine.State.CreateSnapshot"],   │
│     returns: "Task<VirtualMachineSnapshot>"                │
│   }                                                         │
│ }                                                           │
│                                                             │
│ Index structure: HNSW (Hierarchical Navigable Small World) │
│ - Graph-based index for fast similarity search             │
│ - O(log n) search time instead of O(n)                     │
│                                                             │
│ Index size: ~100-150 MB                                     │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 6: Package Everything                                 │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ vim-rs-mcp-v1.0.0-linux-x86_64.tar.gz                      │
│ ├── vim-rs-mcp (binary, ~15 MB)                            │
│ ├── data/                                                   │
│ │   ├── vector_index/           (100-150 MB)              │
│ │   │   └── [LanceDB files]                               │
│ │   ├── text_index/             (20-50 MB)                │
│ │   │   └── [Tantivy files]                               │
│ │   ├── metadata.json           (5-10 MB)                 │
│ │   └── models/                                            │
│ │       └── minilm-l6-v2.onnx   (90 MB)                   │
│ └── README.md                                               │
│                                                             │
│ Total: ~230-320 MB (compressed to ~100-150 MB)            │
└─────────────────────────────────────────────────────────────┘
```

### Phase 2: Runtime (User's Machine)

This happens every time user searches.

```
┌─────────────────────────────────────────────────────────────┐
│ Step 1: MCP Server Starts                                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ 1. Load vector index (instant - pre-built)                 │
│ 2. Load text index (instant - pre-built)                   │
│ 3. Load ONNX embedding model (once, ~50ms)                 │
│                                                             │
│ Total startup: <100ms                                       │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 2: Receive User Query (via MCP)                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ User asks Claude: "How do I backup VMs?"                   │
│                                                             │
│ Claude calls MCP tool:                                      │
│ search_vim_apis(query: "backup VMs")                       │
│                                                             │
│ MCP server receives: "backup VMs"                          │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 3: Embed Query (Same Process as Build Time!)          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Input: "backup VMs"                                         │
│                                                             │
│ 1. Tokenize: [7387, 21386]                                 │
│                                                             │
│ 2. ONNX model inference:                                    │
│    [7387, 21386] → Neural Net → Vector                     │
│                                                             │
│ 3. Output: Query vector                                     │
│    [0.19, -0.41, 0.71, ..., 0.15]  (384 floats)           │
│                                                             │
│ Time: ~10-20ms on CPU                                       │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 4: Vector Similarity Search                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Compare query vector to all indexed vectors:               │
│                                                             │
│ Distance metric: Cosine similarity                          │
│ Formula: similarity = dot(query_vec, doc_vec) / norms      │
│                                                             │
│ Query vector:   [0.19, -0.41, 0.71, ..., 0.15]            │
│                                                             │
│ Document vectors:                                           │
│ 1. "create_snapshot": [0.21, -0.43, 0.69, ...] → 0.87 ✅  │
│ 2. "scheduled_task":  [0.18, -0.39, 0.73, ...] → 0.79 ✅  │
│ 3. "delete_alarm":    [0.89, 0.12, -0.34, ...] → 0.23 ❌  │
│                                                             │
│ Using HNSW index:                                           │
│ - Only checks ~log(n) vectors, not all                     │
│ - Still finds best matches                                  │
│                                                             │
│ Returns top 10 matches sorted by similarity                 │
│ Time: ~10-15ms                                              │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 5: Enhance with Text Search (Hybrid)                  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Also search with keywords:                                  │
│ - "backup" → keyword search in Tantivy                     │
│ - "VM" → keyword search in Tantivy                         │
│                                                             │
│ Combine results:                                            │
│ - Vector search: semantic similarity                        │
│ - Text search: exact keyword matches                        │
│ - Merge and re-rank                                         │
│                                                             │
│ Time: ~5-10ms                                               │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 6: Format and Return Results                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ {                                                           │
│   "results": [                                              │
│     {                                                       │
│       "api": "VirtualMachine::create_snapshot",            │
│       "relevance": 0.87,                                    │
│       "module": "vim_rs::mo::VirtualMachine",              │
│       "description": "Creates a snapshot of VM state...",   │
│       "example": "vm.create_snapshot(...).await?",         │
│       "privileges": ["VirtualMachine.State.CreateSnapshot"]│
│     },                                                      │
│     {                                                       │
│       "api": "ScheduledTaskManager::create_scheduled_task",│
│       "relevance": 0.79,                                    │
│       "description": "Create automated scheduled task...", │
│       "example": "task_mgr.create_scheduled_task(...)",    │
│       ...                                                   │
│     }                                                       │
│   ],                                                        │
│   "workflow_tip": "For automated backups, combine..."     │
│ }                                                           │
│                                                             │
│ Total time: <50ms                                           │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│ Step 7: Claude Uses Results                                │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ Claude receives results and responds to user:              │
│                                                             │
│ "To backup VMs, you can use VirtualMachine::create_snapshot│
│ which creates a point-in-time snapshot. Here's an example: │
│                                                             │
│ ```rust                                                     │
│ let snapshot = vm.create_snapshot(                         │
│     "backup-2024-01-15",                                   │
│     Some("Daily backup"),                                  │
│     true,  // Include memory                               │
│     true   // Quiesce filesystem                           │
│ ).await?;                                                   │
│ ```                                                         │
│                                                             │
│ For automated backups, you can combine this with           │
│ ScheduledTaskManager::create_scheduled_task..."           │
└─────────────────────────────────────────────────────────────┘
```

## Key Concepts Explained

### Token vs Word

```
Tokens ≠ Words exactly

"VirtualMachine" → might be 2-3 tokens
"create"         → 1 token
"snapshot"       → 1 token
"https://..."    → many tokens

Rule of thumb: 1 token ≈ 0.75 words in English
```

### Chunk Size: Why 200-512 Tokens?

```
Too Small (< 100 tokens):
  ❌ Loses context
  ❌ Fragment sentences
  Example: "creates a snapshot" (not enough info)

Sweet Spot (200-512 tokens):
  ✅ Complete paragraphs
  ✅ Full API description
  ✅ Enough context
  Example: Full method doc + parameters + example

Too Large (> 1000 tokens):
  ❌ Multiple topics mixed
  ❌ Less precise matching
  Example: Entire page with many unrelated APIs
```

### Why Overlap Chunks?

```
Without overlap:
  Chunk 1: "...creates a snapshot of VM."
  Chunk 2: "The snapshot includes memory..."

  Problem: "snapshot includes memory" split across chunks!

With 50-token overlap:
  Chunk 1: "...creates a snapshot of VM. The snapshot includes..."
  Chunk 2: "...snapshot of VM. The snapshot includes memory..."

  ✅ Both chunks have full context
```

### Vector Dimensions: 384 vs 768 vs 1536

```
Higher dimensions = More precise, but:
- Larger storage
- Slower search
- Diminishing returns

For our case (technical docs):
  384 dimensions: ✅ Excellent quality, fast, small
  768 dimensions: ⚠️  Slightly better, 2x slower, 2x storage
  1536 dimensions: ⚠️  Minimal improvement, 4x slower, 4x storage

Recommendation: 384 (all-MiniLM-L6-v2)
```

### Do We Need LLM on Client?

```
NO - Embedding model ≠ Chat LLM!

Chat LLM (Claude, GPT-4):
  - Size: 100+ GB
  - Purpose: Generate text, answer questions
  - Runs: In cloud (too large for local)

Embedding model (MiniLM):
  - Size: 90 MB
  - Purpose: Convert text → vector
  - Runs: Locally, CPU, fast (<20ms)

We only need the small embedding model!
```

## Math Behind Similarity

### Cosine Similarity (Simplified)

```
Two vectors:
  A = [0.2, 0.8, 0.4]
  B = [0.3, 0.7, 0.5]

Cosine similarity = dot product / (norm(A) * norm(B))

Dot product: 0.2*0.3 + 0.8*0.7 + 0.4*0.5 = 0.82
Norm(A): sqrt(0.04 + 0.64 + 0.16) = 0.92
Norm(B): sqrt(0.09 + 0.49 + 0.25) = 0.91

Similarity: 0.82 / (0.92 * 0.91) = 0.98

Interpretation:
  1.0  = Identical vectors (same meaning)
  0.8+ = Very similar (good match)
  0.5  = Somewhat similar
  0.0  = Unrelated
  -1.0 = Opposite meaning
```

### Why This Works

Neural networks learn patterns from millions of examples:
- "backup" and "snapshot" appear in similar contexts → similar vectors
- "create" and "make" are synonyms → similar vectors
- "VM" and "VirtualMachine" refer to same concept → similar vectors

So when we search "backup VMs", the vector is automatically close to:
- "create snapshot"
- "scheduled backup"
- "VM protection"

Even though exact words don't match!

## Summary - Simple Mental Model

**Build Time (Once):**
1. Collect all API docs and guides
2. Split into ~300-word chunks
3. Convert each chunk to 384 numbers (embedding)
4. Store in searchable database
5. Ship everything

**Runtime (Every Query):**
1. User asks question
2. Convert question to 384 numbers (same process)
3. Find chunks with similar numbers (fast math)
4. Return matching API docs
5. Done in <50ms!

**No Magic:**
- Just neural networks that learned from lots of text
- Math to find similar number patterns
- Fast because we pre-computed everything
