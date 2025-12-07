//! Embedded static assets for the web UI

#[cfg(feature = "web-ui")]
pub const STYLE_CSS: &str = include_str!("../../web_ui/static/style.css");

