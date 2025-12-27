use std::sync::Arc;

use bytes::Bytes;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

use vim_rs::core::tasks::TaskTracker;
use vim_rs::core::tasks::error::TaskError;
use vim_rs::types::enums::{MoTypesEnum, ObjectUpdateKindEnum, PropertyChangeOpEnum, TaskInfoStateEnum};
use vim_rs::types::structs::{
    ManagedObjectReference, ObjectUpdate, PropertyChange, PropertyFilterUpdate, TaskInfo, UpdateSet,
};
use vim_rs::types::traits::TaskReasonTrait;
use vim_rs::types::structs::TaskReasonUser;
use vim_rs::types::vim_any::VimAny;

mod support {
    pub mod mock_vim_client;
}
use support::mock_vim_client::{MockVimClient, PcEvent};

fn dummy_service_content() -> vim_rs::types::structs::ServiceContent {
    // Construct via JSON so we don't have to manually populate the (very large) struct.
    serde_json::from_value(json!({
        "_typeName": "ServiceContent",
        "rootFolder": {"_typeName":"ManagedObjectReference","type":"Folder","value":"root-1"},
        "propertyCollector": {"_typeName":"ManagedObjectReference","type":"PropertyCollector","value":"pc-1"},
        "viewManager": {"_typeName":"ManagedObjectReference","type":"ViewManager","value":"vmgr-1"},
        "about": {
            "_typeName":"AboutInfo",
            "name":"n",
            "fullName":"f",
            "vendor":"v",
            "version":"1",
            "build":"b",
            "osType":"o",
            "productLineId":"p",
            "apiType":"VirtualCenter",
            "apiVersion":"1"
        }
    }))
    .expect("test ServiceContent should deserialize")
}

