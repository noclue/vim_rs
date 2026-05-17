//! Dedicated wire logging for `vim_rs::wire::json` and `vim_rs::wire::soap` targets.
//!
//! Used by [`crate::core::client::JsonClient`] and (with the `xml` feature) SOAP client code.
//! Summary-style records use [`log::Level::Debug`]; full bodies use [`log::Level::Trace`] when
//! [`crate::core::client::WireLoggingMode::Detailed`] applies and the managed object is not
//! `SessionManager`.

use crate::core::client::WireLoggingMode;
use log::Level;

pub const TARGET_JSON: &str = "vim_rs::wire::json";
#[cfg(feature = "xml")]
pub const TARGET_SOAP: &str = "vim_rs::wire::soap";

#[inline]
pub(crate) fn bodies_allowed(mode: WireLoggingMode, mo_type: &str) -> bool {
    matches!(mode, WireLoggingMode::Detailed) && mo_type != "SessionManager"
}

/// When detailed is selected but MO is denylisted, emit summary-style records with this marker.
#[inline]
pub(crate) fn body_logging_note(mode: WireLoggingMode, mo_type: &str) -> Option<&'static str> {
    if matches!(mode, WireLoggingMode::Detailed) && mo_type == "SessionManager" {
        Some("body_logging=denylisted")
    } else {
        None
    }
}

#[inline]
pub(crate) fn wire_mode_label(mode: WireLoggingMode, mo_type: &str) -> &'static str {
    if matches!(mode, WireLoggingMode::Detailed) && mo_type == "SessionManager" {
        "summary"
    } else {
        match mode {
            WireLoggingMode::Off => "off",
            WireLoggingMode::Summary => "summary",
            WireLoggingMode::Detailed => "detailed",
        }
    }
}

fn level_for_wire_record(mode: WireLoggingMode, mo_type: &str, detailed_content: bool) -> Level {
    if detailed_content && bodies_allowed(mode, mo_type) {
        return Level::Trace;
    }
    Level::Debug
}

pub(crate) fn log_json_line(
    mode: WireLoggingMode,
    mo_type: &str,
    detailed_content: bool,
    msg: &str,
) {
    if !mode.is_enabled() {
        return;
    }
    let level = level_for_wire_record(mode, mo_type, detailed_content);
    log::log!(target: TARGET_JSON, level, "{}", msg);
}
/// Debug-only context for XML deserialization failures (`vim_rs::xml::de`).
/// Never pass secrets or full bodies; callers pass element/type hints only.
#[cfg(feature = "xml")]
pub(crate) fn log_xml_deser_failure(context: impl core::fmt::Display) {
    let mut msg = format!("{context}");
    const MAX: usize = 512;
    if msg.len() > MAX {
        msg.truncate(MAX);
        msg.push_str("...");
    }
    log::log!(target: TARGET_SOAP, Level::Debug, "xml_deser: {}", msg);
}

#[cfg(feature = "xml")]
pub(crate) fn log_soap_line(
    mode: WireLoggingMode,
    mo_type: &str,
    detailed_content: bool,
    msg: &str,
) {
    if !mode.is_enabled() {
        return;
    }
    let level = level_for_wire_record(mode, mo_type, detailed_content);
    log::log!(target: TARGET_SOAP, level, "{}", msg);
}

pub(crate) fn sanitize_utf8(bytes: &[u8]) -> std::borrow::Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

/// Legacy transport `trace!` lines: omit when dedicated wire logging handles the same path.
#[inline]
pub(crate) fn suppress_legacy_transport_trace(wire: WireLoggingMode) -> bool {
    wire.is_enabled()
}

pub(crate) fn log_json_http_error(
    wire: WireLoggingMode,
    svc: &str,
    mo_type: &str,
    mo_id: &str,
    name: &str,
    path: &str,
    is_property_get: bool,
    status: reqwest::StatusCode,
    body: &str,
    duration: std::time::Duration,
) {
    if !wire.is_enabled() {
        return;
    }
    let mode_label = wire_mode_label(wire, mo_type);
    let name_field = if is_property_get {
        format!("property={}", name)
    } else {
        format!("method={}", name)
    };
    let note = body_logging_note(wire, mo_type);
    let deny = note.unwrap_or("");
    let deny_sep = if deny.is_empty() { "" } else { " " };
    let body_part = if bodies_allowed(wire, mo_type) {
        format!(" body={}", sanitize_utf8(body.as_bytes()))
    } else {
        String::new()
    };
    let msg = format!(
        "wire=json mode={} phase=response svc=\"{}\" mo={} id={} {} path={} status={} body_bytes={} duration_ms={}{}{}{}",
        mode_label,
        svc,
        mo_type,
        mo_id,
        name_field,
        path,
        status.as_u16(),
        body.len(),
        duration.as_millis(),
        deny_sep,
        deny,
        body_part
    );
    let level = level_for_wire_record(wire, mo_type, !body_part.is_empty());
    log::log!(target: TARGET_JSON, level, "{}", msg);
}

