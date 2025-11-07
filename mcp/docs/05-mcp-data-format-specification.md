# MCP Data Format Specification (REVISED)

This document defines the JSON output formats that vim_build will generate for the MCP server.

## Overview

vim_build will generate three JSON files containing structured data about the vSphere API:

1. **managed_objects.json** - Managed object types with their methods
2. **data_structures.json** - Data structures (structs) and their fields
3. **enumerations.json** - Enumeration types and their variants

These files will be consumed by the MCP server to build searchable indexes.

## Design Principles

### Use vim_rs Names, Not OpenAPI Names

**Rationale:** Developers use vim_rs API in their code, so MCP results should match what they write.

```rust
// Developer writes vim_rs code:
let vm = VirtualMachine::new(client, "vm-123");
vm.create_snapshot("backup", Some("desc"), true, true).await?;

// MCP should return vim_rs-style results:
{
  "module": "vim_rs::mo::VirtualMachine",
  "method": "create_snapshot",
  "signature": "async fn create_snapshot(&self, name: String, ...) -> Result<...>"
}

// NOT OpenAPI style:
{
  "path": "/VirtualMachine/{moId}/CreateSnapshot_Task",
  "operationId": "VirtualMachine_CreateSnapshot_Task"
}
```

### Work Exclusively with vim_model::Model

**Key Point:** We parse data from vim_build's `Model` structure, NOT directly from OpenAPI spec.

The Model already contains:
- All type information with proper inheritance
- Descriptions for all types, fields, methods
- Computed relationships (parent/children, cycles)
- Rust-friendly names via conversion functions

**Why this matters:**
- No need to re-parse OpenAPI
- Relationships already computed
- Names already converted
- Simpler, more maintainable code

### Include ALL Types Including Pruned

**Important:** Types with `emit_mode = "Prune"` or `"Skip"` (like Event, MethodFault) should STILL be included in MCP JSON.

**Rationale:**
- Rust code generation prunes these for technical reasons (complexity)
- But their documentation is valuable for understanding
- Use cases like Kafka event processing need this info
- MCP is for documentation/discovery, not just Rust codegen

The `emit_mode` field indicates whether type appears in generated Rust code, but all types are useful for MCP search.

### Support Semantic Search Requirements

