use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::HostAuthenticationManagerInfo;
use crate::types::structs::ManagedObjectReference;
/// The *HostAuthenticationManager* managed object provides
/// access to Active Directory configuration information for an
/// ESX host.
/// 
/// It also provides access to methods for adding a host
/// to or removing a host from an Active Directory domain.
/// 
/// The vSphere API supports Microsoft Active Directory management
/// of authentication for ESX hosts. To integrate an ESX host
/// into an Active Directory environment, you use an Active
/// Directory account that has the authority to add
/// a computer to a domain. The ESX Server locates the Active
/// Directory domain controller. When you add a host to a domain,
/// you only need to specify the domain and the account
/// user name and password.
/// 
/// There are two approaches that you can use to add an ESX host
/// to or remove a host from an Active Directory domain.
/// - *HostActiveDirectoryAuthentication.JoinDomain_Task* and
///   *HostActiveDirectoryAuthentication.LeaveCurrentDomain_Task*
///   methods - Your vSphere client application can call
///   these methods directly to add the host to or remove the host
///   from a domain.
/// - Host configuration - Use the *HostActiveDirectory* data object
///   to specify Active Directory configuration, either adding the host to
///   or removing the host from a domain. To apply the Active Directory
///   configuration, use the *HostProfileManager* method
///   (*HostProfileManager.ApplyHostConfig_Task*)
///   to apply the *HostConfigSpec*. When the ESX Server processes
///   the configuration, it will invoke the join or leave method.
///   
/// To take advantage of ESX host membership in an Active Directory domain,
/// grant permissions on the ESX host to Active Directory users and groups
/// who should have direct access to management of the ESX host.
/// Use the *UserDirectory*.*UserDirectory.RetrieveUserGroups*
/// method to obtain information about Active Directory users and groups.
/// After retrieving the Active Directory data, you can use the
/// *AuthorizationManager*.*AuthorizationManager.SetEntityPermissions*
/// method to set the *Permission.principal* property
/// to the appropriate user or group.
/// 
/// By default, the ESX host assigns the Administrator role to the "ESX Admins" group.
/// If the group does not exist when the host joins the domain, the host will
/// not assign the role. In this case, you must create the "ESX Admins"
/// group in the Active Directory. The host will periodically check the domain controller
/// for the group and will assign the role when the group exists.
pub struct HostAuthenticationManager {
    client: Arc<Client>,
    mo_id: String,
}
impl HostAuthenticationManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Information about Active Directory membership.
    pub async fn info(&self) -> Result<HostAuthenticationManagerInfo> {
        let path = format!("/HostAuthenticationManager/{moId}/info", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
    /// An array that can contain managed object references to local and
    /// Active Directory authentication managed objects.
    /// 
    /// <code>supportedStore</code> data implies a connection to a system
    /// that stores information about accounts.
    /// The <code>supportedStore</code> array can include the following objects:
    /// - *HostLocalAuthentication* - Local authentication refers
    ///   to user accounts on the ESX host. Local authentication is always enabled.
    /// - *HostActiveDirectoryAuthentication* - Active Directory authentication
    ///   refers to computer accounts and user accounts on the domain controller.
    ///   This object indicates the domain membership status for the host
    ///   and defines the join and leave methods for Active Directory
    ///   membership.
    ///   
    ///   If <code>supportedStore</code> references
    ///   a *HostActiveDirectoryAuthentication* object, the host
    ///   is capable of joining a domain.
    ///   However, if you try to add a host to a domain when the
    ///   *HostAuthenticationStoreInfo*.*HostAuthenticationStoreInfo.enabled*
    ///   property is <code>True</code> (accessed through the <code>info</code>
    ///   property), the join method will throw a fault.
    ///
    /// ## Returns:
    ///
    /// Refers instances of *HostAuthenticationStore*.
    pub async fn supported_store(&self) -> Result<Vec<ManagedObjectReference>> {
        let path = format!("/HostAuthenticationManager/{moId}/supportedStore", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        self.client.execute(req).await
    }
}
