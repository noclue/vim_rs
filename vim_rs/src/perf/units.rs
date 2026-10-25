//! Unit dimensions and human-scaled display for the 16 vSphere performance units.

use std::fmt;

use crate::types::enums::PerformanceManagerUnitEnum;

/// Wire unit token, including `Other_("gigaBytes")`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitKey(pub PerformanceManagerUnitEnum);

impl UnitKey {
    /// Wire token (`percent`, `kiloBytes`, `gigaBytes`, …).
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for UnitKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A scaled quantity that can be shown as a number.
pub trait ScaledValue: Copy + fmt::Debug + fmt::Display {
    /// Numeric value in this dimension’s scaled unit (bytes, hertz, seconds, …).
    fn as_f64(self) -> f64;
}

/// Physical dimension of a counter. `accepts(unit)` is true exactly when `scale(0, unit)` is `Some`,
/// except [`AnyDimension`], which accepts every unit.
pub trait Dimension: Copy + 'static {
    /// Human-scaled form of a raw sample.
    type Scaled: ScaledValue;

    /// Whether a raw value in `unit` can be scaled in this dimension.
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool;

    /// Scale one raw integer. `None` when `unit` is outside this dimension.
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Self::Scaled>;
}

/// Hundredths of a percent (`percent`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio;

/// Byte quantities (`kiloBytes`, `megaBytes`, `gigaBytes`, `teraBytes`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Memory;

/// `megaHertz`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Frequency;

/// Time units from nanoseconds through seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time;

/// Bytes per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Throughput;

/// `watt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Power;

/// `joule`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Energy;

/// `celsius`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temperature;

/// `number`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Count;

/// Every wire unit. Unknown `Other_` tokens stay as the raw number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnyDimension;

/// A percent already divided by 100. Raw `4523` is `Percent(45.23)`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(pub f64);

/// Exact byte count. Memory scaling never goes through floating point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(pub i128);

/// Bytes per second.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BytesPerSecond(pub f64);

/// Hertz.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(pub f64);

/// Seconds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Seconds(pub f64);

/// Watts.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Watts(pub f64);

/// Joules.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Joules(pub f64);

/// Degrees Celsius.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

/// A plain number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Number(pub f64);

