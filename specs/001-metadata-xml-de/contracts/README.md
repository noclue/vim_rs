# Contracts: XML deserialization (`feature = "xml"`)

**Feature**: [spec.md](../spec.md)  
**Plan**: [plan.md](../plan.md)

These notes describe **public-facing behavior** for library consumers when the **`xml`** feature is enabled. Exact signatures live in rustdoc after implementation.

## Entry-point bounds

| API area | Contract |
|----------|----------|
| **`unmarshal<T>`** | **`T: Deserialize`** always; when **`xml`** feature is on, **`T: DataTypeAware`** additionally required so the transport can supply root **`ApiFieldType`**. |
| **`unmarshal_array<U>`** | Element type **`U: DataTypeAware`** (under **`xml`**) for list roots; container **`Vec`** is not a separate schema root. |
| **`from_xml` / `from_xml_with`**, **`vim_response`**, **`vim_response_list`** | Same conditional **`DataTypeAware`** bound as **`unmarshal`**. |

**Rationale**: Root and list-element types are always fixed by the **generic parameter** at the call site (e.g. explicit type annotation on **`let result: AgentConfigInfo = unmarshal(...)`**).

## Error contract

- **`Result`** failures remain **unit-style miniserde-compatible errors** (no structured deserialization diagnostics in the error value).
- **Debugging**: optional **`tracing`** under **`vim_rs::wire::soap`** (or documented wire targets)—must never log secrets (constitution).

## Wire mapping (stable expectations)

- **Typing**: Structural shape and leaf coercion follow **declared `ApiFieldType`** and **`lookup_xml_type`** only—**no** visitor probing or seq-vs-single inference (**FR-014**).
- **`xsi:type`** ↔ synthetic polymorphism key (same logical contract as JSON **`_typeName`** path).
- **Attributes** ↔ **`@name`** map keys where applicable.
- **Text** ↔ **`#text`** or direct primitive visitor for typed leaves.
- **`xsi:nil`** truthy → **`null`** visitor path; enforcement of optionality remains in generated builders.

## SOAP

Envelope extraction and body slicing remain **`xml/soap.rs`** responsibilities; **`de.rs`** contracts assume **post-envelope** XML consistent with current behavior.

## Breaking-change notice

Call sites that used **`from_xml::<Vec<T>>`** or **`unmarshal::<Option<T>>`** as roots may need to switch to **element-typed** helpers—document in **`CHANGELOG.md`** for releases.
