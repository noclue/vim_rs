//! Emit `vim_rs/src/perf/metrics.rs` from a `vim_rs.perf-catalog/1` document.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::path::Path;

use serde_json::Value;

use crate::printer::{FilePrinter, Printer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Catalog(String),
    #[error(transparent)]
    Print(#[from] crate::printer::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
struct Counter {
    key: i32,
    group: String,
    name: String,
    rollup: String,
    stats: String,
    unit: String,
    level: Option<u8>,
    per_device_level: Option<u8>,
}

#[derive(Clone)]
struct Emitted {
    counter: Counter,
    ident: String,
    tier: u8,
    mask: u8,
    per_instance: bool,
}

/// Read `catalog` and write the `metrics` module body to `output`.
pub fn emit_perf_metrics(catalog: &Path, output: &Path) -> Result<()> {
    let text = std::fs::read_to_string(catalog)
        .map_err(|err| Error::Catalog(format!("failed to read {}: {err}", catalog.display())))?;
    let emitted = plan(&text)?;
    let file = File::create(output)?;
    let mut printer = FilePrinter::new(file, None, None);
    write_module(&mut printer, &emitted)?;
    drop(printer);
    rustfmt(output)?;
    Ok(())
}

fn plan(text: &str) -> Result<Vec<Emitted>> {
    let root: Value = serde_json::from_str(text)?;
    let schema = root
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::Catalog("catalog schema is missing".into()))?;
    if schema != "vim_rs.perf-catalog/1" {
        return Err(Error::Catalog(format!(
            "unsupported catalog schema `{schema}`"
        )));
    }
    let counters = parse_counters(&root)?;
    let (tier1, per_instance) = parse_availability(&root)?;
    let mut group_union: HashMap<String, u8> = HashMap::new();
    let mut by_group_name: HashMap<(String, String), u8> = HashMap::new();
    for counter in &counters {
        let mask = tier1.get(&counter.key).copied().unwrap_or(0);
        if mask != 0 {
            let entry = group_union.entry(counter.group.clone()).or_insert(0);
            *entry |= mask;
            let sibling = by_group_name
                .entry((counter.group.clone(), counter.name.clone()))
                .or_insert(0);
            *sibling |= mask;
        }
    }

    let mut emitted = Vec::with_capacity(counters.len());
    for counter in counters {
        let direct = tier1.get(&counter.key).copied().unwrap_or(0);
        let (tier, mask) = if direct != 0 {
            (1, direct)
        } else if let Some(mask) = by_group_name
            .get(&(counter.group.clone(), counter.name.clone()))
            .copied()
        {
            if mask != 0 {
                (2, mask)
            } else {
                assign_group_tier(
                    &counter.group,
                    group_union.get(&counter.group).copied().unwrap_or(0),
                )?
            }
        } else {
            assign_group_tier(
                &counter.group,
                group_union.get(&counter.group).copied().unwrap_or(0),
            )?
        };
        if mask == 0 || mask == 0x3F {
            return Err(Error::Catalog(format!(
                "counter {} {}.{} has no concrete scope",
                counter.key, counter.group, counter.name
            )));
        }
        emitted.push(Emitted {
            ident: base_ident(&counter.name, &counter.rollup),
            per_instance: per_instance.contains(&counter.key),
            counter,
            tier,
            mask,
        });
    }
    disambiguate_idents(&mut emitted)?;
    emitted.sort_by(|left, right| {
        (&left.counter.group, &left.ident).cmp(&(&right.counter.group, &right.ident))
    });
    Ok(emitted)
}

fn assign_group_tier(group: &str, union: u8) -> Result<(u8, u8)> {
    if union != 0 {
        let mask = tier3a(group).unwrap_or(union);
        return Ok((3, mask));
    }
    tier3b(group)
        .map(|mask| (4, mask))
        .ok_or_else(|| Error::Catalog(format!("group `{group}` has no scope tier")))
}

fn parse_counters(root: &Value) -> Result<Vec<Counter>> {
    let list = root
        .get("counters")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::Catalog("catalog counters array is missing".into()))?;
    let mut counters = Vec::with_capacity(list.len());
    for value in list {
        let key = value
            .get("key")
            .and_then(Value::as_i64)
            .ok_or_else(|| Error::Catalog("counter key is missing".into()))?
            as i32;
        let group = field_str(value, "group")?;
        let name = field_str(value, "name")?;
        let rollup = field_str(value, "rollup")?;
        let stats = field_str(value, "stats")?;
        let unit = field_str(value, "unit")?;
        rollup_variant(&rollup).map_err(|err| Error::Catalog(format!("key {key}: {err}")))?;
        stats_variant(&stats).map_err(|err| Error::Catalog(format!("key {key}: {err}")))?;
        dimension(&unit).map_err(|err| Error::Catalog(format!("key {key}: {err}")))?;
        counters.push(Counter {
            key,
            group,
            name,
            rollup,
            stats,
            unit,
            level: optional_u8(value, "level")?,
            per_device_level: optional_u8(value, "per_device_level")?,
        });
    }
    Ok(counters)
}

