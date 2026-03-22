//! # `disabledMethod` length for a VM (`MO_REF`)
//!
//! Uses a generated managed-object **property getter** (`VirtualMachine::disabled_method`): one property
//! per call, no `PropertyCollector` spec. That is the straightforward way to read a field when you already
//! have a `Type:id` reference. **`VIM_PROTOCOL`** may be `json`, `soap`, or `auto`—getters work over both
//! transports.
//!
//! ## Environment
//!
//! Shared `snippets::connect`: `VIM_SERVER`, `VIM_USERNAME`, `VIM_PASSWORD`, optional `VIM_PROTOCOL`.
//! **`MO_REF`** — managed object as `Type:id` (split on the **first** `:` only). Example: `VirtualMachine:vm-42`.

use anyhow::Context;
use snippets::connect;
use std::env;
use vim_rs::mo::VirtualMachine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let raw = env::var("MO_REF").context("MO_REF e.g. VirtualMachine:vm-42")?;
    let (_t, id) = raw.split_once(':').context("MO_REF must be Type:id")?;
    let client = connect(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")).await?;
    let n = VirtualMachine::new(client, id.trim())
        .disabled_method()
        .await?
        .map(|m| m.len())
        .unwrap_or(0);
    println!("{n}");
    Ok(())
}
