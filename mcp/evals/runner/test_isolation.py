"""Isolation checks that do not need a container or a model."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[1]
sys.path.insert(0, str(ROOT / "runner"))

from scorecard import load_results  # noqa: E402


def test_scaffold_has_no_oracle_generator() -> None:
    scaffold = ROOT / "scaffold"
    assert not (scaffold / "src" / "bin" / "fixture_gen.rs").exists()
    cargo = (scaffold / "Cargo.toml").read_text(encoding="utf-8")
    assert "fixture_gen" not in cargo
    assert (ROOT / "fixture_gen" / "src" / "main.rs").is_file()


def test_image_builds_only_the_agent_bin() -> None:
    text = (ROOT / "Containerfile").read_text(encoding="utf-8")
    assert "cargo build --bin eval_scaffold" in text
    assert "cargo build --bins" not in text
    assert "ENV CARGO_BUILD_JOBS" not in text.split("FROM docker.io/library/rust:${RUST_VERSION}\n", 1)[1]


def test_runner_mounts_agent_dir_only() -> None:
    ps1 = (ROOT / "runner" / "run_one.ps1").read_text(encoding="utf-8")
    sh = (ROOT / "runner" / "run_one.sh").read_text(encoding="utf-8")
    assert '"${AgentDir}:/out"' in ps1
    assert '"${Out}:/out"' not in ps1
    assert '"${AGENT_DIR}:/out"' in sh
    assert '"${OUT}:/out"' not in sh
    assert "--agent-work" in ps1
    assert "--agent-work" in sh


def test_scorecard_skips_grade_result(tmp_path: Path) -> None:
    attempt = tmp_path / "pc01_A_r1"
    grade = attempt / "grade"
    grade.mkdir(parents=True)
    (attempt / "result.json").write_text(
        json.dumps({"problem": "pc01", "arm": "A", "retrieval_pass": True, "turns": 3}),
        encoding="utf-8",
    )
    (grade / "result.json").write_text(
        json.dumps({"problem": "pc01", "arm": "A", "retrieval_pass": False, "turns": None}),
        encoding="utf-8",
    )
    rows = load_results(tmp_path)
    assert len(rows) == 1
    assert rows[0]["turns"] == 3
    rows = load_results(attempt)
    assert len(rows) == 1
    assert rows[0]["retrieval_pass"] is True


def test_seal_hash_ignores_solve_and_sees_other_edits(tmp_path: Path) -> None:
    script = ROOT / "runner" / "lib" / "seal_hash.py"
    canon = tmp_path / "canon"
    work = tmp_path / "work"
    for tree in (canon, work):
        (tree / "src").mkdir(parents=True)
        (tree / "src" / "main.rs").write_text("fn main() {}\n", encoding="utf-8")
        (tree / "src" / "solve.rs").write_text("oracle\n", encoding="utf-8")
    work_solve = work / "src" / "solve.rs"
    work_solve.write_text("agent answer\n", encoding="utf-8")
    (canon / "Cargo.lock").write_text("host lock\n", encoding="utf-8")
    (work / "Cargo.lock").write_text("container lock\n", encoding="utf-8")

    def digest(path: Path) -> str:
        out = subprocess.run(
            [sys.executable, str(script), str(path)],
            check=True,
            capture_output=True,
            text=True,
            encoding="utf-8",
        )
        return out.stdout.strip()

    assert digest(canon) == digest(work)
    (work / "src" / "main.rs").write_text("fn main() { /* edited */ }\n", encoding="utf-8")
    assert digest(canon) != digest(work)


def test_recipe_with_em_dash_is_utf8(tmp_path: Path) -> None:
    from importlib.machinery import SourceFileLoader

    loader = SourceFileLoader("render_recipe", str(ROOT / "runner" / "lib" / "render_recipe.py"))
    mod = loader.load_module()
    tmp_path.mkdir(parents=True, exist_ok=True)
    goal = tmp_path / "goal.txt"
    contract = tmp_path / "contract.txt"
    out = tmp_path / "recipe.yaml"
    goal.write_text("collect MACs\n", encoding="utf-8")
    contract.write_text("MAC — you must downcast\n", encoding="utf-8")
    sys.argv = [
        "render_recipe.py",
        "--out",
        str(out),
        "--goal-file",
        str(goal),
        "--contract-file",
        str(contract),
        "--model",
        "m",
    ]
    mod.main()
    raw = out.read_bytes()
    text = raw.decode("utf-8")
    assert "—".encode("utf-8") in raw
    assert "Do not modify any file other than `src/solve.rs`" in text
    assert "The only verification you should perform is `cargo build`" in text


def test_tier2_decodes_cargo_output_as_utf8() -> None:
    text = (ROOT / "runner" / "lib" / "tier2.py").read_text(encoding="utf-8")
    assert 'encoding="utf-8"' in text
    assert 'env.pop("CARGO_TARGET_DIR", None)' in text
    assert 'Path(target_dir) / "debug"' in text
    assert 'ENTITY_REFS' in text


def test_host_cache_is_outside_the_repo() -> None:
    ps1 = (ROOT / "runner" / "grade.ps1").read_text(encoding="utf-8")
    sh = (ROOT / "runner" / "grade.sh").read_text(encoding="utf-8")
    assert '.cache\\host-target' not in ps1
    assert ".cache/host-target" not in sh
    assert "LOCALAPPDATA" in ps1
    assert "XDG_CACHE_HOME" in sh
    assert "Remove-Item Env:CARGO_TARGET_DIR" in ps1
    assert "unset CARGO_TARGET_DIR" in sh
    ignore = (REPO / ".dockerignore").read_text(encoding="utf-8")
    assert "mcp/evals/.cache" in ignore
    assert "mcp/evals/fixture_gen" in ignore


def test_min_lift_treats_missing_tool_calls_as_zero(tmp_path: Path) -> None:
    from importlib.machinery import SourceFileLoader

    loader = SourceFileLoader("scorecard_mod", str(ROOT / "runner" / "scorecard.py"))
    mod = loader.load_module()
    root = tmp_path / "matrix"
    for name, arm, passed, tools in (
        ("pf01_A_r1", "A", False, 3),
        ("pf01_B_r1", "B", True, None),
    ):
        attempt = root / name
        attempt.mkdir(parents=True)
        (attempt / "result.json").write_text(
            json.dumps(
                {
                    "problem": "pf01_vm_balloon_swapin_rate",
                    "arm": arm,
                    "compiled": True,
                    "retrieval_pass": passed,
                    "tool_calls": tools,
                    "vim_rs_tool_calls": None,
                }
            ),
            encoding="utf-8",
        )
    out = root / "scorecard.json"
    sys.argv = ["scorecard.py", "--min-lift", "0.20", str(root), str(out), "test-model", "1"]
    try:
        mod.main()
    except SystemExit as exc:
        raise AssertionError(f"expected lift to pass, exited {exc.code}") from exc
    card = json.loads(out.read_text(encoding="utf-8"))
    assert card["model"] == "test-model"
    assert card["lift"]["pass"] is True
    assert card["lift"]["mean_tool_calls_a"] == 3.0
    assert card["lift"]["mean_tool_calls_b"] == 0.0
    assert card["lift"]["retrieval_delta"] == 1.0
    assert card["lift"]["mean_vim_rs_tool_calls_b"] == 0.0

    sys.argv = ["scorecard.py", "--min-lift", "0.20", str(root), str(out), "test-model", "1"]
    # Same numbers pass. A zero-lift copy must fail.
    weak = root / "pf01_B_r1" / "result.json"
    weak.write_text(
        json.dumps(
            {
                "problem": "pf01_vm_balloon_swapin_rate",
                "arm": "B",
                "compiled": True,
                "retrieval_pass": False,
                "tool_calls": 0,
            }
        ),
        encoding="utf-8",
    )
    try:
        mod.main()
    except SystemExit as exc:
        assert exc.code == 1
    else:
        raise AssertionError("expected min-lift miss to exit 1")


def test_challenge_starts_follow_the_contract() -> None:
    starts = ROOT / "runner" / "starts"
    problems = sorted(p.stem for p in (ROOT / "problems").glob("*.yaml"))
    assert problems
    for stem in problems:
        text = (starts / f"{stem}.rs").read_text(encoding="utf-8")
        if stem.startswith("pf"):
            assert "vim_retrievable" not in text
            assert "pub async fn " in text
            assert (ROOT / "runner" / "agent_emit" / f"{stem}.rs").is_file()
        else:
            assert "vim_retrievable!" in text
    ps1 = (ROOT / "runner" / "run_one.ps1").read_text(encoding="utf-8")
    sh = (ROOT / "runner" / "run_one.sh").read_text(encoding="utf-8")
    assert "Install-ChallengeStart" in ps1
    assert "install_challenge_start" in sh


def test_docs_mount_agent_dir_only() -> None:
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    ps1 = (ROOT / "runner" / "run_one.ps1").read_text(encoding="utf-8")
    assert "output directory" not in readme
    assert "agent/` directory at `/out`" in readme
    assert "output directory" not in ps1
    assert "agent/ directory" in ps1


if __name__ == "__main__":
    import tempfile

    test_scaffold_has_no_oracle_generator()
    test_image_builds_only_the_agent_bin()
    test_runner_mounts_agent_dir_only()
    test_tier2_decodes_cargo_output_as_utf8()
    test_host_cache_is_outside_the_repo()
    test_docs_mount_agent_dir_only()
    test_challenge_starts_follow_the_contract()
    with tempfile.TemporaryDirectory() as raw:
        base = Path(raw)
        test_scorecard_skips_grade_result(base / "score")
        test_min_lift_treats_missing_tool_calls_as_zero(base / "lift")
        test_seal_hash_ignores_solve_and_sees_other_edits(base / "seal")
        test_recipe_with_em_dash_is_utf8(base / "recipe")
    print("ok")