fn parse_availability(root: &Value) -> Result<(HashMap<i32, u8>, BTreeSet<i32>)> {
    let mut tier1 = HashMap::new();
    let mut per_instance = BTreeSet::new();
    let Some(availability) = root.get("availability").and_then(Value::as_object) else {
        return Ok((tier1, per_instance));
    };
    for (mo_type, body) in availability {
        let bit = mo_bit(mo_type);
        if bit == 0 {
            continue;
        }
        let Some(counters) = body.get("counters").and_then(Value::as_object) else {
            continue;
        };
        for (key, row) in counters {
            let id: i32 = key.parse().map_err(|_| {
                Error::Catalog(format!("availability key `{key}` is not an integer"))
            })?;
            *tier1.entry(id).or_insert(0) |= bit;
            if row.get("per_instance").and_then(Value::as_bool) == Some(true) {
                per_instance.insert(id);
            }
        }
    }
    Ok((tier1, per_instance))
}

fn disambiguate_idents(emitted: &mut [Emitted]) -> Result<()> {
    let mut groups: BTreeMap<(String, String, String), Vec<usize>> = BTreeMap::new();
    for (index, item) in emitted.iter().enumerate() {
        groups
            .entry((
                item.counter.group.clone(),
                item.counter.rollup.clone(),
                item.ident.clone(),
            ))
            .or_default()
            .push(index);
    }
    for indexes in groups.values() {
        if indexes.len() < 2 {
            continue;
        }
        let dotted = indexes
            .iter()
            .any(|index| emitted[*index].counter.name.contains('.'));
        let plain = indexes
            .iter()
            .any(|index| !emitted[*index].counter.name.contains('.'));
        if dotted && plain {
            for index in indexes {
                if !emitted[*index].counter.name.contains('.') {
                    let item = &mut emitted[*index];
                    item.ident = flat_ident(&item.counter.name, &item.counter.rollup);
                }
            }
        }
    }
    let mut siblings: BTreeMap<(String, String, String), Vec<usize>> = BTreeMap::new();
    for (index, item) in emitted.iter().enumerate() {
        siblings
            .entry((
                item.counter.group.clone(),
                item.counter.name.clone(),
                item.counter.rollup.clone(),
            ))
            .or_default()
            .push(index);
    }
    for indexes in siblings.values() {
        if indexes.len() < 2 {
            continue;
        }
        for index in indexes {
            let stats = emitted[*index].counter.stats.as_str();
            if stats == "rate" {
                continue;
            }
            let suffix = if stats == "absolute" {
                "ABSOLUTE"
            } else {
                &emitted[*index].counter.stats.to_ascii_uppercase()
            };
            emitted[*index].ident.push('_');
            emitted[*index].ident.push_str(suffix);
        }
    }
    let mut seen: BTreeMap<(String, String), i32> = BTreeMap::new();
    for item in emitted.iter() {
        let key = (item.counter.group.clone(), item.ident.clone());
        if let Some(previous) = seen.insert(key, item.counter.key) {
            return Err(Error::Catalog(format!(
                "identifier collision {}.{} (keys {previous} and {})",
                item.counter.group, item.ident, item.counter.key
            )));
        }
    }
    Ok(())
}

