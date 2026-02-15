use std::sync::Arc;
use crate::core::client::{VimClient, Result};
/// This managed object type includes methods for logging on and
/// logging off clients, determining which clients are currently
/// logged on, and forcing clients to log off.
#[derive(Clone)]
pub struct SessionManager {
    client: Arc<dyn VimClient>,
    mo_id: String,
}
impl SessionManager {
    pub fn new(client: Arc<dyn VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Acquire a session-specific ticket string which can be used to clone
    /// the current session.
    /// 
    /// The caller of this operation can pass the ticket
    /// value to another entity on the client. The recipient can then call
    /// *SessionManager.CloneSession* with the ticket string on an unauthenticated
    /// session and avoid having to re-enter credentials.
    /// 
    /// The ticket may only be used once and becomes invalid after use. The
    /// ticket is also invalidated when the corresponding session is closed or
    /// expires. The ticket is only valid on the server which issued it.
    /// 
    /// This sequence of operations is conceptually similar to the
    /// functionality provided by *SessionManager.AcquireLocalTicket*, however the
    /// methods can be used by remote clients and do not require a shared
    /// filesystem for transport.
    /// 
    /// See also *SessionManager.CloneSession*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// one-time secret ticket string.
    pub async fn acquire_clone_ticket(&self) -> Result<String> {
        let path = format!("/SessionManager/{moId}/AcquireCloneTicket", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Creates and returns a one-time credential that may be used to make the
    /// specified request.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// specification for the service request which will be
    /// invoked with the ticket.
    ///
    /// ## Returns:
    ///
    /// a ticket that may be used to invoke the specified request.
    /// The first choice for authenticating the host is
    /// *SessionManagerGenericServiceTicket.sslCertificate*.
    /// If *SessionManagerGenericServiceTicket.sslCertificate* is unset, the
    /// following logic is used to authenticate the host:
    /// 1\. If the VC system supports the crypto hash algorithm of
    /// the *SessionManagerGenericServiceTicket.sslThumbprint* or
    /// *SessionManagerGenericServiceTicket.certThumbprintList* (if set),
    /// they will be verified against that of the server certificate. If
    /// they doesn't match, the CA certificates will be used to
    /// authenticate the host.
    /// 2\. If the VC system does not support the crypto hash algorithm
    /// of *SessionManagerGenericServiceTicket.sslThumbprint* or
    /// *SessionManagerGenericServiceTicket.certThumbprintList*, only the CA
    /// certificates will be used to authenticate the host.
    pub async fn acquire_generic_service_ticket(&self, spec: &dyn crate::types::traits::SessionManagerServiceRequestSpecTrait) -> Result<crate::types::structs::SessionManagerGenericServiceTicket> {
        let input = AcquireGenericServiceTicketRequestType {spec, };
        let path = format!("/SessionManager/{moId}/AcquireGenericServiceTicket", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::SessionManagerGenericServiceTicket = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Acquires a one-time ticket for mutual authentication between a server and client.
    /// 
    /// The caller of this operation can use the user name and file content of
    /// the returned object as the userName and password arguments for login
    /// operation. The local ticket that is returned becomes invalid either
    /// after it is used or after a server-determined ticket expiration time
    /// passes. This operation can be used by servers and clients to avoid
    /// re-entering user credentials after authentication by the operating
    /// system has already happened.
    /// 
    /// For example, service console utilities that connect to a host agent
    /// should not require users to re-enter their passwords every time the
    /// utilities run. Since the one-time password file is readable only by
    /// the given user, the identity of the one-time password user is protected
    /// by the operating system file permission.
    /// 
    /// Only local clients are allowed to call this operation. Remote clients
    /// receive an InvalidRequest fault upon calling this operation.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### user_name
    /// User requesting one-time password.
    ///
    /// ## Returns:
    ///
    /// LocalTicket object containing userName and path to file
    /// containing one-time password for use in login operation.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if the userName is invalid.
    /// 
    /// ***NoPermission***: if the user and password are valid, but the user has no access
    /// granted.
    /// 
    /// ***NotSupported***: if the server does not support this operation.
    pub async fn acquire_local_ticket(&self, user_name: &str) -> Result<crate::types::structs::SessionManagerLocalTicket> {
        let input = AcquireLocalTicketRequestType {user_name, };
        let path = format!("/SessionManager/{moId}/AcquireLocalTicket", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::SessionManagerLocalTicket = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Clone the session specified by the clone ticket and associate it with
    /// the current connection.
    /// 
    /// The current session will take on the identity
    /// and authorization level of the UserSession associated with the
    /// specified cloning ticket.
    /// 
    /// See also *SessionManager.AcquireCloneTicket*, *SessionManager.AcquireGenericServiceTicket*.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### clone_ticket
    /// ticket string acquired via *SessionManager.AcquireCloneTicket*.
    ///
    /// ## Returns:
    ///
    /// The new/cloned UserSession object.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if the specified ticket value is not valid.
    /// 
    /// ***NotSupported***: if the server does not support this operation.
    pub async fn clone_session(&self, clone_ticket: &str) -> Result<crate::types::structs::UserSession> {
        let input = CloneSessionRequestType {clone_ticket, };
        let path = format!("/SessionManager/{moId}/CloneSession", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Converts current session to impersonate the specified user.
    /// 
    /// The current session will take on the identity and authorization level of
    /// the user. That user must have a currently-active session.
    /// If the given userName is an extension key and this key does
    /// not overlap with a user name of any currently-active session, it will
    /// take on the identity and authorization level of that extension provided
    /// the current session has the same authorization level of that extension.
    /// 
    /// ***Required privileges:*** Sessions.ImpersonateUser
    ///
    /// ## Parameters:
    ///
    /// ### user_name
    /// The user or extension key to impersonate.
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn impersonate_user(&self, user_name: &str, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = ImpersonateUserRequestType {user_name, locale, };
        let path = format!("/SessionManager/{moId}/ImpersonateUser", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Log on to the server.
    /// 
    /// This method fails if the user name and password are
    /// incorrect, or if the user is valid but has no permissions granted.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### user_name
    /// The *ID*
    /// of the user who is logging on to the server.
    ///
    /// ### password
    /// The *HostAccountSpec.password*
    /// of the user who is logging on to the server.
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Returns:
    ///
    /// The UserSession object.
    /// 
    /// As of vSphere API 5.1 for VirtualCenter login use SSO style
    /// *SessionManager.LoginByToken*
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if the user and password combination is invalid.
    /// 
    /// ***NoPermission***: if the user is valid, but has no access granted.
    /// 
    /// ***InvalidLocale***: if the locale is invalid or unknown to the server.
    pub async fn login(&self, user_name: &str, password: &str, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = LoginRequestType {user_name, password, locale, };
        let path = format!("/SessionManager/{moId}/Login", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 5.1 for VirtualCenter login use SSO style
    /// *SessionManager.LoginByToken*.
    /// 
    /// Log on to the server using SSPI pass-through authentication.
    /// 
    /// This method provides support for passing credentials of the calling
    /// process to the server without using a password, by leveraging the
    /// Windows Security Support Provider Interface (SSPI) library.
    /// 
    /// If the function is not supported, this throws a NotSupported fault.
    /// 
    /// The client first calls AcquireCredentialsHandle(). If Kerberos is
    /// used, this should include the desired credential to pass. The client then
    /// calls InitializeSecurityContext(). The resulting partially-formed
    /// context is passed in Base-64 encoded form to this method.
    /// 
    /// If the context has been successfully formed, the server proceeds with
    /// login and behaves like *SessionManager.Login*. If further
    /// negotiation is needed, the server throws an SSPIChallenge fault with
    /// a challenge token, which the client should again pass to
    /// InitializeSecurityContext(), followed by calling this method again.
    /// 
    /// For more information, see the MSDN documentation on SSPI.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### base_64_token
    /// The partially formed context returned from
    /// InitializeSecurityContext().
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Returns:
    ///
    /// The UserSession object.
    ///
    /// ## Errors:
    ///
    /// ***SSPIChallenge***: if further negotiation is required.
    /// 
    /// ***InvalidLogin***: if the user context could not be passed successfully,
    /// or the context is not valid on the server.
    /// 
    /// ***NoPermission***: if the user is valid, but has no access granted.
    /// 
    /// ***InvalidLocale***: if the locale is invalid or unknown to the server.
    /// 
    /// ***NotSupported***: if the service does not support SSPI authentication.
    pub async fn login_by_sspi(&self, base_64_token: &str, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = LoginBySspiRequestType {base_64_token, locale, };
        let path = format!("/SessionManager/{moId}/LoginBySSPI", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Log on to the server through token representing principal identity.
    /// 
    /// The token is obtained from SSO (single sign-on) service. This method
    /// fails if the token is not valid, or the principal has no permissions
    /// granted. Two type of sso tokens are supported by this method: Bearer
    /// and Holder-of-Key (HoK). If the token type obliges the method caller
    /// to prove his rights to present this token (HoK), then a signature is
    /// supplied as well. The token and the security signature if available
    /// are provided in a transport specific way.
    /// 
    /// If the communication with the VirtualCenter is SOAP based read the
    /// WS-Security specification (SAML Token profile) to understand how
    /// to transport the SSO token and signature.
    /// 
    /// Usual login scenario:
    /// 1. Acquire HoK token from the SSO service. Different authentication
    ///    mechanisms are available for acquiring token (user/password,
    ///    certificate, SSPI and so on). For more details consult the SSO
    ///    documentation. To find the location of your SSO service consult the
    ///    Virtual Infrastructure documentation.
    /// 2. Once SSO token is acquired successfully *SessionManager.LoginByToken* could be
    ///    invoked.
    ///    
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Returns:
    ///
    /// The UserSession object.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if there is no token provided or the token
    /// could not be validated.
    /// 
    /// ***NoPermission***: if the principal is valid, but has no access granted.
    /// 
    /// ***InvalidLocale***: if the locale is invalid or unknown to the server.
    pub async fn login_by_token(&self, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = LoginByTokenRequestType {locale, };
        let path = format!("/SessionManager/{moId}/LoginByToken", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 4.0, use SSO style of login instead
    /// *SessionManager.LoginByToken*.
    /// 
    /// Creates a special privileged session that includes
    /// the Sessions.ImpersonateUser privilege.
    /// 
    /// Requires exchange of
    /// a message signed with the extension's registered public key
    /// and base-64 encoded.
    /// 
    /// As of vSphere API 4.0, the NotFound fault is no longer thrown. Instead, InvalidLogin
    /// is thrown if the specified extension is not registered.
    /// 
    /// As of vSphere API 5.0, this method always throws a NotSupported exception.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of extension that is logging in.
    ///
    /// ### base_64_signed_credentials
    /// base-64 encoding of the SHA-1
    /// digest of the string "login" signed with the extension's
    /// private RSA key using PKCS#1 padding.
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Errors:
    ///
    /// Failure
    pub async fn login_extension(&self, extension_key: &str, base_64_signed_credentials: &str, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = LoginExtensionRequestType {extension_key, base_64_signed_credentials, locale, };
        let path = format!("/SessionManager/{moId}/LoginExtension", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 6.0, use SSO style of login instead
    /// *SessionManager.LoginByToken*.
    /// 
    /// Creates a special privileged session that includes
    /// the Sessions.ImpersonateUser privilege.
    /// 
    /// Requires that the client connect
    /// over SSL and provide an X.509 certificate for which they hold the private key.
    /// The certificate must match the certificate used in an earlier call to
    /// *ExtensionManager.SetExtensionCertificate*.
    /// 
    /// NOTE: Verification of the received certificate (such as expiry, revocation,
    /// and trust chain) is not required for successful authentication using
    /// this method. If certificate verification is desired, use the
    /// *SessionManager.LoginExtensionBySubjectName* method instead.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of extension that is logging in.
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if the extension is not registered, or the
    /// certificate does not match the expected value.
    /// 
    /// ***InvalidLocale***: if the supplied locale is not valid
    /// 
    /// ***NoClientCertificate***: if no certificate was used by the client to connect
    pub async fn login_extension_by_certificate(&self, extension_key: &str, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = LoginExtensionByCertificateRequestType {extension_key, locale, };
        let path = format!("/SessionManager/{moId}/LoginExtensionByCertificate", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Deprecated as of vSphere API 6.0, use SSO style of login instead
    /// *SessionManager.LoginByToken*.
    /// 
    /// Creates a special privileged session that includes
    /// the Sessions.ImpersonateUser privilege.
    /// 
    /// Requires that the extension connected
    /// using SSL, with a certificate that has a subject name that matches the subject
    /// name registered for the extension.
    /// 
    /// As of vSphere API 4.0, the NotFound fault is no longer thrown. Instead, InvalidLogin
    /// is thrown if the specified extension is not registered.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### extension_key
    /// Key of extension that is logging in.
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLogin***: if the extension is not registered, or the subject name
    /// doesn't match the subject name of the extension.
    /// 
    /// ***InvalidLocale***: if the supplied locale is not valid
    /// 
    /// ***NotFound***: if no extension is associated with the given key
    /// 
    /// ***NoClientCertificate***: if no certificate was used by the client to connect
    /// 
    /// ***NoSubjectName***: if the extension was registered without a subject name
    /// 
    /// ***InvalidClientCertificate***: if the client cerificate fails the verification at the server
    pub async fn login_extension_by_subject_name(&self, extension_key: &str, locale: Option<&str>) -> Result<crate::types::structs::UserSession> {
        let input = LoginExtensionBySubjectNameRequestType {extension_key, locale, };
        let path = format!("/SessionManager/{moId}/LoginExtensionBySubjectName", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: crate::types::structs::UserSession = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Log out and terminate the current session.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn logout(&self) -> Result<()> {
        let path = format!("/SessionManager/{moId}/Logout", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        self.client.execute_void(req).await
    }
    /// Validates that a currently-active session exists with the specified
    /// sessionID and userName associated with it.
    /// 
    /// Returns true
    /// if session exists.
    /// 
    /// ***Required privileges:*** Sessions.ValidateSession
    ///
    /// ## Parameters:
    ///
    /// ### session_id
    /// Session ID to validate.
    ///
    /// ### user_name
    /// User name to validate.
    pub async fn session_is_active(&self, session_id: &str, user_name: &str) -> Result<bool> {
        let input = SessionIsActiveRequestType {session_id, user_name, };
        let path = format!("/SessionManager/{moId}/SessionIsActive", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: bool = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// Sets the session locale.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### locale
    /// A two-character ISO-639 language ID (like "en")
    /// optionally followed by an
    /// underscore and a two-character ISO 3166 country ID (like "US").
    /// 
    /// Examples are "de", "fr\_CA", "zh", "zh\_CN", and "zh\_TW".
    /// Note: The method uses the server default locale when
    /// a locale is not provided. This default can be configured in the
    /// server configuration file. If unspecified, it defaults to the
    /// locale of the server environment or English ("en") if unsupported.
    ///
    /// ## Errors:
    ///
    /// ***InvalidLocale***: if the locale is invalid or unknown to the server.
    pub async fn set_locale(&self, locale: &str) -> Result<()> {
        let input = SetLocaleRequestType {locale, };
        let path = format!("/SessionManager/{moId}/SetLocale", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Log off and terminate the provided list of sessions.
    /// 
    /// This method is only transactional for each session ID. The set of sessions
    /// are terminated sequentially, as specified in the list. If a failure
    /// occurs, for example, because of an unknown sessionID, the method aborts with
    /// an exception. When the method aborts, any sessions that have not yet been
    /// terminated are left in their unterminated state.
    /// 
    /// ***Required privileges:*** Sessions.TerminateSession
    ///
    /// ## Parameters:
    ///
    /// ### session_id
    /// A list of sessions to terminate.
    ///
    /// ## Errors:
    ///
    /// ***NotFound***: if a sessionId could not be found as a valid logged-on session.
    /// 
    /// ***InvalidArgument***: if a sessionId matches the current session. Use
    /// the logout method to terminate the current session.
    pub async fn terminate_session(&self, session_id: &[String]) -> Result<()> {
        let input = TerminateSessionRequestType {session_id, };
        let path = format!("/SessionManager/{moId}/TerminateSession", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// Updates the system global message.
    /// 
    /// If not blank, the message is immediately
    /// displayed to currently logged-on users. When set, the message is shown by new
    /// clients upon logging in.
    /// 
    /// ***Required privileges:*** Sessions.GlobalMessage
    ///
    /// ## Parameters:
    ///
    /// ### message
    /// The message to send. Newline characters may be included.
    pub async fn update_service_message(&self, message: &str) -> Result<()> {
        let input = UpdateServiceMessageRequestType {message, };
        let path = format!("/SessionManager/{moId}/UpdateServiceMessage", moId = &self.mo_id);
        let req = self.client.post_json(&path, &input);
        self.client.execute_void(req).await
    }
    /// This property contains information about the client's current session.
    /// 
    /// If the client is not logged on, the value is null.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn current_session(&self) -> Result<Option<crate::types::structs::UserSession>> {
        let path = format!("/SessionManager/{moId}/currentSession", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<crate::types::structs::UserSession>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// This is the default server locale.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn default_locale(&self) -> Result<String> {
        let path = format!("/SessionManager/{moId}/defaultLocale", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes = self.client.execute_bytes(req).await?;
        let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
        let result: String = miniserde::json::from_str(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?;
        Ok(result)
    }
    /// The system global message from the server.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn message(&self) -> Result<Option<String>> {
        let path = format!("/SessionManager/{moId}/message", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<String>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Provides the list of locales for which the server has localized messages.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn message_locale_list(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/SessionManager/{moId}/messageLocaleList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<String>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// The list of currently active sessions.
    /// 
    /// ***Required privileges:*** Sessions.TerminateSession
    pub async fn session_list(&self) -> Result<Option<Vec<crate::types::structs::UserSession>>> {
        let path = format!("/SessionManager/{moId}/sessionList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<crate::types::structs::UserSession>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
    /// Provides the list of locales that the server supports.
    /// 
    /// Listing a locale ensures that some standardized information such as dates appear
    /// in the appropriate format. Other localized information, such as error messages,
    /// are displayed, if available. If localized information is not available, the
    /// message is returned using the system locale.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn supported_locale_list(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/SessionManager/{moId}/supportedLocaleList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        let bytes_opt = self.client.execute_option_bytes(req).await?;
        match bytes_opt {
            Some(bytes) => {
                let text = std::str::from_utf8(bytes.as_ref()).map_err(|e| crate::core::client::VimError::ParseError(e.to_string()))?;
                Ok(Some(miniserde::json::from_str::<Vec<String>>(text).map_err(|_| crate::core::client::VimError::ParseError("miniserde deserialization failed".to_string()))?))
            }
            None => Ok(None),
        }
    }
}
struct AcquireGenericServiceTicketRequestType<'a> {
    spec: &'a dyn crate::types::traits::SessionManagerServiceRequestSpecTrait,
}

impl<'a> miniserde::Serialize for AcquireGenericServiceTicketRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AcquireGenericServiceTicketRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AcquireGenericServiceTicketRequestTypeSer<'b, 'a> {
    data: &'b AcquireGenericServiceTicketRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AcquireGenericServiceTicketRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AcquireGenericServiceTicketRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("spec"), &self.data.spec as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct AcquireLocalTicketRequestType<'a> {
    user_name: &'a str,
}

impl<'a> miniserde::Serialize for AcquireLocalTicketRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(AcquireLocalTicketRequestTypeSer { data: self, seq: 0 }))
    }
}

struct AcquireLocalTicketRequestTypeSer<'b, 'a> {
    data: &'b AcquireLocalTicketRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for AcquireLocalTicketRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"AcquireLocalTicketRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct CloneSessionRequestType<'a> {
    clone_ticket: &'a str,
}

impl<'a> miniserde::Serialize for CloneSessionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(CloneSessionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct CloneSessionRequestTypeSer<'b, 'a> {
    data: &'b CloneSessionRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for CloneSessionRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"CloneSessionRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("cloneTicket"), &self.data.clone_ticket as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct ImpersonateUserRequestType<'a> {
    user_name: &'a str,
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for ImpersonateUserRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(ImpersonateUserRequestTypeSer { data: self, seq: 0 }))
    }
}

struct ImpersonateUserRequestTypeSer<'b, 'a> {
    data: &'b ImpersonateUserRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for ImpersonateUserRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"ImpersonateUserRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LoginRequestType<'a> {
    user_name: &'a str,
    password: &'a str,
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for LoginRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LoginRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LoginRequestTypeSer<'b, 'a> {
    data: &'b LoginRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LoginRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LoginRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("password"), &self.data.password as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LoginBySspiRequestType<'a> {
    base_64_token: &'a str,
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for LoginBySspiRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LoginBySspiRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LoginBySspiRequestTypeSer<'b, 'a> {
    data: &'b LoginBySspiRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LoginBySspiRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LoginBySSPIRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("base64Token"), &self.data.base_64_token as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LoginByTokenRequestType<'a> {
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for LoginByTokenRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LoginByTokenRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LoginByTokenRequestTypeSer<'b, 'a> {
    data: &'b LoginByTokenRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LoginByTokenRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LoginByTokenRequestType")),
                1 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LoginExtensionRequestType<'a> {
    extension_key: &'a str,
    base_64_signed_credentials: &'a str,
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for LoginExtensionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LoginExtensionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LoginExtensionRequestTypeSer<'b, 'a> {
    data: &'b LoginExtensionRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LoginExtensionRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LoginExtensionRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("extensionKey"), &self.data.extension_key as &dyn miniserde::Serialize)),
                2 => return Some((std::borrow::Cow::Borrowed("base64SignedCredentials"), &self.data.base_64_signed_credentials as &dyn miniserde::Serialize)),
                3 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LoginExtensionByCertificateRequestType<'a> {
    extension_key: &'a str,
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for LoginExtensionByCertificateRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LoginExtensionByCertificateRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LoginExtensionByCertificateRequestTypeSer<'b, 'a> {
    data: &'b LoginExtensionByCertificateRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LoginExtensionByCertificateRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LoginExtensionByCertificateRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("extensionKey"), &self.data.extension_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct LoginExtensionBySubjectNameRequestType<'a> {
    extension_key: &'a str,
    locale: Option<&'a str>,
}

impl<'a> miniserde::Serialize for LoginExtensionBySubjectNameRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(LoginExtensionBySubjectNameRequestTypeSer { data: self, seq: 0 }))
    }
}

struct LoginExtensionBySubjectNameRequestTypeSer<'b, 'a> {
    data: &'b LoginExtensionBySubjectNameRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for LoginExtensionBySubjectNameRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        loop {
            let seq = self.seq;
            self.seq += 1;
            match seq {
                0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"LoginExtensionBySubjectNameRequestType")),
                1 => return Some((std::borrow::Cow::Borrowed("extensionKey"), &self.data.extension_key as &dyn miniserde::Serialize)),
                2 => {
                    let Some(ref val) = self.data.locale else { continue; };
                    return Some((std::borrow::Cow::Borrowed("locale"), val as &dyn miniserde::Serialize));
                }
                _ => return None,
            }
        }
    }
}
struct SessionIsActiveRequestType<'a> {
    session_id: &'a str,
    user_name: &'a str,
}

impl<'a> miniserde::Serialize for SessionIsActiveRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SessionIsActiveRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SessionIsActiveRequestTypeSer<'b, 'a> {
    data: &'b SessionIsActiveRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SessionIsActiveRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SessionIsActiveRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("sessionID"), &self.data.session_id as &dyn miniserde::Serialize)),
            2 => return Some((std::borrow::Cow::Borrowed("userName"), &self.data.user_name as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct SetLocaleRequestType<'a> {
    locale: &'a str,
}

impl<'a> miniserde::Serialize for SetLocaleRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(SetLocaleRequestTypeSer { data: self, seq: 0 }))
    }
}

struct SetLocaleRequestTypeSer<'b, 'a> {
    data: &'b SetLocaleRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for SetLocaleRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"SetLocaleRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("locale"), &self.data.locale as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct TerminateSessionRequestType<'a> {
    session_id: &'a [String],
}

impl<'a> miniserde::Serialize for TerminateSessionRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(TerminateSessionRequestTypeSer { data: self, seq: 0 }))
    }
}

struct TerminateSessionRequestTypeSer<'b, 'a> {
    data: &'b TerminateSessionRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for TerminateSessionRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"TerminateSessionRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("sessionId"), &self.data.session_id as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
struct UpdateServiceMessageRequestType<'a> {
    message: &'a str,
}

impl<'a> miniserde::Serialize for UpdateServiceMessageRequestType<'a> {
    fn begin(&self) -> miniserde::ser::Fragment<'_> {
        miniserde::ser::Fragment::Map(Box::new(UpdateServiceMessageRequestTypeSer { data: self, seq: 0 }))
    }
}

struct UpdateServiceMessageRequestTypeSer<'b, 'a> {
    data: &'b UpdateServiceMessageRequestType<'a>,
    seq: usize,
}

impl miniserde::ser::Map for UpdateServiceMessageRequestTypeSer<'_, '_> {
    fn next(&mut self) -> Option<(std::borrow::Cow<'_, str>, &dyn miniserde::Serialize)> {
        let seq = self.seq;
        self.seq += 1;
        match seq {
            0 => return Some((std::borrow::Cow::Borrowed("_typeName"), &"UpdateServiceMessageRequestType")),
            1 => return Some((std::borrow::Cow::Borrowed("message"), &self.data.message as &dyn miniserde::Serialize)),
            _ => return None,
        }
    }
}