Each entry should have:
- **Searchable text**: Names, descriptions, summaries
- **Metadata**: Types, modules, privileges, errors
- **Examples**: Generated vim_rs code snippets
- **Cross-references**: Links between related APIs (from Model's graph)

---

## Format 1: Managed Objects (managed_objects.json)

### Schema

```json
{
  "version": "9.0.0.0",
  "generated_at": "2024-01-15T10:30:00Z",
  "source": "vim_model processed from vi_json_openapi_specification_v9_0_0_0_24798170.json",
  "managed_objects": [
    {
      "name": "VirtualMachine",
      "rust_module": "vim_rs::mo::VirtualMachine",
      "rust_struct": "VirtualMachine",
      "description": "VirtualMachine is the managed object type for...",
      "methods": [
        {
          "name": "create_snapshot_task",
          "rust_name": "create_snapshot_task",
          "signature": {
            "full": "pub async fn create_snapshot_task(&self, name: String, description: Option<String>, memory: bool, quiesce: bool) -> Result<ManagedObjectReference>",
            "parameters": [
              {
                "name": "name",
                "rust_type": "String",
                "required": true,
                "description": "The name for this snapshot"
              },
              {
                "name": "description",
                "rust_type": "Option<String>",
                "required": false,
                "description": "A description for this snapshot"
              },
              {
                "name": "memory",
                "rust_type": "bool",
                "required": true,
                "description": "If TRUE, a dump of the internal state of the virtual machine is included in the snapshot"
              },
              {
                "name": "quiesce",
                "rust_type": "bool",
                "required": true,
                "description": "If TRUE and the virtual machine is powered on, VMware Tools is used to quiesce the file system"
              }
            ],
            "return_type": "Result<ManagedObjectReference>",
            "is_async": true
          },
          "description": "Creates a new snapshot of this virtual machine...",
          "summary": "Creates a new snapshot of this virtual machine.",
          "deprecated": true,
          "deprecation_note": "Deprecated as of vSphere 8.0GA, use CreateSnapshotEx_Task instead",
          "privileges": {
            "raw": "***Required privileges:*** VirtualMachine.State.CreateSnapshot",
            "parsed": ["VirtualMachine.State.CreateSnapshot"]
          },
          "errors": [
            {
              "type": "TaskInProgress",
              "description": "if the virtual machine is busy"
            },
            {
              "type": "NotSupported",
              "description": "if the host product does not support snapshots..."
            },
            {
              "type": "SnapshotFault",
              "description": "if an error occurs during the snapshot operation"
            },
            {
              "type": "InvalidName",
              "description": "if the specified snapshot name is invalid"
            }
          ],
          "related_types": [
            "ManagedObjectReference",
            "VirtualMachineSnapshot"
          ]
        }
      ]
    }
  ]
}
```

### Key Fields Explanation

#### Managed Object Level

- **name**: Model's ManagedObject.name
- **rust_module**: Full vim_rs module path (computed from name)
- **rust_struct**: Rust struct name (computed from name)
- **description**: Model's ManagedObject.description

#### Method Level

- **name**: Model's Method.name
- **rust_name**: Rust function name (via `to_fn_name()`)
- **signature**: Complete Rust method signature information
  - **full**: Complete Rust function signature as string
  - **parameters**: Array extracted from Method.input via Model.request_types
  - **return_type**: Rust return type from Method.output
  - **is_async**: Boolean (always true for vim_rs methods)
- **description**: Model's Method.description
- **summary**: First line of description
- **deprecated**: Parsed from description
- **deprecation_note**: Extracted deprecation text
- **privileges**: Parsed from description
  - **raw**: Original text containing privileges
  - **parsed**: Extracted privilege strings
- **errors**: Parsed from Method.error_description
- **related_types**: Extracted from Method.input and Method.output

**Note:** No `tags` field - removed as redundant

---

## Format 2: Data Structures (data_structures.json)

### Schema

```json
{
  "version": "9.0.0.0",
  "generated_at": "2024-01-15T10:30:00Z",
  "source": "vim_model processed from vi_json_openapi_specification_v9_0_0_0_24798170.json",
  "structures": [
    {
      "name": "VirtualMachineConfigSpec",
      "rust_name": "VirtualMachineConfigSpec",
      "rust_module": "vim_rs::types::structs",
      "description": "An optional data object type containing a set of updates...",
      "parent": "DataObject",
      "children": [],
      "emit_mode": "Emit",
      "fields": [
        {
          "name": "name",
          "rust_name": "name",
          "rust_type": "Option<String>",
          "vim_type": "String",
          "required": false,
          "description": "Display name of the virtual machine",
          "is_array": false,
          "is_boxed": false
        },
        {
          "name": "files",
          "rust_name": "files",
          "rust_type": "Option<Box<dyn VirtualMachineFileInfoTrait>>",
          "vim_type": "VirtualMachineFileInfo",
          "required": false,
          "description": "Information about the files that comprise the virtual machine",
          "is_array": false,
          "is_boxed": true,
          "is_trait": true,
          "trait_name": "VirtualMachineFileInfoTrait"
        },
        {
          "name": "deviceChange",
          "rust_name": "device_change",
          "rust_type": "Option<Vec<Box<dyn VirtualDeviceConfigSpecTrait>>>",
          "vim_type": "VirtualDeviceConfigSpec",
          "required": false,
          "description": "Set of virtual devices being modified by the configuration operation",
          "is_array": true,
          "is_boxed": true,
          "is_trait": true,
          "trait_name": "VirtualDeviceConfigSpecTrait"
        }
      ],
      "related_types": [
        "VirtualMachineFileInfo",
        "VirtualDeviceConfigSpec",
        "DataObject"
      ],
      "inheritance_chain": ["DataObject", "VirtualMachineConfigSpec"]
    },
    {
      "name": "EventEx",
      "rust_name": "EventEx",
      "rust_module": "vim_rs::types::structs",
      "description": "This event is the base data object type from which all events inherit...",
      "parent": "Event",
      "children": ["VmEvent", "HostEvent", "..."],
      "emit_mode": "Skip",
      "skip_reason": "Descendant of pruned type Event",
      "fields": [
        {
          "name": "eventTypeId",
          "rust_name": "event_type_id",
          "rust_type": "String",
          "vim_type": "String",
          "required": true,
          "description": "The type of the event",
          "is_array": false,
          "is_boxed": false
        }
      ],
      "related_types": ["Event"],
      "inheritance_chain": ["Event", "EventEx"]
    }
  ]
}
```

### Key Fields Explanation

#### Structure Level

- **name**: Model's Struct.name
- **rust_name**: Rust struct name (via `Struct.rust_name()`)
- **rust_module**: vim_rs module path
- **description**: Model's Struct.description
- **parent**: Model's Struct.parent (already computed)
- **children**: Model's Struct.children (already computed)
- **emit_mode**: Model's Struct.emit_mode ("Emit", "Prune", or "Skip")
- **skip_reason**: For emit_mode="Skip", explains why (e.g., "Descendant of pruned type Event")
- **fields**: Array from Model's Struct.fields
- **related_types**: Extracted from field types
- **inheritance_chain**: Via Model.inheritance_chain()

**Changes from original:**
- Removed `is_abstract` - misleading name
- Removed `used_by_methods` - can derive if needed, but adds complexity
- Removed `tags` - redundant
- Added `skip_reason` for skipped types
- **Include ALL types** even if emit_mode = "Skip"

#### Field Level

- **name**: Model's Field.name
- **rust_name**: Model's Field.rust_name()
- **rust_type**: Via TypeDefResolver (handles Option, Vec, Box, dyn)
- **vim_type**: Model's Field.vim_type base name
- **required**: !Field.optional
- **description**: Model's Field.description
- **is_array**: Derived from Field.vim_type (DataType::Array)
- **is_boxed**: Model's Field.require_box
- **is_trait**: Detected if vim_type references a type with children
- **trait_name**: Computed from vim_type if is_trait

---

## Format 3: Enumerations (enumerations.json)

### Schema

```json
{
  "version": "9.0.0.0",
  "generated_at": "2024-01-15T10:30:00Z",
  "source": "vim_model processed from vi_json_openapi_specification_v9_0_0_0_24798170.json",
  "enumerations": [
    {
      "name": "ManagedEntityStatus_enum",
      "rust_name": "ManagedEntityStatus",
      "rust_module": "vim_rs::types::enums",
      "description": "The Status enumeration defines a general 'health' value for a managed entity.",
      "variants": [
        {
          "name": "gray",
          "rust_name": "Gray",
          "description": "The status is unknown",
          "discriminator_value": "gray"
        },
        {
          "name": "green",
          "rust_name": "Green",
          "description": "The entity is OK",
          "discriminator_value": "green"
        },
        {
          "name": "yellow",
          "rust_name": "Yellow",
          "description": "The entity might have a problem",
          "discriminator_value": "yellow"
        },
        {
          "name": "red",
          "rust_name": "Red",
          "description": "The entity definitely has a problem",
          "discriminator_value": "red"
        }
      ]
    }
  ]
}
```

### Key Fields Explanation

#### Enumeration Level

- **name**: Model's Enum.name
- **rust_name**: Rust enum name (remove _enum suffix if present)
- **rust_module**: vim_rs module path
- **description**: Model's Enum.description
- **variants**: Array from Model's Enum.variants

**Changes from original:**
- Removed `used_by_structures` - complex to compute, questionable value
- Removed `used_by_methods` - complex to compute, questionable value
- Removed `tags` - redundant

#### Variant Level

- **name**: Original variant name from Enum.variants
- **rust_name**: PascalCase version
- **description**: Parsed from Enum.description
- **discriminator_value**: Model's Enum.discriminator_value or name

---

## Additional Metadata File (metadata.json)

### Schema

```json
{
  "version": "9.0.0.0",
  "vsphere_version": "9.0.0.0",
  "build_number": "24798170",
  "generated_at": "2024-01-15T10:30:00Z",
  "vim_rs_version": "0.1.0",
  "source_files": {
    "openapi_spec": "vi_json_openapi_specification_v9_0_0_0_24798170.json",
    "vim_build_version": "0.1.0"
  },
  "statistics": {
    "managed_objects": 145,
    "managed_objects_with_methods": 145,
    "total_methods": 2195,
    "data_structures_total": 8234,
    "data_structures_emitted": 6000,
    "data_structures_pruned": 2,
    "data_structures_skipped": 2232,
    "enumerations": 1504,
    "request_types": 2195,
    "pruned_types": ["MethodFault", "Event"]
  },
  "indexes_generated": [
    "managed_objects.json",
    "data_structures.json",
    "enumerations.json"
  ],
  "generation_duration_ms": 15234,
  "notes": "data_structures.json includes ALL types (emit, prune, and skip) for complete documentation"
}
```

---

## Data Sources and Extraction

### Source: vim_model::Model

**All data comes from the vim_build Model, NOT directly from OpenAPI.**

```rust
// vim_model is already transformed from OpenAPI
pub struct Model {
    pub enums: IndexMap<String, Enum>,
    pub structs: IndexMap<String, RefCell<Struct>>,
    pub request_types: IndexMap<String, RefCell<Struct>>,
    pub any_value_types: IndexMap<String, BoxType>,
    pub managed_objects: IndexMap<String, ManagedObject>,
}
```

### Extraction Strategy

#### 1. Names and Types

Use Model's existing conversion functions:
```rust
// From rs_emitter module
use crate::rs_emitter::{to_type_name, to_fn_name};

// Struct names
let rust_name = struct_ref.borrow().rust_name();  // Built-in method

// Field names
let rust_name = field.rust_name();  // Built-in method

// Method names
let rust_name = to_fn_name(&method.name);
```

#### 2. Type Resolution

Use Model's TypeDefResolver:
```rust
use crate::rs_emitter::TypeDefResolver;

let tdf = TypeDefResolver::new_with_root_package(model, "crate::types".to_string());

// Resolve field type to Rust type
let rust_type = tdf.resolve_to_rust_type(
    &field.vim_type,
    field.optional,
    field.require_box
);
// Returns: "Option<Box<dyn VirtualDeviceConfigSpecTrait>>"
```

#### 3. Relationships

Use Model's existing graph traversal:
```rust
// Parent/children already in Struct
let parent = struct_ref.borrow().parent.clone();
let children = struct_ref.borrow().children.clone();

// Inheritance chain helper
let chain = model.inheritance_chain(&struct_name)?;

// Traverse children
for child_struct in model.children(&parent_name)? {
    // Process each child
}
```

#### 4. Description Parsing

Parse structured data from Model's description fields:
```rust
// From Method.description or Struct.description

// Parse privileges
fn parse_privileges(description: &Option<String>) -> PrivilegeInfo {
    // Look for: "***Required privileges:*** Privilege.Name"
    // Regex: r"\*\*\*Required privileges:\*\*\*\s*(.+)"
}

// Parse errors
fn parse_errors(description: &Option<String>) -> Vec<ErrorInfo> {
    // Look for: "***ErrorType***: description"
    // Regex: r"\*\*\*(\w+)\*\*\*:\s*(.+)"
}

// Parse deprecation
fn parse_deprecation(description: &Option<String>) -> (bool, Option<String>) {
    // Look for: "Deprecated as of vSphere X.Y"
    // Regex: r"Deprecated as of (.+?)[\.,]"
}

// Extract summary
fn extract_summary(description: &Option<String>) -> Option<String> {
    // First line of description
    description.as_ref().map(|d| d.lines().next().unwrap_or("").trim().to_string())
}
```

#### 5. Include All Types Including Pruned

```rust
// Iterate ALL structs, including those with emit_mode != Emit
for (name, struct_ref) in &model.structs {
    let s = struct_ref.borrow();

    // Include regardless of emit_mode
    let emit_mode = match &s.emit_mode {
        EmitMode::Emit => "Emit",
        EmitMode::Prune => "Prune",
        EmitMode::Skip(parent) => "Skip",
    };

    let skip_reason = match &s.emit_mode {
        EmitMode::Skip(parent) => Some(format!("Descendant of pruned type {}", parent)),
        _ => None,
    };

    // Generate JSON entry even for Skip types
    // Their documentation is valuable for MCP
}
```

### What NOT to Do

❌ Don't re-parse OpenAPI directly
❌ Don't recompute relationships (use Model's graph)
❌ Don't recreate name conversion (use Model's methods)
❌ Don't skip pruned/skipped types (include ALL for docs)