fn write_module(printer: &mut FilePrinter, emitted: &[Emitted]) -> Result<()> {
    printer.println("//! Generated from `vim_build/data/perf-catalog.json`. Do not edit.")?;
    printer.println("//!")?;
    printer
        .println("//! Counter constants and per-kind re-exports. Scopes come from the catalog")?;
    printer.println("//! availability rows and the tier 3a / 3b tables.")?;
    printer.newline()?;
    let mut by_group: BTreeMap<String, Vec<&Emitted>> = BTreeMap::new();
    for item in emitted {
        by_group
            .entry(snake(&item.counter.group))
            .or_default()
            .push(item);
    }
    for (module, items) in &by_group {
        printer.println(&format!("pub mod {module} {{"))?;
        printer.indent();
        printer.println("use super::super::*;")?;
        printer.newline()?;
        for (index, item) in items.iter().enumerate() {
            write_const(printer, item)?;
            if index + 1 != items.len() {
                printer.newline()?;
            }
        }
        printer.dedent();
        printer.println("}")?;
        printer.newline()?;
    }
    printer.println("pub mod by_entity {")?;
    printer.indent();
    for (module, bit) in KIND_MODULES {
        printer.println(&format!("pub mod {module} {{"))?;
        printer.indent();
        let mut any = false;
        for (group_module, items) in &by_group {
            let exported: Vec<_> = items
                .iter()
                .filter(|item| item.tier != 4 && item.mask & bit != 0)
                .collect();
            if exported.is_empty() {
                continue;
            }
            any = true;
            printer.println(&format!("pub mod {group_module} {{"))?;
            printer.indent();
            for item in exported {
                printer.println(&format!(
                    "pub use crate::perf::metrics::{group_module}::{};",
                    item.ident
                ))?;
            }
            printer.dedent();
            printer.println("}")?;
            printer.newline()?;
        }
        if !any {
            printer.println("// No counters admit this kind.")?;
        }
        printer.dedent();
        printer.println("}")?;
        printer.newline()?;
    }
    printer.dedent();
    printer.println("}")?;
    Ok(())
}

fn write_const(printer: &mut FilePrinter, item: &Emitted) -> Result<()> {
    let level = opt_u8(item.counter.level);
    let device = opt_u8(item.counter.per_device_level);
    let kinds = kind_list(item.mask);
    let tier = match item.tier {
        1 => format!("Tier 1: {kinds}"),
        2 => format!("Tier 2: {kinds}"),
        3 => format!("Tier 3a: {kinds}"),
        _ => format!("tier 3b: inferred scope (no active samples in lab dump) · {kinds}"),
    };
    printer.println(&format!(
        "/// `{}.{}.{}` (key {}) · {} · level {level_text}, per_device_level {device_text} · {tier}",
        item.counter.group,
        item.counter.name,
        item.counter.rollup,
        item.counter.key,
        unit_doc(&item.counter.unit),
        level_text = level_text(item.counter.level),
        device_text = level_text(item.counter.per_device_level),
    ))?;
    printer.println(&format!(
        "pub const {}: Metric<{}, {}> = Metric::new(",
        item.ident,
        dimension(&item.counter.unit)?,
        scope_name(item.mask),
    ))?;
    printer.println(&format!("    {:?},", item.counter.group))?;
    printer.println(&format!("    {:?},", item.counter.name))?;
    printer.println(&format!(
        "    Rollup::{},",
        rollup_variant(&item.counter.rollup)?
    ))?;
    printer.println(&format!(
        "    Some(StatsKind::{}),",
        stats_variant(&item.counter.stats)?
    ))?;
    printer.println(&format!("    {level},"))?;
    printer.println(&format!("    {device},"))?;
    printer.println(&format!("    {},", item.per_instance))?;
    printer.println(");")?;
    Ok(())
}

fn rustfmt(output: &Path) -> Result<()> {
    let bin = rustfmt_bin();
    let status = std::process::Command::new(&bin)
        .arg("--edition")
        .arg("2021")
        .arg(output)
        .status()
        .map_err(|err| Error::Catalog(format!("failed to run {bin}: {err}")))?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::Catalog(format!("rustfmt exited with {status}")))
    }
}