fn task_mor(id: &str) -> ManagedObjectReference {
    ManagedObjectReference { r#type: MoTypesEnum::Task, value: id.to_string() }
}

fn make_task_info(task_id: &str, state: TaskInfoStateEnum, cancelled: bool, result: Option<VimAny>, error: Option<vim_rs::types::structs::MethodFault>) -> TaskInfo {
    let reason: Box<dyn TaskReasonTrait> = Box::new(TaskReasonUser { user_name: "u".to_string() });
    TaskInfo {
        key: "k".to_string(),
        task: task_mor(task_id),
        description: None,
        name: None,
        description_id: "d".to_string(),
        entity: None,
        entity_name: None,
        locked: None,
        state,
        cancelled,
        cancelable: true,
        error,
        result,
        progress: None,
        progress_details: None,
        reason,
        queue_time: "t".to_string(),
        start_time: None,
        complete_time: None,
        event_chain_id: 0,
        change_tag: None,
        parent_task_key: None,
        root_task_key: None,
        activation_id: None,
    }
}

fn update_set_for_task(filter_id: &str, task_id: &str, kind: ObjectUpdateKindEnum, info: TaskInfo) -> UpdateSet {
    let pc = PropertyChange {
        name: "info".to_string(),
        op: PropertyChangeOpEnum::Assign,
        val: Some(VimAny::Object(Box::new(info))),
    };

    let ou = ObjectUpdate {
        kind,
        obj: task_mor(task_id),
        change_set: Some(vec![pc]),
        missing_set: None,
    };

    let pfu = PropertyFilterUpdate {
        filter: ManagedObjectReference { r#type: MoTypesEnum::PropertyFilter, value: filter_id.to_string() },
        object_set: Some(vec![ou]),
        missing_set: None,
    };

    UpdateSet { version: "v1".to_string(), filter_set: Some(vec![pfu]), truncated: None }
}

fn update_set_for_tasks(filter_id: &str, updates: Vec<(ObjectUpdateKindEnum, &str, TaskInfo)>) -> UpdateSet {
    let object_set = updates
        .into_iter()
        .map(|(kind, task_id, info)| {
            let pc = PropertyChange {
                name: "info".to_string(),
                op: PropertyChangeOpEnum::Assign,
                val: Some(VimAny::Object(Box::new(info))),
            };
            ObjectUpdate {
                kind,
                obj: task_mor(task_id),
                change_set: Some(vec![pc]),
                missing_set: None,
            }
        })
        .collect();

    let pfu = PropertyFilterUpdate {
        filter: ManagedObjectReference {
            r#type: MoTypesEnum::PropertyFilter,
            value: filter_id.to_string(),
        },
        object_set: Some(object_set),
        missing_set: None,
    };

    UpdateSet {
        version: "v1".to_string(),
        filter_set: Some(vec![pfu]),
        truncated: None,
    }
}

async fn wait_until<F: Fn() -> bool>(deadline: Duration, f: F) {
    let _ = timeout(deadline, async move {
        loop {
            if f() {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("condition not met before timeout");
}

#[tokio::test]
async fn success_single_task() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let t = task_mor("task-1");
    let waiter = tokio::spawn(async move { tracker.wait_any(t).await });

    // Exercise the "no updates yet" path too.
    pc_tx.send(PcEvent::None).unwrap();

    let info = make_task_info("task-1", TaskInfoStateEnum::Success, false, None, None);
    let us = update_set_for_task("filter-1", "task-1", ObjectUpdateKindEnum::Enter, info);
    let bytes = Bytes::from(serde_json::to_vec(&us).unwrap());
    pc_tx.send(PcEvent::Bytes(bytes)).unwrap();

    let res = waiter.await.unwrap().unwrap();
    assert!(res.is_none());
}

#[tokio::test]
async fn success_task_with_result() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let t = task_mor("task-1");
    let waiter = tokio::spawn(async move { tracker.wait_any(t).await });

    // Task completes successfully with a result (e.g., a created VM reference).
    let result_mor = ManagedObjectReference {
        r#type: MoTypesEnum::VirtualMachine,
        value: "vm-123".to_string(),
    };
    let info = make_task_info(
        "task-1",
        TaskInfoStateEnum::Success,
        false,
        Some(VimAny::Object(Box::new(result_mor.clone()))),
        None,
    );
    let us = update_set_for_task("filter-1", "task-1", ObjectUpdateKindEnum::Enter, info);
    let bytes = Bytes::from(serde_json::to_vec(&us).unwrap());
    pc_tx.send(PcEvent::Bytes(bytes)).unwrap();

    let res = waiter.await.unwrap().unwrap();
    assert!(res.is_some());
    
    // Verify the result contains the expected MOR.
    if let Some(VimAny::Object(obj)) = res {
        let mor = obj.as_any_ref().downcast_ref::<ManagedObjectReference>()
            .expect("result should be a ManagedObjectReference");
        assert_eq!(mor.value, "vm-123");
        assert!(matches!(mor.r#type, MoTypesEnum::VirtualMachine));
    } else {
        panic!("expected VimAny::Object result");
    }
}

#[tokio::test]
async fn error_single_task() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let t = task_mor("task-1");
    let waiter = tokio::spawn(async move { tracker.wait_any(t).await });

    let fault = vim_rs::types::structs::MethodFault {
        fault_cause: None,
        fault_message: None,
        type_: None,
        extra_fields_: Default::default(),
    };
    let info = make_task_info("task-1", TaskInfoStateEnum::Error, false, None, Some(fault));
    let us = update_set_for_task("filter-1", "task-1", ObjectUpdateKindEnum::Modify, info);
    let bytes = Bytes::from(serde_json::to_vec(&us).unwrap());
    pc_tx.send(PcEvent::Bytes(bytes)).unwrap();

    let err = waiter.await.unwrap().unwrap_err();
    match err {
        TaskError::TaskFailed(_) => {}
        other => panic!("expected TaskFailed, got {other:?}"),
    }
}

#[tokio::test]
async fn comm_error_terminates_loop_and_fails_waiters() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let t = task_mor("task-1");
    let waiter = tokio::spawn(async move { tracker.wait_any(t).await });

    pc_tx.send(PcEvent::Err(vim_rs::core::client::Error::MissingOrInvalidSessionKey)).unwrap();

    let err = waiter.await.unwrap().unwrap_err();
    // Any error variant is acceptable here; the key is that the waiter completes with error.
    let _ = err;
}

#[tokio::test]
async fn multiple_tasks_interleaved() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let w1 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-1")).await })
    };
    let w2 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-2")).await })
    };

    let us = update_set_for_tasks(
        "filter-1",
        vec![
            (ObjectUpdateKindEnum::Enter, "task-1", make_task_info("task-1", TaskInfoStateEnum::Success, false, None, None)),
            (ObjectUpdateKindEnum::Enter, "task-2", make_task_info("task-2", TaskInfoStateEnum::Success, false, None, None)),
        ],
    );
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us).unwrap())))
        .unwrap();

    assert!(w1.await.unwrap().unwrap().is_none());
    assert!(w2.await.unwrap().unwrap().is_none());
}

