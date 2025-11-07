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
- `vim_build/src/json_emitter/common.rs` - Shared utilities
- `vim_build/src/json_emitter/managed_objects.rs`
- `vim_build/src/json_emitter/data_structures.rs`
- `vim_build/src/json_emitter/enumerations.rs`
- `vim_build/src/json_emitter/metadata.rs`

#### Task 1.2: Define JSON Schema Types

**Location:** `vim_build/src/json_emitter/common.rs`

```rust
// vim_build/src/json_emitter/common.rs

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

// Managed Objects Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedObjectsOutput {
    pub version: String,
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
    pub description: Option<String>,
    pub summary: Option<String>,
    pub openapi_operation_id: String,
    pub openapi_path: String,
    pub openapi_http_method: String,
    pub deprecated: bool,
    pub deprecation_note: Option<String>,
    pub privileges: PrivilegeInfo,
    pub errors: Vec<ErrorInfo>,
    pub related_types: Vec<String>,
    pub tags: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivilegeInfo {
    pub raw: Option<String>,
    pub parsed: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "type")]
    pub error_type: String,
    pub description: String,
}

// Data Structures Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStructuresOutput {
    pub version: String,
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
    pub is_abstract: bool,
    pub emit_mode: String,
    pub fields: Vec<FieldEntry>,
    pub used_by_methods: Vec<MethodUsage>,
    pub related_types: Vec<String>,
    pub inheritance_chain: Vec<String>,
    pub tags: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodUsage {
    pub managed_object: String,
    pub method: String,
    pub role: String, // "input", "output", "field"
}

// Enumerations Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumerationsOutput {
    pub version: String,
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
    pub used_by_structures: Vec<StructureUsage>,
    pub used_by_methods: Vec<MethodUsage>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantEntry {
    pub name: String,
    pub rust_name: String,
    pub description: Option<String>,
    pub discriminator_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureUsage {
    pub structure: String,
    pub field: String,
}

// Metadata Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataOutput {
    pub version: String,
    pub vsphere_version: String,
    pub build_number: String,
    pub generated_at: DateTime<Utc>,
    pub vim_rs_version: String,
    pub source_files: SourceFiles,
    pub statistics: Statistics,
    pub indexes_generated: Vec<String>,
    pub generation_duration_ms: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFiles {
    pub openapi_spec: String,
    pub vim_build_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub managed_objects: usize,
    pub total_methods: usize,
    pub data_structures: usize,
    pub enumerations: usize,
    pub request_types: usize,
    pub pruned_types: Vec<String>,
}

// Utility functions

pub fn extract_summary(description: &Option<String>) -> Option<String> {
    description.as_ref().map(|d| {
        d.lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
    })
}

pub fn extract_tags(name: &str, description: &Option<String>) -> Vec<String> {
    let mut tags = Vec::new();

    // Split camelCase/PascalCase name into words
    tags.extend(split_camel_case(name));

    // Extract keywords from description
    if let Some(desc) = description {
        // Simple keyword extraction (can be enhanced)
        for word in desc.split_whitespace() {
            let word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if word.len() > 3 && !is_common_word(word) {
                tags.push(word.to_lowercase());
            }
        }
    }

    // Deduplicate
    tags.sort();
    tags.dedup();
    tags
}

fn split_camel_case(s: &str) -> Vec<String> {
    // Split "VirtualMachine" into ["virtual", "machine"]
    let mut result = Vec::new();
    let mut current = String::new();

    for c in s.chars() {
        if c.is_uppercase() && !current.is_empty() {
            result.push(current.to_lowercase());
            current = String::new();
        }
        current.push(c);
    }

    if !current.is_empty() {
        result.push(current.to_lowercase());
    }

    result
}

fn is_common_word(word: &str) -> bool {
    matches!(word.to_lowercase().as_str(),
        "the" | "and" | "for" | "that" | "with" | "this" | "from" | "are" | "was"
    )
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
    let mcp_output_dir = Path::new("data/mcp");  // NEW

    let start = Instant::now();

    // Generate Rust bindings
    emit_vim_bindings(vi_json_spec_path, root_folder, Some(&PRUNED_TYPES)).unwrap();
    println!("Total time in Rust generation: {:?}", start.elapsed());

    // Generate MCP JSON data  // NEW
    let mcp_start = Instant::now();
    emit_mcp_data(vi_json_spec_path, mcp_output_dir, Some(&PRUNED_TYPES)).unwrap();
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

### Phase 2: Implement Managed Objects Emitter (Week 1, Days 3-4)

#### Task 2.1: Parse Method Descriptions

**Location:** `vim_build/src/json_emitter/description_parser.rs`

```rust
// Parse privileges from description
pub fn parse_privileges(description: &Option<String>) -> PrivilegeInfo {
    let mut privileges = PrivilegeInfo {
        raw: None,
        parsed: Vec::new(),
    };

    let Some(desc) = description else {
        return privileges;
    };

    // Look for: ***Required privileges:*** Privilege.Name
    let privilege_regex = regex::Regex::new(
        r"\*\*\*Required privileges:\*\*\*\s*(.+?)(?:\n|$)"
    ).unwrap();

    if let Some(cap) = privilege_regex.captures(desc) {
        let priv_text = cap.get(1).unwrap().as_str().trim();
        privileges.raw = Some(priv_text.to_string());

        // Parse privilege names
        // Format: "Priv.Name" or "Priv1, Priv2" or complex descriptions
        for part in priv_text.split(',') {
            let part = part.trim();
            // Extract words that look like privilege names
            if let Some(priv_name) = extract_privilege_name(part) {
                privileges.parsed.push(priv_name);
            }
        }
    }

    privileges
}

