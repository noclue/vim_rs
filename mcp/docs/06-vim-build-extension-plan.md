# vim_build Extension Implementation Plan

This document details how to extend vim_build to generate MCP-ready JSON data files.

## Current vim_build Architecture

### Overview

```
vim_build/
├── src/
│   ├── main.rs              # Entry point
│   ├── generator.rs         # Orchestrates generation
│   ├── printer.rs           # Output abstraction
│   ├── vim_model/          # Internal data model
│   │   ├── types.rs        # Model, Struct, Enum, ManagedObject, etc.
│   │   ├── loader.rs       # OpenAPI → vim_model transformation
│   │   ├── cycles.rs       # Detect cyclic references
│   │   └── struct_order.rs # Dependency ordering
│   └── rs_emitter/         # Rust code generation
│       ├── mo.rs           # Managed object emitter
│       ├── structs.rs      # Struct emitter
│       ├── enums.rs        # Enum emitter
│       ├── traits.rs       # Trait emitter
│       └── ...
└── data/
    └── vi_json_openapi_specification_v9_0_0_0_24798170.json
```

### Current Workflow

```
1. Load OpenAPI spec (generator.rs::load_openapi)
   ↓
2. Transform to vim_model (vim_model::loader::load_vim_model)
   ├─ Parse schemas → Model { enums, structs, managed_objects }
   ├─ Compute hierarchy
   ├─ Mark cycles
   └─ Reorder structs
   ↓
3. Generate Rust code (generator.rs::generate_bindings)
   ├─ emit_types() → vim_rs/src/types/
   │   ├─ structs.rs
   │   ├─ enums.rs
   │   ├─ traits.rs
   │   └─ ...
   └─ emit_managed_objects() → vim_rs/src/mo/
       ├─ virtual_machine.rs
       ├─ host_system.rs
       └─ ...
```

### Key Data Structures

```rust
// vim_model/types.rs

pub struct Model {
    pub enums: IndexMap<String, Enum>,
    pub structs: IndexMap<String, RefCell<Struct>>,
    pub request_types: IndexMap<String, RefCell<Struct>>,
    pub any_value_types: IndexMap<String, BoxType>,
    pub managed_objects: IndexMap<String, ManagedObject>,
}

pub struct ManagedObject {
    pub name: String,
    pub description: Option<String>,
    pub methods: Vec<Method>,
}

pub struct Method {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub http_method: HttpMethod,
    pub input: Option<DataType>,
    pub output: Option<DataType>,
    pub output_description: Option<String>,
    pub error_description: Option<String>,
    pub optional_response: bool,
}

pub struct Struct {
    pub name: String,
    pub description: Option<String>,
    pub fields: IndexMap<String, Field>,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub last_child: String,
    pub emit_mode: EmitMode,
    // ...
}

pub struct Enum {
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<String>,
    // ...
}
```

---

## Implementation Strategy

### Option 1: New JSON Emitter (RECOMMENDED)

Add a new emitter that generates JSON files alongside Rust code generation.

**Pros:**
- Minimal changes to existing code
- Reuses vim_model transformation
- JSON generation separate from Rust generation
- Easy to test independently

**Cons:**
- Slight code duplication for name conversion

### Option 2: Extend Existing Emitters

Modify rs_emitter to also output JSON during Rust generation.

**Pros:**
- No code duplication

**Cons:**
- Mixes concerns (Rust gen + JSON gen)
- Harder to maintain
- Harder to test

**Decision: Use Option 1 - New JSON Emitter**

---

## Implementation Plan

### Phase 1: Create JSON Emitter Foundation (Week 1, Days 1-2)

#### Task 1.1: Create json_emitter Module

**Location:** `vim_build/src/json_emitter/mod.rs`

