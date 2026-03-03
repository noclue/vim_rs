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
//! - [`TaskTracker::wait_any`] → `Result<Option<VimAny>>`
//!
//! For convenience, it also provides:
//! - [`TaskTracker::wait`] which uses `miniserde` to decode the result into a user type `T`.
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
use crate::types::mini_helpers::from_value;

use crate::core::client::VimClientHandle;
use crate::core::error::{Error, Result};
use crate::mo::{ListView, ViewManager};
use crate::types::structs::ManagedObjectReference;
use crate::types::enums::{TaskInfoStateEnum, MoTypesEnum};
use crate::types::vim_any::VimAny;
use vim_macros::vim_updatable;
use crate::core::pc_cache::{CacheAction, CacheManager, ObjectCache, ObjectCacheListener};

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

    /// Request graceful shutdown of the background monitoring loop.
    ///
    /// This will initiate asynchronous cleanup of the background loop:
    /// - Stop the PropertyCollector monitoring
    /// - Destroy the ListView
    /// - Notify all pending waiters with an error
    /// - Clean up all resources
    ///
    /// After shutdown, new `wait_any` calls will start a fresh background loop.
    /// If no background loop is running, this is a no-op.
    pub async fn shutdown(&self) {
        let shutdown_tx = {
            let mut state = self.state.write().await;
            state.shutdown_signal.take()
        };
        if let Some(tx) = shutdown_tx {
            let _ = tx.send(());
            // Note: The actual cleanup happens in background_loop's shutdown path
        }
    }


    /// Wait for a task and return its result as a `VimAny`. This is the most efficient way to get 
    /// the result of a task with only one conversion from JSON to VimAny.
    ///
    /// - `Ok(None)` means the task succeeded but did not return a value.
    /// - `Ok(Some(VimAny::Value(..)))` is a primitive/boxed-array result.
    /// - `Ok(Some(VimAny::Object(..)))` is a data-object result behind `Box<dyn VimObjectTrait>`.
    pub async fn wait_any(&self, task: ManagedObjectReference) -> Result<Option<VimAny>> {
        let (tx, rx) = oneshot::channel();
        let task_id = task.value.clone();

        let list_view = {
            let mut state = self.state.write().await;

            if state.list_view.is_none() {
                let view_manager = self.client.service_content().view_manager.as_ref()
                    .ok_or_else(|| Error::internal("ViewManager not available".to_string()))?;
                let vm = ViewManager::new(self.client.clone(), &view_manager.value);
                let lv_mor = vm.create_list_view(Some(&[])).await?;
                state.list_view = Some(ListView::new(self.client.clone(), &lv_mor.value));
                state.list_view_mor = Some(lv_mor);
            }

            // Insert the task into the pending tasks map, so background loop does not exit 
            // prematurely. We may need to start background loop and create list view before we can
            // add the task to the list view.
            // Do ONLY after the list view is created, so we do not leak the map entry in case of
            // error
            state.pending_tasks.insert(task_id.clone(), tx);

            if !state.is_running {
                let (shutdown_tx, shutdown_rx) = oneshot::channel();
                state.shutdown_signal = Some(shutdown_tx);
                state.is_running = true;

                let tracker = self.clone();
                tokio::spawn(async move {
                    if let Err(e) = tracker.background_loop(shutdown_rx).await {
                        error!("TaskTracker background loop failed: {}", e);
                        // Error case: clean up state and notify pending tasks
                        let (list_view_to_destroy, pending_to_notify) = {
                            let mut state = tracker.state.write().await;
                            state.is_running = false;
                            state.shutdown_signal = None;
                            let lv = state.list_view.take();
                            state.list_view_mor = None;
                            let pending: Vec<_> = state.pending_tasks.drain().map(|(_, tx)| tx).collect();
                            (lv, pending)
                        };
                        // Notify pending tasks outside the lock to avoid deadlock
                        for tx in pending_to_notify {
                            let _ = tx.send(Err(Error::internal("TaskTracker loop terminated.".to_string())));
                        }
                        // Destroy the list view outside the lock
                        if let Some(lv) = list_view_to_destroy {
                            let _ = lv.destroy_view().await;
                        }
                    }
                    // For successful exit, cleanup was done atomically in background_loop
                });
            }

            state.list_view.as_ref().unwrap().clone()
        };

        if let Err(e) = list_view.modify_list_view(Some(&[task.clone()]), Some(&[])).await {
            let mut state = self.state.write().await;
            state.pending_tasks.remove(&task_id);
            return Err(e.into());
        }

        match rx.await {
            Ok(res) => res,
            Err(_) => Err(Error::internal("TaskTracker channel closed".to_string())),
        }
    }

    /// Convenience: wait for a task and deserialize its result into `T` via `miniserde`.
    ///
    /// This is useful when you know the expected result type (e.g. `()` for tasks that return no
    /// value), but it is not a zero-allocation path. Prefer [`TaskTracker::wait_any`] if you want
    /// to avoid JSON conversion and handle `VimAny` directly.
    pub async fn wait<T: miniserde::Deserialize + 'static>(&self, task: ManagedObjectReference) -> Result<T> {
        let val_opt = self.wait_value(task).await?;

        match val_opt {
            Some(val) => {
                let result: T = from_value(&val)?;
                Ok(result)
            },
            None => {
                let result: T = miniserde::json::from_str("null")
                    .map_err(|_| Error::internal("Failed to deserialize null into target type".to_string()))?;
                Ok(result)
            }
        }
    }

    async fn wait_value(&self, task: ManagedObjectReference) -> Result<Option<miniserde::json::Value>> {
        let any_opt = self.wait_any(task).await?;
        match any_opt {
            None => Ok(None),
            Some(any) => {
                let json_str = miniserde::json::to_string(&any);
                let val: miniserde::json::Value = miniserde::json::from_str(&json_str)
                    .map_err(|_| Error::internal("Failed to roundtrip VimAny to json::Value".to_string()))?;
                Ok(Some(val))
            }
        }
    }

    async fn background_loop(&self, mut shutdown_rx: oneshot::Receiver<()>) -> Result<()> {
        let (list_view_mor, list_view) = {
            let state = self.state.read().await;
            let mor = state.list_view_mor.as_ref().ok_or_else(|| Error::internal("ListView MOR is None in background loop".to_string()))?.clone();
            let lv = state.list_view.as_ref().unwrap().clone();
            (mor, lv)
        };

        let mut manager = CacheManager::new(self.client.clone())?;
        let mut monitor = manager.create_monitor()?;

        let (comp_tx, mut comp_rx) = mpsc::unbounded_channel();
        let listener = TaskListener { tx: comp_tx };
        
        let obj_spec = obj_spec_for_view(list_view_mor);

        let cache = ObjectCache::new_with_listener(Box::new(listener));
        manager.add_cache(Box::new(cache), obj_spec).await?;

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
                        Err(e) => return Err(e),
                    }
                }
            }
            
            // Atomically check if empty AND mark as not running to prevent race with new wait_any calls.
            // This ensures that any new wait_any arriving during shutdown will see is_running=false
            // and spawn a fresh loop instead of adding to a dying one.
            let (should_exit, list_view_to_destroy) = {
                let mut state = self.state.write().await;
                if state.pending_tasks.is_empty() {
                    state.is_running = false;
                    state.shutdown_signal = None;
                    // Take the list_view so new wait_any calls create fresh resources
                    let lv = state.list_view.take();
                    state.list_view_mor = None;
                    (true, lv)
                } else {
                    (false, None)
                }
            };
            if should_exit {
                debug!("No pending tasks, exiting background loop");
                // Destroy the manager first (cancels the PropertyCollector filter)
                manager.destroy().await?;
                // Destroy the list view outside the lock
                if let Some(lv) = list_view_to_destroy {
                    let _ = lv.destroy_view().await;
                }
                return Ok(());
            }
        }
        
        // This path is reached only on shutdown signal (not empty exit).
        // Do cleanup atomically: extract pending tasks and list view, then notify waiters outside the lock.
        debug!("Shutdown signal path: cleaning up");
        let (list_view_to_destroy, pending_to_notify) = {
            let mut state = self.state.write().await;
            state.is_running = false;
            state.shutdown_signal = None;
            let lv = state.list_view.take();
            state.list_view_mor = None;
            let pending: Vec<_> = state.pending_tasks.drain().map(|(_, tx)| tx).collect();
            (lv, pending)
        };
        
        // Notify all pending tasks that we're shutting down BEFORE destroying the manager.
        // This ensures waiters receive the proper shutdown error even if manager.destroy() fails.
        for tx in pending_to_notify {
            let _ = tx.send(Err(Error::internal("TaskTracker shutdown requested".to_string())));
        }
        
        // Destroy manager (cancels PropertyCollector filter)
        manager.destroy().await?;
        
        // Destroy list view last
        if let Some(lv) = list_view_to_destroy {
            let _ = lv.destroy_view().await;
        }
        
        Ok(())
    }

    async fn complete_task(&self, list_view: &ListView, task_id: String, final_result: Result<Option<VimAny>>) {

        let tx_opt = {
            let mut state = self.state.write().await;
            state.pending_tasks.remove(&task_id)
        };

        if let Some(tx) = tx_opt {
             let _ = tx.send(final_result);
        }

        let task_mor = ManagedObjectReference { 
            r#type: MoTypesEnum::Task, 
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
    pending_tasks: HashMap<String, oneshot::Sender<Result<Option<VimAny>>>>,
    is_running: bool,
    shutdown_signal: Option<oneshot::Sender<()>>,
}

/// TaskListener is used to listen for TaskUpdate objects and check if the task is complete.
/// 
/// It is used to send the result of the task to the caller.
struct TaskListener {
    tx: mpsc::UnboundedSender<(String, Result<Option<VimAny>>)>,
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
        //
        // Note: We only evict on terminal states (Success/Error), not on the `cancelled` flag.
        // The `cancelled` flag indicates cancellation was *requested*, not that it has completed.
        // A task may have cancelled=true but still reach Success if it completes before the
        // cancellation is processed. Evicting early would incorrectly report task_cancelled
        // when the task actually succeeded.
        match task.info.state {
            TaskInfoStateEnum::Success | TaskInfoStateEnum::Error => CacheAction::Evict,
            _ => CacheAction::Keep,
        }
    }

    fn finish_task(&mut self, task: TaskUpdate) {
        // This is invoked both for natural Leave updates and for listener-requested eviction.
        // Only terminal tasks should be evicted by the listener; still, be defensive.
        let task_id = task.id.value.clone();

        let result: Option<Result<Option<VimAny>>> = match task.info.state {
            TaskInfoStateEnum::Success => {
                Some(Ok(task.info.result))
            }
            TaskInfoStateEnum::Error => {
                if task.info.cancelled {
                    Some(Err(Error::task_cancelled()))
                } else {                    
                    match task.info.error {
                        None => Some(Err(Error::internal(
                            "Task failed but no error detail returned".to_string(),
                        ))),
                        Some(error) => Some(Err(Error::task_failed(error))),
                    }
                }
            }
            _ => {
                if task.info.cancelled {
                    Some(Err(Error::task_cancelled()))
                } else {
                    error!("Task {} removed from cache in unexpected state {:?}", task_id, task.info.state);
                    Some(Err(Error::internal(format!(
                        "Task removed in unexpected state: {:?}", task.info.state
                    ))))
                }
            }
        };

        if let Some(r) = result {
            let _ = self.tx.send((task_id, r));
        }
    }
}

