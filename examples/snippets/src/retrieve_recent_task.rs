//! # Recent tasks via custom PropertyCollector traversal (`vim_retrievable!`)
//!
//! Builds a manual [`ObjectSpec`](vim_rs::types::structs::ObjectSpec) / [`TraversalSpec`](vim_rs::types::structs::TraversalSpec) starting at
//! `TaskManager` and traversing `recentTask`, then retrieves lightweight [`Task`](vim_rs::mo::Task) fields through
//! `vim_retrievable!`. This complements convenience helpers by showing the full collector spec shape.
//!
//! ## Note
//!
//! For many scripts you can call [`TaskManager::recent_task`](vim_rs::mo::TaskManager) directly; this sample is aimed at
//! learning PropertyCollector traversal patterns.

use log::info;
use snippets::connect;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::structs::{ObjectSpec, SelectionSpec, TraversalSpec};
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct TaskInfo: Task {
        description_id = "info.description_id",
        fault = "info.error",
    }
);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
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