fn rustfmt_bin() -> String {
    if let Ok(path) = std::env::var("RUSTFMT") {
        return path;
    }
    if let Ok(home) = std::env::var("CARGO_HOME") {
        let candidate = Path::new(&home).join("bin").join("rustfmt");
        if candidate.is_file() {
            return candidate.display().to_string();
        }
    }
    "rustfmt".into()
}

fn level_text(level: Option<u8>) -> String {
    match level {
        Some(level) => level.to_string(),
        None => "unset".into(),
    }
}

fn opt_u8(level: Option<u8>) -> String {
    match level {
        Some(level) => format!("Some({level}u8)"),
        None => "None".into(),
    }
}

fn unit_doc(unit: &str) -> String {
    if unit == "percent" {
        "percent (hundredths)".into()
    } else {
        unit.to_string()
    }
}

fn kind_list(mask: u8) -> String {
    let mut names = Vec::new();
    for (bit, name) in KIND_NAMES {
        if mask & bit != 0 {
            names.push(name);
        }
    }
    names.join(", ")
}

const KIND_NAMES: [(u8, &str); 6] = [
    (0x04, "Cluster"),
    (0x20, "Datacenter"),
    (0x10, "Datastore"),
    (0x02, "Host"),
    (0x08, "Pool"),
    (0x01, "VM"),
];

const KIND_MODULES: [(&str, u8); 6] = [
    ("virtual_machine", 0x01),
    ("host_system", 0x02),
    ("cluster_compute_resource", 0x04),
    ("resource_pool", 0x08),
    ("datastore", 0x10),
    ("datacenter", 0x20),
];

fn flag(mask: u8, bit: u8) -> &'static str {
    if mask & bit != 0 {
        "true"
    } else {
        "false"
    }
}

fn scope_name(mask: u8) -> String {
    match mask {
        0x01 => "VmOnly".into(),
        0x02 => "HostOnly".into(),
        0x03 => "VmOrHost".into(),
        0x06 => "HostCluster".into(),
        0x07 => "VmHostCluster".into(),
        0x09 => "VmOrPool".into(),
        0x0C => "ClusterPool".into(),
        0x0F => "ComputeScope".into(),
        0x10 => "DatastoreOnly".into(),
        0x11 => "DatastoreVm".into(),
        0x13 => "DatastoreVmHost".into(),
        0x24 => "ClusterDc".into(),
        other => format!(
            "crate::perf::entity::Scope<{}, {}, {}, {}, {}, {}>",
            flag(other, 0x01),
            flag(other, 0x02),
            flag(other, 0x04),
            flag(other, 0x08),
            flag(other, 0x10),
            flag(other, 0x20),
        ),
    }
}

fn tier3a(group: &str) -> Option<u8> {
    Some(match group {
        "cpu" | "mem" | "power" => 0x0F,
        "virtualDisk" => 0x01,
        "storageAdapter" | "storagePath" | "hbr" => 0x02,
        "net" | "sys" | "clusterServices" => 0x03,
        "datastore" | "disk" => 0x13,
        "gpu" => 0x0C,
        _ => return None,
    })
}

fn tier3b(group: &str) -> Option<u8> {
    Some(match group {
        "vmx" => 0x01,
        "managementAgent" | "vflashModule" | "vmotion" | "vcResources" | "vcDebugInfo" => 0x02,
        "pmem" | "lwd" => 0x03,
        "nfs" | "vvol" | "vsanDomObj" => 0x13,
        _ => return None,
    })
}

fn mo_bit(mo_type: &str) -> u8 {
    match mo_type {
        "VirtualMachine" => 0x01,
        "HostSystem" => 0x02,
        "ClusterComputeResource" => 0x04,
        "ResourcePool" | "VirtualApp" => 0x08,
        "Datastore" | "StoragePod" => 0x10,
        "Datacenter" => 0x20,
        _ => 0,
    }
}

fn base_ident(name: &str, rollup: &str) -> String {
    let dotted = name.replace('.', "_");
    let mut out = String::new();
    let chars: Vec<char> = dotted.chars().collect();
    for (index, ch) in chars.iter().enumerate() {
        if index > 0 {
            let prev = chars[index - 1];
            if (prev.is_ascii_lowercase() || prev.is_ascii_digit()) && ch.is_ascii_uppercase() {
                out.push('_');
            }
        }
        out.push(*ch);
    }
    format!(
        "{}_{}",
        out.to_ascii_uppercase(),
        rollup.to_ascii_uppercase()
    )
}

