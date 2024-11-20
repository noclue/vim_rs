use std::sync::Arc;
use crate::vim_client::{VimClient};
/// This managed object provides interfaces to configure the common message bus
/// proxy service running on the ESX host.
pub struct MessageBusProxy {
    client: Arc<VimClient>,
    mo_id: String,
}
impl MessageBusProxy {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
}
