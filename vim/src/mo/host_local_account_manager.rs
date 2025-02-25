use std::sync::Arc;
use crate::core::client::{Client, Result};
/// This managed object type provides an interface
/// through which local accounts on a host are managed.
/// 
/// Note that this
/// managed object applies only to applications that use a local account
/// database on the host to provide authentication (ESX Server, for example).
/// POSIX and win32 hosts may impose different restrictions on the password,
/// ID, and description formats. POSIX host implementation may restrict the
/// user or group name to be lower case letters and less than 16 characters in
/// total. It may also disallow characters such as
/// ";", "\\n", and so on. In short, all the platform dependent rules and
/// restrictions regarding naming of users/groups and password apply here.
/// An InvalidArgument fault is thrown if any of these rules are not obeyed.
pub struct HostLocalAccountManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostLocalAccountManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Deprecated as of vSphere API 5.1, local user groups are not supported
    /// and group specific methods will throw NotSupported.
    /// 
    /// Assigns a user to a group.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### user
    /// User ID of the account whose group membership is
    /// being assigned.
    ///
    /// ### group
    /// Destination group account to which the user is
    /// being assigned.
    ///
    /// ## Errors:
    ///
    /// ***UserNotFound***: if the specified user or group does not exist.
    /// 
    /// ***AlreadyExists***: if the user is already a member of the target group.
    pub async fn assign_user_to_group(&self, user: &str, group: &str) -> Result<()> {
        let input = AssignUserToGroupRequestType {user, group, };
        let path = format!("/HostLocalAccountManager/{moId}/AssignUserToGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates the password of a local user account.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### user
    /// the user whose password will be changed.
    ///
    /// ### old_password
    /// the user's current (old) password.
    ///
    /// ### new_password
    /// the user's new password.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if newPassword has an invalid format.
    /// 
    /// ***InvalidLogin***: if the user and oldPassword combination is not valid.
    pub async fn change_password(&self, user: &str, old_password: &str, new_password: &str) -> Result<()> {
        let input = ChangePasswordRequestType {user, old_password, new_password, };
        let path = format!("/HostLocalAccountManager/{moId}/ChangePassword", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 5.1, local user groups are not supported
    /// and group specific methods will throw NotSupported.
    /// 
    /// Creates a local group account using the parameters defined in the
    /// *HostLocalAccountManagerAccountSpecification*
    /// data object type.
    /// 
    /// For POSIX hosts, passing the
    /// *HostLocalAccountManagerPosixAccountSpecification* data object
    /// type allows you to control
    /// the group ID format of the group account being created.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### group
    /// Specification of group being created.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if specified local group already exists.
    /// 
    /// ***InvalidArgument***: if group name is in invalid format.
    pub async fn create_group(&self, group: &dyn crate::types::traits::HostAccountSpecTrait) -> Result<()> {
        let input = CreateGroupRequestType {group, };
        let path = format!("/HostLocalAccountManager/{moId}/CreateGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Creates a local user account using the parameters defined in the
    /// *HostLocalAccountManagerAccountSpecification*
    /// data object type.
    /// 
    /// For POSIX hosts,
    /// passing *HostLocalAccountManagerPosixAccountSpecification* data object
    /// type allows you to control the
    /// format of the user ID of the user account being created.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### user
    /// Specification of user being created.
    ///
    /// ## Errors:
    ///
    /// ***AlreadyExists***: if the specified local user account
    /// already exists.
    /// 
    /// ***InvalidArgument***: if the user name or password has an
    /// invalid format.
    pub async fn create_user(&self, user: &dyn crate::types::traits::HostAccountSpecTrait) -> Result<()> {
        let input = CreateUserRequestType {user, };
        let path = format!("/HostLocalAccountManager/{moId}/CreateUser", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 5.1, local user groups are not supported
    /// and group specific methods will throw NotSupported.
    /// 
    /// Removes a local group account.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### group_name
    /// Group ID of the group account being removed.
    ///
    /// ## Errors:
    ///
    /// ***UserNotFound***: if the specified groupName does not exist.
    pub async fn remove_group(&self, group_name: &str) -> Result<()> {
        let input = RemoveGroupRequestType {group_name, };
        let path = format!("/HostLocalAccountManager/{moId}/RemoveGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Removes a local user account.
    /// 
    /// As of vSphere API 5.1, this operation will first try to remove all
    /// permissions associated with the specified account. The permissions of
    /// the user are removed one by one, not atomically, and the operation
    /// is not rolled back if the removal of some permission fails.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### user_name
    /// User ID of the user account being removed.
    ///
    /// ## Errors:
    ///
    /// ***SecurityError***: if trying to remove the last local user with
    /// DCUI access,
    /// or if trying to remove the last local
    /// user with full administrative privileges,
    /// or if the system has encountered an error while
    /// trying to remove user's permissions.
    /// or if the account cannot be removed due to
    /// permission issues.
    /// 
    /// ***UserNotFound***: if the specified userName does not exist.
    pub async fn remove_user(&self, user_name: &str) -> Result<()> {
        let input = RemoveUserRequestType {user_name, };
        let path = format!("/HostLocalAccountManager/{moId}/RemoveUser", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 5.1, local user groups are not supported
    /// and group specific methods will throw NotSupported.
    /// 
    /// Unassigns a user from a group.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### user
    /// User being unassigned from group.
    ///
    /// ### group
    /// Group from which the user is being removed.
    ///
    /// ## Errors:
    ///
    /// ***UserNotFound***: if the specified user or group does not exist.
    /// 
    /// ***NoPermission***: if the group is the only group to which the
    /// user belongs.
    pub async fn unassign_user_from_group(&self, user: &str, group: &str) -> Result<()> {
        let input = UnassignUserFromGroupRequestType {user, group, };
        let path = format!("/HostLocalAccountManager/{moId}/UnassignUserFromGroup", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Updates a local user account using the parameters defined in the
    /// *HostLocalAccountManagerAccountSpecification*
    /// data object type.
    /// 
    /// ***Required privileges:*** Host.Local.ManageUserGroups
    ///
    /// ## Parameters:
    ///
    /// ### user
    /// Specification of user being updated.
    ///
    /// ## Errors:
    ///
    /// ***UserNotFound***: if user is not found.
    /// 
    /// ***AlreadyExists***: if new account specification specifies an existing
    /// user's ID.
    /// 
    /// ***InvalidArgument***: if new password or description has an invalid format.
    pub async fn update_user(&self, user: &dyn crate::types::traits::HostAccountSpecTrait) -> Result<()> {
        let input = UpdateUserRequestType {user, };
        let path = format!("/HostLocalAccountManager/{moId}/UpdateUser", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AssignUserToGroupRequestType<'a> {
    user: &'a str,
    group: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ChangePasswordRequestType<'a> {
    user: &'a str,
    #[serde(rename = "oldPassword")]
    old_password: &'a str,
    #[serde(rename = "newPassword")]
    new_password: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateGroupRequestType<'a> {
    group: &'a dyn crate::types::traits::HostAccountSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateUserRequestType<'a> {
    user: &'a dyn crate::types::traits::HostAccountSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveGroupRequestType<'a> {
    #[serde(rename = "groupName")]
    group_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RemoveUserRequestType<'a> {
    #[serde(rename = "userName")]
    user_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UnassignUserFromGroupRequestType<'a> {
    user: &'a str,
    group: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateUserRequestType<'a> {
    user: &'a dyn crate::types::traits::HostAccountSpecTrait,
}
