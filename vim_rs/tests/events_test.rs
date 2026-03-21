#![cfg(feature = "xml")]

use vim_rs::core::client::{unmarshal_array, Transport};
use vim_rs::types::struct_enum::StructType;
use vim_rs::types::structs::Event;
use vim_rs::xml::soap::vim_response_list;

const EVENTS_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
    <soapenv:Envelope xmlns:soapenc="http://schemas.xmlsoap.org/soap/encoding/"
     xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
     xmlns:xsd="http://www.w3.org/2001/XMLSchema"
     xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <soapenv:Body>
            <ReadNextEventsResponse xmlns="urn:vim25">
                <returnval xsi:type="UserLoginSessionEvent"><key>322</key><chainId>322</chainId><createdTime>2026-03-21T08:40:39.553822Z</createdTime><userName>root</userName><fullFormattedMessage>User root@127.0.0.1 logged in as VMware-client/8.0.3</fullFormattedMessage><ipAddress>127.0.0.1</ipAddress><userAgent>VMware-client/8.0.3</userAgent><locale>en</locale><sessionId>523b2442-2fdb-df9d-eaaa-fbb8a8600656</sessionId></returnval>
                <returnval xsi:type="UserLogoutSessionEvent"><key>321</key><chainId>321</chainId><createdTime>2026-03-21T08:38:58.114252Z</createdTime><userName>root</userName><fullFormattedMessage>User root@192.168.50.127 logged out (login time: Saturday, 21 March, 2026 07:59:24 AM, number of API invocations: 41, user agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:148.0) Gecko/20100101 Firefox/148.0)</fullFormattedMessage><ipAddress>192.168.50.127</ipAddress><userAgent>Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:148.0) Gecko/20100101 Firefox/148.0</userAgent><callCount>41</callCount><sessionId>52b8ec37-c05c-bbca-361f-3dc56a57e406</sessionId><loginTime>2026-03-21T07:59:24.693518Z</loginTime></returnval>
            </ReadNextEventsResponse>
        </soapenv:Body>
    </soapenv:Envelope>"#;

#[test]
fn read_next_events_fixture_deserializes_all() {
    let xml = EVENTS_XML;
    let events: Vec<Event> =
        vim_response_list(xml).expect("ReadNextEvents: deserialize all returnval as Event");
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].type_, Some(StructType::UserLoginSessionEvent));
    assert_eq!(events[1].type_, Some(StructType::UserLogoutSessionEvent));
}

/// Regression: paginated event/task reads use repeated `<returnval>` siblings; use
/// [`unmarshal_array`], not [`vim_rs::core::client::unmarshal`], on SOAP.
#[test]
fn unmarshal_array_soap_events_fixture() {
    let xml = EVENTS_XML;
    let events: Vec<Event> = unmarshal_array(Transport::Soap, xml.as_bytes())
        .expect("SOAP repeated returnval list");
    assert_eq!(events.len(), 2);
}
