//! # vim_macros
//!
//! Procedural macros for simplified VMware vSphere property retrieval and monitoring.
//!
//! ## Property Retrieval with Macros
//!
//! The library provides two powerful macros to simplify property retrieval and monitoring:
//!
//! ### One-time Property Retrieval with `vim_retrievable`
//!
//! The `vim_retrievable` macro creates structs for efficient, one-time property retrieval:
//!
//! ```ignore
//! use vim_macros::vim_retrievable;
//! use vim_rs::core::pc_retrieve::ObjectRetriever;
//!
//! // Define a struct mapping to HostSystem properties
//! vim_retrievable!(
//!     struct Host: HostSystem {
//!         name = "name",
//!         power_state = "runtime.power_state",
//!         connected = "runtime.connection_state",
//!         cpu_usage = "summary.quick_stats.overall_cpu_usage",
//!         memory_usage = "summary.quick_stats.overall_memory_usage",
//!         uptime = "summary.quick_stats.uptime",
//!     }
//! );
//!
//! async fn print_hosts(client: &Client) -> Result<()> {
//!    // Create a retriever using the client
//!    let retriever = ObjectRetriever::new(client.clone())?;
//!
//!    // Retrieve all hosts with their properties in a single API call
//!    let hosts: Vec<HostInfo> = retriever
//!            .retrieve_objects_from_container(&client.service_content().root_folder)
//!            .await?;
//!
//!    // Work with strongly-typed host objects
//!    for host in hosts {
//!       println!("Host {} is {:?}", host.name, host.power_state);
//!    }
//!
//!    Ok(())
//! }
//! ```
//!
//! ### Continuous Property Monitoring with `vim_updatable`
//!
//! The `vim_updatable` macro creates structs for continuous property monitoring:
//!
//! ```ignore
//! vim_updatable!(
//!     struct VmDetails: VirtualMachine {
//!         name = "name",
//!         power_state = "runtime.power_state",
//!     }
//! );
//!
//! impl Display for VmDetails {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//!         write!(
//!             f,
//!             "VM ({}): {} with power state: {:?}", self.id.value, self.name, self.power_state
//!         )
//!     }
//! }
//!
//! struct ChangeListener {}
//!
//! impl ObjectCacheListener<VmDetails> for ChangeListener {
//!     fn on_new(&mut self, obj: &VmDetails) {
//!         info!("New VM: {}", obj);
//!     }
//!
//!     fn on_update(&mut self, obj: &VmDetails) {
//!         info!("VM updated: {}", obj);
//!     }
//!
//!     fn on_remove(&mut self, obj: VmDetails) {
//!         info!("VM removed: {}", obj);
//!     }
//! }
//!
//! async fn monitor_vms(client: &Arc<Client>) -> Result<(), Error> {
//!     let cache = Box::new(ObjectCache::new_with_listener(Box::new(ChangeListener {})));
//!     let mut manager = CacheManager::new(client.clone())?;
//!     let mut monitor = manager.create_monitor()?;
//!
//!     manager.add_container_cache(cache, &client.service_content().root_folder).await?;
//!
//!     let start = Instant::now();
//!     loop {
//!         let updates = monitor.wait_updates(10).await?;
//!         if let Some(updates) = updates {
//!             manager.apply_updates(updates)?;
//!         }
//!         if start.elapsed().as_secs() > 60 {
//!             break;
//!         }
//!     }
//!
//!     manager.destroy().await?;
//!     Ok(())
//! }
//! ```
//!
//! ### How the Macros Work
//!
//! Both macros:
//!
//! 1. Generate a struct based on the data structure defined in the macro, corresponding to a vSphere managed object type (VirtualMachine, HostSystem, etc.)
//! 2. Elicit the types of struct fields from the property paths in the vSphere API
//! 3. Handle type conversion between vSphere dynamic types and Rust types
//!
//! The `vim_rs::core::pc_retrieve` module supports one-time property retrieval,
//! while `vim_rs::core::pc_cache` provides infrastructure for continuous property monitoring.
//!
//! ### Macro Syntax
//!
//! ```ignore
//! vim_retrievable!(
//!     struct StructName: ManagedObjectType {
//!         field_name = "property.path",
//!         another_field = "another.property.path"
//!     }
//! );
//! ```
//!
//! The same syntax applies to the `vim_updatable!` macro.
mod resolver;
mod field_data;

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, Token, braced, punctuated::Punctuated, parse::Parse, parse::ParseStream, Result, Ident, LitStr, token};
use syn::token::Comma;
use resolver::get_default_field_data;

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
    field_data: resolver::FieldData,
}



