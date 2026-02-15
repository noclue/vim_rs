use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// The CustomizationSpecManager managed object is used to manage
/// customization specifications stored on the VirtualCenter server.
#[derive(Clone)]
pub struct CustomizationSpecManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl CustomizationSpecManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere 9.0, and there is no replacement for it.
    /// 
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
        let req = self.client.post_json(&path, &input);
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
    pub async fn create_customization_spec(&self, item: &crate::types::structs::CustomizationSpecItem) -> Result<()> {
        let input = CreateCustomizationSpecRequestType {item, };
        let path = format!("/CustomizationSpecManager/{moId}/CreateCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
        let req = self.client.post_json(&path, &input);
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
        let req = self.client.post_json(&path, &input);
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
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn get_customization_spec(&self, name: &str) -> Result<crate::types::structs::CustomizationSpecItem> {
        let input = GetCustomizationSpecRequestType {name, };
        let path = format!("/CustomizationSpecManager/{moId}/GetCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::CustomizationSpecItem = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Whether or not the guest OS is customizable.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### guest_id
    /// Short name from the guest OS descriptor list describing the
    /// OS we intend to check.
    pub async fn is_guest_os_customizable(&self, guest_id: &str) -> Result<bool> {
        let input = IsGuestOsCustomizableRequestType {guest_id, };
        let path = format!("/CustomizationSpecManager/{moId}/IsGuestOsCustomizable", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn overwrite_customization_spec(&self, item: &crate::types::structs::CustomizationSpecItem) -> Result<()> {
        let input = OverwriteCustomizationSpecRequestType {item, };
        let path = format!("/CustomizationSpecManager/{moId}/OverwriteCustomizationSpec", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
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
        let req = self.client.post_json(&path, &input);
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
    pub async fn customization_spec_item_to_xml(&self, item: &crate::types::structs::CustomizationSpecItem) -> Result<String> {
        let input = CustomizationSpecItemToXmlRequestType {item, };
        let path = format!("/CustomizationSpecManager/{moId}/CustomizationSpecItemToXml", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
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
    pub async fn xml_to_customization_spec_item(&self, spec_item_xml: &str) -> Result<crate::types::structs::CustomizationSpecItem> {
        let input = XmlToCustomizationSpecItemRequestType {spec_item_xml, };
        let path = format!("/CustomizationSpecManager/{moId}/XmlToCustomizationSpecItem", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::CustomizationSpecItem = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Gets a binary public encryption key that can be used to encrypt
    /// passwords in stored specifications.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn encryption_key(&self) -> Result<Option<Vec<i8>>> {
        let path = format!("/CustomizationSpecManager/{moId}/encryptionKey", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<i8>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Gets a list of information on available specifications.
    /// 
    /// ***Required privileges:*** VirtualMachine.Provisioning.ReadCustSpecs
    pub async fn info(&self) -> Result<Option<Vec<crate::types::structs::CustomizationSpecInfo>>> {
        let path = format!("/CustomizationSpecManager/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::CustomizationSpecInfo>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct CheckCustomizationResourcesRequestType<'a> {
    guest_os: &'a str,
}

impl<'a> miniserde::Serialize for CheckCustomizationResourcesRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CheckCustomizationResourcesRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CheckCustomizationResourcesRequestTypeSer<'b, 'a> {
    data: &'b CheckCustomizationResourcesRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CheckCustomizationResourcesRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CheckCustomizationResourcesRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("guestOs"), &self.data.guest_os as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateCustomizationSpecRequestType<'a> {
    item: &'a crate::types::structs::CustomizationSpecItem,
}

impl<'a> miniserde::Serialize for CreateCustomizationSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateCustomizationSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateCustomizationSpecRequestTypeSer<'b, 'a> {
    data: &'b CreateCustomizationSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CreateCustomizationSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateCustomizationSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("item"), &self.data.item as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DeleteCustomizationSpecRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for DeleteCustomizationSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DeleteCustomizationSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DeleteCustomizationSpecRequestTypeSer<'b, 'a> {
    data: &'b DeleteCustomizationSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DeleteCustomizationSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DeleteCustomizationSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DuplicateCustomizationSpecRequestType<'a> {
    name: &'a str,
    new_name: &'a str,
}

impl<'a> miniserde::Serialize for DuplicateCustomizationSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DuplicateCustomizationSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DuplicateCustomizationSpecRequestTypeSer<'b, 'a> {
    data: &'b DuplicateCustomizationSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DuplicateCustomizationSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DuplicateCustomizationSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("newName"), &self.data.new_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DoesCustomizationSpecExistRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for DoesCustomizationSpecExistRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DoesCustomizationSpecExistRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DoesCustomizationSpecExistRequestTypeSer<'b, 'a> {
    data: &'b DoesCustomizationSpecExistRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for DoesCustomizationSpecExistRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DoesCustomizationSpecExistRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct GetCustomizationSpecRequestType<'a> {
    name: &'a str,
}

impl<'a> miniserde::Serialize for GetCustomizationSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(GetCustomizationSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct GetCustomizationSpecRequestTypeSer<'b, 'a> {
    data: &'b GetCustomizationSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for GetCustomizationSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"GetCustomizationSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct IsGuestOsCustomizableRequestType<'a> {
    guest_id: &'a str,
}

impl<'a> miniserde::Serialize for IsGuestOsCustomizableRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(IsGuestOsCustomizableRequestTypeSer { data: self, seq: 0 }))
    }
}

struct IsGuestOsCustomizableRequestTypeSer<'b, 'a> {
    data: &'b IsGuestOsCustomizableRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for IsGuestOsCustomizableRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"IsGuestOsCustomizableRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("guestId"), &self.data.guest_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct OverwriteCustomizationSpecRequestType<'a> {
    item: &'a crate::types::structs::CustomizationSpecItem,
}

impl<'a> miniserde::Serialize for OverwriteCustomizationSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(OverwriteCustomizationSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct OverwriteCustomizationSpecRequestTypeSer<'b, 'a> {
    data: &'b OverwriteCustomizationSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for OverwriteCustomizationSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"OverwriteCustomizationSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("item"), &self.data.item as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RenameCustomizationSpecRequestType<'a> {
    name: &'a str,
    new_name: &'a str,
}

impl<'a> miniserde::Serialize for RenameCustomizationSpecRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RenameCustomizationSpecRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RenameCustomizationSpecRequestTypeSer<'b, 'a> {
    data: &'b RenameCustomizationSpecRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for RenameCustomizationSpecRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RenameCustomizationSpecRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("name"), &self.data.name as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("newName"), &self.data.new_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CustomizationSpecItemToXmlRequestType<'a> {
    item: &'a crate::types::structs::CustomizationSpecItem,
}

impl<'a> miniserde::Serialize for CustomizationSpecItemToXmlRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CustomizationSpecItemToXmlRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CustomizationSpecItemToXmlRequestTypeSer<'b, 'a> {
    data: &'b CustomizationSpecItemToXmlRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CustomizationSpecItemToXmlRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CustomizationSpecItemToXmlRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("item"), &self.data.item as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct XmlToCustomizationSpecItemRequestType<'a> {
    spec_item_xml: &'a str,
}

impl<'a> miniserde::Serialize for XmlToCustomizationSpecItemRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(XmlToCustomizationSpecItemRequestTypeSer { data: self, seq: 0 }))
    }
}

struct XmlToCustomizationSpecItemRequestTypeSer<'b, 'a> {
    data: &'b XmlToCustomizationSpecItemRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for XmlToCustomizationSpecItemRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"XmlToCustomizationSpecItemRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("specItemXml"), &self.data.spec_item_xml as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
