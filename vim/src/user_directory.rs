use std::sync::Arc;
use crate::vim_client::{VimClient, Result};
use crate::types::UserSearchResultTrait;
/// The *UserDirectory* managed object provides information about users
/// and groups on a vSphere server and ESX hosts.
/// 
/// The method
/// *UserDirectory.RetrieveUserGroups* returns a list
/// of user account data. The method can perform a search operation based on
/// specific criteria - user name, group name, sub-string or string matching,
/// and, on Windows, domain. Use the results as input
/// to the AuthorizationManager methods
/// *AuthorizationManager.SetEntityPermissions* and
/// *AuthorizationManager.ResetEntityPermissions*.
/// 
/// The content of the returned results depends on the server environment:
/// - On a Windows host, *UserDirectory.RetrieveUserGroups* can search
///   from the set of trusted domains on the host, including the primary
///   domain of the system. A special domain (specified as an
///   empty string - &quot;&quot;) refers to the users and groups local
///   to the host.
/// - On an ESX Server or a Linux host, the search operates on the
///   users and groups defined in the /etc/passwd file. Always specify
///   an empty string (&quot;&quot;) for the domain argument.
///   If the /etc/passwd file contains Sun NIS or NIS+ users and groups,
///   RetrieveUserGroups returns information about these accounts as well.
pub struct UserDirectory {
    client: Arc<VimClient>,
    mo_id: String,
}
impl UserDirectory {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Returns a list of *UserSearchResult* objects describing the
    /// users and groups defined for the server.
    /// - On Windows, the search for users and groups is restricted to
    ///   the given domain. If you omit the domain argument, then
    ///   the search is performed on local users and groups.
    /// - On ESX Server (or Linux systems), the method returns the list
    ///   of users and groups that are specified in the /etc/passwd file.
    ///   If the password file contains Sun NIS or NIS+ users and groups,
    ///   the returned list includes information about those as well.
    ///   
    /// You must hold the Authorization.ModifyPermissions privilege to invoke this
    /// method. If you hold the privilege on any ManagedEntity, you will
    /// have access to user and group information for the server.
    /// 
    /// As of vSphere API 5.1:
    /// - Local user groups on ESXi are not supported and this method will
    ///   not return information about local groups on the ESXi host.
    ///   Information about Active Directory groups is not affected.
    /// - Some special system users on ESXi like 'nfsnobody' and 'daemon'
    ///   will be filtered out by this method.
    ///
    /// ## Parameters:
    ///
    /// ### domain
    /// Domain to be searched. If not set, then the method searches
    /// the local machine.
    ///
    /// ### search_str
    /// Case insensitive substring used to filter results;
    /// the search string is compared to the login and full name for users,
    /// and the name and description for groups. Leave
    /// this blank to match all users.
    ///
    /// ### belongs_to_group
    /// If present, the returned list contains only users or groups
    /// that directly belong to the specified group. Users or groups that
    /// have indirect membership will not be included in the list.
    ///
    /// ### belongs_to_user
    /// If present, the returned list contains only groups that directly
    /// contain the specified user. Groups that indirectly contain
    /// the user will not be included in the list.
    ///
    /// ### exact_match
    /// Indicates the searchStr passed should match a user or
    /// group name exactly.
    ///
    /// ### find_users
    /// True, if users should be included in the result.
    ///
    /// ### find_groups
    /// True, if groups should be included in the result.
    ///
    /// ## Errors:
    ///
    /// ***NotSupported***: If you specify a domain for systems that do not support
    /// domains, such as an ESX Server. The method also throws
    /// NotSupported if you specify membership (belongsToGroup or
    /// belongsToUser) and the server does not support
    /// by-membership queries.
    /// 
    /// ***NotFound***: If any of the domain, belongsToGroup, or belongsToUser
    /// arguments refer to entities that do not exist.
    pub async fn retrieve_user_groups(&self, domain: Option<&str>, search_str: &str, belongs_to_group: Option<&str>, belongs_to_user: Option<&str>, exact_match: bool, find_users: bool, find_groups: bool) -> Result<Option<Vec<Box<dyn UserSearchResultTrait>>>> {
        let input = RetrieveUserGroupsRequestType {domain, search_str, belongs_to_group, belongs_to_user, exact_match, find_users, find_groups, };
        let path = format!("/UserDirectory/{moId}/RetrieveUserGroups", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// List of Windows domains available for user searches, if the underlying
    /// system supports windows domain membership.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn domain_list(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/UserDirectory/{moId}/domainList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrieveUserGroupsRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    domain: Option<&'a str>,
    #[serde(rename = "searchStr")]
    search_str: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "belongsToGroup")]
    belongs_to_group: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "belongsToUser")]
    belongs_to_user: Option<&'a str>,
    #[serde(rename = "exactMatch")]
    exact_match: bool,
    #[serde(rename = "findUsers")]
    find_users: bool,
    #[serde(rename = "findGroups")]
    find_groups: bool,
}
