# One agent attempt inside the eval image.
# The container sees only the sealed work tree and this attempt's agent/ directory.
# Problems, fixtures, grade output, and reference solutions stay on the host.
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\EvalRunner.ps1"

function Show-Usage {
    Write-Host "Usage: .\runner\run_one.ps1 --problem ID --arm A|B --repeat N --model MODEL --out DIR [--image NAME]"
    exit 2
}

try {
    $opt = ConvertFrom-EvalArgs -InputArgs $args -Switches @("help")
} catch {
    Write-Host $_.Exception.Message
    Show-Usage
}
if ($opt.ContainsKey("help") -or -not $opt.ContainsKey("problem") -or -not $opt.ContainsKey("model") -or -not $opt.ContainsKey("out")) {
    Show-Usage
}

$Problem = $opt["problem"]
$Arm = if ($opt.ContainsKey("arm")) { $opt["arm"] } else { "A" }
$Repeat = if ($opt.ContainsKey("repeat")) { [int]$opt["repeat"] } else { 1 }
$Model = $opt["model"]
$Image = if ($opt.ContainsKey("image")) { $opt["image"] } elseif ($env:EVAL_IMAGE) { $env:EVAL_IMAGE } else { "vim-rs-eval" }
if ($Arm -ne "A" -and $Arm -ne "B") { Show-Usage }

$EvalRoot = Get-EvalRoot
Import-RepoDotEnv $EvalRoot
$Python = Get-PythonExe
$Provider = if ($env:EVAL_PROVIDER) { $env:EVAL_PROVIDER } else { "openrouter" }

