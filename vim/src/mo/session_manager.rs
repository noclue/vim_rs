use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::SessionManagerGenericServiceTicket;
use crate::types::structs::SessionManagerLocalTicket;
use crate::types::structs::SessionManagerServiceRequestSpecTrait;
use crate::types::structs::UserSession;
/// This managed object type includes methods for logging on and
/// logging off clients, determining which clients are currently
/// logged on, and forcing clients to log off.
pub struct SessionManager {
    client: Arc<Client>,
    mo_id: String,
}
impl SessionManager {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
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
        Ok(self.client.execute(req).await?)
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
    /// *GenericServiceTicket#sslCertificate*.
    /// If *GenericServiceTicket#sslCertificate* is unset, the
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
    pub async fn acquire_generic_service_ticket(&self, spec: &dyn SessionManagerServiceRequestSpecTrait) -> Result<SessionManagerGenericServiceTicket> {
        let input = AcquireGenericServiceTicketRequestType {spec, };
        let path = format!("/SessionManager/{moId}/AcquireGenericServiceTicket", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn acquire_local_ticket(&self, user_name: &str) -> Result<SessionManagerLocalTicket> {
        let input = AcquireLocalTicketRequestType {user_name, };
        let path = format!("/SessionManager/{moId}/AcquireLocalTicket", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn clone_session(&self, clone_ticket: &str) -> Result<UserSession> {
        let input = CloneSessionRequestType {clone_ticket, };
        let path = format!("/SessionManager/{moId}/CloneSession", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn impersonate_user(&self, user_name: &str, locale: Option<&str>) -> Result<UserSession> {
        let input = ImpersonateUserRequestType {user_name, locale, };
        let path = format!("/SessionManager/{moId}/ImpersonateUser", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn login(&self, user_name: &str, password: &str, locale: Option<&str>) -> Result<UserSession> {
        let input = LoginRequestType {user_name, password, locale, };
        let path = format!("/SessionManager/{moId}/Login", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
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
    /// As of vSphere API 5.1 for VirtualCenter login use SSO style
    /// *SessionManager.LoginByToken*
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
    pub async fn login_by_sspi(&self, base_64_token: &str, locale: Option<&str>) -> Result<UserSession> {
        let input = LoginBySspiRequestType {base_64_token, locale, };
        let path = format!("/SessionManager/{moId}/LoginBySSPI", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn login_by_token(&self, locale: Option<&str>) -> Result<UserSession> {
        let input = LoginByTokenRequestType {locale, };
        let path = format!("/SessionManager/{moId}/LoginByToken", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn login_extension(&self, extension_key: &str, base_64_signed_credentials: &str, locale: Option<&str>) -> Result<UserSession> {
        let input = LoginExtensionRequestType {extension_key, base_64_signed_credentials, locale, };
        let path = format!("/SessionManager/{moId}/LoginExtension", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn login_extension_by_certificate(&self, extension_key: &str, locale: Option<&str>) -> Result<UserSession> {
        let input = LoginExtensionByCertificateRequestType {extension_key, locale, };
        let path = format!("/SessionManager/{moId}/LoginExtensionByCertificate", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
    pub async fn login_extension_by_subject_name(&self, extension_key: &str, locale: Option<&str>) -> Result<UserSession> {
        let input = LoginExtensionBySubjectNameRequestType {extension_key, locale, };
        let path = format!("/SessionManager/{moId}/LoginExtensionBySubjectName", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Log out and terminate the current session.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn logout(&self) -> Result<()> {
        let path = format!("/SessionManager/{moId}/Logout", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
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
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// This property contains information about the client's current session.
    /// 
    /// If the client is not logged on, the value is null.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn current_session(&self) -> Result<UserSession> {
        let path = format!("/SessionManager/{moId}/currentSession", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// This is the default server locale.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn default_locale(&self) -> Result<String> {
        let path = format!("/SessionManager/{moId}/defaultLocale", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute(req).await?)
    }
    /// The system global message from the server.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn message(&self) -> Result<Option<String>> {
        let path = format!("/SessionManager/{moId}/message", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// Provides the list of locales for which the server has localized messages.
    /// 
    /// ***Required privileges:*** System.Anonymous
    pub async fn message_locale_list(&self) -> Result<Option<Vec<String>>> {
        let path = format!("/SessionManager/{moId}/messageLocaleList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
    /// The list of currently active sessions.
    /// 
    /// ***Required privileges:*** Sessions.TerminateSession
    pub async fn session_list(&self) -> Result<Option<Vec<UserSession>>> {
        let path = format!("/SessionManager/{moId}/sessionList", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
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
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AcquireGenericServiceTicketRequestType<'a> {
    spec: &'a dyn SessionManagerServiceRequestSpecTrait,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct AcquireLocalTicketRequestType<'a> {
    #[serde(rename = "userName")]
    user_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CloneSessionRequestType<'a> {
    #[serde(rename = "cloneTicket")]
    clone_ticket: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ImpersonateUserRequestType<'a> {
    #[serde(rename = "userName")]
    user_name: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LoginRequestType<'a> {
    #[serde(rename = "userName")]
    user_name: &'a str,
    password: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(rename = "LoginBySSPIRequestType", tag = "_typeName")]
struct LoginBySspiRequestType<'a> {
    #[serde(rename = "base64Token")]
    base_64_token: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LoginByTokenRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LoginExtensionRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
    #[serde(rename = "base64SignedCredentials")]
    base_64_signed_credentials: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LoginExtensionByCertificateRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct LoginExtensionBySubjectNameRequestType<'a> {
    #[serde(rename = "extensionKey")]
    extension_key: &'a str,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SessionIsActiveRequestType<'a> {
    #[serde(rename = "sessionID")]
    session_id: &'a str,
    #[serde(rename = "userName")]
    user_name: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct SetLocaleRequestType<'a> {
    locale: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct TerminateSessionRequestType<'a> {
    #[serde(rename = "sessionId")]
    session_id: &'a [String],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct UpdateServiceMessageRequestType<'a> {
    message: &'a str,
}
