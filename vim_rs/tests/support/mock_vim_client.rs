use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::sync::mpsc;

use vim_rs::core::client::{BoxFuture, Error, Result, VimClient};
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
/// This mock uses real `reqwest::RequestBuilder` objects so the production stubs can
/// build requests normally; `execute_*` intercepts them via `RequestBuilder::build()`
/// and dispatches based on the request URL path.
pub struct MockVimClient {
    service_content: ServiceContent,
    http: reqwest::Client,
    base_url: String,

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
            http: reqwest::Client::new(),
            base_url: "http://mock.local".to_string(),
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

    fn url_for(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
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

    fn build_path(req: reqwest::RequestBuilder) -> std::result::Result<(HttpVerb, String), Error> {
        let req = req.build().map_err(Error::ReqwestError)?;
        let verb = match req.method().as_str() {
            "GET" => HttpVerb::Get,
            _ => HttpVerb::Post,
        };
        let path = req.url().path().to_string();
        Ok((verb, path))
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
                return Err(Error::SerdeError(serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Simulated CreateListView failure",
                ))));
            }
            
            let mor = vim_rs::types::structs::ManagedObjectReference {
                r#type: MoTypesEnum::ListView,
                value: "listview-1".to_string(),
            };
            return Ok(Bytes::from(serde_json::to_vec(&mor)?));
        }
        if path.contains("/CreateFilter") && path.starts_with("/PropertyCollector/") {
            self.bump(|c| c.create_filter += 1);
            let mor = vim_rs::types::structs::ManagedObjectReference {
                r#type: MoTypesEnum::PropertyFilter,
                value: "filter-1".to_string(),
            };
            return Ok(Bytes::from(serde_json::to_vec(&mor)?));
        }
        Err(Error::SerdeError(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("MockVimClient: unhandled execute_bytes path: {path}"),
        ))))
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
                    return Err(Error::SerdeError(serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "MockVimClient: WaitForUpdatesEx channel closed",
                    ))))
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

    fn get_request(&self, path: &str) -> reqwest::RequestBuilder {
        self.http.get(self.url_for(path))
    }

    fn post_json(&self, path: &str, payload: &dyn erased_serde::Serialize) -> reqwest::RequestBuilder {
        // Use the production erased-serde adapter to avoid a Value tree here too.
        struct ErasedJson<'a>(&'a dyn erased_serde::Serialize);
        impl serde::Serialize for ErasedJson<'_> {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                erased_serde::serialize(self.0, serializer)
            }
        }
        self.http.post(self.url_for(path)).json(&ErasedJson(payload))
    }

    fn post_bare(&self, path: &str) -> reqwest::RequestBuilder {
        self.http.post(self.url_for(path))
    }

    fn execute_bytes<'a>(&'a self, req: reqwest::RequestBuilder) -> BoxFuture<'a, Result<Bytes>> {
        Box::pin(async move {
            let (verb, path) = Self::build_path(req)?;
            self.handle_bytes(verb, &path).await
        })
    }

    fn execute_option_bytes<'a>(
        &'a self,
        req: reqwest::RequestBuilder,
    ) -> BoxFuture<'a, Result<Option<Bytes>>> {
        Box::pin(async move {
            let (verb, path) = Self::build_path(req)?;
            self.handle_option_bytes(verb, &path).await
        })
    }

    fn execute_void<'a>(&'a self, req: reqwest::RequestBuilder) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            let (verb, path) = Self::build_path(req)?;
            self.handle_void(verb, &path).await
        })
    }
}