### What TO Do

✅ Use Model's data structures exclusively
✅ Use Model's helper methods (rust_name(), inheritance_chain())
✅ Use rs_emitter's name conversion and type resolution
✅ Include ALL types in JSON (emit, prune, and skip)
✅ Parse descriptions for structured data (privileges, errors)

---

## Example Usage in MCP

### Searching for "create snapshot"

```rust
// Load managed_objects.json
let mo_data: ManagedObjectsOutput = serde_json::from_str(&json_str)?;

// Build search index
for mo in mo_data.managed_objects {
    for method in mo.methods {
        index.add(SearchEntry {
            id: format!("{}::{}", mo.rust_module, method.rust_name),
            text: format!("{} {}",
                method.summary.unwrap_or_default(),
                method.description.unwrap_or_default()
            ),
            module: mo.rust_module,
            method_name: method.rust_name,
            signature: method.signature.full,
            privileges: method.privileges.parsed,
            errors: method.errors.iter().map(|e| e.error_type.clone()).collect(),
        });
    }
}

// Search
let results = index.search("create snapshot")?;
// Returns: VirtualMachine::create_snapshot_task, etc.
```

### Using Relationship Data

```rust
// Load data_structures.json
let structs: DataStructuresOutput = serde_json::from_str(&json_str)?;

// Find a struct and its children
let config_spec = structs.structures.iter()
    .find(|s| s.name == "VirtualMachineConfigSpec")?;

// Check inheritance
println!("Inheritance: {:?}", config_spec.inheritance_chain);
// Output: ["DataObject", "VirtualMachineConfigSpec"]

// Find children (polymorphic types)
let has_children = !config_spec.children.is_empty();

// Check if this is in generated Rust code
match config_spec.emit_mode.as_str() {
    "Emit" => println!("Available in vim_rs::types::structs"),
    "Prune" => println!("Base type available, children not emitted"),
    "Skip" => println!("Not in Rust code, but documented here"),
    _ => {}
}
```

