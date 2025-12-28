use std::sync::Arc;

use bytes::Bytes;
use serde_json::json;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

use vim_rs::core::tasks::TaskTracker;
use vim_rs::core::error::ErrorKind;
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
    match err.kind() {
        ErrorKind::TaskFailed => {
            // Access the underlying MethodFault
            let fault = err.task_fault().expect("TaskFailed should have a fault");
            // Verify we can access the fault details
            assert!(fault.type_.is_some());
        }
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

#[tokio::test]
async fn shutdown_notifies_pending_waiters() {
    let (_pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    // Start waiting on a task that will never complete
    let w1 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-1")).await })
    };

    // Give the background loop time to start
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Request shutdown
    tracker.shutdown().await;

    // The waiter should receive an error
    let err = w1.await.unwrap().unwrap_err();
    match err.kind() {
        ErrorKind::Internal => {
            if !err.to_string().contains("shutdown") {
                panic!("expected shutdown error, got {err:?}");
            }
        },
        other => panic!("expected Internal error kind, got {other:?}"),
    }

    // Verify the loop cleaned up (ListView was destroyed)
    wait_until(Duration::from_secs(2), || {
        let (_clv, _mlv, dlv, _cf, _df, _wfu) = client.counters_snapshot();
        dlv >= 1
    })
    .await;
}

#[tokio::test]
async fn shutdown_and_restart() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    // Start a task and then shutdown before it completes
    let w1 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-1")).await })
    };
    tokio::time::sleep(Duration::from_millis(50)).await;
    tracker.shutdown().await;
    
    // First waiter should get shutdown error
    assert!(w1.await.unwrap().is_err());

    // Wait for cleanup to complete
    wait_until(Duration::from_secs(2), || {
        let (_clv, _mlv, dlv, _cf, _df, _wfu) = client.counters_snapshot();
        dlv >= 1
    })
    .await;

    // Now start a new task - should create fresh resources
    let w2 = {
        let tracker = tracker.clone();
        tokio::spawn(async move { tracker.wait_any(task_mor("task-2")).await })
    };
    
    // Complete task-2
    let us2 = update_set_for_task(
        "filter-1",
        "task-2",
        ObjectUpdateKindEnum::Enter,
        make_task_info("task-2", TaskInfoStateEnum::Success, false, None, None),
    );
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us2).unwrap())))
        .unwrap();
    
    assert!(w2.await.unwrap().unwrap().is_none());
    
    // Verify we created a new ListView (counter should be 2)
    wait_until(Duration::from_secs(2), || {
        let (clv, _mlv, _dlv, _cf, _df, _wfu) = client.counters_snapshot();
        clv >= 2
    })
    .await;
}

#[tokio::test]
async fn recover_from_create_list_view_failure() {
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    // Configure the mock to fail the first CreateListView call
    client.fail_create_list_view_once();

    // First wait_any should fail during ListView creation
    let t1 = task_mor("task-1");
    let result1 = tracker.wait_any(t1).await;
    assert!(result1.is_err(), "First wait_any should fail due to ListView creation failure");

    // Verify no ListView was successfully created (counter should be 1 attempt)
    let (clv_before, _, _, _, _, _) = client.counters_snapshot();
    assert_eq!(clv_before, 1, "Should have attempted CreateListView once");

    // Second wait_any should succeed in creating ListView and starting the background loop
    let t2 = task_mor("task-2");
    let waiter2 = tokio::spawn({
        let tracker = tracker.clone();
        async move { tracker.wait_any(t2).await }
    });

    // Wait a bit to ensure the background loop starts
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify ListView was created successfully (counter should be 2 now)
    let (clv_after, _, _, _, _, _) = client.counters_snapshot();
    assert_eq!(clv_after, 2, "Should have successfully created ListView on second attempt");

    // Send a "no update" event to exercise the background loop
    pc_tx.send(PcEvent::None).unwrap();

    // Complete the task
    let info2 = make_task_info("task-2", TaskInfoStateEnum::Success, false, None, None);
    let us2 = update_set_for_task("filter-1", "task-2", ObjectUpdateKindEnum::Enter, info2);
    pc_tx.send(PcEvent::Bytes(Bytes::from(serde_json::to_vec(&us2).unwrap()))).unwrap();

    // Task should complete successfully
    let result2 = waiter2.await.unwrap();
    assert!(result2.is_ok(), "Second task should complete successfully");
    assert!(result2.unwrap().is_none(), "Task should complete with no result");

    // Wait for the background loop to terminate (it should exit when pending_tasks is empty)
    wait_until(Duration::from_secs(2), || {
        let (_, _, dlv, _, df, _) = client.counters_snapshot();
        dlv >= 1 && df >= 1
    })
    .await;

    // Verify the background loop cleaned up properly
    let (_, _, dlv, _, df, _) = client.counters_snapshot();
    assert_eq!(dlv, 1, "Should have destroyed the ListView");
    assert_eq!(df, 1, "Should have destroyed the PropertyFilter");
}

