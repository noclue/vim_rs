use vim_build::vim_model::{Method, DataType, Model};
use api_database::{MethodSignature, ParameterInfo};
use vim_build::rs_emitter::names::{TypeDefResolver, to_fn_name};

pub fn generate_method_signature(
    method: &Method,
    _mo_name: &str,
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
                for (_, field) in &req.fields {
                    let rust_type = match tdf.field_type(field) {
                        Ok(t) => t,
                        Err(_) => "UnknownType".to_string(),
                    };
                    params.push(ParameterInfo {
                        name: field.rust_name(),
                        rust_type,
                        required: !field.optional,
                        description: field.description.clone(),
                    });
                }
            }
        }
    }

    // Generate return type
    let return_type = if let Some(output) = &method.output {
        let output_rust_type = match tdf.to_rust_field_type(output) {
            Ok(t) => t,
            Err(_) => "UnknownType".to_string(),
        };
        let wrapped = if method.optional_response {
            format!("Option<{}>", output_rust_type)
        } else {
            output_rust_type
        };
        format!("Result<{}>", wrapped)
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
    }
}