```rust
// vim_build/src/json_emitter/mod.rs

mod common;
mod managed_objects;
mod data_structures;
mod enumerations;
mod metadata;

pub use managed_objects::emit_managed_objects_json;
pub use data_structures::emit_data_structures_json;
pub use enumerations::emit_enumerations_json;
pub use metadata::emit_metadata_json;

use crate::vim_model::Model;
use std::path::Path;
use serde_json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// Generate all MCP JSON files
pub fn emit_mcp_data(
    model: &Model,
    output_dir: &Path,
    version: &str,
) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    let start = std::time::Instant::now();

    emit_managed_objects_json(model, output_dir, version)?;
    emit_data_structures_json(model, output_dir, version)?;
    emit_enumerations_json(model, output_dir, version)?;
    emit_metadata_json(model, output_dir, version, start.elapsed())?;

    Ok(())
}
```

**Files to create:**
- `vim_build/src/json_emitter/mod.rs`
- `vim_build/src/json_emitter/common.rs` - JSON schema types
- `vim_build/src/json_emitter/signature_generator.rs` - Method signature generation
- `vim_build/src/json_emitter/managed_objects.rs`
- `vim_build/src/json_emitter/data_structures.rs`
- `vim_build/src/json_emitter/enumerations.rs`
- `vim_build/src/json_emitter/metadata.rs`

#### Task 1.2: Define JSON Schema Types (Simplified v1)

**Location:** `vim_build/src/json_emitter/common.rs`

```rust
// vim_build/src/json_emitter/common.rs
// v1: Simplified schema - no parsing

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Managed Objects Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedObjectsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub managed_objects: Vec<ManagedObjectEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedObjectEntry {
    pub name: String,
    pub rust_module: String,
    pub rust_struct: String,
    pub description: Option<String>,
    pub methods: Vec<MethodEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodEntry {
    pub name: String,
    pub rust_name: String,
    pub signature: MethodSignature,
    pub description: Option<String>,  // Raw markdown, no parsing
    pub related_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodSignature {
    pub full: String,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: String,
    pub is_async: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub rust_type: String,
    pub required: bool,
    pub description: Option<String>,
}

// Data Structures Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStructuresOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub structures: Vec<StructureEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub emit_mode: String,
    pub skip_reason: Option<String>,
    pub fields: Vec<FieldEntry>,
    pub related_types: Vec<String>,
    pub inheritance_chain: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_type: String,
    pub vim_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub is_array: bool,
    pub is_boxed: bool,
    pub is_trait: bool,
    pub trait_name: Option<String>,
}

// Enumerations Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationsOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub enumerations: Vec<EnumerationEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationEntry {
    pub name: String,
    pub rust_name: String,
    pub rust_module: String,
    pub description: Option<String>,
    pub variants: Vec<VariantEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantEntry {
    pub name: String,
    pub rust_name: String,
    pub description: Option<String>,
    pub discriminator_value: String,
}

// Metadata Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataOutput {
    pub generated_at: DateTime<Utc>,
    pub source: String,
    pub statistics: Statistics,
    pub files_generated: Vec<String>,
    pub generation_duration_ms: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub managed_objects: usize,
    pub total_methods: usize,
    pub data_structures_total: usize,
    pub data_structures_emitted: usize,
    pub data_structures_pruned: usize,
    pub data_structures_skipped: usize,
    pub enumerations: usize,
    pub pruned_types: Vec<String>,
}
```

#### Task 1.3: Integrate into main.rs

```rust
// vim_build/src/main.rs

mod generator;
mod printer;
pub mod rs_emitter;
mod vim_model;
mod json_emitter;  // NEW

use generator::emit_vim_bindings;
use std::{path::Path, time::Instant};

static PRUNED_TYPES: [&str; 2] = ["MethodFault", "Event"];

fn main() {
    let root_folder = Path::new("../");
    let vi_json_spec_path = Path::new("data/vi_json_openapi_specification_v9_0_0_0_24798170.json");
    let mcp_output_dir = root_folder.join("mcp/data");  // Project root: vim_rs/mcp/data

    let start = Instant::now();

    // Generate Rust bindings
    emit_vim_bindings(vi_json_spec_path, root_folder, Some(&PRUNED_TYPES)).unwrap();
    println!("Total time in Rust generation: {:?}", start.elapsed());

    // Generate MCP JSON data  // NEW
    let mcp_start = Instant::now();
    emit_mcp_data(vi_json_spec_path, &mcp_output_dir, Some(&PRUNED_TYPES)).unwrap();
    println!("Total time in MCP JSON generation: {:?}", mcp_start.elapsed());
}

fn emit_mcp_data(
    vi_json_spec_path: &Path,
    output_dir: &Path,
    pruned_types: Option<&[&str]>,
) -> Result<(), Box<dyn std::error::Error>> {
    use json_emitter::emit_mcp_data;

    // Load and transform model (same as Rust generation)
    let openapi = generator::load_openapi(vi_json_spec_path)?;
    let vim_model = vim_model::load_vim_model(&openapi, pruned_types)?;

    // Extract version from file name
    let version = "9.0.0.0"; // TODO: Extract from filename

    // Generate JSON files
    emit_mcp_data(&vim_model, output_dir, version)?;

    Ok(())
}
```