pub fn parse_errors(description: &Option<String>) -> Vec<ErrorInfo> {
    let mut errors = Vec::new();

    let Some(desc) = description else {
        return errors;
    };

    // Look for: ***ErrorType***: description
    let error_regex = regex::Regex::new(
        r"\*\*\*(\w+)\*\*\*:\s*(.+?)(?=\n\*\*\*|\n\n|$)"
    ).unwrap();

    for cap in error_regex.captures_iter(desc) {
        let error_type = cap.get(1).unwrap().as_str().to_string();
        let error_desc = cap.get(2).unwrap().as_str().trim().to_string();

        // Skip if it's "Required privileges" (already parsed)
        if error_type != "Required" {
            errors.push(ErrorInfo {
                error_type,
                description: error_desc,
            });
        }
    }

    errors
}

pub fn check_deprecated(description: &Option<String>) -> (bool, Option<String>) {
    let Some(desc) = description else {
        return (false, None);
    };

    // Look for: "Deprecated as of vSphere X.Y, ..."
    let deprecated_regex = regex::Regex::new(
        r"Deprecated as of (.+?)(?:\.|,)"
    ).unwrap();

    if let Some(cap) = deprecated_regex.captures(desc) {
        let note = cap.get(0).unwrap().as_str().to_string();
        (true, Some(note))
    } else {
        (false, None)
    }
}
```

#### Task 2.2: Generate Method Signatures

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

#### Task 2.3: Emit Managed Objects JSON

**Location:** `vim_build/src/json_emitter/managed_objects.rs`

```rust
use crate::json_emitter::common::*;
use crate::json_emitter::description_parser;
use crate::json_emitter::signature_generator;
use crate::vim_model::Model;
use std::path::Path;
use chrono::Utc;

pub fn emit_managed_objects_json(
    model: &Model,
    output_dir: &Path,
    version: &str,
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

            let (deprecated, deprecation_note) =
                description_parser::check_deprecated(&method.description);

            let privileges = description_parser::parse_privileges(&method.description);
            let errors = description_parser::parse_errors(&method.error_description);

            let related_types = extract_related_types(method, model);
            let tags = extract_tags(&method.name, &method.description);

            methods.push(MethodEntry {
                name: method.name.clone(),
                rust_name: to_fn_name(&method.name),
                signature,
                description: method.description.clone(),
                summary: extract_summary(&method.description),
                openapi_operation_id: format!("{}_{}", mo_name, method.name),
                openapi_path: method.path.clone(),
                openapi_http_method: match method.http_method {
                    HttpMethod::Get => "GET".to_string(),
                    HttpMethod::Post => "POST".to_string(),
                },
                deprecated,
                deprecation_note,
                privileges,
                errors,
                related_types,
                tags,
            });
        }

        managed_objects.push(ManagedObjectEntry {
            name: mo_name.clone(),
            rust_module: format!("vim_rs::mo::{}", to_type_name(mo_name)),
            rust_struct: to_type_name(mo_name),
            description: mo.description.clone(),
            methods,
        });
    }

    let output = ManagedObjectsOutput {
        version: version.to_string(),
        generated_at: Utc::now(),
        source: "vi_json_openapi_specification_v9_0_0_0_24798170.json".to_string(),
        managed_objects,
    };

    let output_path = output_dir.join("managed_objects.json");
    let file = std::fs::File::create(&output_path)?;
    serde_json::to_writer_pretty(file, &output)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}

