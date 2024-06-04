use std::io::Read;
use openapiv3::{OpenAPI, Schema, SchemaKind, ReferenceOr};


// Error is an error type for Project.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot read file")]
    FileIOError(#[from] std::io::Error),
    #[error("Cannot parse JSON")]
    JSONError(#[from] serde_json::Error),    
    #[error("Reference `{}` cannot be resolved", .0)]
    NotFound(String),
    #[error("Cycle detected in reference `{}`", .0)]
    Cycle(String),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;

/// A project manages the stats of an OpenAPI specification.
/// It is created from a file and additional files may be 
/// loaded to resolve references.
pub struct Project {
    spec: openapiv3::OpenAPI,
    // TODO: Support multiple specs and cross file references.
    //specs: HashMap<String, openapiv3::OpenAPI>,
}

impl Project {
    /// Create a new project from a file.
    pub fn new(file: &str) ->  Result<Project> {
        let openapi = load_openapi(file)?;
        Result::Ok(Project {
            spec: openapi,
        })
    }

    /// Get ReferenceOr<Schema> and return the Schema if it is a Schema. If it is a Reference, resolve it recursively and return the referenced Schema.
    /// If the reference is not found return NotFound error. If there is cycle in the reference, return Cycle error.
    pub fn resolve_schema<'a>(&'a self, item: &'a ReferenceOr<Schema>) -> Result<&'a Schema> {
        self.resolve_internal(item, vec![])
    }

    fn resolve_internal<'a>(&'a self, item: &'a ReferenceOr<Schema>, mut stack: Vec<String>) -> Result<&'a Schema> {
        match item {
            ReferenceOr::Item(schema) => Ok(schema),
            ReferenceOr::Reference { reference } => {
                let schema_name = reference.trim_start_matches("#/components/schemas/");
                if stack.contains(&schema_name.to_string()) {
                    return Err(Error::Cycle(format!("{}, {}", stack.join(", "), reference.to_string())));
                }
                stack.push(schema_name.to_string());
                match self.spec.components.as_ref().unwrap().schemas.get(schema_name) {
                    Some(schema) => {
                        let schema = self.resolve_internal(schema, stack)?;
                        Ok(schema)
                    },
                    None => Err(Error::NotFound(reference.to_string()))
                }
            }
        }
    }

    /// Get parent schema name Some(String) from the "allOf" field if there is exactly one reference value.
    /// If there are multiple values, no value or Schema value return None.
    pub fn get_parent_schema(& self, schema: &Schema) -> Result<Option<String>> {
        if let SchemaKind::Any(any_scehma) = &schema.schema_kind {
            let all_of = &any_scehma.all_of;
            if all_of.len() == 1 {
                if let ReferenceOr::Reference { reference } = &all_of[0] {
                    let schema_name = reference.trim_start_matches("#/components/schemas/");
                    return Ok(Some(schema_name.to_string()));
                }
            }
            
        }
        Result::Ok(None)
    }
}

/// Load an OpenAPI specification from a file.
fn load_openapi(file: &str) -> Result<openapiv3::OpenAPI> {
    let mut file = std::fs::File::open(file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;    
    let openapi: openapiv3::OpenAPI = serde_json::from_str(&data)?;
    Result::Ok(openapi)
}

/// Print hierarchy of schemas. Prints ASCII art hirarchy of schemas starting from the given schema.
/// If the schema has child schemas, they will be printed on new line with increased indentation. 
/// If the child schema has child schemas, they will be printed on new line with increased indentation.
pub fn print_schema_hierarchy(project: &Project, schema: String, indent: usize) -> Result<()> {
    let h = resolve_schema_hierarchy(project)?;
    print_schema_hierarchy_internal(&h, &schema, indent)
}

fn print_schema_hierarchy_internal(hierarchy: &std::collections::HashMap<String, Vec<String>>, schema: &String, indent: usize) -> Result<()> {
    match hierarchy.get(schema) {
        Some(children) => {
            println!("{:indent$}{} {}", "", schema, children.len(), indent=indent);
            for child in children {
                print_schema_hierarchy_internal(hierarchy, child, indent + 2)?;
            }
        },
        None => {
            return Err(Error::NotFound(schema.to_string()));
        }
        
    }
    Ok(())
}


/// Resolve hierarchy of schemas. Returns a Map of schema names to their children schema names.
fn resolve_schema_hierarchy(project: &Project) -> Result<std::collections::HashMap<String, Vec<String>>> {
    let mut hierarchy = std::collections::HashMap::new();

    for (name, schema) in project.spec.components.as_ref().unwrap().schemas.iter() {
        let schema = project.resolve_schema(schema)?;
        let parent = project.get_parent_schema(schema)?;
        if let Some(parent_name) = parent {
            let child_hierarchy = hierarchy.entry(parent_name).or_insert(vec![]);
            child_hierarchy.push(name.to_string());            
        }
        hierarchy.entry(name.to_string()).or_insert(vec![]);
    }
    Ok(hierarchy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use serde_json;

    fn load_openapi() -> openapiv3::OpenAPI {
        let mut file = std::fs::File::open("data/vi_json_openapi_specification_v8_0_2_0.json").expect("unable to open file");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("unable to read file");    
        let openapi: openapiv3::OpenAPI = serde_json::from_str(&data).expect("Could not deserialize input");
        openapi
    }

    #[test]
    fn print_schema_hierarchy_test() {
        let openapi = load_openapi();
        let project = Project {
            spec: openapi,
        };
        print_schema_hierarchy(&project, "VirtualDevice".to_string(), 0).expect("Could not print schema hierarchy");
    }

    #[test]
    fn count_schema() {
        let openapi = load_openapi();
        let mut count = 0;
        for (_, schema) in openapi.components.unwrap().schemas.iter() {
            count += 1;
        }
        
        assert_eq!(count, 7182);
    }
}
