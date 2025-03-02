use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::CustomizationSpecInfo;
use crate::types::structs::CustomizationSpecItem;
/// The CustomizationSpecManager managed object is used to manage
/// customization specifications stored on the VirtualCenter server.
pub struct CustomizationSpecManager {
    client: Arc<Client>,
    mo_id: String,
}
impl CustomizationSpecManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Validate that required resources are available on the server to customize a
    /// particular guest operating system.
    /// 
    /// These would include sysprep for Windows
    /// and the debugfs and changefs volume editors for Linux guests.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### guest_os
    /// Short name from the guest OS descriptor list describing the OS
    /// we intend to customize.
    ///
    /// ## Errors:
    ///
    /// ***MissingLinuxCustResources***: 
    /// 
    /// ***MissingWindowsCustResources***: 
    /// 
    /// ***UncustomizableGuest***:
    pub async fn check_customization_resources(&self, guest_os: &str) -> Result<()> {
        let input = CheckCustomizationResourcesRequestType {guest_os, };
        let path = format!("/CustomizationSpecManager/{moId}/CheckCustomizationResources", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Creates a new specification.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ModifyCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### item
    /// -
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: 
    /// 
    /// ***CannotDecryptPasswords***:
    pub async fn create_customization_spec(&self, item: &CustomizationSpecItem) -> Result<()> {
        let input = CreateCustomizationSpecRequestType {item, };
        let path = format!("/CustomizationSpecManager/{moId}/CreateCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Deletes a specification.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ModifyCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***:
    pub async fn delete_customization_spec(&self, name: &str) -> Result<()> {
        let input = DeleteCustomizationSpecRequestType {name, };
        let path = format!("/CustomizationSpecManager/{moId}/DeleteCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Duplicates a specification.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ModifyCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// -
    ///
    /// ### new_name
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: 
    /// 
    /// ***AlreadyExists***:
    pub async fn duplicate_customization_spec(&self, name: &str, new_name: &str) -> Result<()> {
        let input = DuplicateCustomizationSpecRequestType {name, new_name, };
        let path = format!("/CustomizationSpecManager/{moId}/DuplicateCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Whether or not a specification exists.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ReadCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// -
    pub async fn does_customization_spec_exist(&self, name: &str) -> Result<bool> {
        let input = DoesCustomizationSpecExistRequestType {name, };
        let path = format!("/CustomizationSpecManager/{moId}/DoesCustomizationSpecExist", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Obtains a specification for the given name.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ReadCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Unique name identifying the requested customization specification.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***:
    pub async fn get_customization_spec(&self, name: &str) -> Result<CustomizationSpecItem> {
        let input = GetCustomizationSpecRequestType {name, };
        let path = format!("/CustomizationSpecManager/{moId}/GetCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Overwrites an existing specification, possibly after retrieving
    /// (by using 'get') and editing it.
    /// 
    /// If, based on the item's changeVersion
    /// value, the overwrite process detects that the specification has changed
    /// since its retrieval, then the API uses the SpecModified exception to
    /// warn clients that they might overwrite another client's change.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ModifyCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### item
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: 
    /// 
    /// ***ConcurrentAccess***: 
    /// 
    /// ***CannotDecryptPasswords***:
    pub async fn overwrite_customization_spec(&self, item: &CustomizationSpecItem) -> Result<()> {
        let input = OverwriteCustomizationSpecRequestType {item, };
        let path = format!("/CustomizationSpecManager/{moId}/OverwriteCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Renames a specification.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ModifyCustSpecs
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// -
    ///
    /// ### new_name
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: 
    /// 
    /// ***AlreadyExists***:
    pub async fn rename_customization_spec(&self, name: &str, new_name: &str) -> Result<()> {
        let input = RenameCustomizationSpecRequestType {name, new_name, };
        let path = format!("/CustomizationSpecManager/{moId}/RenameCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Converts a specification item to XML text
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### item
    /// -
    pub async fn customization_spec_item_to_xml(&self, item: &CustomizationSpecItem) -> Result<String> {
        let input = CustomizationSpecItemToXmlRequestType {item, };
        let path = format!("/CustomizationSpecManager/{moId}/CustomizationSpecItemToXml", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Converts an XML string to a specification item
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec_item_xml
    /// -
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn xml_to_customization_spec_item(&self, spec_item_xml: &str) -> Result<CustomizationSpecItem> {
        let input = XmlToCustomizationSpecItemRequestType {spec_item_xml, };
        let path = format!("/CustomizationSpecManager/{moId}/XmlToCustomizationSpecItem", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Gets a binary public encryption key that can be used to encrypt
    /// passwords in stored specifications.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn encryption_key(&self) -> Result<Option<Vec<i8>>> {
        let path = format!("/CustomizationSpecManager/{moId}/encryptionKey", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// Gets a list of information on available specifications.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ReadCustSpecs
    pub async fn info(&self) -> Result<Option<Vec<CustomizationSpecInfo>>> {
        let path = format!("/CustomizationSpecManager/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckCustomizationResourcesRequestType<'a> {
    #[serde(rename = "guestOs")]
    guest_os: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateCustomizationSpecRequestType<'a> {
    item: &'a CustomizationSpecItem,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DeleteCustomizationSpecRequestType<'a> {
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DuplicateCustomizationSpecRequestType<'a> {
    name: &'a str,
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct DoesCustomizationSpecExistRequestType<'a> {
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct GetCustomizationSpecRequestType<'a> {
    name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct OverwriteCustomizationSpecRequestType<'a> {
    item: &'a CustomizationSpecItem,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RenameCustomizationSpecRequestType<'a> {
    name: &'a str,
    #[serde(rename = "newName")]
    new_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CustomizationSpecItemToXmlRequestType<'a> {
    item: &'a CustomizationSpecItem,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct XmlToCustomizationSpecItemRequestType<'a> {
    #[serde(rename = "specItemXml")]
    spec_item_xml: &'a str,
}
