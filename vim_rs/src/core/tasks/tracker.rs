//! Task completion tracking for async vSphere operations.
//!
//! vSphere exposes many operations as `*_Task` methods that return a `Task` managed object
//! reference. The returned task transitions through states (`queued`/`running`) and eventually
//! reaches a terminal state (`success`/`error`/`cancelled`).
//!
//! `TaskTracker` provides a lightweight way to **wait for completion** of a task by:
//! - Maintaining a shared `ListView` of tasks being tracked.
//! - Running a background loop that uses `PropertyCollector::wait_for_updates_ex` (via
//!   `CacheManager`/`Monitor`) to receive incremental task updates.
//! - Completing the caller’s `oneshot` when the task reaches a terminal state, and removing the
//!   task from the view.
//!
//! ## Results and narrowing
//!
//! `TaskInfo.result` in the vSphere API is `Option<VimAny>`:
//! - `None` means “no return value”.
//! - `Some(VimAny::Value(..))` represents primitives / boxed arrays (`ValueElements`).
//! - `Some(VimAny::Object(..))` represents a data object behind `Box<dyn VimObjectTrait>`.
//!
//! This module intentionally exposes the **zero-JSON** API:
//! - [`TaskTracker::wait_any`] → `Result<Option<VimAny>, TaskError>`
//!
//! For convenience, it also provides:
//! - [`TaskTracker::wait`] which uses `serde_json` to decode the result into a user type `T`.
//!   This is helpful for cases like `T = ()`, `T = ManagedObjectReference`, etc., but it is not
//!   a zero-allocation path.
//!
//! ## Memory behavior
//!
//! Tasks can contain large payloads in `info.result` / `info.error`. The cache/listener plumbing
//! is configured so that once a terminal state is observed, the task is **immediately evicted**
//! from the cache (`CacheAction::Evict`) and finalized via `on_remove(TaskUpdate)`.
//!
//! ## Example
//!
//! ```ignore
//! let tracker = TaskTracker::new(client.clone());
//! let task_ref = vm.rename_task("new-name").await?;
//! let result = tracker.wait_any(task_ref).await?;
//! if let Some(any) = result {
//!     match any {
//!         VimAny::Value(v) => println!("primitive/boxed result: {v:?}"),
//!         VimAny::Object(o) => println!("object result type: {:?}", o.data_type()),
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, oneshot, mpsc};
use log::{debug, error};
use serde::de::DeserializeOwned;

use crate::core::client::VimClientHandle;
use crate::mo::{ListView, ViewManager};
use crate::types::structs::ManagedObjectReference;
use crate::types::enums::{TaskInfoStateEnum, MoTypesEnum};
use crate::types::vim_any::VimAny;
use vim_macros::vim_updatable;
use crate::core::pc_cache::{CacheAction, CacheManager, ObjectCache, ObjectCacheListener};

use super::error::TaskError;

#[derive(Clone)]
/// Tracks vSphere `Task` objects to completion using the PropertyCollector.
///
/// Create a `TaskTracker` once per `Client` and reuse it to wait on many `*_Task` operations.
/// Internally, the tracker maintains a `ListView` of in-flight tasks and runs a background loop
/// that applies incremental updates until each task reaches a terminal state.
pub struct TaskTracker {
    client: VimClientHandle,
    state: Arc<RwLock<SharedState>>,
}

impl TaskTracker {
    /// Create a new tracker. The background monitoring loop starts lazily on the first wait call.
    pub fn new(client: VimClientHandle) -> Self {
        Self {
            client,
            state: Arc::new(RwLock::new(SharedState {
                list_view: None,
                list_view_mor: None,
                pending_tasks: HashMap::new(),
                is_running: false,
                shutdown_signal: None,
            })),
        }
    }