#[tokio::test]
async fn cancelled_task_completes_successfully() {
    // Verify that a task with cancelled=true but state=Success returns success, not error.
    // This tests the fix for the issue where cancelled flag caused premature eviction.
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let t = task_mor("task-1");
    let waiter = tokio::spawn(async move { tracker.wait_any(t).await });

    // Simulate a task that had cancellation requested but completed successfully anyway.
    // The cancelled flag is true, but the state is Success - this can happen if the task
    // completes before the cancellation is processed.
    let result_mor = ManagedObjectReference {
        r#type: MoTypesEnum::Datastore,
        value: "ds-456".to_string(),
    };
    let info = make_task_info(
        "task-1",
        TaskInfoStateEnum::Success,
        true,  // cancelled=true
        Some(VimAny::Object(Box::new(result_mor.clone()))),
        None,
    );
    let us = update_set_for_task("filter-1", "task-1", ObjectUpdateKindEnum::Enter, info);
    let bytes = Bytes::from(serde_json::to_vec(&us).unwrap());
    pc_tx.send(PcEvent::Bytes(bytes)).unwrap();

    // The waiter should receive success, not a cancellation error
    let res = waiter.await.unwrap().expect("Task should succeed despite cancelled flag");
    assert!(res.is_some(), "Task should return its result");
    
    // Verify the result is correct
    if let Some(VimAny::Object(obj)) = res {
        let mor = obj.as_any_ref().downcast_ref::<ManagedObjectReference>()
            .expect("result should be a ManagedObjectReference");
        assert_eq!(mor.value, "ds-456");
        assert!(matches!(mor.r#type, MoTypesEnum::Datastore));
    } else {
        panic!("expected VimAny::Object result");
    }
}

#[tokio::test]
async fn cancelled_task_with_error_state() {
    // Verify that a task with cancelled=true and state=Error returns task_cancelled error.
    let (pc_tx, pc_rx) = mpsc::unbounded_channel();
    let client = Arc::new(MockVimClient::new(dummy_service_content(), pc_rx));
    let tracker = TaskTracker::new(client.clone());

    let t = task_mor("task-2");
    let waiter = tokio::spawn(async move { tracker.wait_any(t).await });

    // Task enters Error state with cancelled=true - this is a true cancellation
    let fault = vim_rs::types::structs::MethodFault {
        fault_cause: None,
        fault_message: None,
        type_: None,
        extra_fields_: Default::default(),
    };
    let info = make_task_info(
        "task-2",
        TaskInfoStateEnum::Error,
        true,  // cancelled=true
        None,
        Some(fault),
    );
    let us = update_set_for_task("filter-1", "task-2", ObjectUpdateKindEnum::Modify, info);
    let bytes = Bytes::from(serde_json::to_vec(&us).unwrap());
    pc_tx.send(PcEvent::Bytes(bytes)).unwrap();

    // The waiter should receive a task cancelled error
    let err = waiter.await.unwrap().unwrap_err();
    match err.kind() {
        ErrorKind::TaskCancelled => {
            // This is expected - task was cancelled
        }
        other => panic!("expected TaskCancelled, got {other:?}"),
    }
}


