use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This interface is responsible for managing DirectPath profiles in vCenter.
/// 
/// ***Since:*** vSphere API Release 9.0.0.0
#[derive(Clone)]
pub struct DirectPathProfileManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl DirectPathProfileManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Create a DirectPath profile from the specified CreateSpec.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** DirectPathProfileManager.Manage
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Unique identifier of the DirectPath profile created.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: If a DirectPath profile with the attributes in the
    /// createSpec already exists in the target vCenter. There cannot be two
    /// DirectPath profiles with the same name or the same device details.
    /// 
    /// ***InvalidArgument***: If the spec argument does not meet the constraints
    /// specified in *DirectPathProfileManagerCreateSpec*.
    pub async fn direct_path_profile_manager_create(&self, spec: &crate::types::structs::DirectPathProfileManagerCreateSpec) -> Result<String> {
        let input = DirectPathProfileManagerCreateRequestType {spec, };
        let bytes = self.client.invoke("", "DirectPathProfileManager", &self.mo_id, "DirectPathProfileManagerCreate", Some(&input)).await?;
        let result: String = crate::core::client::unmarshal(self.client.transport(), &bytes)?;
        Ok(result)
    }
    /// Delete a DirectPath profile.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** DirectPathProfileManager.Manage
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Unique identifier of the DirectPath profile to be deleted.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: If there is no DirectPath profile found with the
    /// specified identifier.
    /// 
    /// ***ResourceInUse***: If the DirectPath profile with the specified identifier
    /// is being used by a VM or associated with a VM resource profile.
    pub async fn direct_path_profile_manager_delete(&self, id: &str) -> Result<()> {
        let input = DirectPathProfileManagerDeleteRequestType {id, };
        self.client.invoke_void("", "DirectPathProfileManager", &self.mo_id, "DirectPathProfileManagerDelete", Some(&input)).await
    }
    /// List DirectPath profiles in this vCenter that match the specified
    /// filtering criteria.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### filter_spec
    /// -
    ///
    /// ## Returns:
    ///
    /// Information about DirectPath profiles matching the attributes
    /// specified in the input *DirectPathProfileManagerFilterSpec*.
    /// If an empty filterSpec is specified, then all the DirectPath profiles in
    /// the target vCenter are returned. If none of the DirectPath profiles match
    /// the attributes specified in the filterSpec, then an empty list is
    /// returned.
    pub async fn direct_path_profile_manager_list(&self, filter_spec: &crate::types::structs::DirectPathProfileManagerFilterSpec) -> Result<Option<Vec<crate::types::structs::DirectPathProfileInfo>>> {
        let input = DirectPathProfileManagerListRequestType {filter_spec, };
        let bytes_opt = self.client.invoke_optional("", "DirectPathProfileManager", &self.mo_id, "DirectPathProfileManagerList", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Query capacity of DirectPath profiles against a compute resource.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    ///
    /// ## Parameters:
    ///
    /// ### target
    /// specifies the compute resource for which the capacity
    /// needs to be computed. See *DirectPathProfileManagerTargetEntity*. A null or an invalid
    /// target will cause an exception.
    ///
    /// ### query_spec
    /// specifies a list of *DirectPathProfileManagerCapacityQuerySpec*, where each
    /// of them specifies the information about the DirectPath profile for which
    /// capacity needs to be computed.
    ///
    /// ## Returns:
    ///
    /// List of *DirectPathProfileManagerCapacityResult* when target is valid. The
    /// content of the list is subjected to the following conditions:
    /// \- Content of the return list will be in the same order as content of the
    /// list in querySpec argument, except when querySpec is a null or empty
    /// list.
    /// \- If the query specification is of type *DirectPathProfileManagerCapacityQueryById*, then
    /// the capacity of a DirectPath profile with the matching ID is returned.
    /// \- If the query specification is of type *DirectPathProfileManagerCapacityQueryByName*, then
    /// the capacity of a DirectPath profile with the matching name is returned.
    /// \- If the query specification is of type
    /// *DirectPathProfileManagerCapacityQueryByDeviceConfig*, then the capacity of a device with
    /// the specified configuration in the specified Target will be returned.
    /// \- When querySpec is null or an empty list, the returned list contains
    /// *DirectPathProfileManagerCapacityInfo* of all the DirectPath profiles available in the
    /// specified target.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: when target is null or contains invalid content
    /// such as invalid host or cluster.
    pub async fn direct_path_profile_manager_query_capacity(&self, target: &dyn crate::types::traits::DirectPathProfileManagerTargetEntityTrait, query_spec: Option<&[Box<dyn crate::types::traits::DirectPathProfileManagerCapacityQuerySpecTrait>]>) -> Result<Option<Vec<Box<dyn crate::types::traits::DirectPathProfileManagerCapacityResultTrait>>>> {
        let input = DirectPathProfileManagerQueryCapacityRequestType {target, query_spec, };
        let bytes_opt = self.client.invoke_optional("", "DirectPathProfileManager", &self.mo_id, "DirectPathProfileManagerQueryCapacity", Some(&input)).await?;
        match bytes_opt {
            Some(ref b) => Ok(Some(crate::core::client::unmarshal_array(self.client.transport(), b)?)),
            None => Ok(None),
        }
    }
    /// Update a DirectPath profile based on the specified *DirectPathProfileManagerUpdateSpec*.
    /// 
    /// ***Since:*** vSphere API Release 9.0.0.0
    /// 
    /// ***Required privileges:*** DirectPathProfileManager.Manage
    ///
    /// ## Parameters:
    ///
    /// ### id
    /// Unique identifier of the DirectPath profile being updated.
    ///
    /// ### spec
    /// Specification for the DirectPath device being updated.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: If the desired name specified in the spec is
    /// already assigned to a different DirectPath profile.
    /// 
    /// ***InvalidArgument***: If the spec argument does not meet the constraints
    /// specified in *DirectPathProfileManagerUpdateSpec*.
    /// 
    /// ***NotFound***: If there is no DirectPath profile found with the
    /// specified identifier.
    pub async fn direct_path_profile_manager_update(&self, id: &str, spec: &crate::types::structs::DirectPathProfileManagerUpdateSpec) -> Result<()> {
        let input = DirectPathProfileManagerUpdateRequestType {id, spec, };
        self.client.invoke_void("", "DirectPathProfileManager", &self.mo_id, "DirectPathProfileManagerUpdate", Some(&input)).await
    }
}
struct DirectPathProfileManagerCreateRequestType<'a> {
    spec: &'a crate::types::structs::DirectPathProfileManagerCreateSpec,
}

