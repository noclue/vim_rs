#![cfg(feature = "perf")]

//! Compile-fail coverage for generated counter scopes.

#[test]
fn illegal_scopes_fail_and_a_matching_scope_passes() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/perf_scope_fail/cpu_average_on_datastore.rs");
    tests.compile_fail("tests/perf_scope_fail/cpu_maximum_on_datastore.rs");
    tests.compile_fail("tests/perf_scope_fail/mem_average_on_datastore.rs");
    tests.compile_fail("tests/perf_scope_fail/disk_latency_on_vm.rs");
    tests.pass("tests/perf_scope_fail/cpu_demand_on_vm.rs");
}
