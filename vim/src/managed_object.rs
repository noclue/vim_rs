use std::sync::Arc;
use crate::vim_client::{VimClient};
/// This is the built-in base interface implemented by all
/// managed objects.
pub struct ManagedObject {
    client: Arc<VimClient>,
    mo_id: String,
}
impl ManagedObject {
    pub fn new(client: Arc<VimClient>, mo_id: &str) -> Self {
        Self {
            client,
            mo_id: mo_id.to_string(),
        }
    }
}
