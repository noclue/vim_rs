//! Embedded HTML templates for the web UI

#[cfg(feature = "web-ui")]
pub const INDEX_HTML: &str = include_str!("../../web_ui/templates/index.html");

#[cfg(feature = "web-ui")]
pub const TOOL_FORM_HTML: &str = include_str!("../../web_ui/templates/tool_form.html");