impl<'a> miniserde::Serialize for DirectPathProfileManagerCreateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DirectPathProfileManagerCreateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DirectPathProfileManagerCreateRequestTypeSer<'b, 'a> {
    data: &'b DirectPathProfileManagerCreateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DirectPathProfileManagerCreateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DirectPathProfileManagerCreateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DirectPathProfileManagerDeleteRequestType<'a> {
    id: &'a str,
}

impl<'a> miniserde::Serialize for DirectPathProfileManagerDeleteRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DirectPathProfileManagerDeleteRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DirectPathProfileManagerDeleteRequestTypeSer<'b, 'a> {
    data: &'b DirectPathProfileManagerDeleteRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DirectPathProfileManagerDeleteRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DirectPathProfileManagerDeleteRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DirectPathProfileManagerListRequestType<'a> {
    filter_spec: &'a crate::types::structs::DirectPathProfileManagerFilterSpec,
}

impl<'a> miniserde::Serialize for DirectPathProfileManagerListRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DirectPathProfileManagerListRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DirectPathProfileManagerListRequestTypeSer<'b, 'a> {
    data: &'b DirectPathProfileManagerListRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DirectPathProfileManagerListRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DirectPathProfileManagerListRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("filterSpec"), &self.data.filter_spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct DirectPathProfileManagerQueryCapacityRequestType<'a> {
    target: &'a dyn crate::types::traits::DirectPathProfileManagerTargetEntityTrait,
    query_spec: Option<&'a [Box<dyn crate::types::traits::DirectPathProfileManagerCapacityQuerySpecTrait>]>,
}

impl<'a> miniserde::Serialize for DirectPathProfileManagerQueryCapacityRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DirectPathProfileManagerQueryCapacityRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DirectPathProfileManagerQueryCapacityRequestTypeSer<'b, 'a> {
    data: &'b DirectPathProfileManagerQueryCapacityRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DirectPathProfileManagerQueryCapacityRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DirectPathProfileManagerQueryCapacityRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("target"), &self.data.target as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.query_spec else { continue; };
                    return Some((std::borrow::Cow::Borrowed("querySpec"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct DirectPathProfileManagerUpdateRequestType<'a> {
    id: &'a str,
    spec: &'a crate::types::structs::DirectPathProfileManagerUpdateSpec,
}

impl<'a> miniserde::Serialize for DirectPathProfileManagerUpdateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(DirectPathProfileManagerUpdateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct DirectPathProfileManagerUpdateRequestTypeSer<'b, 'a> {
    data: &'b DirectPathProfileManagerUpdateRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for DirectPathProfileManagerUpdateRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"DirectPathProfileManagerUpdateRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("id"), &self.data.id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
