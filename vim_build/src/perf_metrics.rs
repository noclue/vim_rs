//! Emit `vim_rs/src/perf/metrics.rs` from a `vim_rs.perf-catalog/1` document.
//!
//! [`plan_counters`] is read-only. The emitter is its only caller. `esxi_supported`
//! comes from the ESXi catalog. A counter present only in the ESXi catalog is an error.

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

/// How a counter's compile-time scope was chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanTier {
    /// Observed on sample entities.
    Tier1Direct,
    /// Inherited from sibling rollups of the same `(group, name)`.
    Tier2SiblingRollup,
    /// Per-counter table for counters the lab never reported on the object type
    /// they describe, such as cluster-level `clusterServices` counters.
    Tier3KnownScope,
    /// Group table, or the union of tier-1 masks in the group.
    Tier3aGroupUnion,
    /// Inferred for a group with no lab samples. Not re-exported from `by_entity`.
    Tier3bInferred,
}

/// One planned vCenter counter. Identifier rules stay inside [`plan_counters`].
#[derive(Debug, Clone)]
pub struct PlannedCounter {
    pub group: String,
    pub name: String,
    pub rollup: String,
    pub stats: String,
    pub unit: String,
    pub level: Option<u8>,
    pub per_device_level: Option<u8>,
    pub per_instance: bool,
    pub rust_module: String,
    pub rust_ident: String,
    pub dimension: &'static str,
    pub scope_mask: u8,
    /// Named alias (`VmHostCluster`) or generic `Scope<true, false, …>`.
    /// The emitter adds `crate::perf::entity::` only when writing the generic form.
    pub scope_type: String,
    pub tier: PlanTier,
    /// True when the ESXi catalog contains this `(group, name, rollup, stats)`.
    pub esxi_supported: bool,
    /// Catalog `labels.name_summary`, collapsed to one line. Empty when the catalog has none.
    pub summary: String,
}

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
    summary: String,
}

#[derive(Clone)]
struct Emitted {
    counter: Counter,
    ident: String,
    tier: u8,
    mask: u8,
    per_instance: bool,
}

/// Read both catalogs and write the `metrics` module body to `output`.
///
/// Planning is read-only. `esxi_supported` comes from `esxi_catalog`. A counter
/// present only in the ESXi catalog is an error.
pub fn emit_perf_metrics(catalog: &Path, esxi_catalog: &Path, output: &Path) -> Result<()> {
    let text = read_catalog(catalog)?;
    let esxi = read_catalog(esxi_catalog)?;
    let planned = plan_counters(&text, Some(&esxi))?;
    let file = File::create(output)?;
    let mut printer = FilePrinter::new(file, None, None);
    write_module(&mut printer, &planned)?;
    drop(printer);
    rustfmt(output)?;
    Ok(())
}

fn read_catalog(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|err| Error::Catalog(format!("failed to read {}: {err}", path.display())))
}

/// Plan every vCenter counter.
///
/// Planning is read-only. With `Some(esxi)`, each counter's `esxi_supported`
/// flag comes from the ESXi catalog, and an ESXi `(group, name, rollup, stats)`
/// absent from vCenter is an error. `None` leaves every `esxi_supported` flag false.
/// Identifier disambiguation (flat camelCase versus dotted names, SCSI `_ABSOLUTE`)
/// stays inside this function.
pub fn plan_counters(vcenter_catalog: &str, esxi_catalog: Option<&str>) -> Result<Vec<PlannedCounter>> {
    let mut planned = plan_vcenter(vcenter_catalog)?;
    let Some(esxi_catalog) = esxi_catalog else {
        return Ok(planned);
    };
    let esxi = identity_set(esxi_catalog)?;
    let vcenter: BTreeSet<_> = planned.iter().map(identity_of).collect();
    for key in &esxi {
        if !vcenter.contains(key) {
            return Err(Error::Catalog(format!(
                "ESXi catalog lists {}.{} {} {} which is absent from vCenter",
                key.0, key.1, key.2, key.3
            )));
        }
    }
    for counter in &mut planned {
        counter.esxi_supported = esxi.contains(&identity_of(counter));
    }
    Ok(planned)
}

fn identity_of(counter: &PlannedCounter) -> (String, String, String, String) {
    (
        counter.group.clone(),
        counter.name.clone(),
        counter.rollup.clone(),
        counter.stats.clone(),
    )
}