**Deliverables:**
- ✅ json_emitter module structure
- ✅ JSON schema types defined
- ✅ Integration into main.rs
- ✅ Compiles without errors

---

### Phase 2: Implement Managed Objects Emitter (Week 1, Days 2-3)

#### Task 2.1: Generate Method Signatures

**Location:** `vim_build/src/json_emitter/signature_generator.rs`

```rust
use crate::vim_model::{Method, DataType, Model};
use crate::json_emitter::common::{MethodSignature, ParameterInfo};
use crate::rs_emitter::{TypeDefResolver, to_fn_name, to_type_name};

pub fn generate_method_signature(
    method: &Method,
    mo_name: &str,
    model: &Model,
) -> MethodSignature {
    let tdf = TypeDefResolver::new_with_root_package(model, "crate::types".to_string());

    // Generate parameters
    let mut params = Vec::new();
    if let Some(input_type) = &method.input {
        // Parse input type struct to extract fields
        if let DataType::Reference(type_name) = input_type {
            if let Some(request_struct) = model.request_types.get(type_name) {
                let req = request_struct.borrow();
                for (field_name, field) in &req.fields {
                    params.push(ParameterInfo {
                        name: field.rust_name(),
                        rust_type: tdf.resolve_to_rust_type(&field.vim_type, field.optional, field.require_box),
                        required: !field.optional,
                        description: field.description.clone(),
                    });
                }
            }
        }
    }

    // Generate return type
    let return_type = if let Some(output) = &method.output {
        format!("Result<{}>", tdf.resolve_to_rust_type(output, method.optional_response, false))
    } else {
        "Result<()>".to_string()
    };

    // Generate full signature
    let param_str = params.iter()
        .map(|p| format!("{}: {}", p.name, p.rust_type))
        .collect::<Vec<_>>()
        .join(", ");

    let full = format!(
        "pub async fn {}(&self{}{}) -> {}",
        to_fn_name(&method.name),
        if param_str.is_empty() { "" } else { ", " },
        param_str,
        return_type
    );

    MethodSignature {
        full,
        parameters: params,
        return_type,
        is_async: true,
    }
}
```

#### Task 2.2: Emit Managed Objects JSON (Simplified)

**Location:** `vim_build/src/json_emitter/managed_objects.rs`

```rust
use crate::json_emitter::common::*;
use crate::json_emitter::signature_generator;
use crate::vim_model::{Model, Method, DataType};
use crate::rs_emitter::to_fn_name;
use std::path::Path;
use chrono::Utc;

pub fn emit_managed_objects_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let mut managed_objects = Vec::new();

    for (mo_name, mo) in &model.managed_objects {
        if mo.methods.is_empty() {
            continue; // Skip MOs without methods
        }

        let mut methods = Vec::new();
        for method in &mo.methods {
            let signature = signature_generator::generate_method_signature(
                method, mo_name, model
            );

            let related_types = extract_related_types(method);

            methods.push(MethodEntry {
                name: method.name.clone(),
                rust_name: to_fn_name(&method.name),
                signature,
                description: method.description.clone(),  // Raw markdown - no parsing!
                related_types,
            });
        }

        managed_objects.push(ManagedObjectEntry {
            name: mo_name.clone(),
            rust_module: format!("vim_rs::mo::{}", mo_name),
            rust_struct: mo_name.clone(),
            description: mo.description.clone(),
            methods,
        });
    }

    let output = ManagedObjectsOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        managed_objects,
    };

    let output_path = output_dir.join("managed_objects.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}

fn extract_related_types(method: &Method) -> Vec<String> {
    let mut types = Vec::new();

    // Add input types
    if let Some(DataType::Reference(type_name)) = &method.input {
        types.push(type_name.clone());
    }

    // Add output types
    if let Some(DataType::Reference(type_name)) = &method.output {
        types.push(type_name.clone());
    }

    types.sort();
    types.dedup();
    types
}
```

