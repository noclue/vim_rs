//! A counter identity plus the dimension and entity scope it was requested with.

use std::marker::PhantomData;

use crate::perf::entity::{AnyEntity, EntityScope};
use crate::perf::units::{AnyDimension, Dimension};

/// Rollup stored on a generated constant. Wire catalogs may still carry `Other_`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Rollup {
    /// Average over the interval.
    Average,
    /// Maximum over the interval.
    Maximum,
    /// Minimum over the interval.
    Minimum,
    /// Most recent value.
    Latest,
    /// Sum over the interval.
    Summation,
    /// The counter is not rolled up.
    None,
}

impl Rollup {
    /// Lowercase wire token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Rollup::Average => "average",
            Rollup::Maximum => "maximum",
            Rollup::Minimum => "minimum",
            Rollup::Latest => "latest",
            Rollup::Summation => "summation",
            Rollup::None => "none",
        }
    }
}

/// Stats kind stored on a generated constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StatsKind {
    /// An absolute level.
    Absolute,
    /// A change since the previous sample.
    Delta,
    /// A rate.
    Rate,
}

impl StatsKind {
    /// Lowercase wire token.
    pub const fn as_str(self) -> &'static str {
        match self {
            StatsKind::Absolute => "absolute",
            StatsKind::Delta => "delta",
            StatsKind::Rate => "rate",
        }
    }
}

/// One performance counter request.
///
/// `D` is the unit dimension and `S` is the entity scope. Neither is stored at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Metric<D: Dimension, S: EntityScope> {
    /// Catalog group, such as `cpu`.
    pub group: &'static str,
    /// Catalog name. Internal dots are preserved (`capacity.contention`).
    pub name: &'static str,
    /// Rollup suffix.
    pub rollup: Rollup,
    /// `Some` on generated constants. `None` on [`Metric::custom`], which prefers rate.
    pub stats: Option<StatsKind>,
    /// Collection level, when the catalog recorded one.
    pub level: Option<u8>,
    /// Per-device collection level, when the catalog recorded one.
    pub per_device_level: Option<u8>,
    /// Whether any availability row for this counter was per-instance.
    pub per_instance: bool,
    _p: PhantomData<fn() -> (D, S)>,
}

impl<D: Dimension, S: EntityScope> Metric<D, S> {
    /// Build a constant. Generated code calls this; callers usually use the catalog.
    #[doc(hidden)]
    pub const fn new(
        group: &'static str,
        name: &'static str,
        rollup: Rollup,
        stats: Option<StatsKind>,
        level: Option<u8>,
        per_device_level: Option<u8>,
        per_instance: bool,
    ) -> Self {
        Self {
            group,
            name,
            rollup,
            stats,
            level,
            per_device_level,
            per_instance,
            _p: PhantomData,
        }
    }

    /// Keep the dimension and replace the entity scope.
    pub const fn with_scope<S2: EntityScope>(self) -> Metric<D, S2> {
        Metric {
            group: self.group,
            name: self.name,
            rollup: self.rollup,
            stats: self.stats,
            level: self.level,
            per_device_level: self.per_device_level,
            per_instance: self.per_instance,
            _p: PhantomData,
        }
    }

    /// `group.name.rollup`.
    pub fn full_name(&self) -> String {
        format!("{}.{}.{}", self.group, self.name, self.rollup.as_str())
    }
}

impl Metric<AnyDimension, AnyEntity> {
    /// A counter that is not in the generated catalog. Stats kind is omitted (rate is preferred).
    pub const fn custom(group: &'static str, name: &'static str, rollup: Rollup) -> Self {
        Self::new(group, name, rollup, None, None, None, false)
    }
}
