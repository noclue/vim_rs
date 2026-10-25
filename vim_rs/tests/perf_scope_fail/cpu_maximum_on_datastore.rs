use vim_rs::mo::Datastore;
use vim_rs::perf::entity::EntityScope;
use vim_rs::perf::metrics::cpu::USAGE_MAXIMUM;
use vim_rs::perf::typed::{Admits, EntityKind};
use vim_rs::perf::units::Dimension;
use vim_rs::perf::Metric;

fn rejects<K: EntityKind, D: Dimension, S: EntityScope + Admits<K>>(_: Metric<D, S>) {}

fn main() {
    rejects::<Datastore, _, _>(USAGE_MAXIMUM);
}
