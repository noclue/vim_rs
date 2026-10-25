//! Per-entity high-water marks for incremental export.

use std::collections::{HashMap, HashSet};

use crate::perf::query::Instance;
use crate::perf::registry::CounterId;
use crate::perf::result::{PerfResult, Row, Timestamp};
use crate::types::structs::ManagedObjectReference;

/// Newest exported sample time for each entity, counter, and instance.
#[derive(Debug, Clone, Default)]
pub struct SampleWatermark {
    high_water: HashMap<ManagedObjectReference, HashMap<(CounterId, Instance), Timestamp>>,
}

impl SampleWatermark {
    /// Empty watermark.
    pub fn new() -> Self {
        Self {
            high_water: HashMap::new(),
        }
    }

    /// Rows strictly newer than the stored mark. Marks advance before the iterator is returned,
    /// so dropping it still keeps the update.
    pub fn filter_new<'a>(&mut self, result: &'a PerfResult) -> std::vec::IntoIter<Row<'a>> {
        let mut rows = Vec::new();
        for row in result.rows() {
            let key = (row.counter.id, row.instance.clone());
            if let Some(series) = self.high_water.get_mut(row.entity) {
                let newer = match series.get(&key) {
                    Some(mark) => row.ts > *mark,
                    None => true,
                };
                if newer {
                    series.insert(key, row.ts);
                    rows.push(row);
                }
            } else {
                self.high_water
                    .entry(row.entity.clone())
                    .or_default()
                    .insert(key, row.ts);
                rows.push(row);
            }
        }
        rows.into_iter()
    }

    /// Earliest mark among this entity’s series, or `None` if it has never reported.
    pub fn entity_high_water(&self, entity: &ManagedObjectReference) -> Option<Timestamp> {
        self.high_water
            .get(entity)
            .and_then(|series| series.values().copied().min())
    }

    /// Earliest mark among `entities` that have reported.
    pub fn min_high_water(&self, entities: &[ManagedObjectReference]) -> Option<Timestamp> {
        entities
            .iter()
            .filter_map(|entity| self.entity_high_water(entity))
            .min()
    }

    /// Drop marks for entities that are no longer in the inventory.
    pub fn retain_entities(&mut self, active: &HashSet<ManagedObjectReference>) {
        self.high_water.retain(|entity, _| active.contains(entity));
    }
}