/// A macro to generate a struct and implementation necessary to work with
/// PropertyCollector::RetrievePropertiesEx API. Developers need to select the managed object type
/// of interest and the properties they need retrieved. For example to retrieve properties of a
/// VirtualMachine object, the macro can be used as follows:
///
/// ```ignore
/// vim_retrievable!(
///    struct VM: VirtualMachine {
///       name = "name",
///       os = "summary.guest.guest_full_name",
///       storage = "summary.storage",
///       host_cpu = "summary.quick_stats.overall_cpu_usage",
///       host_memory = "summary.quick_stats.host_memory_usage",
///       status = "summary.overall_status",
///       power_state = "runtime.power_state",
///       devices = "config.hardware.device",
///       ft_info = "config.ft_info",
///   }
/// );
/// ```
/// The macro will generate a struct `VM` with the specified properties and extract their types from
/// the vSphere API. The generated struct will implement the `TryFrom<vim_rs::types::structs::ObjectContent>`
/// trait, allowing you to convert the output of the `PropertyCollector::retrieve_properties_ex`
/// into vector of objects from the generated struct.
#[proc_macro]
pub fn vim_retrievable(input: TokenStream) -> TokenStream {
    let VimObjectMacro { struct_token: _, struct_name, colon_token: _, object_type: managed_object_type, brace_token: _, fields } =
        parse_macro_input!(input as VimObjectMacro);

    let (field_infos, errors) = resolve_fields(&managed_object_type, &fields);

    let struct_tokens = generate_struct_decl(&struct_name, &field_infos);

    let struct_impl_tokens = generate_retrieve_struct_impl(&struct_name, &managed_object_type, &field_infos);

    let try_from_object_content = generate_try_from_object_content(&struct_name, &field_infos);

    let output = quote! {
        #( #errors )*

        #struct_tokens
        #struct_impl_tokens

        #try_from_object_content
    };
    output.into()
}

/// A macro to generate a struct and implementation necessary to work with
/// PropertyCollector::wait_for_updates_ex API. Developers need to select the managed object type
/// of interest and the properties they need replicated. For example to replicate properties of a
/// VirtualMachine object, the macro can be used as follows:
///
/// ```ignore
/// vim_updatable!(
///    struct VM: VirtualMachine {
///       name = "name",
///       os = "summary.guest.guest_full_name",
///       storage = "summary.storage",
///       host_cpu = "summary.quick_stats.overall_cpu_usage",
///       host_memory = "summary.quick_stats.host_memory_usage",
///       status = "summary.overall_status",
///       power_state = "runtime.power_state",
///       devices = "config.hardware.device",
///       ft_info = "config.ft_info",
///   }
/// );
/// ```
/// The macro will generate a struct `VM` with the specified properties and extract their types from
/// the vSphere API. The generated struct will implement the `TryFrom<vim_rs::types::structs::ObjectUpdate>`
/// trait, allowing you to convert the output of the `PropertyCollector::wait_for_updates_ex`
/// into vector of objects from the generated struct. Subsequently, the generated struct content can
/// be updated using the `apply_update` method. The generated struct will also implement the
/// `Queriable` trait, allowing you to use the `prop_spec` method to generate a `PropertySpec` for
/// the specified properties.
///
/// The generated struct is usable with the `ObjectCache` and `CacheManager` utility objects.
#[proc_macro]
pub fn vim_updatable(input: TokenStream) -> TokenStream {
    let VimObjectMacro { struct_token: _, struct_name, colon_token: _, object_type: managed_object_type, brace_token: _, fields } =
        parse_macro_input!(input as VimObjectMacro);

    let (field_infos, errors) = resolve_fields(&managed_object_type, &fields);

    let struct_tokens = generate_struct_decl(&struct_name, &field_infos);

    let struct_impl_tokens = generate_updateable_struct_impl(&struct_name, &managed_object_type, &field_infos);

    let try_from_object_content = generate_try_from_object_update(&struct_name, &field_infos);

    let output = quote! {
        #( #errors )*

        #struct_tokens
        #struct_impl_tokens

        #try_from_object_content
    };
    output.into()
}

