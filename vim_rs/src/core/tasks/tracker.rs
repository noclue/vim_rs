use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;
use tokio::sync::{RwLock, oneshot, mpsc};
use log::{debug, error};
use serde::de::DeserializeOwned;

use crate::core::client::Client;
use crate::mo::{ListView, ViewManager};
use crate::types::structs::ManagedObjectReference;
use crate::types::enums::{TaskInfoStateEnum, MoTypesEnum};
use vim_macros::vim_updatable;
use crate::core::pc_cache::{CacheManager, ObjectCache, ObjectCacheListener};

use super::error::TaskError;

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
    pending_tasks: HashMap<String, oneshot::Sender<Result<Option<serde_json::Value>, TaskError>>>,
    is_running: bool,
    shutdown_signal: Option<oneshot::Sender<()>>,
}

/// TaskListener is used to listen for TaskUpdate objects and check if the task is complete.
/// 
/// It is used to send the result of the task to the caller.
struct TaskListener {
    tx: mpsc::UnboundedSender<(String, Result<Option<serde_json::Value>, TaskError>)>,
}

impl ObjectCacheListener<TaskUpdate> for TaskListener {
    fn on_new(&mut self, task: &TaskUpdate) {
        self.check_task(task);
    }

    fn on_update(&mut self, task: &TaskUpdate) {
        self.check_task(task);
    }

    fn on_remove(&mut self, _task: TaskUpdate) {
        // Do nothing
    }
}

/// Clones an object by serializing and deserializing it
fn clone_object<T: Serialize + DeserializeOwned>(obj: &T) -> Result<T, TaskError> {
    let val = serde_json::to_value(obj)?;
    let cloned = serde_json::from_value(val)?;
    Ok(cloned)
}

impl TaskListener {
    fn check_task(&self, task: &TaskUpdate) {
        let result: Option<Result<Option<serde_json::Value>, TaskError>> = match task.info.state {
            TaskInfoStateEnum::Success => {
                // Task Success
                if let Ok(res) = serde_json::to_value(&task.info.result) {
                    Some(Ok(Some(res)))
                } else {
                    Some(Err(TaskError::new_other("Failed to clone TaskInfo result".to_string())))
                }
            }
            TaskInfoStateEnum::Error => {
                // Task Error
                if let Ok(fault) = clone_object(&task.info.error) {
                    if let Some(fault) = fault {
                        Some(Err(TaskError::new_task_error(fault)))
                    } else {
                        Some(Err(TaskError::new_other("Task failed but no error detail returned".to_string())))
                    }
                } else {
                    Some(Err(TaskError::new_other("Failed to clone MethodFault".to_string())))
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
            let _ = self.tx.send((task.id.value.clone(), r));
        }
    }
}

#[derive(Clone)]
pub struct TaskTracker {
    client: Arc<Client>,
    state: Arc<RwLock<SharedState>>,
}

impl TaskTracker {
    pub fn new(client: Arc<Client>) -> Self {
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

    pub async fn wait_value(&self, task: ManagedObjectReference) -> Result<Option<serde_json::Value>, TaskError> {
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

    async fn complete_task(&self, list_view: &ListView, task_id: String, final_result: Result<Option<serde_json::Value>, TaskError>) {

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
