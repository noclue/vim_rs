use std::env;
use log::info;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use vim_rs::types::structs::{ObjectSpec, TraversalSpec};
use utils::connect;

vim_retrievable!(
    struct TaskInfo: Task {
        description_id = "info.description_id",
        fault = "info.error",
    }
);

type StaticStr = &'static str;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let task_manager_ref = client.service_content().task_manager.clone().unwrap();

    let object_specs = vec![
        ObjectSpec {
            obj: task_manager_ref.clone(),
            skip: Some(false),
            select_set: Some(vec![Box::new(TraversalSpec {
                name: Some("expandProperty".to_string()),
                r#type: StaticStr::from(task_manager_ref.r#type).to_string(),
                path: "recentTask".to_string(),
                skip: Some(false),
                select_set: None,
            })])
        }
    ];

    let retriever = ObjectRetriever::new(client.clone())?;
    let tasks: Vec<TaskInfo> = retriever
        .retrieve_objects(object_specs)
        .await?;


    for task in tasks {
        info!("Task: {:?}", task);
    }

    Ok(())
}