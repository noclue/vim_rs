//! Connection settings loaded from one env file, and a logged-in client.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use vim_rs::core::client::{
    Client, ClientBuilder, Transport, TransportMode, VimClient, WireLoggingMode,
};
use vim_rs::mo::PerformanceManager;
use vim_rs::types::enums::MoTypesEnum;
use vim_rs::types::structs::ManagedObjectReference;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportKind {
    Json,
    Soap,
}

impl TransportKind {
    pub fn parse(raw: &str) -> Result<Self> {
        match raw {
            "json" => Ok(Self::Json),
            "soap" => Ok(Self::Soap),
            _ => bail!("transport must be json or soap, got {raw}"),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Soap => "soap",
        }
    }

    fn mode(self) -> TransportMode {
        match self {
            Self::Json => TransportMode::Json,
            Self::Soap => TransportMode::Soap,
        }
    }
}

pub fn transport_name(transport: Transport) -> &'static str {
    match transport {
        Transport::Json => "json",
        Transport::Soap => "soap",
    }
}

#[derive(Debug, Clone)]
pub struct ConnectOptions {
    pub env_file: PathBuf,
    pub target: String,
    pub transport: TransportKind,
    pub wire_log: bool,
}

/// Variables read from the env file. The password is never displayed.
#[derive(Debug, Clone)]
pub struct EnvFile {
    pub server: String,
    pub username: String,
    pub password: String,
    pub insecure: bool,
    pub mo_ref: Option<ManagedObjectReference>,
    pub rust_log: Option<String>,
}

pub fn load_env_file(path: &Path) -> Result<EnvFile> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading env file {}", path.display()))?;
    let mut map = HashMap::new();
    for (line_no, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            bail!("{}:{} is not KEY=VALUE", path.display(), line_no + 1);
        };
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .unwrap_or(value);
        map.insert(key.trim().to_string(), value.to_string());
    }
    let require = |name: &str| -> Result<String> {
        map.get(name)
            .filter(|v| !v.is_empty())
            .cloned()
            .ok_or_else(|| anyhow!("{name} is missing in {}", path.display()))
    };
    let insecure = match map.get("VIM_INSECURE").map(String::as_str) {
        Some("true") | Some("1") | Some("yes") => true,
        Some("false") | Some("0") | Some("no") => false,
        Some(other) => bail!("VIM_INSECURE must be true or false, got {other}"),
        None => false,
    };
    Ok(EnvFile {
        server: require("VIM_SERVER")?,
        username: require("VIM_USERNAME")?,
        password: require("VIM_PASSWORD")?,
        insecure,
        mo_ref: map.get("MO_REF").and_then(|v| parse_mo_ref(v)),
        rust_log: map.get("RUST_LOG").cloned(),
    })
}

/// `VirtualMachine:vm-42` or `VirtualMachine:5`.
pub fn parse_mo_ref(raw: &str) -> Option<ManagedObjectReference> {
    let (ty, value) = raw.split_once(':')?;
    if ty.is_empty() || value.is_empty() {
        return None;
    }
    Some(ManagedObjectReference {
        r#type: MoTypesEnum::from_str(ty),
        value: value.to_string(),
    })
}

pub fn apply_log_env(env: &EnvFile) {
    if let Some(level) = &env.rust_log {
        // The env file is the source of truth for this process.
        std::env::set_var("RUST_LOG", level);
    }
}

pub struct Session {
    pub client: Arc<Client>,
    pub perf: PerformanceManager,
    pub server: String,
    pub target: String,
    pub transport: TransportKind,
    pub mo_ref: Option<ManagedObjectReference>,
}

pub async fn connect(opts: &ConnectOptions) -> Result<Session> {
    let env = load_env_file(&opts.env_file)?;
    let mut builder = ClientBuilder::new(&env.server)
        .basic_authn(&env.username, &env.password)
        .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .insecure(env.insecure)
        .transport(opts.transport.mode());
    if opts.wire_log {
        builder = builder.wire_logging(WireLoggingMode::Detailed);
    }
    let client = builder.build().await.with_context(|| {
        format!(
            "connecting to {} over {}",
            env.server,
            opts.transport.as_str()
        )
    })?;
    let perf_moref = client
        .service_content()
        .perf_manager
        .clone()
        .ok_or_else(|| anyhow!("ServiceContent.perf_manager is absent"))?;
    let perf = PerformanceManager::new(client.clone(), &perf_moref.value);
    let actual = transport_name(client.transport());
    if actual != opts.transport.as_str() {
        log::warn!(
            "requested transport {} but client is using {actual}",
            opts.transport.as_str()
        );
    }
    Ok(Session {
        client,
        perf,
        server: env.server,
        target: opts.target.clone(),
        transport: opts.transport,
        mo_ref: env.mo_ref,
    })
}

pub fn clone_moref(moref: &ManagedObjectReference) -> ManagedObjectReference {
    ManagedObjectReference {
        r#type: moref.r#type.clone(),
        value: moref.value.clone(),
    }
}

pub fn moref_key(moref: &ManagedObjectReference) -> String {
    format!("{}:{}", moref.r#type.as_str(), moref.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_env_and_moref() {
        let dir = std::env::temp_dir().join(format!("perf-catalog-env-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("sample.env");
        std::fs::write(
            &path,
            "\
# comment
VIM_SERVER=vc.example
VIM_USERNAME=user
VIM_PASSWORD=p@ss
VIM_INSECURE=true
MO_REF=VirtualMachine:vm-7
RUST_LOG=debug
",
        )
        .unwrap();
        let env = load_env_file(&path).unwrap();
        assert_eq!(env.server, "vc.example");
        assert!(env.insecure);
        assert_eq!(env.password, "p@ss");
        let moref = env.mo_ref.unwrap();
        assert_eq!(moref.value, "vm-7");
        assert_eq!(moref.r#type.as_str(), "VirtualMachine");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
