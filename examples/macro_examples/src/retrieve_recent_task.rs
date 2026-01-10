//! # Retrieve Recent Tasks Example
//!
//! This example demonstrates advanced PropertyCollector usage with custom traversal
//! specifications to retrieve recent tasks from the vSphere TaskManager.
//!
//! ## Key Concepts
//!
//! **TaskManager**: A vSphere service object that tracks all tasks (operations) in
//! the system. The `recentTask` property contains recently completed or running tasks.
//!
//! **TraversalSpec**: A PropertyCollector feature that allows following object
//! relationships. Instead of just retrieving properties of a single object, you can
//! traverse to related objects. In this case, we traverse from TaskManager to its
//! recentTask array.
//!
//! **Custom ObjectSpec**: While `retrieve_objects_from_container()` is convenient,
//! this example shows how to manually construct PropertyCollector specifications for
//! advanced scenarios. This gives you full control over:
//! - Which objects to start from
//! - Which relationships to traverse
//! - How deep to traverse
//!
//! **vim_retrievable! with Task**: Tasks are first-class managed objects in vSphere.
//! We define a minimal struct to capture just the description and error information.
//!
//! ## How It Works
//!
//! 1. Get the TaskManager ManagedObjectReference from ServiceContent
//! 2. Create a TraversalSpec to traverse the "recentTask" property
//! 3. Wrap it in an ObjectSpec specifying the TaskManager as starting point
//! 4. Use ObjectRetriever with custom specs to fetch all recent tasks
//! 5. Display task information (description, errors if any)
//!
//! ## TraversalSpec Explained
//!
//! ```rust
//! TraversalSpec {
//!     name: "expandProperty",           // Arbitrary name for this traversal
//!     type: "TaskManager",              // Starting object type
//!     path: "recentTask",               // Property to traverse
//!     skip: false,                      // Include TaskManager in results
//!     select_set: None,                 // No nested traversals
//! }
//! ```
//!
//! ## Use Cases
//!
//! - Auditing recent vSphere operations
//! - Monitoring task failures and errors
//! - Building task history reports
//! - Learning advanced PropertyCollector patterns for complex queries
//!
//! ## Alternative Approach
//!
//! For simpler use cases, you can get recent tasks directly:
//! ```rust
//! let task_manager = TaskManager::new(client, &task_manager_ref.value);
//! let recent_tasks = task_manager.recent_task().await?;
//! ```
//!
//! This example demonstrates the PropertyCollector approach for educational purposes.

use log::info;
use std::env;
use utils::connect;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::structs::{ObjectSpec, SelectionSpec, TraversalSpec};

vim_retrievable!(
    struct TaskInfo: Task {
        description_id = "info.description_id",
        fault = "info.error",
    }
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let task_manager_ref = client.service_content().task_manager.clone().unwrap();

    let object_specs = vec![ObjectSpec {
        obj: task_manager_ref.clone(),
        skip: Some(false),
        select_set: Some(vec![Box::new(TraversalSpec {
            selection_spec_: SelectionSpec {
                name: Some("expandProperty".to_string()),
            },
            r#type: task_manager_ref.r#type.as_str().to_string(),
            path: "recentTask".to_string(),
            skip: Some(false),
            select_set: None,
        })]),
    }];

    let retriever = ObjectRetriever::new(client.clone())?;
    let tasks: Vec<TaskInfo> = retriever.retrieve_objects(object_specs).await?;

    for task in tasks {
        info!("Task: {:?}", task);
    }

    Ok(())
}