fn flat_ident(name: &str, rollup: &str) -> String {
    format!(
        "{}_{}",
        name.replace('.', "_").to_ascii_uppercase(),
        rollup.to_ascii_uppercase()
    )
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    for (index, ch) in name.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn field_str(value: &Value, key: &str) -> Result<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| Error::Catalog(format!("counter field `{key}` is missing")))
}

fn optional_u8(value: &Value, key: &str) -> Result<Option<u8>> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(other) => {
            let number = other
                .as_i64()
                .ok_or_else(|| Error::Catalog(format!("`{key}` is not an integer")))?;
            u8::try_from(number)
                .map(Some)
                .map_err(|_| Error::Catalog(format!("`{key}` does not fit in u8")))
        }
    }
}

fn rollup_variant(rollup: &str) -> Result<&'static str> {
    Ok(match rollup {
        "average" => "Average",
        "maximum" => "Maximum",
        "minimum" => "Minimum",
        "latest" => "Latest",
        "summation" => "Summation",
        "none" => "None",
        other => return Err(Error::Catalog(format!("unknown rollup `{other}`"))),
    })
}

fn stats_variant(stats: &str) -> Result<&'static str> {
    Ok(match stats {
        "absolute" => "Absolute",
        "delta" => "Delta",
        "rate" => "Rate",
        other => return Err(Error::Catalog(format!("unknown stats kind `{other}`"))),
    })
}

fn dimension(unit: &str) -> Result<&'static str> {
    Ok(match unit {
        "percent" => "Ratio",
        "kiloBytes" | "megaBytes" | "gigaBytes" | "teraBytes" => "Memory",
        "megaHertz" => "Frequency",
        "nanosecond" | "microsecond" | "millisecond" | "second" => "Time",
        "kiloBytesPerSecond" | "megaBytesPerSecond" => "Throughput",
        "watt" => "Power",
        "joule" => "Energy",
        "celsius" => "Temperature",
        "number" => "Count",
        other => return Err(Error::Catalog(format!("unknown unit `{other}`"))),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcenter_catalog_plans_732_constants() {
        let text = include_str!("../data/perf-catalog.json");
        let emitted = plan(text).unwrap();
        assert_eq!(emitted.len(), 732);
        let usage = emitted.iter().find(|item| item.counter.key == 2).unwrap();
        assert_eq!(usage.ident, "USAGE_AVERAGE");
        assert_eq!(usage.counter.group, "cpu");
        assert_eq!(usage.mask, 0x07);
        assert_eq!(usage.tier, 1);
        let maximum = emitted.iter().find(|item| item.counter.key == 4).unwrap();
        assert_eq!(maximum.ident, "USAGE_MAXIMUM");
        assert_eq!(maximum.tier, 2);
        assert_eq!(maximum.mask, 0x07);
        assert!(emitted.iter().any(|item| item.ident == "SWAP_IN_AVERAGE"));
        assert!(emitted.iter().any(|item| item.ident == "SWAPIN_AVERAGE"));
        assert!(emitted.iter().any(|item| item.ident == "ZIP_SAVED_LATEST"));
        assert!(emitted.iter().any(|item| item.ident == "ZIPSAVED_LATEST"));
        assert!(emitted
            .iter()
            .any(|item| item.ident == "SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE"
                && item.counter.stats == "rate"));
        assert!(emitted
            .iter()
            .any(|item| item.ident == "SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE"));
        assert!(emitted
            .iter()
            .all(|item| item.mask != 0x3F && item.mask != 0));
        let tier3b = emitted.iter().filter(|item| item.tier == 4).count();
        assert_eq!(tier3b, 174);
    }

    #[test]
    fn emit_metrics_when_requested() {
        if std::env::var_os("VIM_RS_EMIT_PERF").is_none() {
            return;
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        emit_perf_metrics(
            &root.join("data/perf-catalog.json"),
            &root.join("../vim_rs/src/perf/metrics.rs"),
        )
        .unwrap();
    }
}