    /// Wait for a task and return its result as a `VimAny`. This is the most efficient way to get 
    /// the result of a task with only one conversion from JSON to VimAny.
    ///
    /// - `Ok(None)` means the task succeeded but did not return a value.
    /// - `Ok(Some(VimAny::Value(..)))` is a primitive/boxed-array result.
    /// - `Ok(Some(VimAny::Object(..)))` is a data-object result behind `Box<dyn VimObjectTrait>`.
    pub async fn wait_any(&self, task: ManagedObjectReference) -> Result<Option<VimAny>, TaskError> {
        let (tx, rx) = oneshot::channel();
        let task_id = task.value.clone();

        let list_view = {
            let mut state = self.state.write().await;
            state.pending_tasks.insert(task_id.clone(), tx);

            if state.list_view.is_none() {
                let view_manager = self.client.service_content().view_manager.as_ref()
                    .ok_or_else(|| TaskError::new_other("ViewManager not available".to_string()))?;
                let vm = ViewManager::new(self.client.clone(), &view_manager.value);
                let lv_mor = vm.create_list_view(Some(&[])).await.map_err(TaskError::from)?;
                state.list_view = Some(ListView::new(self.client.clone(), &lv_mor.value));
                state.list_view_mor = Some(lv_mor);
            }

            if !state.is_running {
                let (shutdown_tx, shutdown_rx) = oneshot::channel();
                state.shutdown_signal = Some(shutdown_tx);
                state.is_running = true;

                let tracker = self.clone();
                tokio::spawn(async move {
                    if let Err(e) = tracker.background_loop(shutdown_rx).await {
                        error!("TaskTracker background loop failed: {}", e);
                    }
                    let mut state = tracker.state.write().await;
                    state.is_running = false;
                    state.shutdown_signal = None;
                    if let Some(lv) = state.list_view.take() {
                        let _ = lv.destroy_view().await;
                    }
                    state.list_view_mor = None;
                    for (_, tx) in state.pending_tasks.drain() {
                        let _ = tx.send(Err(TaskError::new_other("TaskTracker loop terminated".to_string())));
                    }
                });
            }

            state.list_view.as_ref().unwrap().clone()
        };

        if let Err(e) = list_view.modify_list_view(Some(&[task.clone()]), Some(&[])).await {
            let mut state = self.state.write().await;
            state.pending_tasks.remove(&task_id);
            return Err(TaskError::from(e));
        }

        match rx.await {
            Ok(res) => res,
            Err(_) => Err(TaskError::new_other("TaskTracker channel closed".to_string())),
        }
    }

    /// Convenience: wait for a task and deserialize its result into `T` using `serde_json`.
    ///
    /// This is useful when you know the expected result type (e.g. `()` for tasks that return no
    /// value), but it is not a zero-allocation path. Prefer [`TaskTracker::wait_any`] if you want
    /// to avoid JSON conversion and handle `VimAny` directly.
    pub async fn wait<T: DeserializeOwned + 'static>(&self, task: ManagedObjectReference) -> Result<T, TaskError> {
        let val_opt = self.wait_value(task).await?;
        
        match val_opt {
            Some(val) => {
                let result: T = serde_json::from_value(val)?;
                Ok(result)
            },
            None => {
                let result: T = serde_json::from_value(serde_json::Value::Null)?;
                Ok(result)
            }
        }
    }

    async fn wait_value(&self, task: ManagedObjectReference) -> Result<Option<serde_json::Value>, TaskError> {
        let any_opt = self.wait_any(task).await?;
        match any_opt {
            None => Ok(None),
            Some(any) => Ok(Some(serde_json::to_value(&any)?)),
        }
    }

    async fn background_loop(&self, mut shutdown_rx: oneshot::Receiver<()>) -> Result<(), TaskError> {
        let (list_view_mor, list_view) = {
            let state = self.state.read().await;
            let mor = state.list_view_mor.as_ref().ok_or_else(|| TaskError::new_other("ListView MOR is None in background loop".to_string()))?.clone();
            let lv = state.list_view.as_ref().unwrap().clone();
            (mor, lv)
        };

        let mut manager = CacheManager::new(self.client.clone()).map_err(TaskError::from)?;
        let mut monitor = manager.create_monitor().map_err(TaskError::from)?;

        let (comp_tx, mut comp_rx) = mpsc::unbounded_channel();
        let listener = TaskListener { tx: comp_tx };
        
        let obj_spec = obj_spec_for_view(list_view_mor);

        let cache = ObjectCache::new_with_listener(Box::new(listener));
        manager.add_cache(Box::new(cache), obj_spec).await.map_err(TaskError::from)?;

        loop {
            let wait_future = monitor.wait_updates(10);
            
            tokio::select! {
                _ = &mut shutdown_rx => {
                    debug!("Shutdown signal received");
                    break;
                }
                res = wait_future => {
                    match res {
                        Ok(Some(updates)) => {
                            if let Err(e) = manager.apply_updates(updates) {
                                error!("Failed to apply updates: {}", e);
                            }
                            while let Ok((task_id, result)) = comp_rx.try_recv() {
                                self.complete_task(&list_view, task_id, result).await;
                            }
                        }
                        Ok(None) => {}
                        Err(e) => return Err(TaskError::from(e)),
                    }
                }
            }
            
            let empty = {
                 let state = self.state.read().await;
                 state.pending_tasks.is_empty()
            };
            if empty {
                 debug!("No pending tasks, exiting background loop");
                 break;
            }
        }
        
        manager.destroy().await.map_err(TaskError::from)?;
        Ok(())
    }

    async fn complete_task(&self, list_view: &ListView, task_id: String, final_result: Result<Option<VimAny>, TaskError>) {

        let tx_opt = {
            let mut state = self.state.write().await;
            state.pending_tasks.remove(&task_id)
        };

        if let Some(tx) = tx_opt {
             let _ = tx.send(final_result);
        }

        let task_mor = ManagedObjectReference { 
            r#type: MoTypesEnum::Other_("Task".to_string()), 
            value: task_id 
        };
        
        if let Err(e) = list_view.modify_list_view(Some(&[]), Some(&[task_mor])).await {
             error!("Failed to remove completed task from ListView: {}", e);
        }
    }
}