/// After a logged JSON request, emit a matching line when `send()` fails (DNS, TLS, connect, etc.).
pub(crate) fn log_json_transport_failure(
    wire: WireLoggingMode,
    svc: &str,
    mo_type: &str,
    mo_id: &str,
    name: &str,
    path: &str,
    is_property_get: bool,
    duration: std::time::Duration,
    err: &reqwest::Error,
) {
    if !wire.is_enabled() {
        return;
    }
    let mode_label = wire_mode_label(wire, mo_type);
    let name_field = if is_property_get {
        format!("property={}", name)
    } else {
        format!("method={}", name)
    };
    let note = body_logging_note(wire, mo_type);
    let deny = note.unwrap_or("");
    let deny_sep = if deny.is_empty() { "" } else { " " };
    let msg = format!(
        "wire=json mode={} phase=response svc=\"{}\" mo={} id={} {} path={} error=transport duration_ms={}{}{} detail={}",
        mode_label,
        svc,
        mo_type,
        mo_id,
        name_field,
        path,
        duration.as_millis(),
        deny_sep,
        deny,
        err
    );
    let level = level_for_wire_record(wire, mo_type, false);
    log::log!(target: TARGET_JSON, level, "{}", msg);
}

/// Hello negotiation (`build_json`): request was logged; pair with this on transport failure.
pub(crate) fn log_json_negotiate_transport_failure(
    wire: WireLoggingMode,
    path: &str,
    duration: std::time::Duration,
    err: &reqwest::Error,
) {
    if !wire.is_enabled() {
        return;
    }
    let mode_label = wire_mode_label(wire, "");
    let msg = format!(
        "wire=json mode={} phase=response kind=negotiate path={} error=transport body_bytes=0 duration_ms={} detail={}",
        mode_label,
        path,
        duration.as_millis(),
        err
    );
    let level = level_for_wire_record(wire, "", false);
    log::log!(target: TARGET_JSON, level, "{}", msg);
}

#[cfg(feature = "xml")]
pub(crate) fn log_soap_transport_failure(
    wire: WireLoggingMode,
    mo_type: &str,
    mo_id: &str,
    method_name: &str,
    duration: std::time::Duration,
    err: &reqwest::Error,
) {
    if !wire.is_enabled() {
        return;
    }
    let mode_label = wire_mode_label(wire, mo_type);
    let note = body_logging_note(wire, mo_type);
    let deny = note.unwrap_or("");
    let deny_sep = if deny.is_empty() { "" } else { " " };
    let msg = format!(
        "wire=soap mode={} phase=response mo={} id={} method={} error=transport duration_ms={}{}{} detail={}",
        mode_label,
        mo_type,
        mo_id,
        method_name,
        duration.as_millis(),
        deny_sep,
        deny,
        err
    );
    let level = level_for_wire_record(wire, mo_type, false);
    log::log!(target: TARGET_SOAP, level, "{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_manager_denies_bodies_in_detailed() {
        assert!(!bodies_allowed(WireLoggingMode::Detailed, "SessionManager"));
        assert!(bodies_allowed(WireLoggingMode::Detailed, "VirtualMachine"));
    }

    #[test]
    fn body_logging_note_for_session_manager_detailed() {
        assert_eq!(
            body_logging_note(WireLoggingMode::Detailed, "SessionManager"),
            Some("body_logging=denylisted")
        );
        assert_eq!(body_logging_note(WireLoggingMode::Summary, "SessionManager"), None);
    }

    #[test]
    fn wire_mode_label_falls_back_for_session_manager_when_detailed() {
        assert_eq!(
            wire_mode_label(WireLoggingMode::Detailed, "SessionManager"),
            "summary"
        );
    }

}
