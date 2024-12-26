use std::sync::Arc;
use crate::core::client::{Client, Result};
use crate::types::structs::ManagedObjectReference;
use crate::types::structs::ObjectContent;
use crate::types::structs::PropertyFilterSpec;
use crate::types::structs::RetrieveOptions;
use crate::types::structs::RetrieveResult;
use crate::types::structs::UpdateSet;
use crate::types::structs::WaitOptions;
/// The *PropertyCollector* managed object retrieves and detects changes
/// to the properties of other managed objects.
/// 
/// The *PropertyCollector.RetrievePropertiesEx* method provides one-time property retrieval. The
/// *PropertyCollector.WaitForUpdatesEx* method provides incremental change detection and
/// supports both polling and notification.
/// 
/// For change detection a client creates one or more filters to specify the
/// subset of managed objects in which the client is interested. Filters keep
/// per-session state to track incremental changes. Because this state is
/// per-session:
/// - A session cannot share its *PropertyCollector* filters with other
///   sessions
/// - two different clients can share the same session, and so can
///   share the same filters, but this is not recommended
/// - When a session terminates, the associated *PropertyCollector* filters
///   are automatically destroyed.
pub struct PropertyCollector {
    client: Arc<Client>,
    mo_id: String,
}
impl PropertyCollector {
    pub fn new(client: Arc<Client>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
    /// Discards remaining results from a retrieval started by *PropertyCollector.RetrievePropertiesEx* on the same session on the same *PropertyCollector*.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### token
    /// the token returned in the previous *RetrieveResult* returned on the same session by the
    /// same *PropertyCollector*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If the token does not match the token from the
    /// previous *RetrieveResult* returned on the same
    /// session by the same *PropertyCollector*.
    pub async fn cancel_retrieve_properties_ex(&self, token: &str) -> Result<()> {
        let input = CancelRetrievePropertiesExRequestType {token, };
        let path = format!("/PropertyCollector/{moId}/CancelRetrievePropertiesEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_void(req).await?)
    }
    /// Attempts to cancel outstanding calls to *PropertyCollector.WaitForUpdates* or *PropertyCollector.WaitForUpdatesEx* in the current session.
    /// 
    /// If an update calculation is
    /// in process this method has no effect. If an update calculation is not in
    /// process any waiting calls complete quickly and report a *RequestCanceled* fault.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn cancel_wait_for_updates(&self) -> Result<()> {
        let path = format!("/PropertyCollector/{moId}/CancelWaitForUpdates", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 4.1, use
    /// *PropertyCollector.WaitForUpdatesEx* with a
    /// *WaitOptions.maxWaitSeconds* of 0.
    /// 
    /// Checks for updates on properties specified by the union of all current
    /// filters.
    /// 
    /// If no updates are pending, this method returns null.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### version
    /// The data version currently known to the client. The value
    /// must be either
    /// - the special initial version (an empty string)
    /// - a data version returned from *PropertyCollector.CheckForUpdates* or *PropertyCollector.WaitForUpdates* by the same *PropertyCollector* on the same session.
    /// - a non-truncated data version returned from *PropertyCollector.WaitForUpdatesEx* by the same *PropertyCollector* on the same
    ///   session.
    ///
    /// ## Returns:
    ///
    /// Changes since the passed in data version. If no updates are
    /// pending, then this method returns null.
    ///
    /// ## Errors:
    ///
    /// ***InvalidCollectorVersion***: if the data version does not meet the
    /// requirements above.
    /// 
    /// ***RequestCanceled***: if *PropertyCollector.CancelWaitForUpdates* has been called or the session was closed
    /// or the *PropertyCollector* was destroyed at some point after the call was
    /// received but before the update calculation was actually started
    pub async fn check_for_updates(&self, version: Option<&str>) -> Result<UpdateSet> {
        let input = CheckForUpdatesRequestType {version, };
        let path = format!("/PropertyCollector/{moId}/CheckForUpdates", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Retrieves additional results from a retrieval started by *PropertyCollector.RetrievePropertiesEx* on the same session on the same *PropertyCollector*.
    /// 
    /// ***Required privileges:*** System.Read
    ///
    /// ## Parameters:
    ///
    /// ### token
    /// the token returned in the previous *RetrieveResult* returned on the same session by the
    /// same *PropertyCollector*.
    ///
    /// ## Returns:
    ///
    /// retrieved objects.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: If the token does not match the token from the
    /// previous *RetrieveResult* returned on the same
    /// session by the same *PropertyCollector*.
    pub async fn continue_retrieve_properties_ex(&self, token: &str) -> Result<RetrieveResult> {
        let input = ContinueRetrievePropertiesExRequestType {token, };
        let path = format!("/PropertyCollector/{moId}/ContinueRetrievePropertiesEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a new filter for the given set of managed objects.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### spec
    /// The specifications for the filter.
    ///
    /// ### partial_updates
    /// Flag to specify whether a change to a nested property should report
    /// only the nested change or the entire specified property value. If the
    /// value is true, a change should report only the nested property. If
    /// the value is false, a change should report the enclosing property
    /// named in the filter.
    ///
    /// ## Returns:
    ///
    /// A reference to the new filter.
    /// 
    /// Refers instance of *PropertyFilter*.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any of the following is true:
    /// - "spec" is empty.
    /// - "spec" contains a selection with properties not defined on its
    ///   type.
    ///   
    /// ***InvalidProperty***: if "spec" has a property that is not defined on one of the objects.
    /// 
    /// ***InvalidType***: if "spec" contains, directly or indirectly, a type name that
    /// does not refer to a known type.
    /// 
    /// ***ManagedObjectNotFound***: See *PropertyFilterSpec.reportMissingObjectsInResults*.
    pub async fn create_filter(&self, spec: &PropertyFilterSpec, partial_updates: bool) -> Result<ManagedObjectReference> {
        let input = CreateFilterRequestType {spec, partial_updates, };
        let path = format!("/PropertyCollector/{moId}/CreateFilter", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Creates a new session-specific *PropertyCollector* that can
    /// be used to retrieve property updates independent of any other
    /// *PropertyCollector*.
    /// 
    /// The newly created *PropertyCollector*
    /// is not tied to the creating *PropertyCollector* in any way and
    /// exists until it is destroyed
    /// by a call to *PropertyCollector.DestroyPropertyCollector* or until the session
    /// on which the PropertyCollector was created is closed. This is in contrast
    /// to the default *PropertyCollector*, which always exists, but still has
    /// session-specific data such as filters and unfinished update calculations
    /// that are discarded when the associated session is closed.
    /// 
    /// A new *PropertyCollector* can be useful when multiple modules or even
    /// multiple clients that share the
    /// same session need to create their own *PropertyFilter*
    /// objects and process updates independently. They may also be useful
    /// to allow important updates to be seen on one *PropertyCollector* while
    /// a large update is being calculated on another. The underlying
    /// issue that this addresses is that any call to *PropertyCollector.WaitForUpdates*,
    /// *PropertyCollector.CheckForUpdates*, or *PropertyCollector.WaitForUpdatesEx* does updates on all the
    /// filters created on a given *PropertyCollector* on a given session.
    /// 
    /// A more subtle use of multiple *PropertyCollector* objects is implied
    /// by the fact that the returned version value for the various updates
    /// calculations is strongly ordered. The only way this can make sense is that
    /// two different versions calculated on the same *PropertyCollector* on
    /// the same session cannot ever be created in parallel. This means that multiple
    /// calls to *PropertyCollector.WaitForUpdates*, *PropertyCollector.CheckForUpdates*, or
    /// *PropertyCollector.WaitForUpdatesEx* made to the same *PropertyCollector* on the
    /// same session on different threads of the same client, or even on different
    /// clients that share the same session are still handled on the server serially.
    /// Use of different *PropertyCollector* instances allows the server to
    /// handle these calculations in parallel.
    /// 
    /// Typically a service that supports the *PropertyCollector* managed
    /// object type will automatically create a default *PropertyCollector*
    /// and provide some way to obtain a reference to this
    /// *PropertyCollector*. If not, it will have to provide some
    /// service-specific way to create the a *PropertyCollector*.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// A reference to the new *PropertyCollector*.
    /// 
    /// Refers instance of *PropertyCollector*.
    pub async fn create_property_collector(&self) -> Result<ManagedObjectReference> {
        let path = format!("/PropertyCollector/{moId}/CreatePropertyCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute(req).await?)
    }
    /// Destroys this *PropertyCollector*.
    /// 
    /// A *PropertyCollector* that was created by
    /// *PropertyCollector.CreatePropertyCollector* is automatically destroyed when the
    /// session on which it was created is closed. This method can be used to
    /// destroy them explicitly.
    /// 
    /// An automatically created *PropertyCollector* provided by a service
    /// is not session specific and may not be destroyed.
    /// 
    /// ***Required privileges:*** System.View
    pub async fn destroy_property_collector(&self) -> Result<()> {
        let path = format!("/PropertyCollector/{moId}/DestroyPropertyCollector", moId = &self.mo_id);
        let req = self.client.post_bare(&path);
        Ok(self.client.execute_void(req).await?)
    }
    /// Deprecated as of vSphere API 4.1, use *PropertyCollector.RetrievePropertiesEx*.
    /// 
    /// Retrieves the specified properties of the specified managed objects.
    /// 
    /// This method is similar to creating the filters, receiving the
    /// property values, and destroying the filters. The main difference is that
    /// the output blends the results from all the filters and reports a given
    /// managed object at most once no matter how many filters apply.
    /// 
    /// The filter creation step can throw all of the same faults as *PropertyCollector.CreateFilter*.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### spec_set
    /// Specifies the properties to retrieve.
    ///
    /// ## Returns:
    ///
    /// The data contents of the specified objects.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: See *PropertyCollector.CreateFilter*
    /// 
    /// ***InvalidProperty***: See *PropertyCollector.CreateFilter*
    /// 
    /// ***InvalidType***: See *PropertyCollector.CreateFilter*
    /// 
    /// ***ManagedObjectNotFound***: See *PropertyCollector.CreateFilter*
    pub async fn retrieve_properties(&self, spec_set: &[PropertyFilterSpec]) -> Result<Option<Vec<ObjectContent>>> {
        let input = RetrievePropertiesRequestType {spec_set, };
        let path = format!("/PropertyCollector/{moId}/RetrieveProperties", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute_option(req).await?)
    }
    /// Retrieves the specified properties of the specified managed objects.
    /// 
    /// This method is similar to creating the filters, receiving the
    /// property values, and destroying the filters. The main difference is that
    /// the output blends the results from all the filters and reports a given
    /// managed object at most once no matter how many filters apply.
    /// 
    /// The filter creation step can throw all of the same faults as *PropertyCollector.CreateFilter*.
    /// 
    /// ***Required privileges:*** System.Anonymous
    ///
    /// ## Parameters:
    ///
    /// ### spec_set
    /// Specifies the properties to retrieve.
    ///
    /// ### options
    /// Additional method options. If omitted, equivalent to an options
    /// argument with no fields set.
    ///
    /// ## Returns:
    ///
    /// retrieved objects or null if there are no matching objects.
    ///
    /// ## Errors:
    ///
    /// ***InvalidArgument***: if any of the following is true:
    /// See *PropertyCollector.CreateFilter*
    /// 
    /// ***InvalidProperty***: See *PropertyCollector.CreateFilter*
    /// 
    /// ***InvalidType***: See *PropertyCollector.CreateFilter*
    /// 
    /// ***ManagedObjectNotFound***: See *PropertyCollector.CreateFilter*
    pub async fn retrieve_properties_ex(&self, spec_set: &[PropertyFilterSpec], options: &RetrieveOptions) -> Result<RetrieveResult> {
        let input = RetrievePropertiesExRequestType {spec_set, options, };
        let path = format!("/PropertyCollector/{moId}/RetrievePropertiesEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Deprecated as of vSphere API 4.1, use *PropertyCollector.WaitForUpdatesEx*.
    /// 
    /// Calculate the set of updates for each existing filter in the session,
    /// returning when at least one filter has updates.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### version
    /// The data version currently known to the client. The value
    /// must be either
    /// - the special initial version (an empty string)
    /// - a data version returned from *PropertyCollector.CheckForUpdates* or *PropertyCollector.WaitForUpdates* by the same *PropertyCollector* on the same session
    /// - a non-truncated data version returned from *PropertyCollector.WaitForUpdatesEx* by the same *PropertyCollector* on the same
    ///   session.
    ///
    /// ## Returns:
    ///
    /// Changes since the passed in data version.
    ///
    /// ## Errors:
    ///
    /// ***InvalidCollectorVersion***: if the data version does not meet the
    /// requirements above.
    /// 
    /// ***RequestCanceled***: if *PropertyCollector.CancelWaitForUpdates* has been called or the session was closed
    /// or the *PropertyCollector* was destroyed at some point after the call was
    /// received
    pub async fn wait_for_updates(&self, version: Option<&str>) -> Result<UpdateSet> {
        let input = WaitForUpdatesRequestType {version, };
        let path = format!("/PropertyCollector/{moId}/WaitForUpdates", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// Calculate the set of updates for each existing filter in the session.
    /// 
    /// *PropertyCollector.WaitForUpdatesEx* may return only partial update
    /// calculations. See *UpdateSet.truncated* for a
    /// more detailed explanation. *PropertyCollector.WaitForUpdatesEx* may also return null
    /// after a timeout, either as requested by *WaitOptions.maxWaitSeconds* or due to *PropertyCollector*
    /// policy.
    /// 
    /// If an application uses waitForUpdatesEx it is strongly recommended
    /// that it not make concurrent calls to *PropertyCollector.WaitForUpdates*, *PropertyCollector.CheckForUpdates*, or *PropertyCollector.WaitForUpdatesEx* in the same
    /// session. Concurrent calls may cause suspended change calculations to be
    /// discarded.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Parameters:
    ///
    /// ### version
    /// The data version currently known to the client. The value must be
    /// either
    /// - the special initial data version (an empty string),
    /// - a data version returned from *PropertyCollector.CheckForUpdates* or *PropertyCollector.WaitForUpdates*
    /// - a non-truncated data version returned from *PropertyCollector.WaitForUpdatesEx*
    /// - a truncated data version returned from the last call to *PropertyCollector.WaitForUpdatesEx* with no intervening calls to *PropertyCollector.WaitForUpdates* or *PropertyCollector.CheckForUpdates*.
    ///
    /// ### options
    /// Additional options controlling the change calculation. If omitted,
    /// equivalent to an options argument with no fields set.
    ///
    /// ## Returns:
    ///
    /// Changes since the passed in version or null if there are no
    /// changes.
    ///
    /// ## Errors:
    ///
    /// ***InvalidCollectorVersion***: if the data version does not meet the
    /// requirements above.
    /// 
    /// ***RequestCanceled***: if *PropertyCollector.CancelWaitForUpdates* has been called or the session was closed
    /// or the *PropertyCollector* was destroyed at some point after the call was
    /// received
    pub async fn wait_for_updates_ex(&self, version: Option<&str>, options: Option<&WaitOptions>) -> Result<UpdateSet> {
        let input = WaitForUpdatesExRequestType {version, options, };
        let path = format!("/PropertyCollector/{moId}/WaitForUpdatesEx", moId = &self.mo_id);
        let req = self.client.post_request(&path, &input);
        Ok(self.client.execute(req).await?)
    }
    /// The filters that this *PropertyCollector* uses to determine the list of
    /// properties for which it detects incremental changes.
    /// 
    /// ***Required privileges:*** System.View
    ///
    /// ## Returns:
    ///
    /// Refers instances of *PropertyFilter*.
    pub async fn filter(&self) -> Result<Option<Vec<ManagedObjectReference>>> {
        let path = format!("/PropertyCollector/{moId}/filter", moId = &self.mo_id);
        let req = self.client.get_request(&path);
        Ok(self.client.execute_option(req).await?)
    }
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CancelRetrievePropertiesExRequestType<'a> {
    token: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CheckForUpdatesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct ContinueRetrievePropertiesExRequestType<'a> {
    token: &'a str,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct CreateFilterRequestType<'a> {
    spec: &'a PropertyFilterSpec,
    #[serde(rename = "partialUpdates")]
    partial_updates: bool,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrievePropertiesRequestType<'a> {
    #[serde(rename = "specSet")]
    spec_set: &'a [PropertyFilterSpec],
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct RetrievePropertiesExRequestType<'a> {
    #[serde(rename = "specSet")]
    spec_set: &'a [PropertyFilterSpec],
    options: &'a RetrieveOptions,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct WaitForUpdatesRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
}
#[derive(serde::Serialize)]
#[serde(tag="_typeName")]
struct WaitForUpdatesExRequestType<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    options: Option<&'a WaitOptions>,
}