if ($Provider -eq "openrouter") {
    if ([string]::IsNullOrEmpty($env:OPENROUTER_API_KEY)) {
        Write-Host "OPENROUTER_API_KEY required for agent arms"
        New-Item -ItemType Directory -Force -Path $opt["out"] | Out-Null
        Write-ResultJson -OutFile (Join-Path $opt["out"] "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -FailureClass "no_output"
        exit 6
    }
} elseif ($Provider -eq "openai") {
    if ([string]::IsNullOrEmpty($env:OPENAI_HOST)) {
        Write-Host "OPENAI_HOST required when EVAL_PROVIDER=openai"
        New-Item -ItemType Directory -Force -Path $opt["out"] | Out-Null
        Write-ResultJson -OutFile (Join-Path $opt["out"] "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -FailureClass "no_output"
        exit 6
    }
    if ([string]::IsNullOrEmpty($env:OPENAI_API_KEY)) { $env:OPENAI_API_KEY = "none" }
    if ([string]::IsNullOrEmpty($env:OPENAI_BASE_PATH)) { $env:OPENAI_BASE_PATH = "v1/chat/completions" }
} else {
    Write-Host "unsupported EVAL_PROVIDER: $Provider"
    exit 2
}

$Out = $opt["out"]
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$Out = (Resolve-Path $Out).Path

$ProblemFile = Join-Path $EvalRoot "problems\$Problem.yaml"
$GoalFile = Join-Path $Out "goal.txt"
$ContractFile = Join-Path $Out "contract.txt"
& $Python -c "import sys; from pathlib import Path; sys.path.insert(0, sys.argv[1]); from load_problem import load_problem; d=load_problem(Path(sys.argv[2])); Path(sys.argv[3]).write_text(d['goal'], encoding='utf-8'); Path(sys.argv[4]).write_text(d['contract'], encoding='utf-8')" (Join-Path $EvalRoot "runner\lib") $ProblemFile $GoalFile $ContractFile
if ($LASTEXITCODE -ne 0) { throw "failed to write goal and contract" }

# The container sees only this directory: the recipe it must follow, plus the
# trace and solve.rs it writes. grade/, goal notes, and a previous attempt's
# problem.json stay outside the mount.
$AgentDir = Join-Path $Out "agent"
if (Test-Path $AgentDir) { Remove-Item -LiteralPath $AgentDir -Recurse -Force }
New-Item -ItemType Directory -Force -Path $AgentDir | Out-Null

$RenderArgs = @(
    (Join-Path $EvalRoot "runner\lib\render_recipe.py"),
    "--out", (Join-Path $AgentDir "recipe.yaml"),
    "--goal-file", $GoalFile,
    "--contract-file", $ContractFile,
    "--model", $Model,
    "--provider", $Provider
)
if ($Arm -eq "B") { $RenderArgs += "--mcp" }
& $Python @RenderArgs
if ($LASTEXITCODE -ne 0) { throw "render_recipe failed" }

$Work = Join-Path $Out "work"
Initialize-Work $Work
Install-ChallengeStart $Work $Problem

$Trace = Join-Path $AgentDir "trace.jsonl"
$WallStart = Get-Date
$MaxTurns = if ($env:GOOSE_MAX_TURNS) { $env:GOOSE_MAX_TURNS } else { "40" }

$PodmanArgs = @(
    "run", "--rm",
    "--network=pasta",
    "-e", "GOOSE_PROVIDER=$Provider",
    "-e", "GOOSE_MODEL=$Model",
    "-e", "GOOSE_MODE=auto",
    "-e", "GOOSE_DISABLE_SESSION_NAMING=true",
    "-e", "GOOSE_MAX_TURNS=$MaxTurns",
    "-e", "CARGO_NET_OFFLINE=true",
    "-e", "VIM_MCP_MODEL_CACHE=/opt/vim_mcp_model_cache",
    "-v", "${Work}:/work",
    "-v", "${AgentDir}:/out",
    "-w", "/work",
    $Image,
    "sh", "-c", "goose run --recipe /out/recipe.yaml --no-session --output-format stream-json > /out/trace.jsonl; ec=`$?; cp -f /work/src/solve.rs /out/solve.rs 2>/dev/null || true; exit `$ec"
)
$ProviderEnv = if ($Provider -eq "openrouter") {
    @("-e", "OPENROUTER_API_KEY")
} else {
    @("-e", "OPENAI_HOST=$($env:OPENAI_HOST)", "-e", "OPENAI_BASE_PATH=$($env:OPENAI_BASE_PATH)", "-e", "OPENAI_API_KEY")
}
$PodmanArgs = $PodmanArgs[0..2] + $ProviderEnv + $PodmanArgs[3..($PodmanArgs.Length - 1)]
if ($env:GOOSE_MAX_TOKENS) {
    $PodmanArgs = $PodmanArgs[0..2] + @("-e", "GOOSE_MAX_TOKENS=$($env:GOOSE_MAX_TOKENS)") + $PodmanArgs[3..($PodmanArgs.Length - 1)]
}
if ($env:EVAL_COST_CEILING) {
    $PodmanArgs = $PodmanArgs[0..1] + @("-e", "EVAL_COST_CEILING=$($env:EVAL_COST_CEILING)") + $PodmanArgs[2..($PodmanArgs.Length - 1)]
}

& podman @PodmanArgs
$GooseRc = $LASTEXITCODE
$WallS = [string][int]((Get-Date) - $WallStart).TotalSeconds

function Get-TraceField([string]$Json, [string]$Name) {
    $python = Get-PythonExe
    $tmp = Join-Path $env:TEMP ("eval-metrics-" + [guid]::NewGuid().ToString("N") + ".json")
    [System.IO.File]::WriteAllText($tmp, $Json)
    $value = & $python -c "import json,sys; m=json.load(open(sys.argv[1], encoding='utf-8-sig')); v=m.get(sys.argv[2]); print('null' if v is None else v)" $tmp $Name
    Remove-Item -LiteralPath $tmp -Force -ErrorAction SilentlyContinue
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($value)) { return "null" }
    return $value.Trim()
}

if (Test-Path $Trace) { Copy-Item -LiteralPath $Trace -Destination (Join-Path $Out "trace.jsonl") -Force }

if ($GooseRc -ne 0) {
    $metrics = "{}"
    $Failure = "no_output"
    if (Test-Path $Trace) {
        $metrics = & $Python (Join-Path $EvalRoot "runner\parse_trace.py") $Trace
        if ((Get-TraceField $metrics "turns") -ne "null") { $Failure = "timeout" }
    }
    Write-ResultJson -OutFile (Join-Path $Out "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat `
        -FailureClass $Failure -WallS $WallS `
        -Turns (Get-TraceField $metrics "turns") `
        -ToolCalls (Get-TraceField $metrics "tool_calls") `
        -VimRsToolCalls (Get-TraceField $metrics "vim_rs_tool_calls") `
        -TokensIn (Get-TraceField $metrics "tokens_in") `
        -TokensOut (Get-TraceField $metrics "tokens_out")
    exit 7
}

$SolveSrc = Join-Path $AgentDir "solve.rs"
if (-not (Test-Path $SolveSrc)) {
    $FromWork = Join-Path $Work "src\solve.rs"
    if (Test-Path $FromWork) { Copy-Item $FromWork $SolveSrc }
}
if (Test-Path $SolveSrc) { Copy-Item -LiteralPath $SolveSrc -Destination (Join-Path $Out "solve.rs") -Force }
if (-not (Test-Path $SolveSrc)) {
    Write-ResultJson -OutFile (Join-Path $Out "result.json") -Problem $Problem -Arm $Arm -Repeat $Repeat -FailureClass "no_output" -WallS $WallS
    exit 8
}

$GradeOut = Join-Path $Out "grade"
& powershell -NoProfile -File "$PSScriptRoot\grade.ps1" --problem $Problem --solve $SolveSrc --out $GradeOut --arm $Arm --repeat "$Repeat" --check-tamper --agent-work $Work
if ($LASTEXITCODE -ne 0) {
    Write-Host "grade exited $LASTEXITCODE"
}

$GradeJson = Join-Path $GradeOut "result.json"
& $Python (Join-Path $EvalRoot "runner\lib\merge_result.py") (Join-Path $Out "result.json") $GradeJson $WallS $Trace
if ($LASTEXITCODE -ne 0) { throw "failed to merge result.json" }

Write-Host "run_one OK: $(Join-Path $Out 'result.json')"
exit 0
