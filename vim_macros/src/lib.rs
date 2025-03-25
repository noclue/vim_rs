mod hierarchy;
mod model;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Token, braced, punctuated::Punctuated, parse::Parse, parse::ParseStream, Result, Ident, LitStr, token};
use syn::token::Comma;

#[allow(dead_code)]
struct PropertyField {
    name: Ident,
    colon_token: Token![=],
    path: LitStr,
}

#[allow(dead_code)]
struct VimObjectMacro {
    struct_token: Token![struct],
    struct_name: Ident,
    colon_token: Token![:],
    object_type: Ident,
    brace_token: token::Brace,
    fields: Punctuated<PropertyField, Token![,]>,
}

impl Parse for PropertyField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PropertyField {
            name: input.parse()?,
            colon_token: input.parse()?,
            path: input.parse()?,
        })
    }
}

impl Parse for VimObjectMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(VimObjectMacro {
            struct_token: input.parse()?,
            struct_name: input.parse()?,
            colon_token: input.parse()?,
            object_type: input.parse()?,
            brace_token: braced!(content in input),
            fields: Punctuated::parse_terminated(&content)?,
        })
    }
}

#[proc_macro]
pub fn vim_updatable(input: TokenStream) -> TokenStream {
    let VimObjectMacro { struct_token: _, struct_name, colon_token: _, object_type: managed_object_type, brace_token: _, fields } =
        parse_macro_input!(input as VimObjectMacro);

    let struct_tokens = generate_struct_decl(&struct_name, &managed_object_type, &fields);

    let struct_impl_tokens = generate_struct_impl(struct_name, managed_object_type, fields);

    let output = quote! {
        #struct_tokens
        #struct_impl_tokens
    };
    output.into()
}

fn generate_struct_decl(struct_name: &Ident, managed_object_type: &Ident, fields: &Punctuated<PropertyField, Comma>) -> proc_macro2::TokenStream {
    let mut field_declarations: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut errors: Vec<proc_macro2::TokenStream> = Vec::new();
    for f in fields {
        let path_str = f.path.value();
        let field_name = &f.name;
        let res = hierarchy::resolve_path(&managed_object_type.to_string(), &path_str);
        let parsed_field_type: syn::Type = match res {
            Ok(field_type) => syn::parse_str(&field_type).unwrap(),
            Err(e) => {
                let msg = format!("Error resolving path: {}", e);
                errors.push(syn::Error::new(f.path.span(), msg).to_compile_error());
                syn::parse_str("Option<String>").unwrap()
            }
        };
        let decl = quote! { #field_name : #parsed_field_type };
        field_declarations.push(decl);
    }

    let struct_tokens = quote! {
        #( #errors )*

        #[derive(Debug, Clone)]
        pub struct #struct_name {
            id: String,
            #(#field_declarations,)*
        }
    };
    struct_tokens
}

fn generate_struct_impl(struct_name: Ident, managed_object_type: Ident, fields: Punctuated<PropertyField, Comma>) -> proc_macro2::TokenStream {
    let field_paths: Vec<&LitStr> = fields.iter().map(|f| &f.path).collect();
    let prop_paths_quoted: Vec<proc_macro2::TokenStream> = field_paths
        .iter()
        .map(|path| quote! { #path.into() })
        .collect();

    let struct_impl_tokens = quote! {
        impl #struct_name {
            pub fn prop_spec() -> vim_rs::types::structs::PropertySpec {
                vim_rs::types::structs::PropertySpec {
                    all: Some(false),
                    path_set: Some(vec![
                        #(#prop_paths_quoted),*
                    ]),
                    r#type: Into::<&str>::into(vim_rs::types::enums::MoTypesEnum::#managed_object_type).to_string(),
                }
            }

            pub fn id(&self) -> &str {
                &self.id
            }
        }
    };
    struct_impl_tokens
}

// Templates for TryFrom<ObjectContent> generated code

// 1. Generate ValueElements enum members deserialize code
//                 "<property path>" => {
//                     <field name with ordinal> = match prop.val {
//                         VimAny::Value(ValueElements::<enum field name>(vd)) => Some(vd),
//                         ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "<property path>".to_string(), expected: "<enum field name>".to_string(), got: pc_helpers::type_name(val)}),
//                     };
//                 }
// 2. Generate stuct type deserialize code ofr structs without children
//                "<property path>" => {
//                     <field name with ordinal> = match prop.val {
//                         VimAny::Object(obj) => {
//                             let name: &'static str = obj.data_type().into();
//                             match obj.as_any_box().downcast() {
//                                 Ok(val) => Some(*val),
//                                 Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "<property path>".to_string(), expected: "<struct type name>".to_string(), got: name.to_string()}),
//                             }
//                         },
//                         ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: "<property path>".to_string(), expected: "<struct type name>".to_string(), got: pc_helpers::type_name(val)}),
//                     };
//                 }
// 3. Generate trait type deserialize code for structs with children
//                "<property path>" => {
//                     <field name with ordinal> = match prop.val {
//                         VimAny::Object(obj) => {
//                             let name: &'static str = obj.data_type().into();
//                             match obj.into_box() {
//                                 Ok(val) => Some(val),
//                                 Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "<property path>".to_string(), expected: "<trait type name>".to_string(), got: name.to_string()}),
//                             }
//                         },
//                         ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: "<property path>".to_string(), expected: "<trait type name>".to_string(), got: pc_helpers::type_name(val)}),
//                     };
//                 }
// we need the following parameters for each path:
// 1. property path
// 2. field name with ordinal
// 3. field data type
// 4. optional/required field
// 5. enum field name / struct type name / trait type name
// 6. field processing type - enum / struct / trait