### Finding Event Documentation (Pruned Type)

```rust
// Even though Event hierarchy is pruned in Rust,
// it's fully documented in JSON for understanding

let event_ex = structs.structures.iter()
    .find(|s| s.name == "EventEx")?;

assert_eq!(event_ex.emit_mode, "Skip");
println!("Reason: {}", event_ex.skip_reason.as_ref().unwrap());
// Output: "Descendant of pruned type Event"

// But we can still see its fields and description
for field in &event_ex.fields {
    println!("Field: {} - {}", field.rust_name, field.description.as_ref().unwrap_or(&"".to_string()));
}
// This is useful for Kafka event processing even though
// there's no EventEx struct in vim_rs!
```

---

## File Locations

Generated files will be placed in:

```
vim_rs/
├── vim_build/
│   └── data/
│       ├── vi_json_openapi_specification_v9_0_0_0_24798170.json  (input)
│       └── mcp/  (NEW - generated output)
│           ├── managed_objects.json
│           ├── data_structures.json
│           ├── enumerations.json
│           └── metadata.json
```

---

## Validation Requirements

Generated JSON must be:

1. **Valid JSON** - Well-formed, parseable
2. **Complete** - ALL types included (emit, prune, skip)
3. **Consistent** - Cross-references are valid
4. **Accurate** - vim_rs names match generated code (where applicable)

Validation tests:
- JSON schema validation
- Cross-reference validation (all referenced types exist in Model)
- vim_rs name matching (for emitted types)
- Statistics check (counts match Model.structs.len(), etc.)
- Emit mode consistency (pruned types have Skip descendants)

---

## Summary of Changes from Original

### Removed

- ❌ `tags` fields - redundant, auto-generated
- ❌ `is_abstract` - misleading name
- ❌ `used_by_methods` - complex, low value
- ❌ `used_by_structures` - complex, low value
- ❌ Direct OpenAPI parsing - use Model instead

### Added

- ✅ `skip_reason` for skipped types
- ✅ Clarification: include ALL types including pruned/skipped
- ✅ Emphasis on using Model exclusively
- ✅ Examples of using Model's graph traversal

### Clarified

- ✅ `emit_mode` field explains Rust code generation status
- ✅ Pruned types (Event, MethodFault) included in JSON
- ✅ Source is vim_model, not OpenAPI
- ✅ Use Model's existing relationships and helpers
