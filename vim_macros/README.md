# vim_macros

Procedural macros for simplified VMware vSphere property retrieval and monitoring.

## Property Retrieval with Macros

The library provides two powerful macros to simplify property retrieval and monitoring:

### One-time Property Retrieval with `vim_retrievable`

The `vim_retrievable` macro creates structs for efficient, one-time property retrieval:

```rust,ignore
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;

// Define a struct mapping to HostSystem properties
vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        power_state = "runtime.power_state",
        connected = "runtime.connection_state",
        cpu_usage = "summary.quick_stats.overall_cpu_usage",
        memory_usage = "summary.quick_stats.overall_memory_usage",
        uptime = "summary.quick_stats.uptime",
    }
);

async fn print_hosts(client: &Client) -> Result<()> {
   // Create a retriever using the client
   let retriever = ObjectRetriever::new(client.clone())?;

   // Retrieve all hosts with their properties in a single API call
   let hosts: Vec<HostInfo> = retriever
           .retrieve_objects_from_container(&client.service_content().root_folder)
           .await?;

   // Work with strongly-typed host objects
   for host in hosts {
      println!("Host {} is {:?}", host.name, host.power_state);
   }

   Ok(())
}
```

### Continuous Property Monitoring with `vim_updatable`

The `vim_updatable` macro creates structs for continuous property monitoring:

```rust,ignore
vim_updatable!(
    struct VmDetails: VirtualMachine {
        name = "name",
        power_state = "runtime.power_state",
    }
);

impl Display for VmDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VM ({}): {} with power state: {:?}", self.id.value, self.name, self.power_state
        )
    }
}

struct ChangeListener {}

impl ObjectCacheListener<VmDetails> for ChangeListener {
    fn on_new(&mut self, obj: &VmDetails) {
        info!("New VM: {}", obj);
    }

    fn on_update(&mut self, obj: &VmDetails) {
        info!("VM updated: {}", obj);
    }

    fn on_remove(&mut self, obj: VmDetails) {
        info!("VM removed: {}", obj);
    }
}

async fn monitor_vms(client: &Arc<Client>) -> Result<(), Error> {
    let cache = Box::new(ObjectCache::new_with_listener(Box::new(ChangeListener {})));
    let mut manager = CacheManager::new(client.clone())?;
    let mut monitor = manager.create_monitor()?;

    manager.add_container_cache(cache, &client.service_content().root_folder).await?;

    let start = Instant::now();
    loop {
        let updates = monitor.wait_updates(10).await?;
        if let Some(updates) = updates {
            manager.apply_updates(updates)?;
        }
        if start.elapsed().as_secs() > 60 {
            break;
        }
    }

    manager.destroy().await?;
    Ok(())
}
```

### How the Macros Work

Both macros:

1. Generate a struct based on the data structure defined in the macro, corresponding to a vSphere managed object type (VirtualMachine, HostSystem, etc.)
2. Elicit the types of struct fields from the property paths in the vSphere API
3. Handle type conversion between vSphere dynamic types and Rust types

The `vim_rs::core::pc_retrieve` module supports one-time property retrieval,
while `vim_rs::core::pc_cache` provides infrastructure for continuous property monitoring.

### Macro Syntax

```rust,ignore
vim_retrievable!(
    struct StructName: ManagedObjectType {
        field_name = "property.path",
        another_field = "another.property.path"
    }
);
```

The same syntax applies to the `vim_updatable!` macro.