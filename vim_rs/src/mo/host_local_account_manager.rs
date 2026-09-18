use std::sync::Arc;
use crate::core::client::{VimClient, Result};
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
#[derive(Clone)]
pub struct HostLocalAccountManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl HostLocalAccountManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "AssignUserToGroup", Some(&input)).await
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "ChangePassword", Some(&input)).await
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "CreateGroup", Some(&input)).await
    }
    /// Creates a local user account using the specified parameters.
    /// 
    /// As of vSphere API 9.1, it is allowed to create a user without a password
    /// by not setting the corresponding property of the input argument.
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "CreateUser", Some(&input)).await
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "RemoveGroup", Some(&input)).await
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "RemoveUser", Some(&input)).await
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "UnassignUserFromGroup", Some(&input)).await
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
        self.client.invoke_void("", "HostLocalAccountManager", &self.mo_id, "UpdateUser", Some(&input)).await
    }
}
struct AssignUserToGroupRequestType<'a> {
    user: &'a str,
    group: &'a str,
}

impl<'a> miniserde::Serialize for AssignUserToGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AssignUserToGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AssignUserToGroupRequestTypeSer<'b, 'a> {
    data: &'b AssignUserToGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for AssignUserToGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AssignUserToGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("user"), &self.data.user as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("group"), &self.data.group as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ChangePasswordRequestType<'a> {
    user: &'a str,
    old_password: &'a str,
    new_password: &'a str,
}

impl<'a> miniserde::Serialize for ChangePasswordRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ChangePasswordRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ChangePasswordRequestTypeSer<'b, 'a> {
    data: &'b ChangePasswordRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for ChangePasswordRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ChangePasswordRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("user"), &self.data.user as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("oldPassword"), &self.data.old_password as &dyn miniserde::Serialize)),
            3 => return Some((std::borrow::Cow::Borrowed("newPassword"), &self.data.new_password as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateGroupRequestType<'a> {
    group: &'a dyn crate::types::traits::HostAccountSpecTrait,
}

impl<'a> miniserde::Serialize for CreateGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateGroupRequestTypeSer<'b, 'a> {
    data: &'b CreateGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("group"), &self.data.group as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CreateUserRequestType<'a> {
    user: &'a dyn crate::types::traits::HostAccountSpecTrait,
}

impl<'a> miniserde::Serialize for CreateUserRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CreateUserRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CreateUserRequestTypeSer<'b, 'a> {
    data: &'b CreateUserRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for CreateUserRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CreateUserRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("user"), &self.data.user as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveGroupRequestType<'a> {
    group_name: &'a str,
}

impl<'a> miniserde::Serialize for RemoveGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveGroupRequestTypeSer<'b, 'a> {
    data: &'b RemoveGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("groupName"), &self.data.group_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct RemoveUserRequestType<'a> {
    user_name: &'a str,
}

impl<'a> miniserde::Serialize for RemoveUserRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(RemoveUserRequestTypeSer { data: self, seq: 0 }))
    }
}

struct RemoveUserRequestTypeSer<'b, 'a> {
    data: &'b RemoveUserRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for RemoveUserRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"RemoveUserRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UnassignUserFromGroupRequestType<'a> {
    user: &'a str,
    group: &'a str,
}

impl<'a> miniserde::Serialize for UnassignUserFromGroupRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UnassignUserFromGroupRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UnassignUserFromGroupRequestTypeSer<'b, 'a> {
    data: &'b UnassignUserFromGroupRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UnassignUserFromGroupRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UnassignUserFromGroupRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("user"), &self.data.user as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("group"), &self.data.group as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateUserRequestType<'a> {
    user: &'a dyn crate::types::traits::HostAccountSpecTrait,
}

impl<'a> miniserde::Serialize for UpdateUserRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateUserRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateUserRequestTypeSer<'b, 'a> {
    data: &'b UpdateUserRequestType<'a>,
    seq: usize,
}

impl<'b, 'a> miniserde::ser::Map for UpdateUserRequestTypeSer<'b, 'a> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateUserRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("user"), &self.data.user as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
