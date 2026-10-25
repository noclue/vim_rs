//! Optional columnar export. Polars is outside the `vim_rs` semver.

use polars::prelude::{Column, DataFrame, PolarsResult};

use crate::perf::result::PerfResult;

impl PerfResult {
    /// One row per non-negative sample. `scaled` is the numeric view of `raw`.
    /// A unit [`crate::perf::units::AnyDimension`] cannot name stays as that raw number, so the column is not null.
    pub fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        let mut entity = Vec::new();
        let mut counter = Vec::new();
        let mut instance = Vec::new();
        let mut timestamp = Vec::new();
        let mut interval = Vec::new();
        let mut raw = Vec::new();
        let mut scaled = Vec::new();
        for row in self.rows() {
            entity.push(format!(
                "{}:{}",
                row.entity.r#type.as_str(),
                row.entity.value
            ));
            counter.push(row.counter.name.full.to_string());
            instance.push(row.instance.as_wire_str().to_string());
            timestamp.push(row.ts.to_rfc3339_utc());
            interval.push(row.interval_secs);
            raw.push(row.raw);
            scaled.push(row.scaled());
        }
        DataFrame::new_infer_height(vec![
            Column::new("entity".into(), entity),
            Column::new("counter".into(), counter),
            Column::new("instance".into(), instance),
            Column::new("timestamp".into(), timestamp),
            Column::new("interval".into(), interval),
            Column::new("raw".into(), raw),
            Column::new("scaled".into(), scaled),
        ])
    }
}
