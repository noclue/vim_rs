use sw4rm_rs::{from_path, shared::Schema};
//use sw4rm_rs_generation::read_parse_write_single_spec;




// fn primitive_field_to_rust(name: &String, schema: &Schema) -> syn::Field {
//     syn::Field {
//         attrs: vec![],
//         vis: syn::Visibility::Inherited,
//         mutability: syn::FieldMutability::None,
//         ident: Some(syn::Ident::new(name, proc_macro2::Span::call_site())),
//         colon_token: Some(syn::token::Colon {
//             spans: [proc_macro2::Span::call_site()],
//         }),
//         ty: syn::Type::Path {
//             qself: None,
//             path: syn::Path {
//                 leading_colon: None,
//                 segments: vec![syn::PathSegment {
//                     ident: syn::Ident::new(
//                         schema.type_name().as_str(),
//                         proc_macro2::Span::call_site(),
//                     ),
//                     arguments: syn::PathArguments::None,
//                 }],
//             },
//         },
    
//     }
// }



#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::vec;
    use sw4rm_rs::Resolvable;
    use syn;

    use super::*;

    #[test]
    fn it_works() {
        let mut file = File::open("src/model.rs").expect("unable to open file");

        let mut src = String::new();
        file.read_to_string(&mut src).expect("unable to read file");

        let syntax = syn::parse_file(&src).expect("unable to parse file");

        // Debug impl is available if Syn is built with "extra-traits" feature.
        println!("{:#?}", syntax);
    }

    // #[test]
    // fn test_generation() {
    //     read_parse_write_single_spec("data/recursive.yaml").unwrap();
    // }

    #[test]
    fn load_vi_json() {
        let result = from_path("data/vi_json_openapi_specification_v8_0_2_0.yaml");
        assert_eq!(result.is_ok(), true);

        let doc = result.unwrap();
        assert_eq!(doc.spec_version, "3.0.0".to_string());
        let mut rs_file = syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![],
        };
        doc.schemas().iter().for_each(|(k, v)| {
            //println!("Schema: {}", k);
            //println!("Schema: {:?}", v);
            match v {
                sw4rm_rs::RefOr::Item(ref item) => {
                    if k == "VirtualDevice" {
                        println!("Schema: {:?}", item);
                    }
                    //println!("Schema: {:?}", item);
                    item.all_of.iter().for_each(|all_of_item| {
                        //println!("    All of Schema: {:?}", item);
                        if let sw4rm_rs::RefOr::Reference { reference_path } = all_of_item {
                            let ref_schema =
                                sw4rm_rs::shared::schema::Schema::resolve(&doc, reference_path)
                                    .unwrap();
                            //println!("    All of Schema {}: {:?}", reference_path, ref_schema);
                        }
                    });
                }
                sw4rm_rs::RefOr::Reference { reference_path } => {
                    println!("Schema: {:?}", reference_path);
                }
            }
        });
    }
}
