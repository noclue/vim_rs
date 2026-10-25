use vim_rs::mo::VirtualMachine;
use vim_rs::perf::entity::EntityScope;
use vim_rs::perf::metrics::cpu::DEMAND_AVERAGE;
use vim_rs::perf::typed::{Admits, EntityKind};
use vim_rs::perf::units::Dimension;
use vim_rs::perf::Metric;

fn accepts<K: EntityKind, D: Dimension, S: EntityScope + Admits<K>>(_: Metric<D, S>) {}

fn main() {
    accepts::<VirtualMachine, _, _>(DEMAND_AVERAGE);
}
