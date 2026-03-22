//! JSON-oriented `VimClient` test double: records paths like the VI JSON API and scripts
//! PropertyCollector long-poll behavior. Implements `invoke`, `invoke_optional`, `invoke_void`,
//! and `fetch_property_raw` (GET-style paths).

use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::sync::mpsc;

use vim_rs::core::client::{BoxFuture, Error, PropertyValue, Result, Transport, VimClient};
use vim_rs::types::structs::ServiceContent;
use vim_rs::types::enums::MoTypesEnum;

#[derive(Debug, Clone, Copy)]
pub enum HttpVerb {
    Get,
    Post,
}

#[derive(Debug, Clone)]
pub struct RecordedRequest {
    pub verb: HttpVerb,
    pub path: String,
}

#[derive(Debug)]
pub enum PcEvent {
    /// Return `Ok(None)` from WaitForUpdatesEx.
    None,
    /// Return `Ok(Some(bytes))` from WaitForUpdatesEx.
    Bytes(Bytes),
    /// Return an error from WaitForUpdatesEx.
    Err(Error),
}

#[derive(Default)]
struct Counters {
    create_list_view: usize,
    modify_list_view: usize,
    destroy_list_view: usize,
    create_filter: usize,
    destroy_filter: usize,
    wait_for_updates_ex: usize,
}

#[derive(Clone)]
enum CreateListViewBehavior {
    Success,
    FailOnce,
}

/// Scripted `VimClient` used by integration tests.
///
/// Implements `VimClient` and dispatches based on the method/property path
/// (`/{mo_type}/{mo_id}/{method_name}` or `/{mo_type}/{mo_id}/{property}`), matching VI JSON URL layout.
pub struct MockVimClient {
    service_content: ServiceContent,

    requests: Mutex<Vec<RecordedRequest>>,
    counters: Mutex<Counters>,

    // PropertyCollector wait events (long-poll simulation).
    pc_rx: tokio::sync::Mutex<mpsc::UnboundedReceiver<PcEvent>>,

    // Optional gate to block the Nth ModifyListView call (used for race tests).
    block_modify_list_view_at: Mutex<Option<(usize, Arc<tokio::sync::Notify>)>>,

    // Control CreateListView behavior.
    create_list_view_behavior: Mutex<CreateListViewBehavior>,
}

impl MockVimClient {
    pub fn new(service_content: ServiceContent, pc_rx: mpsc::UnboundedReceiver<PcEvent>) -> Self {
        Self {
            service_content,
            requests: Mutex::new(Vec::new()),
            counters: Mutex::new(Counters::default()),
            pc_rx: tokio::sync::Mutex::new(pc_rx),
            block_modify_list_view_at: Mutex::new(None),
            create_list_view_behavior: Mutex::new(CreateListViewBehavior::Success),
        }
    }

