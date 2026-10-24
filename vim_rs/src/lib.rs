//! # VMware vSphere API Client for Rust
//!
//! Async client for the VMware vSphere Virtual Infrastructure API. The default
//! transport is VI/JSON. SOAP/XML is optional (`xml` feature) for direct ESXi
//! connectivity.
//!
//! This page is the API reference. The user guide — connecting, TLS, property
//! retrieval, tasks, and the XML transport — is the
//! [crate README](https://github.com/noclue/vim_rs/blob/main/vim_rs/README.md).
//! Design notes are in
//! [`vim_rs/docs`](https://github.com/noclue/vim_rs/tree/main/vim_rs/docs).
//!
//! ## Connecting to vCenter
//!
//! ```rust
//! use vim_rs::core::ClientBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ClientBuilder::new("vcenter.example.com")
//!         .basic_authn("administrator@vsphere.local", "password")
//!         .app_details(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
//!         .insecure(true) // self-signed certificates
//!         .build()
//!         .await?;
//!
//!     client.close().await?;
//!     Ok(())
//! }
//! ```
//!
//! `build` returns `Arc<Client>` with a session and service content loaded.
//! Clone the `Arc` to share it. `.app_details` names the session in the
//! vCenter UI and SessionManager. `.insecure(true)` skips hostname and
//! certificate checks.
//!
//! The **`default-client`** feature (on by default) builds a `reqwest` client
//! that uses **rustls**. To supply your own TLS stack, set
//! `default-features = false` and call `ClientBuilder::new(server, http_client)`.
//!
//! Call [`Client::close`](core::client::Client::close) at shutdown. Destructor
//! logout is a 0.6.x fallback on the multi-thread Tokio runtime and will be
//! removed in 0.7.0.
//!
//! ## Managed objects
//!
//! A stub needs the session, the managed-object type, and an id. Root objects
//! are on service content. `PropertyCollector` is always present.
//!
//! ```rust
//! let content = client.service_content();
//! let property_collector = PropertyCollector::new(client.clone(), &content.property_collector.value);
//! ```
//!
//! Properties and methods are both remote calls:
//!
//! ```rust
//! let events = collector.read_next_events(10).await?;
//! let vms = view.view().await?;
//! ```
//!
//! ## Property retrieval
//!
//! [`vim_retrievable!`] maps property paths onto a struct and loads them in one
//! PropertyCollector call. [`vim_updatable!`] keeps a local replica. Both are
//! exported from this crate. Full samples are
//! in the [crate README](https://github.com/noclue/vim_rs/blob/main/vim_rs/README.md#property-retrieval).
//!
//! ```rust
//! use vim_rs::vim_retrievable;
//! use vim_rs::core::pc_retrieve::ObjectRetriever;
//!
//! vim_retrievable!(
//!     struct Host: HostSystem {
//!         name = "name",
//!         power_state = "runtime.power_state",
//!     }
//! );
//!
//! let retriever = ObjectRetriever::new(client.clone())?;
//! let hosts: Vec<Host> = retriever
//!     .retrieve_objects_from_container(&client.service_content().root_folder)
//!     .await?;
//! ```
//!
//! ## Features
//!
//! | Feature | Default | Effect |
//! |---------|---------|--------|
//! | `default-client` | on | `reqwest` client with rustls. |
//! | `xml` | off | Experimental SOAP/XML. About +500 KB release binary size and +30–40% debug build time. Core VIM only. |
//! | `defaults` | off | `Default` for generated types. Large extra generated source and compile time. |
//! | `vcsim_compat` | off | Tolerant SOAP parsing for govc vcsim. Requires `xml`. |
//!
//! XML does not negotiate the API release via Hello. Use
//! `client.service_content().about.api_version` when server capabilities matter.
//!
//! ## Type system
//!
//! Deep VIM inheritance uses traits and compositional inheritance: a child
//! struct holds its parent and `Deref`s to it. Wide sums (`VimAny`,
//! `ValueElements`) are enums. `MethodFault` and `Event` subtypes are pruned;
//! subtype fields live in `extra_fields_`. See the
//! [type system notes](https://github.com/noclue/vim_rs/blob/main/vim_rs/docs/TYPE_SYSTEM.md).
//!
// Allow proc-macros that generate `vim_rs::...` paths to work within this
// crate as well.
// (Without this, `vim_rs::` resolves only as an external crate path.)
extern crate self as vim_rs;

pub use vim_macros::vim_retrievable;
pub use vim_macros::vim_updatable;

pub mod core;
pub use core::WireLoggingMode;
pub mod types;

pub mod mo;

#[cfg(feature = "xml")]
pub mod xml;