fn extract_related_types(method: &Method, model: &Model) -> Vec<String> {
    let mut types = Vec::new();

    // Add input types
    if let Some(input) = &method.input {
        if let DataType::Reference(type_name) = input {
            types.push(type_name.clone());
        }
    }

    // Add output types
    if let Some(output) = &method.output {
        if let DataType::Reference(type_name) = output {
            types.push(type_name.clone());
        }
    }

    types.sort();
    types.dedup();
    types
}
```

**Deliverables:**
- ✅ Description parser (privileges, errors, deprecation)
- ✅ Signature generator
- ✅ Managed objects JSON emitter
- ✅ Generated managed_objects.json validates

---

### Phase 3: Implement Data Structures & Enums Emitters (Week 1, Days 4-5)

#### Task 3.1: Emit Data Structures JSON

**Location:** `vim_build/src/json_emitter/data_structures.rs`

```rust
// Similar pattern to managed_objects.rs
// Iterate model.structs, extract field info, build StructureEntry
```

#### Task 3.2: Emit Enumerations JSON

**Location:** `vim_build/src/json_emitter/enumerations.rs`

```rust
// Similar pattern to managed_objects.rs
// Iterate model.enums, extract variants, build EnumerationEntry
```

#### Task 3.3: Emit Metadata JSON

**Location:** `vim_build/src/json_emitter/metadata.rs`

```rust
// Collect statistics from model
// Generate metadata.json
```

**Deliverables:**
- ✅ Data structures JSON emitter
- ✅ Enumerations JSON emitter
- ✅ Metadata JSON emitter
- ✅ All three JSON files generated and validate

---

### Phase 4: Testing & Validation (Week 2, Days 1-2)

#### Task 4.1: Unit Tests

```rust
// vim_build/src/json_emitter/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privilege_parsing() {
        let desc = Some("***Required privileges:*** VirtualMachine.State.CreateSnapshot".to_string());
        let priv_info = parse_privileges(&desc);
        assert_eq!(priv_info.parsed, vec!["VirtualMachine.State.CreateSnapshot"]);
    }

    #[test]
    fn test_error_parsing() {
        let desc = Some("***TaskInProgress***: if the virtual machine is busy".to_string());
        let errors = parse_errors(&desc);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, "TaskInProgress");
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

### New Cargo Dependencies

```toml
# vim_build/Cargo.toml

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
regex = "1.10"

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

## Timeline

### Week 1: Core Implementation
- **Day 1-2:** Phase 1 - Foundation & JSON schemas
- **Day 3-4:** Phase 2 - Managed objects emitter
- **Day 4-5:** Phase 3 - Data structures & enums emitters

### Week 2: Testing & Polish
- **Day 1-2:** Phase 4 - Testing & validation
- **Day 3:** Documentation & examples
- **Day 4:** Code review & refinements
- **Day 5:** Final testing & merge

**Total:** 9-10 days

---

## Success Criteria

### Functional Requirements
- ✅ Generates valid JSON files for all three categories
- ✅ All non-pruned types included
- ✅ Rust names match generated code
- ✅ Descriptions parsed correctly
- ✅ Cross-references are valid

### Quality Requirements
- ✅ Unit test coverage >80%
- ✅ Integration tests pass
- ✅ Validation script reports no errors
- ✅ JSON files compress to <50MB total
- ✅ Generation completes in <30 seconds

### Documentation Requirements
- ✅ JSON schema documented
- ✅ Usage examples provided
- ✅ vim_build changes documented
- ✅ Migration guide for future updates

---

## Risk Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Description parsing fails for edge cases | Medium | Medium | Extensive test cases, fallback to raw text |
| Type resolution errors | High | Low | Reuse existing TypeDefResolver, comprehensive tests |
| JSON files too large | Low | Low | Gzip compression, streaming for MCP |
| Performance issues | Low | Low | Optimize only if >1 minute generation time |
| Regex pattern misses privileges | Medium | Medium | Multiple pattern variants, manual verification sample |

---

## Future Enhancements

### Phase 2 (Post-Initial Release)

1. **Enhanced Description Parsing**
   - ML-based privilege extraction
   - Semantic grouping of related APIs
   - Extract code examples from descriptions

2. **Incremental Generation**
   - Only regenerate changed types
   - Track version differences
   - Migration guides between versions

3. **Additional Metadata**
   - Performance characteristics
   - Typical usage patterns
   - Common error scenarios

4. **Validation Improvements**
   - JSON schema validation
   - Semantic consistency checks
   - Coverage reports

---

## Next Steps

1. **Review this plan** - Confirm approach and timeline
2. **Create feature branch** - `feature/mcp-json-generation`
3. **Begin Phase 1** - Set up json_emitter foundation
4. **Iterate and test** - Build incrementally with tests
5. **Documentation** - Keep docs updated as we build