**Deliverables:**
- ✅ Signature generator
- ✅ Managed objects JSON emitter
- ✅ Generated managed_objects.json validates

---

### Phase 3: Implement Data Structures & Enums Emitters (Week 1, Days 3-4)

#### Task 3.1: Emit Data Structures JSON

**Location:** `vim_build/src/json_emitter/data_structures.rs`

```rust
use crate::json_emitter::common::*;
use crate::vim_model::{Model, EmitMode};
use crate::rs_emitter::TypeDefResolver;
use std::path::Path;
use chrono::Utc;

pub fn emit_data_structures_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let tdf = TypeDefResolver::new_with_root_package(model, "crate::types".to_string());
    let mut structures = Vec::new();

    for (name, struct_ref) in &model.structs {
        let s = struct_ref.borrow();

        let emit_mode = match &s.emit_mode {
            EmitMode::Emit => "Emit",
            EmitMode::Prune => "Prune",
            EmitMode::Skip(_) => "Skip",
        };

        let skip_reason = match &s.emit_mode {
            EmitMode::Skip(parent) => Some(format!("Descendant of pruned type {}", parent)),
            _ => None,
        };

        let mut fields = Vec::new();
        for (field_name, field) in &s.fields {
            fields.push(FieldEntry {
                name: field_name.clone(),
                rust_name: field.rust_name(),
                rust_type: tdf.resolve_to_rust_type(&field.vim_type, field.optional, field.require_box),
                vim_type: field.vim_type.to_string(),
                required: !field.optional,
                description: field.description.clone(),
                is_array: matches!(&field.vim_type, DataType::Array(_)),
                is_boxed: field.require_box,
                is_trait: field.vim_type.has_trait(model),
                trait_name: field.vim_type.trait_name(),
            });
        }

        structures.push(StructureEntry {
            name: name.clone(),
            rust_name: s.rust_name(),
            rust_module: "vim_rs::types::structs".to_string(),
            description: s.description.clone(),
            parent: s.parent.clone(),
            children: s.children.clone(),
            emit_mode: emit_mode.to_string(),
            skip_reason,
            fields,
            related_types: extract_related_types(&s),
            inheritance_chain: model.inheritance_chain(name).unwrap_or_default(),
        });
    }

    let output = DataStructuresOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        structures,
    };

    let output_path = output_dir.join("data_structures.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}
```

#### Task 3.2: Emit Enumerations JSON

**Location:** `vim_build/src/json_emitter/enumerations.rs`

```rust
use crate::json_emitter::common::*;
use crate::vim_model::Model;
use std::path::Path;
use chrono::Utc;

pub fn emit_enumerations_json(
    model: &Model,
    output_dir: &Path,
) -> super::Result<()> {
    let mut enumerations = Vec::new();

    for (name, enum_def) in &model.enums {
        let rust_name = name.trim_end_matches("_enum");

        let variants = enum_def.variants.iter().map(|v| {
            VariantEntry {
                name: v.clone(),
                rust_name: to_pascal_case(v),
                description: None,  // Can enhance later
                discriminator_value: v.clone(),
            }
        }).collect();

        enumerations.push(EnumerationEntry {
            name: name.clone(),
            rust_name: rust_name.to_string(),
            rust_module: "vim_rs::types::enums".to_string(),
            description: enum_def.description.clone(),
            variants,
        });
    }

    let output = EnumerationsOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        enumerations,
    };

    let output_path = output_dir.join("enumerations.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}
```

#### Task 3.3: Emit Metadata JSON

**Location:** `vim_build/src/json_emitter/metadata.rs`