// Define TaskUpdate using vim_updatable!
vim_updatable!(
    struct TaskUpdate: Task {
        info = "info",
    }
);

use crate::core::pc_helpers::{obj_spec_for_view};

/// Shared state for the TaskTracker. This is used to store the list view, the list view MOR,
/// the pending tasks, and the shutdown signal.
struct SharedState {
    list_view: Option<ListView>,
    list_view_mor: Option<ManagedObjectReference>,
    /// Pending tasks keyed by task MoID. Each sender is completed exactly once on terminal state.
    pending_tasks: HashMap<String, oneshot::Sender<Result<Option<VimAny>, TaskError>>>,
    is_running: bool,
    shutdown_signal: Option<oneshot::Sender<()>>,
}

/// TaskListener is used to listen for TaskUpdate objects and check if the task is complete.
/// 
/// It is used to send the result of the task to the caller.
struct TaskListener {
    tx: mpsc::UnboundedSender<(String, Result<Option<VimAny>, TaskError>)>,
}

impl ObjectCacheListener<TaskUpdate> for TaskListener {
    fn on_new(&mut self, task: &TaskUpdate) -> CacheAction {
        self.check_task(task)
    }

    fn on_update(&mut self, task: &TaskUpdate) -> CacheAction {
        self.check_task(task)
    }

    fn on_remove(&mut self, task: TaskUpdate) {
        self.finish_task(task);
    }
}

impl TaskListener {
    fn check_task(&self, task: &TaskUpdate) -> CacheAction {
        // If the task reached a terminal state, request immediate eviction.
        // The owned `TaskUpdate` will be delivered to `on_remove`, where we can move
        // the result/error out without any cloning.
        if task.info.cancelled {
            return CacheAction::Evict;
        }
        match task.info.state {
            TaskInfoStateEnum::Success | TaskInfoStateEnum::Error => CacheAction::Evict,
            _ => CacheAction::Keep,
        }
    }

    fn finish_task(&mut self, task: TaskUpdate) {
        // This is invoked both for natural Leave updates and for listener-requested eviction.
        // Only terminal tasks should be evicted by the listener; still, be defensive.
        let task_id = task.id.value.clone();

        let result: Option<Result<Option<VimAny>, TaskError>> = match task.info.state {
            TaskInfoStateEnum::Success => {
                Some(Ok(task.info.result))
            }
            TaskInfoStateEnum::Error => {
                if task.info.cancelled {
                    Some(Err(TaskError::new_cancelled()))
                } else {                    
                    match task.info.error {
                        None => Some(Err(TaskError::new_other(
                            "Task failed but no error detail returned".to_string(),
                        ))),
                        Some(error) => Some(Err(TaskError::new_task_error(error))),
                    }
                }
            }
            _ => {
                if task.info.cancelled {
                    Some(Err(TaskError::new_cancelled()))
                } else {
                    None
                }
            }
        };

        if let Some(r) = result {
            let _ = self.tx.send((task_id, r));
        }
    }
}