    /// Make CreateListView fail once, then succeed on subsequent calls.
    pub fn fail_create_list_view_once(&self) {
        *self.create_list_view_behavior.lock().unwrap() = CreateListViewBehavior::FailOnce;
    }

    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().unwrap().clone()
    }

    pub fn counters_snapshot(&self) -> (usize, usize, usize, usize, usize, usize) {
        let c = self.counters.lock().unwrap();
        (
            c.create_list_view,
            c.modify_list_view,
            c.destroy_list_view,
            c.create_filter,
            c.destroy_filter,
            c.wait_for_updates_ex,
        )
    }

    pub fn block_modify_list_view_call(&self, call_index_1_based: usize, gate: Arc<tokio::sync::Notify>) {
        *self.block_modify_list_view_at.lock().unwrap() = Some((call_index_1_based, gate));
    }

    fn record(&self, verb: HttpVerb, path: &str) {
        self.requests.lock().unwrap().push(RecordedRequest {
            verb,
            path: path.to_string(),
        });
    }

    fn bump<F: FnOnce(&mut Counters)>(&self, f: F) {
        let mut c = self.counters.lock().unwrap();
        f(&mut c);
    }

    async fn handle_void(&self, verb: HttpVerb, path: &str) -> Result<()> {
        self.record(verb, path);
        if path.contains("/DestroyView") && path.starts_with("/ListView/") {
            self.bump(|c| c.destroy_list_view += 1);
            return Ok(());
        }
        if path.contains("/DestroyPropertyFilter") && path.starts_with("/PropertyFilter/") {
            self.bump(|c| c.destroy_filter += 1);
            return Ok(());
        }
        Ok(())
    }

    async fn handle_bytes(&self, verb: HttpVerb, path: &str) -> Result<Bytes> {
        self.record(verb, path);
        if path.contains("/CreateListView") && path.starts_with("/ViewManager/") {
            self.bump(|c| c.create_list_view += 1);
            
            // Check if we should fail this call
            let should_fail = {
                let mut behavior = self.create_list_view_behavior.lock().unwrap();
                match *behavior {
                    CreateListViewBehavior::Success => false,
                    CreateListViewBehavior::FailOnce => {
                        *behavior = CreateListViewBehavior::Success;
                        true
                    }
                }
            };
            
            if should_fail {
                return Err(Error::ParseError("Simulated CreateListView failure".to_string()));
            }
            
            let mor = vim_rs::types::structs::ManagedObjectReference {
                r#type: MoTypesEnum::ListView,
                value: "listview-1".to_string(),
            };
            return Ok(Bytes::from(miniserde::json::to_string(&mor).into_bytes()));
        }
        if path.contains("/CreateFilter") && path.starts_with("/PropertyCollector/") {
            self.bump(|c| c.create_filter += 1);
            let mor = vim_rs::types::structs::ManagedObjectReference {
                r#type: MoTypesEnum::PropertyFilter,
                value: "filter-1".to_string(),
            };
            return Ok(Bytes::from(miniserde::json::to_string(&mor).into_bytes()));
        }
        Err(Error::ParseError(format!("MockVimClient: unhandled invoke path: {path}")))
    }

    async fn handle_option_bytes(&self, verb: HttpVerb, path: &str) -> Result<Option<Bytes>> {
        self.record(verb, path);

        if path.contains("/ModifyListView") && path.starts_with("/ListView/") {
            self.bump(|c| c.modify_list_view += 1);
            // Optional gating for race tests.
            let block = { self.block_modify_list_view_at.lock().unwrap().clone() };
            if let Some((n, gate)) = block {
                let call_idx = self.counters.lock().unwrap().modify_list_view;
                if call_idx == n {
                    gate.notified().await;
                }
            }
            // Returning None means empty body -> Ok(None) for Option return type.
            return Ok(None);
        }

        if path.contains("/WaitForUpdatesEx") && path.starts_with("/PropertyCollector/") {
            self.bump(|c| c.wait_for_updates_ex += 1);
            let mut rx = self.pc_rx.lock().await;
            match rx.recv().await {
                Some(PcEvent::None) => return Ok(None),
                Some(PcEvent::Bytes(b)) => return Ok(Some(b)),
                Some(PcEvent::Err(e)) => return Err(e),
                None => {
                    return Err(Error::ParseError(
                        "MockVimClient: WaitForUpdatesEx channel closed".to_string(),
                    ))
                }
            }
        }

        Ok(None)
    }
}

impl VimClient for MockVimClient {
    fn service_content(&self) -> &ServiceContent {
        &self.service_content
    }

    fn transport(&self) -> Transport {
        Transport::Json
    }

    fn api_release(&self) -> String {
        self.service_content.about.api_version.clone()
    }

    fn invoke<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        _params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Bytes>> {
        let path = vi_json_path(svc, mo_type, mo_id, method_name);
        Box::pin(async move {
            self.handle_bytes(HttpVerb::Post, &path).await
        })
    }

    fn invoke_optional<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        _params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        let path = vi_json_path(svc, mo_type, mo_id, method_name);
        Box::pin(async move {
            self.handle_option_bytes(HttpVerb::Post, &path).await
        })
    }

    fn invoke_void<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        method_name: &'a str,
        _params: Option<&'a (dyn miniserde::Serialize + Send + Sync)>,
    ) -> BoxFuture<'a, Result<()>> {
        let path = vi_json_path(svc, mo_type, mo_id, method_name);
        Box::pin(async move {
            self.handle_void(HttpVerb::Post, &path).await
        })
    }

    fn fetch_property_raw<'a>(
        &'a self,
        svc: &'a str,
        mo_type: &'a str,
        mo_id: &'a str,
        property: &'a str,
    ) -> BoxFuture<'a, Result<Option<PropertyValue>>> {
        let path = vi_json_path(svc, mo_type, mo_id, property);
        Box::pin(async move {
            Ok(self
                .handle_option_bytes(HttpVerb::Get, &path)
                .await?
                .map(PropertyValue::Json))
        })
    }
}

fn vi_json_path(svc: &str, mo_type: &str, mo_id: &str, method_name: &str) -> String {
    if svc.is_empty() {
        format!("/{mo_type}/{mo_id}/{method_name}")
    } else {
        format!("/{svc}/{mo_type}/{mo_id}/{method_name}")
    }
}