impl ScaledValue for Percent {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Bytes {
    fn as_f64(self) -> f64 {
        self.0 as f64
    }
}
impl ScaledValue for BytesPerSecond {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Hertz {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Seconds {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Watts {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Joules {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Celsius {
    fn as_f64(self) -> f64 {
        self.0
    }
}
impl ScaledValue for Number {
    fn as_f64(self) -> f64 {
        self.0
    }
}

impl Bytes {
    /// Scale a memory raw value into bytes.
    pub fn from_raw(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Bytes> {
        memory_factor(unit).map(|factor| Bytes(i128::from(raw) * factor))
    }

    /// Saturating conversion. Negative values become `0`.
    pub fn as_u64(self) -> u64 {
        if self.0 <= 0 {
            0
        } else if self.0 >= i128::from(u64::MAX) {
            u64::MAX
        } else {
            self.0 as u64
        }
    }
}

impl Hertz {
    /// This value in megahertz.
    pub fn as_mhz(self) -> f64 {
        self.0 / 1_000_000.0
    }
}

impl Seconds {
    /// This value in milliseconds.
    pub fn as_millis(self) -> f64 {
        self.0 * 1_000.0
    }
}

impl Dimension for Ratio {
    type Scaled = Percent;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        matches!(unit, PerformanceManagerUnitEnum::Percent)
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Percent> {
        Self::accepts(unit).then_some(Percent(raw as f64 * 0.01))
    }
}

impl Dimension for Memory {
    type Scaled = Bytes;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        memory_factor(unit).is_some()
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Bytes> {
        Bytes::from_raw(raw, unit)
    }
}

impl Dimension for Frequency {
    type Scaled = Hertz;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        matches!(unit, PerformanceManagerUnitEnum::MegaHertz)
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Hertz> {
        Self::accepts(unit).then_some(Hertz(raw as f64 * 1_000_000.0))
    }
}

impl Dimension for Time {
    type Scaled = Seconds;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        time_seconds(0, unit).is_some()
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Seconds> {
        time_seconds(raw, unit).map(Seconds)
    }
}

impl Dimension for Throughput {
    type Scaled = BytesPerSecond;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        throughput_factor(unit).is_some()
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<BytesPerSecond> {
        throughput_factor(unit).map(|factor| BytesPerSecond(raw as f64 * factor))
    }
}

impl Dimension for Power {
    type Scaled = Watts;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        matches!(unit, PerformanceManagerUnitEnum::Watt)
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Watts> {
        Self::accepts(unit).then_some(Watts(raw as f64))
    }
}

impl Dimension for Energy {
    type Scaled = Joules;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        matches!(unit, PerformanceManagerUnitEnum::Joule)
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Joules> {
        Self::accepts(unit).then_some(Joules(raw as f64))
    }
}

impl Dimension for Temperature {
    type Scaled = Celsius;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        matches!(unit, PerformanceManagerUnitEnum::Celsius)
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Celsius> {
        Self::accepts(unit).then_some(Celsius(raw as f64))
    }
}

impl Dimension for Count {
    type Scaled = Number;
    fn accepts(unit: &PerformanceManagerUnitEnum) -> bool {
        matches!(unit, PerformanceManagerUnitEnum::Number)
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Number> {
        Self::accepts(unit).then_some(Number(raw as f64))
    }
}

impl Dimension for AnyDimension {
    type Scaled = Number;
    fn accepts(_unit: &PerformanceManagerUnitEnum) -> bool {
        true
    }
    fn scale(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<Number> {
        Some(Number(scale_any(raw, unit)))
    }
}

/// `false` for `Other_` tokens other than `gigaBytes`.
pub fn unit_is_scalable(unit: &PerformanceManagerUnitEnum) -> bool {
    match unit {
        PerformanceManagerUnitEnum::Other_(name) => name == "gigaBytes",
        _ => true,
    }
}

/// Display text for one raw sample in `unit`, using the spec’s display column.
pub fn display_raw(raw: i64, unit: &PerformanceManagerUnitEnum) -> String {
    match unit {
        PerformanceManagerUnitEnum::Percent => {
            format!("{}", Percent(raw as f64 * 0.01))
        }
        PerformanceManagerUnitEnum::KiloBytes
        | PerformanceManagerUnitEnum::MegaBytes
        | PerformanceManagerUnitEnum::TeraBytes => {
            format!(
                "{}",
                Bytes(i128::from(raw) * memory_factor(unit).unwrap_or(1))
            )
        }
        PerformanceManagerUnitEnum::Other_(name) if name == "gigaBytes" => {
            format!(
                "{}",
                Bytes(i128::from(raw) * memory_factor(unit).unwrap_or(1))
            )
        }
        PerformanceManagerUnitEnum::KiloBytesPerSecond
        | PerformanceManagerUnitEnum::MegaBytesPerSecond => {
            let factor = throughput_factor(unit).unwrap_or(1.0);
            format!("{}", BytesPerSecond(raw as f64 * factor))
        }
        PerformanceManagerUnitEnum::MegaHertz => format!("{}", Hertz(raw as f64 * 1_000_000.0)),
        PerformanceManagerUnitEnum::Nanosecond
        | PerformanceManagerUnitEnum::Microsecond
        | PerformanceManagerUnitEnum::Millisecond
        | PerformanceManagerUnitEnum::Second => {
            format!("{}", Seconds(time_seconds(raw, unit).unwrap_or(raw as f64)))
        }
        PerformanceManagerUnitEnum::Watt => format!("{}", Watts(raw as f64)),
        PerformanceManagerUnitEnum::Joule => format!("{}", Joules(raw as f64)),
        PerformanceManagerUnitEnum::Celsius => format!("{}", Celsius(raw as f64)),
        PerformanceManagerUnitEnum::Number => format_number(raw as f64),
        PerformanceManagerUnitEnum::Other_(_) => format_number(scale_any(raw, unit)),
    }
}

/// Milliseconds represented by a raw time sample. `None` when `unit` is not a time unit.
pub fn raw_as_millis(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<f64> {
    time_seconds(raw, unit).map(|seconds| seconds * 1_000.0)
}

fn scale_any(raw: i64, unit: &PerformanceManagerUnitEnum) -> f64 {
    if let Some(percent) = Ratio::scale(raw, unit) {
        return percent.as_f64();
    }
    if let Some(bytes) = Memory::scale(raw, unit) {
        return bytes.as_f64();
    }
    if let Some(hz) = Frequency::scale(raw, unit) {
        return hz.as_f64();
    }
    if let Some(seconds) = Time::scale(raw, unit) {
        return seconds.as_f64();
    }
    if let Some(bps) = Throughput::scale(raw, unit) {
        return bps.as_f64();
    }
    if let Some(watts) = Power::scale(raw, unit) {
        return watts.as_f64();
    }
    if let Some(joules) = Energy::scale(raw, unit) {
        return joules.as_f64();
    }
    if let Some(celsius) = Temperature::scale(raw, unit) {
        return celsius.as_f64();
    }
    if let Some(number) = Count::scale(raw, unit) {
        return number.as_f64();
    }
    raw as f64
}

fn memory_factor(unit: &PerformanceManagerUnitEnum) -> Option<i128> {
    match unit {
        PerformanceManagerUnitEnum::KiloBytes => Some(1024),
        PerformanceManagerUnitEnum::MegaBytes => Some(1024 * 1024),
        PerformanceManagerUnitEnum::TeraBytes => Some(1024_i128.pow(4)),
        PerformanceManagerUnitEnum::Other_(name) if name == "gigaBytes" => Some(1024_i128.pow(3)),
        _ => None,
    }
}

fn throughput_factor(unit: &PerformanceManagerUnitEnum) -> Option<f64> {
    match unit {
        PerformanceManagerUnitEnum::KiloBytesPerSecond => Some(1024.0),
        PerformanceManagerUnitEnum::MegaBytesPerSecond => Some(1_048_576.0),
        _ => None,
    }
}

fn time_seconds(raw: i64, unit: &PerformanceManagerUnitEnum) -> Option<f64> {
    let raw = raw as f64;
    match unit {
        PerformanceManagerUnitEnum::Nanosecond => Some(raw * 1e-9),
        PerformanceManagerUnitEnum::Microsecond => Some(raw * 1e-6),
        PerformanceManagerUnitEnum::Millisecond => Some(raw * 1e-3),
        PerformanceManagerUnitEnum::Second => Some(raw),
        _ => None,
    }
}

fn format_number(n: f64) -> String {
    if n.is_finite() && n.fract() == 0.0 {
        format!("{n:.0}")
    } else {
        format!("{n:.2}")
    }
}

fn format_iec_bytes(n: i128) -> String {
    const KIB: u128 = 1024;
    const MIB: u128 = 1024 * 1024;
    const GIB: u128 = 1024 * 1024 * 1024;
    const TIB: u128 = 1024 * 1024 * 1024 * 1024;
    let abs = n.unsigned_abs();
    let signed = n as f64;
    if abs < KIB {
        format!("{n} B")
    } else if abs < MIB {
        format!("{:.2} KiB", signed / 1024.0)
    } else if abs < GIB {
        format!("{:.2} MiB", signed / (1024.0 * 1024.0))
    } else if abs < TIB {
        format!("{:.2} GiB", signed / (1024.0 * 1024.0 * 1024.0))
    } else {
        format!("{:.2} TiB", signed / (1024.0 * 1024.0 * 1024.0 * 1024.0))
    }
}

fn format_iec_rate(bytes_per_sec: f64) -> String {
    let abs = bytes_per_sec.abs();
    if abs < 1024.0 {
        format!("{bytes_per_sec:.0} B/s")
    } else if abs < 1024.0 * 1024.0 {
        format!("{:.2} KiB/s", bytes_per_sec / 1024.0)
    } else if abs < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} MiB/s", bytes_per_sec / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GiB/s", bytes_per_sec / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_hertz(hz: f64) -> String {
    if hz.abs() < 1e9 {
        format!("{:.0} MHz", hz / 1_000_000.0)
    } else {
        format!("{:.2} GHz", hz / 1e9)
    }
}

fn format_seconds(seconds: f64) -> String {
    let abs = seconds.abs();
    if abs < 1e-6 {
        format!("{:.0} ns", seconds * 1e9)
    } else if abs < 1e-3 {
        format!("{:.2} µs", seconds * 1e6)
    } else if abs < 1.0 {
        format!("{:.2} ms", seconds * 1e3)
    } else {
        format!("{:.2} s", seconds)
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} %", self.0)
    }
}
impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_iec_bytes(self.0))
    }
}
impl fmt::Display for BytesPerSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_iec_rate(self.0))
    }
}
impl fmt::Display for Hertz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_hertz(self.0))
    }
}
impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_seconds(self.0))
    }
}
impl fmt::Display for Watts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} W", self.0)
    }
}
impl fmt::Display for Joules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} J", self.0)
    }
}
impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} °C", self.0)
    }
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_number(self.0))
    }
}
