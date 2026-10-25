//! Checklist ledger shared by the dump and the probe.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Confirmed,
    Corrected,
    Na,
    Deferred,
    Inconclusive,
    Failed,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Corrected => "corrected",
            Self::Na => "na",
            Self::Deferred => "deferred",
            Self::Inconclusive => "inconclusive",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerRecord {
    pub id: String,
    pub target: String,
    pub transport: String,
    pub status: Status,
    pub summary: String,
    pub artifact: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ledger {
    pub records: Vec<LedgerRecord>,
}

impl Ledger {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading ledger {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing ledger {}", path.display()))
    }

    pub fn upsert(&mut self, record: LedgerRecord) {
        if let Some(existing) = self.records.iter_mut().find(|row| {
            row.id == record.id && row.target == record.target && row.transport == record.transport
        }) {
            *existing = record;
        } else {
            self.records.push(record);
        }
        self.records.sort_by(|a, b| {
            (&a.id, &a.target, &a.transport).cmp(&(&b.id, &b.target, &b.transport))
        });
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(path, text + "\n").with_context(|| format!("writing {}", path.display()))?;
        Ok(())
    }
}

pub fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(value)?;
    std::fs::write(path, text + "\n").with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_replaces_same_key() {
        let mut ledger = Ledger::default();
        ledger.upsert(LedgerRecord {
            id: "V-01".into(),
            target: "vcenter".into(),
            transport: "json".into(),
            status: Status::Failed,
            summary: "first".into(),
            artifact: None,
        });
        ledger.upsert(LedgerRecord {
            id: "V-01".into(),
            target: "vcenter".into(),
            transport: "json".into(),
            status: Status::Confirmed,
            summary: "second".into(),
            artifact: None,
        });
        assert_eq!(ledger.records.len(), 1);
        assert_eq!(ledger.records[0].status, Status::Confirmed);
    }
}