fn resolve_fields<'a>(managed_object_type: &Ident, fields: &'a Punctuated<PropertyField, Comma>) -> (Vec<FieldInfo<'a>>, Vec<proc_macro2::TokenStream>) {
    let mut field_infos = Vec::new();
    let mut errors: Vec<proc_macro2::TokenStream> = Vec::new();
    for property_field in fields {
        let path_str = property_field.path.value();
        let res = resolver::resolve_path(&managed_object_type.to_string(), &path_str);
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
    (field_infos, errors)
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
        #[derive(Debug)]
        pub struct #struct_name {
            pub id: vim_rs::types::structs::ManagedObjectReference,
            #(pub #field_declarations,)*
        }
    };
    struct_tokens
}

fn generate_retrieve_struct_impl(struct_name: &Ident, managed_object_type: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {
    let prop_spec = prop_spec(managed_object_type, fields);
    let id = id();
    quote! {
        impl vim_rs::core::pc_helpers::Queriable for #struct_name {
            #prop_spec
        }

        impl #struct_name {
            pub #id
        }
    }
}

fn generate_updateable_struct_impl(struct_name: &Ident, managed_object_type: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {
    let prop_spec = prop_spec(managed_object_type, fields);
    let id = id();
    let apply_update = generate_apply_update(fields);
    quote! {
        impl vim_rs::core::pc_helpers::Queriable for #struct_name {
            #prop_spec
        }

        impl vim_rs::core::pc_cache::Cacheable for #struct_name {
            #id
            #apply_update
        }
    }
}

