// use std::collections::HashMap;
// use std::vec;
// use sw4rm_rs::RefOr::{Item, Reference};

// use sw4rm_rs::shared::schema::Schema;
// use sw4rm_rs::shared::schema::SchemaType::Object;

// pub fn print_schema() {
//     let mut schema =  Schema {
//         format: None,
//         title: None,
//         description: Some("VirtualDevice is the base data object type for devices in a virtual machine.\n\nThis type contains enough information about a virtual device to allow clients\nto display devices they do not recognize. For example, a client with an\nolder version than the server to which it connects may see a device\nwithout knowing what it is.\n".into()),
//         default: None,
//         multiple_of: None,
//         maximum: None,
//         exclusive_maximum: None,
//         minimum: None,
//         exclusive_minimum: None,
//         max_length: None,
//         min_length: None,
//         pattern: None,
//         max_items: None,
//         min_items: None, unique_items: None,
//         max_properties: None,
//         min_properties: None,
//         required: vec!["key".into()],
//         enum_values: vec![], 
//         schema_type: Some(Object), items: None,
//         all_of: vec![ Reference { reference_path: "#/components/schemas/DataObject".into() }], 
//         properties: HashMap::new(),
//         additional_properties: None, discriminator: None, read_only: None, xml: None, external_docs: None, example: None,
//         x_fields: HashMap::new(),
//         one_of: vec![],
//         any_of: vec![],
//         not: None,
//         nullable: None,
//         write_only: None,
//         deprecated: None,
//     };
//     schema.properties.insert(
//         "backing".into(),
//         Reference {
//             reference_path: "#/components/schemas/VirtualDeviceBackingInfo".into(),
//         },
//     );
//     schema.properties.insert(
//         "connectable".into(),
//         Reference {
//             reference_path: "#/components/schemas/VirtualDeviceConnectInfo".into(),
//         },
//     );
//     schema.properties.insert(
//         "slotInfo".into(),
//         Reference {
//             reference_path: "#/components/schemas/VirtualDeviceBusSlotInfo".into(),
//         },
//     );
//     schema.properties.insert(
//         "deviceInfo".into(),
//         Reference {
//             reference_path: "#/components/schemas/Description".into(),
//         },
//     );
//     schema.properties.insert("key".into(), Item(
//         Box::new(Schema { 
//             format: Some("int32".into()), 
//             title: None, 
//             description: Some("A unique key that distinguishes this device from other\ndevices in the same virtual machine.\n\nKeys are immutable but may be\nrecycled; that is, a key does not change as long as the device is\nassociated with a particular virtual machine. However, once a device is\nremoved, its key may be used when another device is added.\n\nThis property is not read-only, but the client cannot control its value.\nPersistent device keys are always assigned and managed by the server, which\nguarantees that all devices will have non-negative key values.\n\nWhen adding new devices, it may be necessary for a client to assign keys\ntemporarily in order to associate controllers with devices in\nconfiguring a virtual machine. However, the server does not allow a\nclient to reassign a device key, and the server may assign a different\nvalue from the one passed during configuration. Clients should ensure\nthat existing device keys are not reused as temporary key values for the\nnew device to be added (for example, by using unique negative integers as\ntemporary keys).\n\nWhen editing or deleting a device, clients must use the server-provided key\nto refer to an existing device.\n".into()),
//             default: None, 
//             multiple_of: None, 
//             maximum: None, 
//             exclusive_maximum: None, 
//             minimum: None, 
//             exclusive_minimum: None, 
//             max_length: None, 
//             min_length: None, 
//             pattern: None, 
//             max_items: None, 
//             min_items: None, 
//             unique_items: None,
//             max_properties: None,
//             min_properties: None,
//             required: vec![],
//             enum_values: vec![],
//             schema_type: Some(sw4rm_rs::shared::SchemaType::Integer),
//             items: None, 
//             all_of: vec![], 
//             properties: HashMap::new(), 
//             additional_properties: None,
//             discriminator: None, 
//             read_only: None, 
//             xml: None,
//             external_docs: None,
//             example: None, 
//             x_fields: HashMap::new(), 
//             one_of: vec![],
//             any_of: vec![], 
//             not: None,
//             nullable: None,
//             write_only: None, 
//             deprecated: None 
//         })));
//     schema.properties.insert("controllerKey".into(), Item(
//         Box::new(Schema { 
//             format: Some("int32".into()),
//             title: None, 
//             description: Some("Object key for the controller object for this device.\n\nThis property contains the key property value of the controller device\nobject.\n".into()), 
//             default: None,
//             multiple_of: None,
//             maximum: None,
//             exclusive_maximum: None,
//             minimum: None,
//             exclusive_minimum: None,
//             max_length: None,
//             min_length: None, pattern: None, max_items: None, min_items: None, unique_items: None, max_properties: None, 
//             min_properties: None,
//             required: vec![],
//             enum_values: vec![],
//             schema_type: Some(sw4rm_rs::shared::SchemaType::Integer),
//             items: None,
//             all_of: vec![],
//             properties: HashMap::new(),
//             additional_properties: None, discriminator: None, read_only: None, xml: None, external_docs: None,
//             example: None,
//             x_fields: HashMap::new(),
//             one_of: vec![],
//             any_of: vec![],
//             not: None,
//             nullable: None, 
//             write_only: None,
//             deprecated: None 
//         })));
//     schema.properties.insert("unitNumber".into(), Item(Box::new(Schema {
//             format: Some("int32".into()),
//             title: None, 
//             description: Some("The unit number of this device on its controller.\n\nThis property is null if\nthe controller property is null (for example, when the device is not\nattached to a specific controller object).\n\nNormally, two devices on the same controller\nmay not be assigned the same unit number. If\nmultiple devices could exist on a controller,\nthen unit number has to be specified to\nconfigure respective devices.\n".into()), 
//             default: None, 
//             multiple_of: None, maximum: None, exclusive_maximum: None, 
//             minimum: None, exclusive_minimum: None, max_length: None, min_length: None, pattern: None, max_items: None, 
//             min_items: None, unique_items: None, max_properties: None, min_properties: None,
//             required: vec![],
//             enum_values: vec![],
//             schema_type: Some(sw4rm_rs::shared::SchemaType::Integer), 
//             items: None, 
//             all_of: vec![], 
//             properties: HashMap::new(), 
//             additional_properties: None, 
//             discriminator: None, 
//             read_only: None, xml: None, external_docs: None, example: None, 
//             x_fields: HashMap::new(), 
//             one_of: vec![],
//             any_of: vec![],
//             not: None, nullable: None, write_only: None, deprecated: None
//         })));
//     schema.properties.insert(
//         "deviceGroupInfo".into(),
//         Reference {
//             reference_path: "#/components/schemas/VirtualDeviceDeviceGroupInfo".into(),
//         },
//     );
//     schema.properties.insert("numaNode".into(), Item(Box::new(Schema { 
//             format: Some("int32".into()),
//             title: None,
//             description: Some("The virtual NUMA node.\n\nA negative number means there is no\naffinity for the device. A positive number is a vNUMA node.\nAn unset value of numaNode is status-quo during Reconfigure time.\nIf numaNode is unset during ConfigInfo, then it means there is no\naffinity for the device.\n\n***Since:*** vSphere API Release 8.0.0.1\n".into()),
//             default: None, multiple_of: None, maximum: None, exclusive_maximum: None, minimum: None, exclusive_minimum: None,
//             max_length: None, min_length: None, pattern: None, max_items: None, min_items: None, unique_items: None,
//             max_properties: None, min_properties: None,
//             required: vec![],
//             enum_values: vec![],
//             schema_type: Some(sw4rm_rs::shared::SchemaType::Integer),
//             items: None, all_of: vec![],
//             properties: HashMap::new(), additional_properties: None, discriminator: None, read_only: None,
//             xml: None, external_docs: None, example: None, 
//             x_fields: HashMap::new(),
//             one_of: vec![],
//             any_of: vec![], not: None, nullable: None, write_only: None, deprecated: None })));
//     println!("{:?}", schema);
// }

// #[cfg(test)]
// mod tests {
//     use std::fs::File;
//     use std::io::Read;
//     use std::vec;
//     use sw4rm_rs::Resolvable;
//     use syn;

//     use super::*;

//     #[test]
//     fn it_works() {
//         print_schema();
//     }
// }
