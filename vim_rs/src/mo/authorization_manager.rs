use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::AuthorizationDescription;
use crate::types::structs::AuthorizationPrivilege;
use crate::types::structs::AuthorizationRole;
use crate::types::structs::EntityPrivilege;
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::Permission;
use crate::types::structs::UserPrivilegeResult;
/// This managed object provides operations to query and update
/// roles and permissions.
/// 
/// **Privileges** are the basic individual rights required to
/// perform operations. They are statically defined and
/// never change for a single version of a product. Examples
/// of privileges are &quot;Power on a virtual machine&quot;
/// or &quot;Configure a host.&quot;
/// 
/// **Roles** are aggregations of privileges, used for convenience.
/// For user-defined roles, the system-defined privileges, "System.Anonymous",
/// "System.View", and "System.Read" are always present.
/// 
/// **Permissions** are the actual access-control rules. A
/// permission is defined on a ManagedEntity and
/// specifies the user or group (&quot;principal&quot;) to which
/// the rule applies. The role specifies the
/// privileges to apply, and the propagate flag
/// specifies whether or not the rule applies to sub-objects
/// of the managed entity.
/// 
/// A ManagedEntity may have multiple permissions,
/// but may have only one permission per user or group. If, when logging
/// in, a user has both a user permission and a group permission
/// (as a group member) for the same entity, then the
/// user-specific permission takes precedent. If there is no
/// user-specific permission, but two or more group permissions
/// are present, and the user is a member of the groups, then the
/// privileges are the union of the specified roles.
/// 
/// Managed entities may be collected together into a &quot;complex entity&quot; for
/// the purpose of applying permissions consistently. Complex entities may have a
/// Datacenter, ComputeResource, or ClusterComputeResource as a parent, with other
/// child managed objects as additional parts of the complex entity:
/// - A Datacenter's child objects are the root virtual machine and host Folders.
/// - A ComputeResource's child objects are the root ResourcePool and HostSystem.
/// - A ClusterComputeResource has only the root ResourcePool as a child object.
///   
/// Child objects in a complex entity are forced to inherit permissions from the
/// parent object. When query operations are used to discover permissions on child
/// objects of complex entities, different results may be returned for the owner of the
/// permission. In some cases, the child object of the complex entity is returned as
/// the object that defines the permission, and in other cases, the parent from which
/// the permission is propagated is returned as the object that defines the permission.
/// In both cases, the information about the owner of the permission is correct, since
/// the entities within a complex entity are considered equivalent. Permissions
/// defined on complex entities are always applicable on the child entities,
/// regardless of the propagation flag, but may only be defined or modified on the
/// parent object.
/// 
/// In a group of fault-tolerance (FT) protected VirtualMachines, the secondary
/// VirtualMachines are forced to inherit permissions from the primary VirtualMachine.
/// Queries to discover permissions on FT secondary VMs always return the primary VM
/// as the object that defines the permissions. Permissions defined on an FT primary
/// VM are always applicable on its secondary VMs, but can only be defined or modified
/// on the primary VM.
pub struct AuthorizationManager {
    client: Arc<Client>,
    mo_id: String,
}
impl AuthorizationManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Adds a new role.
    /// 
    /// This method will add a user-defined role with given list of privileges
    /// and three system-defined privileges, "System.Anonymous", "System.View",
    /// and "System.Read".
    /// 
    /// ***Required privileges:*** Authorization.ModifyRoles
    ///
    /// ## Parameters:
    ///
    /// ### name
    /// Name of the new role.
    ///
    /// ### priv_ids
    /// List of privileges to assign to the role.
    ///
    /// ## Returns:
    ///
    /// The roleId assigned to the new role.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if a role with the given name already exists.
    /// 
    /// ***InvalidName***: if the role name is empty.
    /// 
    /// ***InvalidArgument***: if privIds contains an unknown privilege.
    pub async fn add_authorization_role(&self, name: &str, priv_ids: Option<&[String]>) -> Result<i32> {
        let input = AddAuthorizationRoleRequestType {name, priv_ids, };
        let path = format!("/AuthorizationManager/{moId}/AddAuthorizationRole", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute(req).await
    }
    /// Get the list of effective privileges for a user,
    /// either granted explicitly, or through group membership.
    /// 
    /// This API is implemented only by vCenter Server.
    ///
    /// ## Parameters:
    ///
    /// ### entities
    /// are the entities to retrieve privileges on
    /// 
    /// ***Required privileges:*** System.View
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ### user_name
    /// is the user to retrieve privileges for
    ///
    /// ## Returns:
    ///
    /// the privilege check result for each entity
    pub async fn fetch_user_privilege_on_entities(&self, entities: &[ManagedObjectReference], user_name: &str) -> Result<Option<Vec<UserPrivilegeResult>>> {
        let input = FetchUserPrivilegeOnEntitiesRequestType {entities, user_name, };
        let path = format!("/AuthorizationManager/{moId}/FetchUserPrivilegeOnEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Check whether a session holds a set of privileges on a set of managed entities.
    /// 
    /// If the session does not exist, false is returned for all privileges of
    /// all the entities.
    /// 
    /// This API is implemented only by vCenter Server.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The set of entities on which the privileges are checked.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instances of *ManagedEntity*.
    ///
    /// ### session_id
    /// The session ID to check privileges for. A sesssion ID can be
    /// obtained from *UserSession.key*.
    ///
    /// ### priv_id
    /// The array of privilege IDs to check.
    ///
    /// ## Returns:
    ///
    /// The privilege check result.
    pub async fn has_privilege_on_entities(&self, entity: &[ManagedObjectReference], session_id: &str, priv_id: Option<&[String]>) -> Result<Option<Vec<EntityPrivilege>>> {
        let input = HasPrivilegeOnEntitiesRequestType {entity, session_id, priv_id, };
        let path = format!("/AuthorizationManager/{moId}/HasPrivilegeOnEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Check whether a session holds a set of privileges on a managed entity.
    /// 
    /// If the session does not exist, false is returned for all privileges.
    /// 
    /// This API is implemented only by vCenter Server.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity on which the privileges are checked.
    /// 
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### session_id
    /// The session ID to check privileges for. A sesssion ID can be
    /// obtained from *UserSession.key*.
    ///
    /// ### priv_id
    /// The array of privilege IDs to check.
    ///
    /// ## Returns:
    ///
    /// a boolean value for each privilege indicating whether the session holds the
    /// privilege.
    pub async fn has_privilege_on_entity(&self, entity: &ManagedObjectReference, session_id: &str, priv_id: Option<&[String]>) -> Result<Option<Vec<bool>>> {
        let input = HasPrivilegeOnEntityRequestType {entity, session_id, priv_id, };
        let path = format!("/AuthorizationManager/{moId}/HasPrivilegeOnEntity", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Checks if a user holds a certain set of privileges on a number of
    /// managed entities.
    /// 
    /// Privileges may be granted to users through their
    /// respective group membership. If a privilege is granted to a group it is
    /// implicitly granted to its members.
    /// 
    /// This API is implemented only by vCenter Server.
    ///
    /// ## Parameters:
    ///
    /// ### entities
    /// are the managed objects to check privileges on. If they
    /// refer to managed objects that are not managed entities
    /// the privilege check will be done on the root folder.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ### user_name
    /// is the name of the user to check privileges for. Both
    /// UPN and PreWindows2000LogonName user name formats
    /// are supported.
    ///
    /// ### priv_id
    /// is the set of privileges to check for
    ///
    /// ## Returns:
    ///
    /// the privilege check result
    pub async fn has_user_privilege_on_entities(&self, entities: &[ManagedObjectReference], user_name: &str, priv_id: Option<&[String]>) -> Result<Option<Vec<EntityPrivilege>>> {
        let input = HasUserPrivilegeOnEntitiesRequestType {entities, user_name, priv_id, };
        let path = format!("/AuthorizationManager/{moId}/HasUserPrivilegeOnEntities", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Reassigns all permissions of a role to another role.
    /// 
    /// ***Required privileges:*** Authorization.ReassignRolePermissions
    ///
    /// ## Parameters:
    ///
    /// ### src_role_id
    /// The ID of the source role providing the permissions
    /// which are changing.
    ///
    /// ### dst_role_id
    /// The ID of the destination role to which the
    /// permissions are reassigned.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if either the source or destination role does not exist.
    /// 
    /// ***InvalidArgument***: if dstRoleId is the View or Anonymous role or if
    /// both role IDs are the same.
    /// 
    /// ***AuthMinimumAdminPermission***: if srcRoleId is the Administrator role, meaning
    /// that applying the change would leave the system with
    /// no Administrator permission on the root node.
    /// 
    /// ***NoPermission***: if current session does not have any privilege
    /// in the source or destination role or
    /// "Authorization.ReassignRolePermissions"
    /// privilege on the root folder.
    pub async fn merge_permissions(&self, src_role_id: i32, dst_role_id: i32) -> Result<()> {
        let input = MergePermissionsRequestType {src_role_id, dst_role_id, };
        let path = format!("/AuthorizationManager/{moId}/MergePermissions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Removes a permission rule from an entity.
    /// 
    /// This will fail with an InvalidArgument fault if called on: the direct child
    /// folders of a datacenter managed object, the root resource pool of a
    /// ComputeResource or ClusterComputeResource, or a HostSystem that is part of
    /// a ComputeResource (Stand-alone Host). These objects always have the same
    /// permissions as their parent.
    /// 
    /// This will fail with an InvalidArgument fault if called on a fault-tolerance (FT)
    /// secondary VirtualMachine. Such a VirtualMachine always has the same permissions
    /// as its FT primary VirtualMachine.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// Entity on which a permission is removed.
    /// 
    /// ***Required privileges:*** Authorization.ModifyPermissions
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### user
    /// User or group for which the permission is defined.
    ///
    /// ### is_group
    /// True, if user refers to a group name; false, for a user name.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a permission for this entity and user or group
    /// does not exist.
    /// 
    /// ***AuthMinimumAdminPermission***: if this change would leave the system with
    /// no Administrator permission on the root node.
    /// 
    /// ***InvalidArgument***: if one of the new role IDs is the View or
    /// Anonymous role, or the entity does not support
    /// removing permissions.
    /// 
    /// ***NoPermission***: if current session does not have any privilege
    /// in the permission to be removed or
    /// "Authorization.ModifyPermissions" privilege
    /// on the entity.
    pub async fn remove_entity_permission(&self, entity: &ManagedObjectReference, user: &str, is_group: bool) -> Result<()> {
        let input = RemoveEntityPermissionRequestType {entity, user, is_group, };
        let path = format!("/AuthorizationManager/{moId}/RemoveEntityPermission", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Removes a role.
    /// 
    /// ***Required privileges:*** Authorization.ModifyRoles
    ///
    /// ## Parameters:
    ///
    /// ### role_id
    /// -
    ///
    /// ### fail_if_used
    /// If true, prevents the role from being
    /// removed if any permissions are using it.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the role does not exist.
    /// 
    /// ***InvalidArgument***: if the role is a system role, meaning it cannot be
    /// changed.
    /// 
    /// ***RemoveFailed***: if failIfUsed is true and the role has permissions.
    pub async fn remove_authorization_role(&self, role_id: i32, fail_if_used: bool) -> Result<()> {
        let input = RemoveAuthorizationRoleRequestType {role_id, fail_if_used, };
        let path = format!("/AuthorizationManager/{moId}/RemoveAuthorizationRole", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Update the entire set of permissions defined on an entity.
    /// 
    /// Any
    /// existing permissions on the entity are removed and replaced with the
    /// provided set.
    /// 
    /// If a permission is specified multiple times for the same user or group, the
    /// last permission specified takes effect.
    /// 
    /// The operation is transactional per permission and could partially fail. The
    /// updates are performed in the order of the permission array argument. The first
    /// failed update will abort the operation and throw the appropriate exception. When
    /// the operation aborts, any permissions that have not yet been removed are left in
    /// their original state.
    /// 
    /// After updates are applied, original permissions that are not in the new set
    /// are removed. A failure to remove a permission, such as a violation of
    /// the minimum administrator permission rule, will abort the operation and could
    /// leave remaining original permissions still effective on the entity.
    /// 
    /// This will fail with an InvalidArgument fault if called on: the direct child
    /// folders of a datacenter managed object, the root resource pool of a
    /// ComputeResource or ClusterComputeResource, or a HostSystem that is part of
    /// a ComputeResource (Stand-alone Host). These objects always have the same
    /// permissions as their parent.
    /// 
    /// This will fail with an InvalidArgument fault if called on a fault-tolerance (FT)
    /// secondary VirtualMachine. Such a VirtualMachine always has the same permissions
    /// as its FT primary VirtualMachine.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity on which permissions are updated.
    /// 
    /// ***Required privileges:*** Authorization.ModifyPermissions
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### permission
    /// The list of Permission objects that define
    /// the new rules for access to the entity and
    /// potentially entities below it. If the list
    /// is empty, all permissions on the entity are removed.
    ///
    /// ## Errors:
    ///
    /// ***ManagedObjectNotFound***: if the given entity does not exist.
    /// 
    /// ***UserNotFound***: if one of the given users or groups does not exist.
    /// 
    /// ***NotFound***: if a permission for this entity and user or group
    /// does not exist.
    /// 
    /// ***AuthMinimumAdminPermission***: if this change would leave the system with
    /// no Administrator permission on the root node, or it
    /// would grant further permission to a user or group who
    /// already has Administrator permission on the root node.
    /// 
    /// ***InvalidArgument***: if one of the new role IDs is the View or
    /// Anonymous role, or the entity does not support
    /// assigning permissions.
    /// 
    /// ***NoPermission***: if current session does not have any privilege
    /// in the updated permission or
    /// "Authorization.ModifyPermissions" privilege on
    /// the entity.
    pub async fn reset_entity_permissions(&self, entity: &ManagedObjectReference, permission: Option<&[Permission]>) -> Result<()> {
        let input = ResetEntityPermissionsRequestType {entity, permission, };
        let path = format!("/AuthorizationManager/{moId}/ResetEntityPermissions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Finds all permissions defined in the system.
    /// 
    /// The result is restricted to the managed entities visible to the
    /// user making the call.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn retrieve_all_permissions(&self) -> Result<Option<Vec<Permission>>> {
        let path = format!("/AuthorizationManager/{moId}/RetrieveAllPermissions", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Gets permissions defined on or effective on a managed entity.
    /// 
    /// This returns the actual permission objects defined in the system for all
    /// users and groups relative to the managed entity. The inherited
    /// flag specifies whether or not to include permissions defined by the
    /// parents of this entity that propagate to this entity.
    /// 
    /// For complex entities, the entity reported as defining the permission may
    /// be either the parent or a child entity belonging to the complex entity.
    /// 
    /// The purpose of this method is to discover permissions
    /// for administration purposes, not to determine the current
    /// permissions. The current user's permissions are found on the *ManagedEntity.effectiveRole* property of the user's ManagedEntity.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// ***Required privileges:*** System.Read
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### inherited
    /// Whether or not to include propagating permissions
    /// defined by parent entities.
    pub async fn retrieve_entity_permissions(&self, entity: &ManagedObjectReference, inherited: bool) -> Result<Option<Vec<Permission>>> {
        let input = RetrieveEntityPermissionsRequestType {entity, inherited, };
        let path = format!("/AuthorizationManager/{moId}/RetrieveEntityPermissions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Finds all the permissions that use a particular role.
    /// 
    /// The result is restricted to managed entities that are visible to the
    /// user making the call.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### role_id
    /// -
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the role does not exist.
    pub async fn retrieve_role_permissions(&self, role_id: i32) -> Result<Option<Vec<Permission>>> {
        let input = RetrieveRolePermissionsRequestType {role_id, };
        let path = format!("/AuthorizationManager/{moId}/RetrieveRolePermissions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_option(req).await
    }
    /// Defines one or more permission rules on an entity or updates rules if already
    /// present for the given user or group on the entity.
    /// 
    /// If a permission is specified multiple times for the same user or group, then the
    /// last permission specified takes effect.
    /// 
    /// The operation is applied transactionally per permission and is applied to the
    /// entity following the order of the elements in the permission array argument. This
    /// means that if a failure occurs, the method terminates at that point in the
    /// permission array with an exception, leaving at least one and as many as all
    /// permissions unapplied.
    /// 
    /// This will fail with an InvalidArgument fault if called on: the direct child
    /// folders of a datacenter managed object, the root resource pool of a
    /// ComputeResource or ClusterComputeResource, or a HostSystem that is part of
    /// a ComputeResource (Stand-alone Host). These objects always have the same
    /// permissions as their parent.
    /// 
    /// This will fail with an InvalidArgument fault if called on a fault-tolerance (FT)
    /// secondary VirtualMachine. Such a VirtualMachine always has the same permissions
    /// as its FT primary VirtualMachine.
    ///
    /// ## Parameters:
    ///
    /// ### entity
    /// The entity on which to set permissions.
    /// 
    /// ***Required privileges:*** Authorization.ModifyPermissions
    /// 
    /// Refers instance of *ManagedEntity*.
    ///
    /// ### permission
    /// An array of specifications for permissions on the entity.
    ///
    /// ## Errors:
    ///
    /// ***ManagedObjectNotFound***: if the given entity does not exist.
    /// 
    /// ***UserNotFound***: if a given user or group does not exist.
    /// 
    /// ***AuthMinimumAdminPermission***: if this change would leave the system with
    /// no Administrator permission on the root node, or it
    /// would grant further permission to a user or group who
    /// already has Administrator permission on the root node.
    /// 
    /// ***NotFound***: if a permission's roleId is not valid.
    /// 
    /// ***InvalidArgument***: if one of the new role IDs is the View or
    /// Anonymous role, or the entity does not support assigning
    /// permissions.
    /// 
    /// ***NoPermission***: if current session does not have any privilege
    /// in any permission that being set or
    /// "Authorization.ModifyPermissions" privilege on
    /// the entity.
    pub async fn set_entity_permissions(&self, entity: &ManagedObjectReference, permission: Option<&[Permission]>) -> Result<()> {
        let input = SetEntityPermissionsRequestType {entity, permission, };
        let path = format!("/AuthorizationManager/{moId}/SetEntityPermissions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Updates a role's name or privileges.
    /// 
    /// If the new set of privileges are assigned to the role, the
    /// system-defined privileges, "System.Anonymous", "System.View",
    /// and "System.Read" will be assigned to the role too.
    /// This operation might return before the new privileges are effective.
    /// A timeout of 100 ms is possible, but it might vary depending on
    /// the configuration and the load of the system.
    /// 
    /// ***Required privileges:*** Authorization.ModifyRoles
    ///
    /// ## Parameters:
    ///
    /// ### role_id
    /// The ID of the role that is updated.
    ///
    /// ### new_name
    /// The new name for the role.
    ///
    /// ### priv_ids
    /// The new set of privileges to assign to the role.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if the role does not exist, or if a privilege
    /// in the list cannot be found.
    /// 
    /// ***InvalidArgument***: if the role is a system role, meaning it cannot be
    /// changed.
    /// 
    /// ***InvalidName***: if the new role name is empty.
    /// 
    /// ***AlreadyExists***: if another role with the given name already exists.
    /// 
    /// ***NoPermission***: if current session does not have any privilege
    /// that being updated in the new role or
    /// "Authorization.ModifyRoles" privilege on the
    /// root folder.
    pub async fn update_authorization_role(&self, role_id: i32, new_name: &str, priv_ids: Option<&[String]>) -> Result<()> {
        let input = UpdateAuthorizationRoleRequestType {role_id, new_name, priv_ids, };
        let path = format!("/AuthorizationManager/{moId}/UpdateAuthorizationRole", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Static, descriptive strings for system roles and privileges.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn description(&self) -> Result<AuthorizationDescription> {
        let path = format!("/AuthorizationManager/{moId}/description", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// The list of system-defined privileges.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn privilege_list(&self) -> Result<Option<Vec<AuthorizationPrivilege>>> {
        let path = format!("/AuthorizationManager/{moId}/privilegeList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
    /// The currently defined roles in the system, including
    /// static system-defined roles.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn role_list(&self) -> Result<Option<Vec<AuthorizationRole>>> {
        let path = format!("/AuthorizationManager/{moId}/roleList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute_option(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AddAuthorizationRoleRequestType<'a> {
    name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privIds")]
    priv_ids: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct FetchUserPrivilegeOnEntitiesRequestType<'a> {
    entities: &'a [ManagedObjectReference],
    #[serde(rename = "userName")]
    user_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HasPrivilegeOnEntitiesRequestType<'a> {
    entity: &'a [ManagedObjectReference],
    #[serde(rename = "sessionId")]
    session_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privId")]
    priv_id: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HasPrivilegeOnEntityRequestType<'a> {
    entity: &'a ManagedObjectReference,
    #[serde(rename = "sessionId")]
    session_id: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privId")]
    priv_id: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct HasUserPrivilegeOnEntitiesRequestType<'a> {
    entities: &'a [ManagedObjectReference],
    #[serde(rename = "userName")]
    user_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privId")]
    priv_id: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct MergePermissionsRequestType {
    #[serde(rename = "srcRoleId")]
    src_role_id: i32,
    #[serde(rename = "dstRoleId")]
    dst_role_id: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveEntityPermissionRequestType<'a> {
    entity: &'a ManagedObjectReference,
    user: &'a str,
    #[serde(rename = "isGroup")]
    is_group: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveAuthorizationRoleRequestType {
    #[serde(rename = "roleId")]
    role_id: i32,
    #[serde(rename = "failIfUsed")]
    fail_if_used: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ResetEntityPermissionsRequestType<'a> {
    entity: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    permission: Option<&'a [Permission]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveEntityPermissionsRequestType<'a> {
    entity: &'a ManagedObjectReference,
    inherited: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveRolePermissionsRequestType {
    #[serde(rename = "roleId")]
    role_id: i32,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetEntityPermissionsRequestType<'a> {
    entity: &'a ManagedObjectReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    permission: Option<&'a [Permission]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateAuthorizationRoleRequestType<'a> {
    #[serde(rename = "roleId")]
    role_id: i32,
    #[serde(rename = "newName")]
    new_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privIds")]
    priv_ids: Option<&'a [String]>,
}
