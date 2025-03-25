mod hierarchy;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Token, braced, punctuated::Punctuated, parse::Parse, parse::ParseStream, Result, Ident, LitStr, token};
use crate::hierarchy::get_default_field_data;

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

struct FieldInfo<'a> {
    property_field: &'a PropertyField,
    field_data: hierarchy::FieldData,
}

#[proc_macro]
pub fn vim_updatable(input: TokenStream) -> TokenStream {
    let VimObjectMacro { struct_token: _, struct_name, colon_token: _, object_type: managed_object_type, brace_token: _, fields } =
        parse_macro_input!(input as VimObjectMacro);

    let mut field_infos = Vec::new();
    let mut errors: Vec<proc_macro2::TokenStream> = Vec::new();
    for property_field in &fields {
        let path_str = property_field.path.value();
        let res = hierarchy::resolve_path(&managed_object_type.to_string(), &path_str);
        let field_data = match res {
            Ok(field_type) => field_type,
            Err(e) => {
                let msg = format!("Error resolving path: {}", e);
                errors.push(syn::Error::new(property_field.path.span(), msg).to_compile_error());
                get_default_field_data()
            }
        };
        field_infos.push(FieldInfo { property_field, field_data });
    };

    let struct_tokens = generate_struct_decl(&struct_name, &field_infos);

    let struct_impl_tokens = generate_struct_impl(&struct_name, &managed_object_type, &field_infos);

    let try_from_object_content = generate_try_from_object_content(&struct_name, &field_infos);

    let output = quote! {
        #( #errors )*

        #struct_tokens
        #struct_impl_tokens

        #try_from_object_content
    };
    output.into()
}

fn generate_struct_decl(struct_name: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {
    let mut field_declarations: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    for f in fields {
        let field_name = &f.property_field.name;
        let parsed_field_type: syn::Type = syn::parse_str(&f.field_data.data_type).unwrap();
        let decl = quote! { #field_name : #parsed_field_type };
        field_declarations.push(decl);
    }

    let struct_tokens = quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            id: String,
            #(#field_declarations,)*
        }
    };
    struct_tokens
}

fn generate_struct_impl(struct_name: &Ident, managed_object_type: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {
    let field_paths: Vec<&str> = fields.iter().map(|f| f.field_data.vim_path.as_str()).collect();
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


fn generate_try_from_object_content(struct_name: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {

    let mut field_declarations: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut field_conversions: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut field_assignments: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut idx = 1;
    for field in fields {
        let field_alias: Ident = syn::parse_str(&format!("field{}", idx)).unwrap();
        let field_name = &field.property_field.name;
        field_declarations.push(quote! { let mut #field_alias = None; });
        match field.field_data.processing_type {
            hierarchy::FieldProcessingType::Enum(enum_field_name) => {
                field_conversions.push(generate_enum_field_deserialize_code(field, &field_alias, &enum_field_name));
            },
            hierarchy::FieldProcessingType::Struct => {
                field_conversions.push(generate_struct_field_deserialize_code(field, &field_alias, &field.field_data.data_type));
            },
            hierarchy::FieldProcessingType::Trait => {
                field_conversions.push(generate_trait_field_deserialize_code(field, &field_alias, &field.field_data.data_type));
            },
        }
        if field.field_data.is_optional {
            field_assignments.push(quote! { #field_name: #field_alias, });
        } else {
            let field_name_str = field.field_data.vim_path.as_str();
            field_assignments.push(quote! { #field_name: #field_alias.ok_or_else(|| vim_rs::core::pc_helpers::Error::NoneValueForRequiredField(#field_name_str.to_string()))?, });
        }
        idx += 1;
    }

    quote! {
        impl core::convert::TryFrom<vim_rs::types::structs::ObjectContent> for #struct_name {
            type Error = vim_rs::core::pc_helpers::Error;

            fn try_from(row: vim_rs::types::structs::ObjectContent) -> vim_rs::core::pc_helpers::Result<Self> {
                let id = row.obj;
                let Some(row) = row.prop_set else {
                    return Err(vim_rs::core::pc_helpers::Error::NoDataFound);
                };

                #(#field_declarations)*

                for prop in row {
                    match prop.name.as_str() {
                        #(#field_conversions)*
                    }
                }

                Ok(#struct_name {
                    id,
                    #(#field_assignments),*
                })
            }
        }
    }
}

// Templates for TryFrom<ObjectContent> generated code

// 1. Generate ValueElements enum members deserialize code
//                 "<property path>" => {
//                     <field name with ordinal> = match prop.val {
//                         VimAny::Value(ValueElements::<enum field name>(vd)) => Some(vd),
//                         ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: "<property path>".to_string(), expected: "<enum field name>".to_string(), got: pc_helpers::type_name(val)}),
//                     };
//                 }
fn generate_enum_field_deserialize_code(field: &FieldInfo, field_alias: &Ident, enum_field_name: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    let enum_field = Ident::new(enum_field_name, field.property_field.path.span());
    quote! {
        #path => {
            #field_alias = match prop.val {
                VimAny::Value(ValueElements::#enum_field(vd)) => Some(vd),
                ref val => return Err(pc_helpers::Error::InvalidPropertyType { property: #path.to_string(), expected: #enum_field_name.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

// 2. Generate struct type deserialize code ofr structs without children
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
fn generate_struct_field_deserialize_code(field: &FieldInfo, field_alias: &Ident, struct_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;

    quote! {
        #path => {
            #field_alias = match prop.val {
                VimAny::Object(obj) => {
                    let name: &'static str = obj.data_type().into();
                    match obj.as_any_box().downcast() {
                        Ok(val) => Some(*val),
                        Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: name.to_string()}),
                    }
                },
                ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: pc_helpers::type_name(val)}),
            };
        }
    }
}


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
fn generate_trait_field_deserialize_code(field: &FieldInfo, field_alias: &Ident, trait_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    quote! {
        #path => {
            #field_alias = match prop.val {
                VimAny::Object(obj) => {
                    let name: &'static str = obj.data_type().into();
                    match obj.into_box() {
                        Ok(val) => Some(val),
                        Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: name.to_string()}),
                    }
                },
                ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: pc_helpers::type_name(val)}),
            };
        }
    }
}