fn prop_spec(managed_object_type: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {
    let field_paths: Vec<&str> = fields.iter().map(|f| f.field_data.vim_path.as_str()).collect();
    let prop_paths_quoted: Vec<proc_macro2::TokenStream> = field_paths
        .iter()
        .map(|path| quote! { #path.into() })
        .collect();

    quote! {
        fn prop_spec() -> vim_rs::types::structs::PropertySpec {
            vim_rs::types::structs::PropertySpec {
                all: Some(false),
                path_set: Some(vec![
                    #(#prop_paths_quoted),*
                ]),
                r#type: Into::<&str>::into(vim_rs::types::enums::MoTypesEnum::#managed_object_type).to_string(),
            }
        }
    }
}

fn id() -> proc_macro2::TokenStream {
    quote! {
        fn id(&self) -> &vim_rs::types::structs::ManagedObjectReference {
            &self.id
        }
    }
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
            resolver::FieldProcessingType::Enum(enum_field_name) => {
                field_conversions.push(generate_enum_field_from_content(field, &field_alias, &enum_field_name));
            },
            resolver::FieldProcessingType::Struct => {
                field_conversions.push(generate_struct_field_from_content(field, &field_alias, &field.field_data.data_type));
            },
            resolver::FieldProcessingType::Trait => {
                field_conversions.push(generate_trait_field_from_content(field, &field_alias, &field.field_data.data_type));
            },
        }
        if field.field_data.is_optional {
            field_assignments.push(quote! { #field_name: #field_alias });
        } else {
            let field_name_str = field.field_data.vim_path.as_str();
            field_assignments.push(quote! { #field_name: #field_alias.ok_or_else(|| vim_rs::core::pc_helpers::Error::NoneValueForRequiredField(#field_name_str.to_string()))? });
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
                        name => {
                            return Err(vim_rs::core::pc_helpers::Error::UnexpectedPropertyPath(name.to_string()));
                        }
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

fn generate_try_from_object_update(struct_name: &Ident, fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {

    let mut field_declarations: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut field_conversions: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut field_assignments: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    let mut idx = 1;
    for field in fields {
        let field_alias: Ident = syn::parse_str(&format!("field{}", idx)).unwrap();
        let field_name = &field.property_field.name;
        field_declarations.push(quote! { let mut #field_alias = None; });
        match field.field_data.processing_type {
            resolver::FieldProcessingType::Enum(enum_field_name) => {
                field_conversions.push(generate_enum_field_from_update(field, &field_alias, &enum_field_name));
            },
            resolver::FieldProcessingType::Struct => {
                field_conversions.push(generate_struct_field_from_update(field, &field_alias, &field.field_data.data_type));
            },
            resolver::FieldProcessingType::Trait => {
                field_conversions.push(generate_trait_field_from_update(field, &field_alias, &field.field_data.data_type));
            },
        }
        if field.field_data.is_optional {
            field_assignments.push(quote! { #field_name: #field_alias });
        } else {
            let field_name_str = field.field_data.vim_path.as_str();
            field_assignments.push(quote! { #field_name: #field_alias.ok_or_else(|| vim_rs::core::pc_helpers::Error::NoneValueForRequiredField(#field_name_str.to_string()))? });
        }
        idx += 1;
    }

    quote! {
        impl core::convert::TryFrom<vim_rs::types::structs::ObjectUpdate> for #struct_name {
            type Error = vim_rs::core::pc_helpers::Error;

            fn try_from(row: vim_rs::types::structs::ObjectUpdate) -> vim_rs::core::pc_helpers::Result<Self> {
                let id = row.obj;
                let Some(row) = row.change_set else {
                    return Err(vim_rs::core::pc_helpers::Error::NoDataFound);
                };

                #(#field_declarations)*

                for prop in row {
                    if matches!(prop.op, vim_rs::types::enums::PropertyChangeOpEnum::Add | vim_rs::types::enums::PropertyChangeOpEnum::Remove | vim_rs::types::enums::PropertyChangeOpEnum::Other_(_)) {
                        // It is assumption of the code here that create_filter was called with `partial_updates = false`.
                        // This flag value implies only `assign` and `indirectRemove` operations are returned. `add` and
                        // `remove` operations are used when `partial_updates = true`. The big problem is that with
                        // `partial_updates` server will return paths to sub-properties that we cannot easily resolve in
                        // Rust hence the assumption that `partial_updates = false` is made.`Add` and `Remove` operations
                        // are not supported in this code.
                        // error!("Unsupported PropertyChangeOp: {:?} for property {} in  object {:?}", prop.op, prop.name, id);
                        continue;
                    }
                    match prop.name.as_str() {
                        #(#field_conversions)*
                        name => {
                            return Err(vim_rs::core::pc_helpers::Error::UnexpectedPropertyPath(name.to_string()));
                        }
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


fn generate_apply_update(fields: &Vec<FieldInfo>) -> proc_macro2::TokenStream {

    let mut field_conversions: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields.len());
    for field in fields {

        match field.field_data.processing_type {
            resolver::FieldProcessingType::Enum(enum_field_name) => {
                field_conversions.push(generate_enum_field_apply(field, &enum_field_name));
            },
            resolver::FieldProcessingType::Struct => {
                field_conversions.push(generate_struct_field_apply(field, &field.field_data.data_type));
            },
            resolver::FieldProcessingType::Trait => {
                field_conversions.push(generate_trait_field_apply(field, &field.field_data.data_type));
            },
        }
    }

    quote! {
        fn apply_update(&mut self, update: vim_rs::types::structs::ObjectUpdate) -> vim_rs::core::pc_helpers::Result<()> {
            let Some(row) = update.change_set else {
                return Ok(());
            };

            for prop in row {
                if matches!(prop.op, vim_rs::types::enums::PropertyChangeOpEnum::Add | vim_rs::types::enums::PropertyChangeOpEnum::Remove | vim_rs::types::enums::PropertyChangeOpEnum::Other_(_)) {
                    // It is assumption of the code here that create_filter was called with `partial_updates = false`.
                    // This flag value implies only `assign` and `indirectRemove` operations are returned. `add` and
                    // `remove` operations are used when `partial_updates = true`. The big problem is that with
                    // `partial_updates` server will return paths to sub-properties that we cannot easily resolve in
                    // Rust hence the assumption that `partial_updates = false` is made.`Add` and `Remove` operations
                    // are not supported in this code.
                    // error!("Unsupported PropertyChangeOp: {:?} for property {} in  object {:?}", prop.op, prop.name, id);
                    continue;
                }
                match prop.name.as_str() {
                    #(#field_conversions)*
                    name => {
                        return Err(vim_rs::core::pc_helpers::Error::UnexpectedPropertyPath(name.to_string()));
                    }
                }
            }
            Ok(())
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
fn generate_enum_field_from_content(field: &FieldInfo, field_alias: &Ident, enum_field_name: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    let enum_field = Ident::new(enum_field_name, field.property_field.path.span());
    quote! {
        #path => {
            #field_alias = match prop.val {
                vim_rs::types::vim_any::VimAny::Value(vim_rs::types::boxed_types::ValueElements::#enum_field(vd)) => Some(vd),
                ref val => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType { property: #path.to_string(), expected: #enum_field_name.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

fn generate_enum_field_from_update(field: &FieldInfo, field_alias: &Ident, enum_field_name: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    let enum_field = Ident::new(enum_field_name, field.property_field.path.span());
    quote! {
        #path => {
            #field_alias = match prop.val {
                Some(vim_rs::types::vim_any::VimAny::Value(vim_rs::types::boxed_types::ValueElements::#enum_field(vd))) => Some(vd),
                None => continue,
                Some(ref val) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType { property: #path.to_string(), expected: #enum_field_name.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

fn generate_enum_field_apply(field: &FieldInfo, enum_field_name: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    let enum_field = Ident::new(enum_field_name, field.property_field.path.span());
    let field_name = &field.property_field.name;
    let none_code;
    let value_code;
    if field.field_data.is_optional {
        none_code = quote! { None };
        value_code = quote! { Some(vd) };
    } else {
        none_code = quote! { return Err(vim_rs::core::pc_helpers::Error::NoneValueForRequiredField(#path.to_string())) };
        value_code = quote! { vd };
    };

    quote! {
        #path => {
            self.#field_name = match prop.val {
                Some(vim_rs::types::vim_any::VimAny::Value(vim_rs::types::boxed_types::ValueElements::#enum_field(vd))) => #value_code,
                None => #none_code,
                Some(ref val) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType { property: #path.to_string(), expected: #enum_field_name.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
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
fn generate_struct_field_from_content(field: &FieldInfo, field_alias: &Ident, struct_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;

    quote! {
        #path => {
            #field_alias = match prop.val {
                vim_rs::types::vim_any::VimAny::Object(obj) => {
                    let name: &'static str = obj.data_type().into();
                    match obj.as_any_box().downcast() {
                        Ok(val) => Some(*val),
                        Err(_) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: name.to_string()}),
                    }
                },
                ref val => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

fn generate_struct_field_from_update(field: &FieldInfo, field_alias: &Ident, struct_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    quote! {
        #path => {
            #field_alias = match prop.val {
                Some(vim_rs::types::vim_any::VimAny::Object(obj)) => {
                    let name: &'static str = obj.data_type().into();
                    match obj.as_any_box().downcast() {
                        Ok(val) => Some(*val),
                        Err(_) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: name.to_string()}),
                    }
                },
                None => continue,
                Some(ref val) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

fn generate_struct_field_apply(field: &FieldInfo, struct_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    let field_name = &field.property_field.name;
    let none_code;
    let value_code;
    if field.field_data.is_optional {
        none_code = quote! { None };
        value_code = quote! { Some(*val) };
    } else {
        none_code = quote! { return Err(vim_rs::core::pc_helpers::Error::NoneValueForRequiredField(#path.to_string())) };
        value_code = quote! { *val };
    };
    quote! {
        #path => {
            self.#field_name = match prop.val {
                Some(vim_rs::types::vim_any::VimAny::Object(obj)) => {
                    let name: &'static str = obj.data_type().into();
                    match obj.as_any_box().downcast() {
                        Ok(val) => #value_code,
                        Err(_) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: name.to_string()}),
                    }
                },
                None => #none_code,
                Some(ref val) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #struct_type.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

// 3. Generate trait type deserialize code for structs with children
//                "<property path>" => {
//                     <field name with ordinal> = match prop.val {
//                         vim_rs::types::vim_any::VimAny::Object(obj) => {
//                             let name: &'static str = obj.data_type().into();
//                             match obj.into_box() {
//                                 Ok(val) => Some(val),
//                                 Err(_) => return Err(pc_helpers::Error::InvalidPropertyType {property: "<property path>".to_string(), expected: "<trait type name>".to_string(), got: name.to_string()}),
//                             }
//                         },
//                         ref val => return Err(pc_helpers::Error::InvalidPropertyType {property: "<property path>".to_string(), expected: "<trait type name>".to_string(), got: pc_helpers::type_name(val)}),
//                     };
//                 }
fn generate_trait_field_from_content(field: &FieldInfo, field_alias: &Ident, trait_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    quote! {
        #path => {
            #field_alias = match prop.val {
                vim_rs::types::vim_any::VimAny::Object(obj) => {
                    let name: &'static str = obj.data_type().into();
                    match vim_rs::types::convert::CastInto::into_box(obj) {
                        Ok(val) => Some(val),
                        Err(_) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: name.to_string()}),
                    }
                },
                ref val => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

fn generate_trait_field_from_update(field: &FieldInfo, field_alias: &Ident, trait_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    quote! {
        #path => {
            #field_alias = match prop.val {
                Some(vim_rs::types::vim_any::VimAny::Object(obj)) => {
                    let name: &'static str = obj.data_type().into();
                    match vim_rs::types::convert::CastInto::into_box(obj) {
                        Ok(val) => Some(val),
                        Err(_) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: name.to_string()}),
                    }
                },
                None => continue,
                Some(ref val) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

fn generate_trait_field_apply(field: &FieldInfo, trait_type: &str) -> proc_macro2::TokenStream {
    let path = &field.field_data.vim_path;
    let field_name = &field.property_field.name;
    let none_code;
    let value_code;
    if field.field_data.is_optional {
        none_code = quote! { None };
        value_code = quote! { Some(val) };
    } else {
        none_code = quote! { return Err(vim_rs::core::pc_helpers::Error::NoneValueForRequiredField(#path.to_string())) };
        value_code = quote! { val };
    };

    quote! {
        #path => {
            self.#field_name = match prop.val {
                Some(vim_rs::types::vim_any::VimAny::Object(obj)) => {
                    let name: &'static str = obj.data_type().into();
                    match vim_rs::types::convert::CastInto::into_box(obj) {
                        Ok(val) => #value_code,
                        Err(_) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: name.to_string()}),
                    }
                },
                None => #none_code,
                Some(ref val) => return Err(vim_rs::core::pc_helpers::Error::InvalidPropertyType {property: #path.to_string(), expected: #trait_type.to_string(), got: vim_rs::core::pc_helpers::type_name(val)}),
            };
        }
    }
}

