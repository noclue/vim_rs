//! Print `disabledMethod` array length for the VM in **`MO_REF`** (`Type:id`). Use **`VIM_PROTOCOL=soap`**
//! with `VIM_SERVER` / `VIM_USERNAME` / `VIM_PASSWORD` for XML-only hosts.

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
