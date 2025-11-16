# Data Transformer

Orchestrator program that runs all data processing tools in sequence to transform raw vSphere documentation and API specifications into structured data for the vim_rs MCP server.

## Overview

The Data Transformer coordinates six processing steps that convert:
- PDF documentation → structured markdown chunks
- OpenAPI specifications → API definition JSON files
- Code examples → indexed example JSON
- All structured data → vector embeddings database

## Workflow

The transformer executes the following steps in order, with each step depending on the output of previous steps:

```mermaid
graph TD
    A[PDF Files<br/>mcp/data/guides/pdf/] -->|Step 1: PDF Parser| B[TXT Files<br/>mcp/data/guides/txt/]
    B -->|Step 2: Text Processor| C[Markdown Files<br/>mcp/data/guides/md/]
    C -->|Step 3: Build Guides| D[Guide JSON<br/>mcp/data/api_definitions/]
    
    E[OpenAPI Spec<br/>vim_build/data/] -->|Step 4: Build API Definitions| F[API Definition JSON<br/>mcp/data/api_definitions/]
    
    G[Example Code<br/>examples/] -->|Step 5: Build Examples| H[Examples JSON<br/>mcp/data/api_definitions/]
    
    D -->|Step 6: Build Embeddings| I[Embeddings Database<br/>mcp/data/embeddings.lancedb/]
    F -->|Step 6: Build Embeddings| I
    H -->|Step 6: Build Embeddings| I

```

### Step Details

1. **PDF Parser** (`pdf_parser`)
   - Extracts text content from PDF documentation files
   - Input: `mcp/data/guides/pdf/*.pdf`
   - Output: `mcp/data/guides/txt/*.txt`

2. **Text Processor** (`text_processor`)
   - Processes raw text files into structured markdown
   - Parses table of contents, marks headings, cleans up formatting
   - Input: `mcp/data/guides/txt/*.txt`
   - Output: `mcp/data/guides/md/*.md`

3. **Build Guides** (`build_guides`)
   - Chunks markdown files into semantic units
   - Extracts topics and creates searchable guide chunks
   - Input: `mcp/data/guides/md/vmware-vsphere-9-0.md`
   - Output: `mcp/data/api_definitions/vmware-vsphere-9-0_guide.json`

4. **Build API Definitions** (`build_api_definitions`)
   - Transforms OpenAPI specification into structured JSON
   - Generates managed objects, methods, data structures, enumerations, and traits
   - Input: `vim_build/data/vi_json_openapi_specification_v9_0_0_0_24798170.json`
   - Output: `mcp/data/api_definitions/*.json` (multiple files)

5. **Build Examples** (`build_examples`)
   - Scans code examples directory and indexes them
   - Extracts metadata, descriptions, and categories
   - Input: `examples/**/*.rs`
   - Output: `mcp/data/api_definitions/examples.json`

6. **Build Embeddings** (`build_embeddings`)
   - Generates vector embeddings for all structured data
   - Creates searchable LanceDB database
   - Input: All JSON files in `mcp/data/api_definitions/`
   - Output: `mcp/data/embeddings.lancedb/`

## Usage

### Prerequisites

- Rust toolchain (stable)
- CUDA-capable GPU (optional, for faster embeddings generation)
- PDF files in `mcp/data/guides/pdf/`
- OpenAPI specification in `vim_build/data/`

### Running the Transformer

From the workspace root:

```bash
cargo run --bin data-transformer
```

Or from the `data_transformer` directory:

```bash
cargo run
```

The program will:
- Calculate paths relative to the workspace root automatically
- Execute each step sequentially
- Log progress and timing information to stderr
- Stop on first error with clear error messages

### Output

All output is written to `mcp/data/`:
- `mcp/data/guides/txt/` - Extracted text files
- `mcp/data/guides/md/` - Processed markdown files
- `mcp/data/api_definitions/` - All JSON definition files
- `mcp/data/embeddings.lancedb/` - Vector embeddings database
- `mcp/data/model_cache/` - Cached ML models

## Dependencies

The transformer depends on all data processing tools:
- `pdf_parser` - PDF text extraction
- `text-processor` - Text to markdown conversion
- `build_guides` - Markdown chunking
- `build-api-definitions` - API definition generation
- `build-examples` - Example indexing
- `build-embeddings` - Embedding generation (with optional CUDA support)

## Error Handling

Each step is executed independently with error handling:
- If any step fails, the transformer stops immediately
- Clear error messages indicate which step failed
- Previous steps' outputs are preserved
- Partial runs can be resumed by running the transformer again

## Performance

The transformer logs timing information for each step:
- PDF parsing: Depends on PDF file sizes
- Text processing: Fast, processes multiple files
- Guide building: Moderate, processes large markdown files
- API definitions: Moderate, processes large OpenAPI spec
- Example indexing: Fast, scans Rust source files
- Embedding generation: Slowest step, benefits from CUDA acceleration

Total execution time varies but typically takes several minutes for a complete run.