```rust
use crate::json_emitter::common::*;
use crate::vim_model::Model;
use std::path::Path;
use std::time::Duration;
use chrono::Utc;

pub fn emit_metadata_json(
    model: &Model,
    output_dir: &Path,
    generation_duration: Duration,
) -> super::Result<()> {
    let output = MetadataOutput {
        generated_at: Utc::now(),
        source: "vim_model processed from OpenAPI specification".to_string(),
        statistics: Statistics {
            managed_objects: model.managed_objects.len(),
            total_methods: model.managed_objects.values()
                .map(|mo| mo.methods.len())
                .sum(),
            data_structures_total: model.structs.len(),
            data_structures_emitted: model.structs.values()
                .filter(|s| matches!(s.borrow().emit_mode, EmitMode::Emit))
                .count(),
            data_structures_pruned: model.structs.values()
                .filter(|s| matches!(s.borrow().emit_mode, EmitMode::Prune))
                .count(),
            data_structures_skipped: model.structs.values()
                .filter(|s| matches!(s.borrow().emit_mode, EmitMode::Skip(_)))
                .count(),
            enumerations: model.enums.len(),
            pruned_types: vec!["MethodFault".to_string(), "Event".to_string()],
        },
        files_generated: vec![
            "managed_objects.json".to_string(),
            "data_structures.json".to_string(),
            "enumerations.json".to_string(),
        ],
        generation_duration_ms: generation_duration.as_millis(),
    };

    let output_path = output_dir.join("metadata.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}
```

**Deliverables:**
- ✅ Data structures JSON emitter (includes ALL types)
- ✅ Enumerations JSON emitter
- ✅ Metadata JSON emitter
- ✅ All four JSON files generated and validate

---

### Phase 4: Testing & Validation (Week 1, Day 5)

#### Task 4.1: Unit Tests

```rust
// vim_build/src/json_emitter/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_generation() {
        // Test method signature generation
        let model = create_test_model();
        let method = create_test_method();
        let sig = generate_method_signature(&method, "VirtualMachine", &model);
        assert!(sig.full.contains("pub async fn"));
        assert!(sig.is_async);
    }

    #[test]
    fn test_related_types_extraction() {
        let method = create_test_method();
        let types = extract_related_types(&method);
        assert!(types.len() > 0);
    }

    #[test]
    fn test_emit_mode_serialization() {
        // Test that emit_mode is correctly serialized
        let entry = StructureEntry {
            emit_mode: "Skip".to_string(),
            skip_reason: Some("Descendant of pruned type Event".to_string()),
            // ...
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("Skip"));
    }

    // More tests...
}
```

#### Task 4.2: Integration Tests

```rust
// vim_build/tests/json_generation_test.rs

#[test]
fn test_full_json_generation() {
    let spec_path = Path::new("data/vi_json_openapi_specification_v9_0_0_0_24798170.json");
    let output_dir = tempfile::tempdir().unwrap();

    emit_mcp_data(spec_path, output_dir.path(), None).unwrap();

    // Verify files exist
    assert!(output_dir.path().join("managed_objects.json").exists());
    assert!(output_dir.path().join("data_structures.json").exists());
    assert!(output_dir.path().join("enumerations.json").exists());
    assert!(output_dir.path().join("metadata.json").exists());

    // Verify JSON is valid
    let mo_json = std::fs::read_to_string(output_dir.path().join("managed_objects.json")).unwrap();
    let mo: ManagedObjectsOutput = serde_json::from_str(&mo_json).unwrap();
    assert!(mo.managed_objects.len() > 0);
}
```

#### Task 4.3: Validation Scripts

```rust
// vim_build/tools/validate_mcp_json.rs

// Validate:
// 1. JSON schema compliance
// 2. Cross-references are valid
// 3. Statistics match
// 4. Rust names match generated .rs files
```

**Deliverables:**
- ✅ Unit tests passing
- ✅ Integration tests passing
- ✅ Validation script confirms data quality

---

## Dependencies

### New Cargo Dependencies (v1 Simplified)

