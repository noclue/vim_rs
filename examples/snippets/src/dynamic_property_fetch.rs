//! # Dynamic Property Fetch Example
//!
//! This example demonstrates how to dynamically fetch arbitrary properties from
//! vSphere managed objects without using the `vim_retrievable!` macro.
//!
//! The `Client::fetch_property()` method allows you to:
//! - Fetch any property by name from any managed object
//! - Deserialize the result into any type that implements `miniserde::Deserialize`
//! - Use `miniserde::json::Value` for dynamic/untyped property access
//!
//! This is useful when:
//! - You need to access properties not commonly used
//! - You want to explore the vSphere object model interactively
//! - You need maximum flexibility in property access
//!
//! In this example:
//! 1. We connect to vSphere using the utils::connect helper
//! 2. We fetch the "permission" property from the root folder
//! 3. We deserialize it as a JSON value for inspection
//!
//! **Note**: For production code with well-defined property sets, prefer using
//! the `vim_retrievable!` macro as shown in other examples, as it provides
//! type safety and better performance for bulk operations.

use log::info;
use utils::connect;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;

    let permissions = client
        .fetch_property::<miniserde::json::Value>(
            client.service_content().root_folder.clone(),
            "permission",
        )
        .await?;
    info!(
        "Root folder permissions: {}",
        miniserde::json::to_string(&permissions)
    );

    Ok(())
}
