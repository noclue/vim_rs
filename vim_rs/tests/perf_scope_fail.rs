#![cfg(feature = "perf")]

//! Compile-fail coverage for generated counter scopes.
//!
//! These checks do not snapshot rustc output. The trait-solver "help" text
//! changes between compiler releases, so a `.stderr` golden file fails after
//! an upgrade even when the rejection is unchanged. The assertions below lock
//! the diagnostic this crate owns: `Admits` is unimplemented, and the note
//! tells the caller how to widen a tier-3b counter.

use std::path::PathBuf;
use std::process::Command;

#[test]
fn illegal_scopes_fail_and_a_matching_scope_passes() {
    for (bin, entity) in [
        ("cpu_average_on_datastore", "Datastore"),
        ("cpu_maximum_on_datastore", "Datastore"),
        ("mem_average_on_datastore", "Datastore"),
        ("disk_latency_on_vm", "VirtualMachine"),
    ] {
        let output = cargo_check(bin);
        let text = combined(&output);
        assert!(
            !output.status.success(),
            "{bin} compiled, but {entity} is outside this counter's scope:\n{text}"
        );
        assert!(
            text.contains("error[E0277]"),
            "{bin} failed for a reason other than a missing trait:\n{text}"
        );
        assert!(
            text.contains("does not include") && text.contains(entity),
            "{bin} did not report that the scope excludes {entity}:\n{text}"
        );
        assert!(
            text.contains("metric.with_scope::<AnyEntity>()"),
            "{bin} dropped the tier-3b widening note:\n{text}"
        );
    }

    let output = cargo_check("cpu_demand_on_vm");
    assert!(
        output.status.success(),
        "cpu.demand.average should admit VirtualMachine:\n{}",
        combined(&output)
    );
}

fn cargo_check(bin: &str) -> std::process::Output {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest = manifest_dir.join("tests/perf_scope_fail/Cargo.toml");
    let parent_target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| manifest_dir.join("target"));
    // A child `cargo` cannot share the target dir of the `cargo test` that
    // launched it: the parent holds that directory's lock.
    let target = parent_target.join("perf-scope-check");
    Command::new(env!("CARGO"))
        .args(["check", "--offline", "--color", "never", "--manifest-path"])
        .arg(&manifest)
        .args(["--bin", bin])
        .env("CARGO_TARGET_DIR", &target)
        .output()
        .unwrap_or_else(|err| panic!("failed to run cargo check for {bin}: {err}"))
}

fn combined(output: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stderr).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text
}
