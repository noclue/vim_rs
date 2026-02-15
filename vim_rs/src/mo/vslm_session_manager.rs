use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// *VslmSessionManager* managed object manages client sessions.
/// 
/// Login to VSLM service is done through this interface.
/// It is SSO enabled so only login by using SamlToken is allowed.
/// This API is intended for internal use only.
#[derive(Clone)]
pub struct VslmSessionManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl VslmSessionManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Login to the VSLM service by using SSO token.
    /// 
    /// VSLM will validate the user
    /// token from the context. The delegated token passed as a parameter will be
    /// used by VSLM to login to VC for authorization purposes.
    /// Once login successfully returns, a new session is established for the
    /// client. This session is only valid for the lifetime of the supplied
    /// delegated token. Any calls made on a session which exceeds this lifetime
    /// will result in a SecurityError. The client is expected to logout of the
    /// current session and subsequently re-login with a new delegated token to
    /// establish a new session.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### delegated_token_xml
    /// The delegated token will be retrieved by the
    /// client and delegated to VSLM. VSLM will use this token, on user's
    /// behalf, to login to VC for authorization purposes. It is necessary
    /// to convert the token to XML because the SAML token itself is
    /// not a VMODL Data Object and cannot be used as a parameter.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if there is no token provided or the token
    /// could not be validated.
    pub async fn vslm_login_by_token(&self, delegated_token_xml: &str) -> Result<()> {
        let input = VslmLoginByTokenRequestType {delegated_token_xml, };
        let path = format!("/vslm/VslmSessionManager/{moId}/VslmLoginByToken", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Logs out of the VSLM Service.
    /// 
    /// ***Required privileges:*** StoragLifecycle.View
    pub async fn vslm_logout(&self) -> Result<()> {
        let path = format!("/vslm/VslmSessionManager/{moId}/VslmLogout", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
}
struct VslmLoginByTokenRequestType<'a> {
    delegated_token_xml: &'a str,
}

impl<'a> miniserde::Serialize for VslmLoginByTokenRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(VslmLoginByTokenRequestTypeSer { data: self, seq: 0 }))
    }
}

struct VslmLoginByTokenRequestTypeSer<'b, 'a> {
    data: &'b VslmLoginByTokenRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for VslmLoginByTokenRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"VslmLoginByTokenRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("delegatedTokenXml"), &self.data.delegated_token_xml as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
