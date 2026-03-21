use miniserde::de::Deserialize;
use miniserde::ser::Serialize;
use miniserde::{Error, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use crate::xml::{de, ser};

const SOAP_NS: &str = "http://schemas.xmlsoap.org/soap/envelope/";

pub fn envelope(body: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
         <soapenv:Envelope \
            xmlns:soapenv=\"{SOAP_NS}\" \
            xmlns:soapenc=\"http://schemas.xmlsoap.org/soap/encoding/\" \
            xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" \
            xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\
            <soapenv:Body>{body}</soapenv:Body>\
         </soapenv:Envelope>"
    )
}

pub fn vim_request<T: Serialize>(method_name: &str, params: &T) -> String {
    let body_xml = ser::to_xml(params, method_name);
    envelope(&body_xml)
}

fn find_start(reader: &mut Reader<&[u8]>, name_suffix: &str) -> Result<()> {
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(e) => {
                let n = de::start_name(&e)?;
                if n == name_suffix || n.ends_with(&format!(":{name_suffix}")) {
                    return Ok(());
                }
            }
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}

fn skip_element(reader: &mut Reader<&[u8]>) -> Result<()> {
    let mut depth = 1u32;
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(_) => depth += 1,
            Event::End(_) => {
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
            }
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}

pub fn vim_response<T: Deserialize>(response_xml: &str) -> Result<T> {
    let mut reader = Reader::from_str(response_xml);
    reader.config_mut().trim_text(false);

    find_start(&mut reader, "Body")?;
    find_start_any(&mut reader)?;
    let returnval = find_start_returning(&mut reader, "returnval")?;

    let mut out = None;
    let visitor = T::begin(&mut out);
    de::stream_drive(&mut reader, &returnval, visitor)?;
    out.ok_or(Error)
}

pub fn vim_response_list<T: Deserialize>(response_xml: &str) -> Result<Vec<T>> {
    let mut reader = Reader::from_str(response_xml);
    reader.config_mut().trim_text(false);

    find_start(&mut reader, "Body")?;
    find_start_any(&mut reader)?;

    let mut results = Vec::new();
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(e) => {
                let n = de::start_name(&e)?;
                if n == "returnval" {
                    let mut out = None;
                    let visitor = T::begin(&mut out);
                    de::stream_drive(&mut reader, &e, visitor)?;
                    results.push(out.ok_or(Error)?);
                } else {
                    skip_element(&mut reader)?;
                }
            }
            Event::Empty(_) => continue,
            Event::End(_) => break,
            Event::Eof => break,
            _ => continue,
        }
    }
    Ok(results)
}

/// Returns true if the SOAP response is an empty body (no returnval).
/// Useful for optional-return methods like WaitForUpdatesEx.
pub fn vim_response_is_empty(response_xml: &str) -> Result<bool> {
    let mut reader = Reader::from_str(response_xml);
    reader.config_mut().trim_text(false);

    find_start(&mut reader, "Body")?;
    find_start_any(&mut reader)?;

    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(e) => {
                let n = de::start_name(&e)?;
                if n == "returnval" {
                    return Ok(false);
                }
                skip_element(&mut reader)?;
            }
            Event::Empty(_) => continue,
            Event::End(_) => return Ok(true),
            Event::Eof => return Ok(true),
            _ => continue,
        }
    }
}

fn find_start_any(reader: &mut Reader<&[u8]>) -> Result<()> {
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(_) => return Ok(()),
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}

fn find_start_returning(
    reader: &mut Reader<&[u8]>,
    name_suffix: &str,
) -> Result<BytesStart<'static>> {
    loop {
        match reader.read_event().map_err(|_| Error)? {
            Event::Start(e) => {
                let n = de::start_name(&e)?;
                if n == name_suffix || n.ends_with(&format!(":{name_suffix}")) {
                    return Ok(e.into_owned());
                }
                skip_element(reader)?;
            }
            Event::Eof => return Err(Error),
            _ => continue,
        }
    }
}