#[tokio::test]
async fn loop_stops_when_drained_and_restarts() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    // First task completes -> loop should stop and destroy the ListView.
    let w1 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-1")).await })
    };
    let us1 = update_set_for_task(
        "filter-1",
        "task-1",
        ObjectUpdateKindEnum::Enter,
        make_task_info("task-1", TaskInfoStateEnum::Success, false, None, None),
    );
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us1).unwrap())))
        .unwrap();
    assert!(w1.await.unwrap().unwrap().is_none());

    // Wait until the background loop teardown destroys the ListView.
    wait_until(Duration::from_secs(2), || {
        let (_clv, _mlv, dlv, _cf, _df, _wfu) = client.counters_snapshot();
        dlv >= 1
    })
    .await;

    // Second task should restart loop and create a new ListView.
    let w2 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-2")).await })
    };
    let us2 = update_set_for_task(
        "filter-1",
        "task-2",
        ObjectUpdateKindEnum::Enter,
        make_task_info("task-2", TaskInfoStateEnum::Success, false, None, None),
    );
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us2).unwrap())))
        .unwrap();
    assert!(w2.await.unwrap().unwrap().is_none());

    wait_until(Duration::from_secs(2), || {
        let (clv, _mlv, _dlv, _cf, _df, _wfu) = client.counters_snapshot();
        clv >= 2
    })
    .await;

    // Sanity-check we hit the expected endpoints at least once.
    let reqs = client.requests();
    assert!(reqs
        .iter()
        .any(|r| matches!(r.verb, support::mock_vim_client::HttpVerb::Post) && r.path.contains("/CreateListView")));
    assert!(reqs.iter().any(|r| r.path.contains("/WaitForUpdatesEx")));
}

#[tokio::test]
async fn race_add_task_during_drain_does_not_drop_loop() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    // Block the second ModifyListView call (the removal for task-1 completion).
    let gate = Arc::new(tokio::sync::Notify::new());
    client.block_modify_list_view_call(2, gate.clone());

    let w1 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-1")).await })
    };

    // Complete task-1; background loop will attempt removal and block in the mock.
    let us1 = update_set_for_task(
        "filter-1",
        "task-1",
        ObjectUpdateKindEnum::Modify,
        make_task_info("task-1", TaskInfoStateEnum::Success, false, None, None),
    );
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us1).unwrap())))
        .unwrap();

    // Wait until the remove call is in-flight (ModifyListView count reaches 2).
    wait_until(Duration::from_secs(2), || {
        let (_clv, mlv, _dlv, _cf, _df, _wfu) = client.counters_snapshot();
        mlv >= 2
    })
    .await;

    // Enqueue task-2 while the background loop is blocked mid-completion of task-1.
    let w2 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-2")).await })
    };

    // Unblock the removal call.
    gate.notify_one();

    // Now complete task-2.
    let us2 = update_set_for_task(
        "filter-1",
        "task-2",
        ObjectUpdateKindEnum::Modify,
        make_task_info("task-2", TaskInfoStateEnum::Success, false, None, None),
    );
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us2).unwrap())))
        .unwrap();

    assert!(w1.await.unwrap().unwrap().is_none());
    assert!(w2.await.unwrap().unwrap().is_none());
}


