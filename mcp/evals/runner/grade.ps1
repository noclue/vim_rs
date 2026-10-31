# Offline grade. Runs on the host after the agent container has exited.
# Fixtures stay on the host; they are not visible to the agent.
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\EvalRunner.ps1"

function Repair-HostVimRsPath([string]$WorkDir, [string]$EvalRoot) {
    $CargoToml = Join-Path $WorkDir "Cargo.toml"
    $text = [System.IO.File]::ReadAllText($CargoToml)
    $needle = 'path = "../../../vim_rs"'
    if (-not $text.Contains($needle)) { return }
    $repoVim = (Resolve-Path (Join-Path $EvalRoot "..\..\vim_rs")).Path.Replace('\', '/')
    $text = $text.Replace($needle, "path = `"$repoVim`"")
    $utf8 = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($CargoToml, $text, $utf8)
}

function Show-Usage {
    Write-Host "Usage: .\runner\grade.ps1 --problem ID --solve PATH [--work-dir DIR] [--out DIR] [--arm A|B] [--repeat N] [--check-tamper] [--agent-work DIR]"
    exit 2
}

try {
    $opt = ConvertFrom-EvalArgs -InputArgs $args -Switches @("check-tamper", "help")
} catch {
    Write-Host $_.Exception.Message
    Show-Usage
}
if ($opt.ContainsKey("help") -or -not $opt.ContainsKey("problem") -or -not $opt.ContainsKey("solve")) {
    Show-Usage
}

$Problem = $opt["problem"]
$Solve = $opt["solve"]
$Arm = if ($opt.ContainsKey("arm")) { $opt["arm"] } else { "A" }
$Repeat = if ($opt.ContainsKey("repeat")) { [int]$opt["repeat"] } else { 1 }
$CheckTamper = [bool]$opt["check-tamper"]
$EvalRoot = Get-EvalRoot
$Python = Get-PythonExe

$ProblemFile = Join-Path $EvalRoot "problems\$Problem.yaml"
if (-not (Test-Path $ProblemFile)) {
    Write-Host "missing problem: $ProblemFile"
    exit 2
}

$DeleteWork = -not $opt.ContainsKey("work-dir")
if ($opt.ContainsKey("work-dir")) {
    $WorkDir = $opt["work-dir"]
} else {
    $WorkDir = Join-Path ([System.IO.Path]::GetTempPath()) ("eval-work-" + [guid]::NewGuid().ToString("N"))
}
if ($opt.ContainsKey("out")) {
    $OutDir = $opt["out"]
} else {
    $OutDir = Join-Path $EvalRoot "results\grade_${Problem}_$PID"
}
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$OutDir = (Resolve-Path $OutDir).Path

$EmitSrc = Join-Path $EvalRoot "runner\problem_emit\$Problem.rs"
if (-not (Test-Path $EmitSrc)) {
    $EmitSrc = Join-Path $EvalRoot "scaffold\src\problem_emit.rs"
}

# Hash the tree the agent actually edited. The compile tree below is a fresh
# scaffold plus solve.rs, so hashing it cannot see other agent edits.
$TamperDir = if ($opt.ContainsKey("agent-work")) { $opt["agent-work"] } else { $null }
if ($CheckTamper -and $TamperDir -and (Test-Tamper $TamperDir)) {
    Write-ResultJson -OutFile (Join-Path $OutDir "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -FailureClass "tamper" -Tamper $true
    if ($DeleteWork -and (Test-Path $WorkDir)) { Remove-Item -LiteralPath $WorkDir -Recurse -Force -ErrorAction SilentlyContinue }
    exit 3
}

Initialize-Work $WorkDir
Install-Solve $WorkDir $Solve
Install-ProblemEmit $WorkDir $EmitSrc
# The copied tree is not mcp/evals/scaffold, so ../../../vim_rs misses the repo.
# The image still compiles that relative path from /work. Rewrite only this copy.
Repair-HostVimRsPath $WorkDir $EvalRoot

$BuildLog = Join-Path $OutDir "build.log"
# Reuse one host target so later grades do not rebuild vim_rs. Keep it outside
# the repo: the graded binary's current_exe() is inside this directory, and a
# cache under mcp/evals can walk up to fixtures/ and fixture_gen/. cmd.exe owns
# the redirect: PowerShell 5 turns cargo's stderr warnings into a stuck
# error pipeline when ErrorAction is Stop.
if ($env:EVAL_HOST_TARGET) {
    $HostTarget = $env:EVAL_HOST_TARGET
} else {
    $CacheRoot = if ($env:LOCALAPPDATA) { $env:LOCALAPPDATA } else { Join-Path $env:USERPROFILE "AppData\Local" }
    $HostTarget = Join-Path $CacheRoot "vim-rs-eval\host-target"
}
New-Item -ItemType Directory -Force -Path $HostTarget | Out-Null
$env:CARGO_TARGET_DIR = $HostTarget
Push-Location $WorkDir
# Copies keep the source mtimes. A shared host target then treats the previous
# grade's binary as fresh even though solve.rs and problem_emit.rs changed.
cmd /c "cargo clean -p eval_scaffold >nul 2>&1"
cmd /c "cargo build --bin eval_scaffold > `"$BuildLog`" 2>&1"
$BuildRc = $LASTEXITCODE
Pop-Location
Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue
if ($BuildRc -ne 0) {
    if ($DeleteWork -and (Test-Path $WorkDir)) { Remove-Item -LiteralPath $WorkDir -Recurse -Force -ErrorAction SilentlyContinue }
    Write-ResultJson -OutFile (Join-Path $OutDir "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -FailureClass "compile_error"
    exit 4
}

$ProblemJsonFile = Join-Path $OutDir "problem.json"
& $Python -c "import json,sys; from pathlib import Path; sys.path.insert(0, sys.argv[1]); from load_problem import load_problem; p=load_problem(Path(sys.argv[2])); Path(sys.argv[3]).write_text(json.dumps(p), encoding='utf-8')" (Join-Path $EvalRoot "runner\lib") $ProblemFile $ProblemJsonFile
if ($LASTEXITCODE -ne 0) { throw "load_problem failed" }
$MoType = & $Python -c "import json,sys; print(json.load(open(sys.argv[1], encoding='utf-8-sig')).get('mo_type','VirtualMachine'))" $ProblemJsonFile
if ($LASTEXITCODE -ne 0) { throw "mo_type parse failed" }

& $Python (Join-Path $EvalRoot "runner\lib\tier2.py") $WorkDir (Join-Path $EvalRoot "fixtures") $ProblemJsonFile $MoType $HostTarget
$GradeRc = $LASTEXITCODE
if ($DeleteWork -and (Test-Path $WorkDir)) { Remove-Item -LiteralPath $WorkDir -Recurse -Force -ErrorAction SilentlyContinue }
if ($GradeRc -ne 0) {
    Write-ResultJson -OutFile (Join-Path $OutDir "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -Compiled $true -FailureClass "retrieval_fail"
    exit 5
}

Write-ResultJson -OutFile (Join-Path $OutDir "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -Compiled $true -RetrievalPass $true -FailureClass "null"
Write-Host "grade OK: $(Join-Path $OutDir 'result.json')"
exit 0