fn identity_set(text: &str) -> Result<BTreeSet<(String, String, String, String)>> {
    let root = parse_root(text)?;
    let counters = parse_counters(&root)?;
    Ok(counters
        .into_iter()
        .map(|counter| (counter.group, counter.name, counter.rollup, counter.stats))
        .collect())
}

fn parse_root(text: &str) -> Result<serde_json::Value> {
    let root: serde_json::Value = serde_json::from_str(text)?;
    let schema = root
        .get("schema")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| Error::Catalog("catalog schema is missing".into()))?;
    if schema != "vim_rs.perf-catalog/1" {
        return Err(Error::Catalog(format!(
            "unsupported catalog schema `{schema}`"
        )));
    }
    Ok(root)
}

fn plan_vcenter(text: &str) -> Result<Vec<PlannedCounter>> {
    let root = parse_root(text)?;
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
        } else if let Some(mask) = known_scope(&counter.group, &counter.name) {
            (5, mask)
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
    let mut planned = Vec::with_capacity(emitted.len());
    for item in emitted {
        planned.push(PlannedCounter {
            group: item.counter.group.clone(),
            name: item.counter.name,
            rollup: item.counter.rollup,
            stats: item.counter.stats,
            unit: item.counter.unit.clone(),
            level: item.counter.level,
            per_device_level: item.counter.per_device_level,
            per_instance: item.per_instance,
            rust_module: snake(&item.counter.group),
            rust_ident: item.ident,
            dimension: dimension(&item.counter.unit)?,
            scope_mask: item.mask,
            scope_type: scope_type_name(item.mask),
            tier: match item.tier {
                1 => PlanTier::Tier1Direct,
                2 => PlanTier::Tier2SiblingRollup,
                3 => PlanTier::Tier3aGroupUnion,
                5 => PlanTier::Tier3KnownScope,
                _ => PlanTier::Tier3bInferred,
            },
            esxi_supported: false,
            summary: item.counter.summary,
        });
    }
    Ok(planned)
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
        let summary = value
            .get("labels")
            .and_then(|labels| labels.get("name_summary"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .replace("*/", "* /");
        counters.push(Counter {
            key,
            group,
            name,
            rollup,
            stats,
            unit,
            level: optional_u8(value, "level")?,
            per_device_level: optional_u8(value, "per_device_level")?,
            summary,
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

fn write_module(printer: &mut FilePrinter, emitted: &[PlannedCounter]) -> Result<()> {
    for line in MODULE_DOC {
        printer.println(line)?;
    }
    printer.newline()?;
    let mut by_group: BTreeMap<String, Vec<&PlannedCounter>> = BTreeMap::new();
    for item in emitted {
        by_group.entry(item.rust_module.clone()).or_default().push(item);
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
                .filter(|item| item.tier != PlanTier::Tier3bInferred && item.scope_mask & bit != 0)
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
                    item.rust_ident
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

fn write_const(printer: &mut FilePrinter, item: &PlannedCounter) -> Result<()> {
    let level = opt_u8(item.level);
    let device = opt_u8(item.per_device_level);
    let kinds = kind_list(item.scope_mask);
    let tier = match item.tier {
        PlanTier::Tier1Direct => "Direct (1)",
        PlanTier::Tier2SiblingRollup => "Sibling Rollup (2)",
        PlanTier::Tier3KnownScope => "Known Scope (3k)",
        PlanTier::Tier3aGroupUnion => "Group Union (3a)",
        PlanTier::Tier3bInferred => "Inferred (3b)",
    };
    let availability = if item.esxi_supported {
        "vCenter and ESX"
    } else {
        "vCenter only"
    };
    if !item.summary.is_empty() {
        printer.println(&format!("/// {}", item.summary))?;
    }
    printer.println(&format!(
        "/// `{}.{}.{}` · {} · {availability}",
        item.group,
        item.name,
        item.rollup,
        unit_doc(&item.unit),
    ))?;
    printer.println(&format!(
        "/// Level: {level_text}, Per-device level: {device_text} · Tier: {tier}",
        level_text = level_text(item.level),
        device_text = level_text(item.per_device_level),
    ))?;
    printer.println(&format!("/// Types: {kinds}"))?;
    printer.println(&format!(
        "pub const {}: Metric<{}, {}> = Metric::new(",
        item.rust_ident,
        item.dimension,
        scope_path(&item.scope_type),
    ))?;
    printer.println(&format!("    {:?},", item.group))?;
    printer.println(&format!("    {:?},", item.name))?;
    printer.println(&format!(
        "    Rollup::{},",
        rollup_variant(&item.rollup)?
    ))?;
    printer.println(&format!(
        "    Some(StatsKind::{}),",
        stats_variant(&item.stats)?
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

/// Module docs for `vim_rs/src/perf/metrics.rs`. Field meanings follow
/// [`PerfCounterInfo`](https://developer.broadcom.com/xapis/vsphere-web-services-api/latest/vim.PerformanceManager.CounterInfo.html).
const MODULE_DOC: &[&str] = &[
    "//! Generated from vim_build/data/perf-catalog.json (vCenter 8.0.3) and",
    "//! vim_build/data/esxi-perf-catalog.json (ESXi 8.0.3 availability). Do not edit.",
    "//!",
    "//! Each constant is one performance counter. The comment above it reads:",
    "//!",
    "//! ```text",
    "//! {catalog summary}",
    "//! `group.name.rollup` · {unit} · {vCenter and ESX | vCenter only}",
    "//! Level: {1-4 | unset}, Per-device level: {1-4 | unset} · Tier: {how Types was chosen}",
    "//! Types: {managed object types}",
    "//! ```",
    "//!",
    "//! The host assigns the numeric counter id ([`PerfCounterInfo.key`][counter-info]).",
    "//! This file names a counter by `group.name.rollup`.",
    "//!",
    "//! **Level** is the lowest vCenter statistics collection level (1-4) that stores",
    "//! the aggregate series. A historical interval stores counters at its collection",
    "//! level and every lower level. The four built-in intervals collect at level 1",
    "//! until that interval is raised, so `Level: 4` is stored only at collection",
    "//! level 4.",
    "//!",
    "//! **Per-device level** is the lowest collection level that stores the same",
    "//! counter split by device, such as `vmnic0` or `scsi0:0`. `Per-device level: 4`",
    "//! means that split is stored only when the interval's collection level is 4.",
    "//! The per-device level is equal to Level or higher: collecting the device",
    "//! series includes the aggregate. `unset` means the catalog left that level empty.",
    "//!",
    "//! **Rollup** is the last part of the name (`average`, `maximum`, `minimum`,",
    "//! `latest`, `summation`, or `none`). The `StatsKind` on the constant is",
    "//! `absolute` (a point-in-time value), `delta` (change since the previous sample),",
    "//! or `rate`.",
    "//!",
    "//! **Unit** is the catalog unit. `percent (hundredths)` counts hundredths of a",
    "//! percent: 10000 is 100%.",
    "//!",
    "//! **vCenter and ESX** means the counter is in both catalogs. **vCenter only**",
    "//! means the counter appears in the vCenter catalog alone.",
    "//!",
    "//! **Tier** is this crate's note on how **Types** was chosen. `Direct (1)` was",
    "//! seen on sample entities. `Sibling Rollup (2)` was copied from another rollup",
    "//! of the same group and name. `Group Union (3a)` is the group's combined set.",
    "//! `Inferred (3b)` is a guess for a group with no samples. [`by_entity`] re-exports",
    "//! Direct, Sibling Rollup, and Group Union. `ResourcePool` includes `VirtualApp`.",
    "//!",
    "//! [counter-info]: https://developer.broadcom.com/xapis/vsphere-web-services-api/latest/vim.PerformanceManager.CounterInfo.html",
];

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
    (0x04, "ClusterComputeResource"),
    (0x20, "Datacenter"),
    (0x10, "Datastore"),
    (0x02, "HostSystem"),
    (0x08, "ResourcePool"),
    (0x01, "VirtualMachine"),
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

fn scope_type_name(mask: u8) -> String {
    match mask {
        0x01 => "VmOnly".into(),
        0x02 => "HostOnly".into(),
        0x03 => "VmOrHost".into(),
        0x04 => "ClusterOnly".into(),
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
            "Scope<{}, {}, {}, {}, {}, {}>",
            flag(other, 0x01),
            flag(other, 0x02),
            flag(other, 0x04),
            flag(other, 0x08),
            flag(other, 0x10),
            flag(other, 0x20),
        ),
    }
}

fn scope_path(scope_type: &str) -> String {
    if scope_type.starts_with("Scope<") {
        format!("crate::perf::entity::{scope_type}")
    } else {
        scope_type.to_string()
    }
}

/// Scope for a counter whose lab samples miss the object type it describes.
/// The lab cluster has DRS and HA off, so these cluster counters never reported on it.
fn known_scope(group: &str, name: &str) -> Option<u8> {
    match (group, name) {
        ("clusterServices", "clusterDrsScore" | "effectivecpu" | "effectivemem" | "failover") => {
            Some(0x04)
        }
        _ => None,
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
    fn esxi_catalog_is_byte_identical_to_fixture() {
        let committed = include_bytes!("../data/esxi-perf-catalog.json");
        let fixture = include_bytes!("../../vim_rs/tests/fixtures/perf/esxi_catalog.json");
        assert_eq!(committed, fixture);
    }

    #[test]
    fn vcenter_catalog_plans_732_constants() {
        let text = include_str!("../data/perf-catalog.json");
        let esxi = include_str!("../data/esxi-perf-catalog.json");
        let emitted = plan_counters(text, Some(esxi)).unwrap();
        assert_eq!(emitted.len(), 732);
        let usage = emitted
            .iter()
            .find(|item| item.group == "cpu" && item.name == "usage" && item.rollup == "average")
            .unwrap();
        assert_eq!(usage.rust_ident, "USAGE_AVERAGE");
        assert_eq!(usage.group, "cpu");
        assert_eq!(usage.scope_mask, 0x07);
        assert_eq!(usage.tier, PlanTier::Tier1Direct);
        assert!(usage.esxi_supported);
        let maximum = emitted
            .iter()
            .find(|item| item.group == "cpu" && item.name == "usage" && item.rollup == "maximum")
            .unwrap();
        assert_eq!(maximum.rust_ident, "USAGE_MAXIMUM");
        assert_eq!(maximum.tier, PlanTier::Tier2SiblingRollup);
        assert_eq!(maximum.scope_mask, 0x07);
        assert!(emitted.iter().any(|item| item.rust_ident == "SWAP_IN_AVERAGE"));
        assert!(emitted.iter().any(|item| item.rust_ident == "SWAPIN_AVERAGE"));
        assert!(emitted.iter().any(|item| item.rust_ident == "ZIP_SAVED_LATEST"));
        assert!(emitted.iter().any(|item| item.rust_ident == "ZIPSAVED_LATEST"));
        assert!(emitted
            .iter()
            .any(|item| item.rust_ident == "SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE"
                && item.stats == "rate"));
        assert!(emitted
            .iter()
            .any(|item| item.rust_ident == "SCSI_RESERVATION_CNFLCTS_PCT_AVERAGE_ABSOLUTE"));
        assert!(emitted
            .iter()
            .all(|item| item.scope_mask != 0x3F && item.scope_mask != 0));
        assert_eq!(
            emitted
                .iter()
                .filter(|item| item.tier == PlanTier::Tier1Direct)
                .count(),
            265
        );
        assert_eq!(
            emitted
                .iter()
                .filter(|item| item.tier == PlanTier::Tier2SiblingRollup)
                .count(),
            96
        );
        assert_eq!(
            emitted
                .iter()
                .filter(|item| item.tier == PlanTier::Tier3aGroupUnion)
                .count(),
            193
        );
        let known: Vec<_> = emitted
            .iter()
            .filter(|item| item.tier == PlanTier::Tier3KnownScope)
            .collect();
        assert_eq!(known.len(), 4);
        assert!(known
            .iter()
            .all(|item| item.group == "clusterServices" && item.scope_mask == 0x04));
        assert_eq!(
            emitted
                .iter()
                .filter(|item| item.tier == PlanTier::Tier3bInferred)
                .count(),
            174
        );
        assert_eq!(emitted.iter().filter(|item| item.esxi_supported).count(), 353);
        let balloon = emitted
            .iter()
            .find(|item| item.group == "mem" && item.name == "vmmemctl" && item.rollup == "average")
            .unwrap();
        assert!(balloon.summary.to_ascii_lowercase().contains("balloon driver"));
        assert!(usage.summary.to_ascii_lowercase().contains("cpu usage"));
    }

    #[test]
    fn esxi_only_counter_is_an_error() {
        let text = include_str!("../data/perf-catalog.json");
        let err = plan_counters(text, Some(r#"{"schema":"vim_rs.perf-catalog/1","counters":[{"key":1,"group":"not","name":"real","rollup":"average","stats":"rate","unit":"number"}]}"#)).unwrap_err();
        assert!(err.to_string().contains("absent from vCenter"));
    }

    #[test]
    fn emit_metrics_when_requested() {
        if std::env::var_os("VIM_RS_EMIT_PERF").is_none() {
            return;
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        emit_perf_metrics(
            &root.join("data/perf-catalog.json"),
            &root.join("data/esxi-perf-catalog.json"),
            &root.join("../vim_rs/src/perf/metrics.rs"),
        )
        .unwrap();
    }
}
