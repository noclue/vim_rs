//! Per-entity high-water marks for incremental export.

use std::collections::{HashMap, HashSet};

use crate::perf::error::{EntityStatus, PerfWarning};
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
    ///
    /// Delivery is at most once. Rows the caller fails to store, because of a crash or a failed
    /// write, are not returned again. To retry a failed write, keep a clone of the watermark from
    /// before the call and restore it.
    ///
    /// For an entity that ended [`EntityStatus::Ok`] in `result`, marks of series it no
    /// longer returns (a removed vNIC or disk) are dropped, so they cannot pin
    /// [`Self::entity_high_water`]. A [`PerfWarning::PartialChunk`] entity keeps its marks.
    pub fn filter_new<'a>(&mut self, result: &'a PerfResult) -> std::vec::IntoIter<Row<'a>> {
        self.drop_stopped_series(result);
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

    fn drop_stopped_series(&mut self, result: &PerfResult) {
        let partial: HashSet<&ManagedObjectReference> = result
            .warnings
            .iter()
            .filter_map(|warning| match warning {
                PerfWarning::PartialChunk { entity, .. } => Some(entity),
                _ => None,
            })
            .collect();
        for entity in result.entities() {
            if !matches!(entity.status, EntityStatus::Ok) || partial.contains(&entity.entity) {
                continue;
            }
            let Some(marks) = self.high_water.get_mut(&entity.entity) else {
                continue;
            };
            let returned: HashSet<(CounterId, &Instance)> = entity
                .series
                .iter()
                .map(|series| (series.counter.id, &series.instance))
                .collect();
            marks.retain(|(counter, instance), _| returned.contains(&(*counter, instance)));
        }
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