```toml
# vim_build/Cargo.toml

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
# Note: No regex needed for v1 (no parsing)

# Existing dependencies
openapi30 = { path = "../openapi30" }
convert_case = "0.6"
check_keyword = "0.3"
indexmap = { version = "2.1", features = ["serde"] }
log = "0.4"

[dev-dependencies]
tempfile = "3.8"
```

---

## Timeline (Simplified v1 - No Parsing)

### Week 1: Implementation & Testing
- **Day 1:** Phase 1 - Foundation & JSON schemas
- **Day 2-3:** Phase 2 - Managed objects emitter with signatures
- **Day 3-4:** Phase 3 - Data structures & enums emitters
- **Day 5:** Phase 4 - Testing & validation

**Total:** 5 days

**Key Simplifications from Original Plan:**
- ✅ No description parsing (saves ~4 days)
- ✅ No regex patterns for privileges/errors
- ✅ No complex field extraction
- ✅ Simpler schema types
- ✅ Faster development & testing

**v2 Features (Future):**
- Add structured fields for filtering (privileges, errors, deprecation)
- Enhanced metadata and cross-references
- Incremental generation support

---

## Success Criteria (v1 Simplified)

### Functional Requirements
- ✅ Generates valid JSON files for all four categories (MOs, structs, enums, metadata)
- ✅ ALL types included (Emit, Prune, Skip modes)
- ✅ Rust names match generated code where applicable
- ✅ Descriptions preserved as raw markdown
- ✅ Cross-references are valid
- ✅ Output to `{project_root}/mcp/data/`

### Quality Requirements
- ✅ Unit test coverage >70% (simpler code, fewer tests needed)
- ✅ Integration tests pass
- ✅ JSON schema validation passes
- ✅ JSON files compress to <50MB total
- ✅ Generation completes in <30 seconds

### Documentation Requirements
- ✅ JSON schema documented
- ✅ Usage examples provided
- ✅ vim_build changes documented
- ✅ Migration guide for future updates

---

## Risk Mitigation (v1 Simplified)

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Type resolution errors | High | Low | Reuse existing TypeDefResolver, comprehensive tests |
| JSON files too large | Low | Low | Gzip compression, streaming for MCP |
| Performance issues | Low | Low | Optimize only if >30 seconds generation time |
| Missing types in output | Medium | Low | Include ALL emit modes, validation checks |
| Path resolution issues | Medium | Low | Use consistent path handling, integration tests |

**Risks Removed from v1:**
- ❌ Description parsing edge cases (no parsing!)
- ❌ Regex pattern failures (no regex!)
- ❌ Privilege extraction errors (deferred to v2)

---

## Future Enhancements

### v0.0.2 - Description Parsing & Structured Fields

1. **Description Parsing**
   - Parse privileges into structured field
   - Parse errors into array of error types
   - Extract deprecation info into dedicated fields
   - Parse version information

2. **Enhanced Metadata**
   - Summary field (first line of description)
   - Structured privilege requirements
   - Error type cross-references
   - Version-specific behavior notes

### v0.0.3+ - Advanced Features

1. **Incremental Generation**
   - Only regenerate changed types
   - Track version differences
   - Migration guides between versions

2. **Additional Metadata**
   - Performance characteristics
   - Typical usage patterns
   - Code examples extraction

3. **Validation Improvements**
   - JSON schema validation
   - Semantic consistency checks
   - Coverage reports

---

## Next Steps (v1 Simplified - 5 Days)

1. **Review this simplified plan** - Confirm v1 approach (no parsing)
2. **Create feature branch** - `feature/mcp-v1-json-generation`
3. **Day 1: Phase 1** - Set up json_emitter foundation with simplified schemas
4. **Day 2-3: Phase 2** - Implement managed objects emitter with signatures
5. **Day 3-4: Phase 3** - Implement data structures & enums emitters
6. **Day 5: Phase 4** - Testing, validation, and documentation
7. **Commit & Push** - Output to `vim_rs/mcp/data/`

**Key v1 Goals:**
- ✅ Generate all JSON files with raw markdown descriptions
- ✅ Include ALL types (Emit, Prune, Skip)
- ✅ Fast iteration (5 days vs 9-10 days)
- ✅ Solid foundation for v2 enhancements