## Data Model

The transformer produces a structured data model that represents the vSphere API and documentation. The following diagram illustrates the relationships between different data entities:

```mermaid
erDiagram
    ManagedObject ||--o{ Method : "has"
    Method ||--o{ Parameter : "has"
    Method }o--|| DataStructure : "returns"
    Method }o--|| Enumeration : "returns"
    Method }o--|| Trait : "returns"
    
    Parameter }o--|| DataStructure : "references"
    Parameter }o--|| Enumeration : "references"
    Parameter }o--|| Trait : "references"
    
    DataStructure ||--o{ Field : "has"
    Field }o--|| DataStructure : "references"
    Field }o--|| Enumeration : "references"
    Field }o--|| Trait : "references"
    
    DataStructure ||--o| DataStructure : "parent-child"
    DataStructure }o--o{ Trait : "implements"
    Trait ||--o{ DataStructure : "implemented_by"
    
    GuideChunk }o--o{ ManagedObject : "mentions"
    GuideChunk }o--o{ Method : "mentions"
    GuideChunk }o--o{ DataStructure : "mentions"
    
    CodeExample }o--o{ ManagedObject : "demonstrates"
    CodeExample }o--o{ Method : "demonstrates"
    
    ManagedObject {
        string name
        string rust_module
        string rust_struct
        string description
        Method[] methods
    }
    
    Method {
        string name
        string rust_name
        MethodSignature signature
        string description
        string[] related_types
    }
    
    Parameter {
        string name
        string rust_type
        bool required
        string description
    }
    
    DataStructure {
        string name
        string rust_name
        string rust_module
        string description
        string parent
        string[] children
        Field[] fields
        string[] related_types
        string[] implements_traits
        string[] inheritance_chain
    }
    
    Field {
        string name
        string rust_name
        string rust_type
        bool required
        bool is_trait
        string trait_name
    }
    
    Trait {
        string name
        string rust_name
        string rust_module
        string description
        string parent_trait
        Getter[] getters
        string[] implementing_types
    }
    
    Enumeration {
        string name
        string rust_name
        string rust_module
        string description
        Variant[] variants
    }
    
    GuideChunk {
        string heading_h1
        string heading_h2
        string heading_h3
        string content
        string[] topics
        string chunk_id
    }
    
    CodeExample {
        string name
        string title
        string description
        string category
        string source_code
    }
```

### Entity Relationships

**Core API Entities:**

1. **Managed Objects** (e.g., `VirtualMachine`, `HostSystem`)
   - Contain multiple **Methods** that can be invoked
   - Represent vSphere managed entities

2. **Methods** (e.g., `PowerOnVM_Task`, `CreateVM`)
   - Belong to a **Managed Object**
   - Have **Parameters** that reference **Data Structures**, **Enumerations**, or **Traits**
   - Return **Data Structures**, **Enumerations**, or **Traits**
   - Track `related_types` for type discovery

3. **Data Structures** (e.g., `VirtualMachineConfigSpec`, `HostConfigInfo`)
   - Contain **Fields** that reference other types
   - Support inheritance: have `parent` and `children`
   - Implement **Traits** when they have child types (polymorphic)
   - Fields can reference **Traits** for polymorphic fields
   - Track `related_types` and `inheritance_chain`

4. **Traits** (e.g., `VirtualDeviceTrait`, `MethodFaultTrait`)
   - Represent polymorphic interfaces
   - Have multiple **Data Structures** that implement them
   - Provide getter methods for accessing common fields
   - Support trait inheritance via `parent_trait`

5. **Enumerations** (e.g., `VirtualMachinePowerState`, `TaskInfoState`)
   - Contain **Variants** with discriminator values
   - Used as field types and method parameters/returns

**Documentation & Examples:**

6. **Guide Chunks**
   - Documentation segments extracted from PDF guides
   - Linked to API entities through topic keywords
   - Organized by hierarchical headings (H1/H2/H3)

7. **Code Examples**
   - Demonstrative code snippets
   - Categorized by usage pattern (connection, property_collector, etc.)
   - Reference **Managed Objects** and **Methods** in source code

### Key Relationships

- **Method → Type References**: Methods connect to types through:
  - Parameter types (`signature.parameters[].rust_type`)
  - Return types (`signature.return_type`)
  - Related types (`related_types[]`)

- **Data Structure → Trait**: When a structure has child types, it implements a trait that provides a polymorphic interface. The trait lists all implementing types in `implementing_types[]`.

- **Field → Type References**: Fields reference types through:
  - Direct type (`rust_type`)
  - Trait type (`is_trait: true`, `trait_name`)

- **Inheritance**: Data structures form inheritance hierarchies via `parent`/`children` relationships, with `inheritance_chain` tracking the full path.

- **Cross-References**: The `related_types` arrays in Methods and Data Structures enable discovery of interconnected types.

This data model enables semantic search, type navigation, and intelligent code generation for the vSphere API.
