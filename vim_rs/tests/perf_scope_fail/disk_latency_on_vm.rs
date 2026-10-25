use vim_rs::mo::VirtualMachine;
use vim_rs::perf::entity::EntityScope;
use vim_rs::perf::metrics::disk::DEVICE_READ_LATENCY_AVERAGE;
use vim_rs::perf::typed::{Admits, EntityKind};
use vim_rs::perf::units::Dimension;
use vim_rs::perf::Metric;

fn rejects<K: EntityKind, D: Dimension, S: EntityScope + Admits<K>>(_: Metric<D, S>) {}

fn main() {
    rejects::<VirtualMachine, _, _>(DEVICE_READ_LATENCY_AVERAGE);
}
