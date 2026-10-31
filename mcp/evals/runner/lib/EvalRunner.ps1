# Shared helpers for the Windows eval runner.
# Dot-source from grade.ps1, run_one.ps1, and run_matrix.ps1.

Set-StrictMode -Version Latest

function Get-EvalRoot {
    return (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
}

function Get-PythonExe {
    foreach ($name in @("python", "py")) {
        $cmd = Get-Command $name -ErrorAction SilentlyContinue
        if ($cmd) { return $cmd.Source }
    }
    throw "Python 3 is required (python or py on PATH)."
}

function Invoke-Python {
    param(
        [Parameter(Mandatory = $true)][string]$Script,
        [Parameter(ValueFromRemainingArguments = $true)][string[]]$Arguments
    )
    $python = Get-PythonExe
    & $python $Script @Arguments
    if ($null -ne $LASTEXITCODE -and $LASTEXITCODE -ne 0) {
        throw "python exited $LASTEXITCODE ($Script)"
    }
}

function Import-RepoDotEnv {
    param([string]$EvalRoot)
    $repoRoot = (Resolve-Path (Join-Path $EvalRoot "..\..")).Path
    $path = Join-Path $repoRoot ".env"
    if (-not (Test-Path $path)) { return }
    foreach ($line in Get-Content -Path $path) {
        if ($line -match '^\s*#' -or $line -notmatch '^\s*([A-Za-z_][A-Za-z0-9_]*)=(.*)$') { continue }
        $name = $Matches[1]
        $value = $Matches[2].Trim().Trim('"').Trim("'")
        if ([string]::IsNullOrEmpty([Environment]::GetEnvironmentVariable($name, "Process"))) {
            Set-Item -Path "Env:$name" -Value $value
        }
    }
    if ([string]::IsNullOrEmpty($env:OPENROUTER_API_KEY) -and -not [string]::IsNullOrEmpty($env:OPEN_ROUTER_API_KEY)) {
        $env:OPENROUTER_API_KEY = $env:OPEN_ROUTER_API_KEY
    }
}

function Copy-EvalTree {
    param(
        [Parameter(Mandatory = $true)][string]$Source,
        [Parameter(Mandatory = $true)][string]$Destination
    )
    if (Test-Path $Destination) {
        Remove-Item -LiteralPath $Destination -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $Destination | Out-Null
    & robocopy $Source $Destination /E /XD target /NFL /NDL /NJH /NJS /NC /NS /NP | Out-Null
    if ($LASTEXITCODE -ge 8) {
        throw "robocopy failed copying $Source ($LASTEXITCODE)"
    }
}

function Get-SealHash {
    param([Parameter(Mandatory = $true)][string]$Root)
    $python = Get-PythonExe
    $script = Join-Path $PSScriptRoot "seal_hash.py"
    $hash = & $python $script $Root
    if ($LASTEXITCODE -ne 0) { throw "seal hash failed for $Root" }
    return ($hash | Select-Object -Last 1).Trim()
}

function Initialize-Work {
    param([Parameter(Mandatory = $true)][string]$WorkDir)
    $canonical = Join-Path (Get-EvalRoot) "scaffold"
    Copy-EvalTree -Source $canonical -Destination $WorkDir
}

function Install-Solve {
    param(
        [Parameter(Mandatory = $true)][string]$WorkDir,
        [Parameter(Mandatory = $true)][string]$SolveSrc
    )
    Copy-Item -LiteralPath $SolveSrc -Destination (Join-Path $WorkDir "src\solve.rs") -Force
}

function Install-ProblemEmit {
    param(
        [Parameter(Mandatory = $true)][string]$WorkDir,
        [Parameter(Mandatory = $true)][string]$EmitSrc
    )
    Copy-Item -LiteralPath $EmitSrc -Destination (Join-Path $WorkDir "src\problem_emit.rs") -Force
}

function Install-ChallengeStart {
    param(
        [Parameter(Mandatory = $true)][string]$WorkDir,
        [Parameter(Mandatory = $true)][string]$Problem
    )
    $root = Get-EvalRoot
    $start = Join-Path $root "runner\starts\$Problem.rs"
    if (Test-Path -LiteralPath $start) {
        Install-Solve $WorkDir $start
    }
    $agentEmit = Join-Path $root "runner\agent_emit\$Problem.rs"
    if (Test-Path -LiteralPath $agentEmit) {
        Install-ProblemEmit $WorkDir $agentEmit
        return
    }
    $gradeEmit = Join-Path $root "runner\problem_emit\$Problem.rs"
    if (Test-Path -LiteralPath $gradeEmit) {
        Install-ProblemEmit $WorkDir $gradeEmit
    }
}

function Test-Tamper {
    param([Parameter(Mandatory = $true)][string]$WorkDir)
    $canonical = Join-Path (Get-EvalRoot) "scaffold"
    $canon = Get-SealHash $canonical
    $work = Get-SealHash $WorkDir
    return ($canon -ne $work)
}

function Write-ResultJson {
    param(
        [Parameter(Mandatory = $true)][string]$OutFile,
        [Parameter(Mandatory = $true)][string]$Problem,
        [string]$Arm = "A",
        [int]$Repeat = 1,
        [bool]$Compiled = $false,
        [bool]$RetrievalPass = $false,
        [string]$FailureClass = "null",
        [bool]$Tamper = $false,
        [string]$Turns = "null",
        [string]$ToolCalls = "null",
        [string]$VimRsToolCalls = "null",
        [string]$TokensIn = "null",
        [string]$TokensOut = "null",
        [string]$WallS = "null"
    )
    $python = Get-PythonExe
    $dir = Split-Path $OutFile -Parent
    if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Force -Path $dir | Out-Null }
    $script = Join-Path $PSScriptRoot "write_result.py"
    & $python $script $OutFile $Problem $Arm "$Repeat" $(if ($Compiled) { "true" } else { "false" }) $(if ($RetrievalPass) { "true" } else { "false" }) $FailureClass $(if ($Tamper) { "true" } else { "false" }) $Turns $ToolCalls $VimRsToolCalls $TokensIn $TokensOut $WallS
    if ($LASTEXITCODE -ne 0) { throw "failed to write $OutFile" }
}

function ConvertFrom-EvalArgs {
    param(
        [string[]]$InputArgs,
        [string[]]$Switches = @()
    )
    $map = @{}
    for ($i = 0; $i -lt $InputArgs.Count; $i++) {
        $token = $InputArgs[$i]
        if ($token -eq "-h" -or $token -eq "--help") {
            $map["help"] = $true
            continue
        }
        if ($token -notmatch "^--") { throw "unknown arg: $token" }
        $key = $token.Substring(2)
        if ($Switches -contains $key) {
            $map[$key] = $true
            continue
        }
        $i++
        if ($i -ge $InputArgs.Count) { throw "missing value for --$key" }
        $map[$key] = $InputArgs[$i]
    }
    return $map
}
