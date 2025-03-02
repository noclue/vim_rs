use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostAccessControlEntry;
/// Managed object used to control direct access to the host.
/// 
/// This should be used to control users and privileges on the host directly,
/// which are different from the users and privileges defined in vCenter.
/// 
/// See *AuthorizationManager* for more information on permissions.
pub struct HostAccessManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostAccessManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Update the access mode for a user or group.
    /// 
    /// If the host is in lockdown mode, this operation is allowed only on
    /// users in the exceptions list - see *HostAccessManager.QueryLockdownExceptions*,
    /// and trying to change the access mode of other users or groups
    /// will fail with SecurityError.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### principal
    /// The affected user or group.
    ///
    /// ### is_group
    /// True if principal refers to a group account,
    /// false otherwise.
    ///
    /// ### access_mode
    /// AccessMode to be granted.
    /// *accessOther* is meaningless and
    /// will result in InvalidArgument exception.
    ///
    /// ## Errors:
    ///
    /// ***AuthMinimumAdminPermission***: if this change would render the
    /// ESXi host inaccessible for local non-system users.
    /// The API *HostAccessManager.ChangeLockdownMode* may be used
    /// instead.
    /// 
    /// ***InvalidArgument***: if accessMode is not valid.
    /// 
    /// ***SecurityError***: if the host is in lockdown mode and 'principal'
    /// is not in the exceptions list.
    /// 
    /// ***UserNotFound***: if the specified user is not found.
    pub async fn change_access_mode(&self, principal: &str, is_group: bool, access_mode: crate::types::enums::HostAccessModeEnum) -> Result<()> {
        let input = ChangeAccessModeRequestType {principal, is_group, access_mode, };
        let path = format!("/HostAccessManager/{moId}/ChangeAccessMode", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Changes the lockdown state of the ESXi host.
    /// 
    /// This operation will do nothing if the host is already in the desired
    /// lockdown state.
    /// 
    /// When the host is in lockdown mode it can be managed only through vCenter
    /// and through DCUI (Direct Console User Interface) if the DCUI service is
    /// running.
    /// This is achieved by removing all permissions on the host, except those
    /// of the exception users defined with *HostAccessManager.UpdateLockdownExceptions*.
    /// 
    /// In addition, the permissions for users 'dcui' and 'vpxuser'
    /// are always preserved.
    /// 
    /// When lockdown mode is disabled, the system will try to restore all
    /// permissions that have been removed when lockdown mode was enabled.
    /// It is possible that not all permissions may be restored and this is not
    /// an error, e.g. if in the meantime some user or managed object was deleted.
    /// 
    /// It may be possible that after exiting lockdown mode the only permissions
    /// on the host will be those of users 'dcui' and 'vpxuser'. This will render the
    /// host unmanageable if it is not already managed by vCenter, or if the
    /// connection to vCenter is lost. To prevent this, the users in the
    /// "DCUI.Access" list will be assigned Admin roles.
    /// 
    /// While the host is in lockdown mode, some operations will fail with
    /// SecurityError. This ensures that the conditions for lockdown mode cannot
    /// be changed. For example it is allowed to change the access mode only for
    /// users in the exceptions list.
    /// 
    /// When the host is in lockdown mode, changing the running state of service
    /// DCUI through *HostServiceSystem* will also fail with
    /// SecurityError accompanied with an appropriate localizeable message.
    /// 
    /// ***Required privileges:*** Host.Config.Settings
    ///
    /// ## Parameters:
    ///
    /// ### mode
    /// The new desired lockdown mode.
    /// 
    /// If this is the same as the current lockdown mode state, the
    /// operation will silently succeed and nothing will be changed.
    /// 
    /// If this is *lockdownDisabled*
    /// then lockdown mode will be disabled and the system will
    /// start service DCUI if it is not running.
    /// 
    /// If this is *lockdownNormal*
    /// then lockdown mode will be enabled and the system will
    /// start service DCUI if it is not running.
    /// 
    /// If this is *lockdownStrict*
    /// then lockdown mode will be enabled and the system will
    /// stop service DCUI if it is running.
    ///
    /// ## Errors:
    ///
    /// ***AuthMinimumAdminPermission***: if the user invoking the operation
    /// is not in the exceptions list - see
    /// *HostAccessManager.QueryLockdownExceptions*.
    /// 
    /// ***NoPermission***: if the current session does not have enough
    /// permissions to perform the operation.
    pub async fn change_lockdown_mode(&self, mode: crate::types::enums::HostLockdownModeEnum) -> Result<()> {
        let input = ChangeLockdownModeRequestType {mode, };
        let path = format!("/HostAccessManager/{moId}/ChangeLockdownMode", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Get the list of users which are exceptions for lockdown mode.
    /// 
    /// See *HostAccessManager.UpdateLockdownExceptions*.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Returns:
    ///
    /// the list of users which will not lose their permissions when
    /// the host enters lockdown mode.
    pub async fn query_lockdown_exceptions(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostAccessManager/{moId}/QueryLockdownExceptions", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Get the list of local system users.
    /// 
    /// These are special users like 'vpxuser' and 'dcui',
    /// which may be used for authenticating different sub-components of the
    /// vSphere system and may be essential for its correct functioning.
    /// 
    /// Usually these users may not be used by human operators to connect
    /// directly to the host and the UI may choose to show them only in some
    /// "advanced" UI view.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Returns:
    ///
    /// the list of local system users.
    pub async fn query_system_users(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/HostAccessManager/{moId}/QuerySystemUsers", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Retrieve access entries.
    /// 
    /// Returns a list of AccessEntry objects for each VIM user or group which
    /// have explicitly assigned permissions on the host. This means that
    /// *accessNone* will not be present in the result.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Returns:
    ///
    /// a list of AccessEntry objects.
    pub async fn retrieve_host_access_control_entries(&self) -> Result<Option<Vec<HostAccessControlEntry>>> {
        let path = format!("/HostAccessManager/{moId}/RetrieveHostAccessControlEntries", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_option(req).await
    }
    /// Update the list of users which are exceptions for lockdown mode.
    /// 
    /// Usually these are user accounts used by third party solutions and
    /// external applications which need to continue to function in lockdown
    /// mode. It is not advised to add user accounts used by human operators,
    /// because this will compromise the purpose of lockdown mode.
    /// 
    /// Both local and domain users are supported. The format for domain accounts
    /// is "DOMAIN\\login".
    /// 
    /// When this API is called when the host is in lockdown mode,
    /// the behaviour is as follows:
    /// - if a user is removed from the exceptions list,
    ///   then the permissions of that user are removed.
    /// - if a user is added to the exceptions list,
    ///   then the permissions of that user are restored.
    ///   
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### users
    /// the new list of lockdown mode exceptions.
    ///
    /// ## Errors:
    ///
    /// ***AuthMinimumAdminPermission***: if the user invoking the operation
    /// is not present in the new list of exceptions.
    /// 
    /// ***UserNotFound***: if one of the specified users is not found.
    pub async fn update_lockdown_exceptions(&self, users: Option<&[String]>) -> Result<()> {
        let input = UpdateLockdownExceptionsRequestType {users, };
        let path = format!("/HostAccessManager/{moId}/UpdateLockdownExceptions", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Update the list of local system users.
    /// 
    /// The special users 'dcui' and 'vpxuser' need not be specified.
    /// They are always reported in the list of system users.
    /// 
    /// ***Required privileges:*** Global.Settings
    ///
    /// ## Parameters:
    ///
    /// ### users
    /// the new list of local system users.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If one of the specified user names is not valid,
    /// or is in Active Directory domain format.
    /// 
    /// ***UserNotFound***: If one of the specified users is not found.
    pub async fn update_system_users(&self, users: Option<&[String]>) -> Result<()> {
        let input = UpdateSystemUsersRequestType {users, };
        let path = format!("/HostAccessManager/{moId}/UpdateSystemUsers", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        self.client.execute_void(req).await
    }
    /// Current lockdown state of the host.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn lockdown_mode(&self) -> Result<crate::types::enums::HostLockdownModeEnum> {
        let path = format!("/HostAccessManager/{moId}/lockdownMode", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ChangeAccessModeRequestType<'a> {
    principal: &'a str,
    #[serde(rename = "isGroup")]
    is_group: bool,
    #[serde(rename = "accessMode")]
    access_mode: crate::types::enums::HostAccessModeEnum,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ChangeLockdownModeRequestType {
    mode: crate::types::enums::HostLockdownModeEnum,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateLockdownExceptionsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    users: Option<&'a [String]>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateSystemUsersRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    users: Option<&'a [String]>,
}
