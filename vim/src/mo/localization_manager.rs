use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::LocalizationManagerMessageCatalog;
/// *LocalizationManager* provides access to descriptions of
/// the message catalogs that are available for client-side message
/// localization.
/// 
/// Clients of the VIM API may use
/// *SessionManager*.*SessionManager.SetLocale*
/// to cause the server to emit localized messages, or may perform
/// client-side localization based on message catalogs provided by the
/// *LocalizationManager*.
/// 
/// A message catalog is a file that contains a set of key-value pairs.
/// - The key is an ASCII string that identifies the message.
/// - The value is a UTF-8 string that contains the text of the message, sometimes
///   containing substitution variables.
///   
/// The server will localize fields tagged with 'localizable' based on the
/// value of the *UserSession*.*UserSession.locale*
/// and *UserSession.messageLocale* properties which are set via
/// *SessionManager*.*SessionManager.SetLocale*.
/// 
/// The following list shows some of the ways that vSphere uses localized
/// messages.
/// - Current task status (*TaskInfo*.*TaskInfo.description*)
/// - Events (*VirtualMachineMessage*.*VirtualMachineMessage.text* and
///   Questions (*VirtualMachineQuestionInfo*.*VirtualMachineQuestionInfo.text*)
/// - Faults (*MethodFault*.*MethodFault.faultMessage*)
/// - *HostProfile* and
///   *ClusterProfile* descriptions
///   (*Profile*.*ProfileDescription*.
///   *Profile.description* returned by the
///   *Profile*.*Profile.RetrieveDescription* method)
pub struct LocalizationManager {
    client: Arc<Client>,
    mo_id: String,
}
impl LocalizationManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Fetches the descriptions of all the client-side localization message
    /// catalogs available for the current session locale.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// the message catalogs for the current locale
    pub async fn catalog(&self) -> Result<Option<Vec<LocalizationManagerMessageCatalog>>> {
        let path = format!("/LocalizationManager/{moId}/catalog", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
