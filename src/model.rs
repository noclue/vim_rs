#[derive(Debug, serde::Deserialize)]
struct Device {
    key: i32,
    controller_key: Option<i32>,
    unit_number: Option<i32>,
}