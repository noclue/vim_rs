# Sequential matrix. Each attempt is its own container.
$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\EvalRunner.ps1"

function Show-Usage {
    Write-Host "Usage: .\runner\run_matrix.ps1 --problems pc01,pc02 --arms A,B --repeats N --model MODEL"
    exit 2
}

try {
    $opt = ConvertFrom-EvalArgs -InputArgs $args -Switches @("help")
} catch {
    Write-Host $_.Exception.Message
    Show-Usage
}
if ($opt.ContainsKey("help") -or -not $opt.ContainsKey("problems") -or -not $opt.ContainsKey("model")) {
    Show-Usage
}

$Arms = if ($opt.ContainsKey("arms")) { $opt["arms"] } else { "A,B" }
$Repeats = if ($opt.ContainsKey("repeats")) { [int]$opt["repeats"] } else { 3 }
$Model = $opt["model"]
$EvalRoot = Get-EvalRoot
Import-RepoDotEnv $EvalRoot

$Stamp = Get-Date -Format "yyyyMMdd_HHmmss"
$MatrixDir = Join-Path $EvalRoot "results\matrix_$Stamp"
New-Item -ItemType Directory -Force -Path $MatrixDir | Out-Null

foreach ($problem in $opt["problems"].Split(",")) {
    foreach ($arm in $Arms.Split(",")) {
        for ($r = 1; $r -le $Repeats; $r++) {
            $out = Join-Path $MatrixDir "${problem}_${arm}_r$r"
            Write-Host "matrix: $problem arm=$arm repeat=$r"
            & powershell -NoProfile -File "$PSScriptRoot\run_one.ps1" --problem $problem.Trim() --arm $arm.Trim() --repeat "$r" --model $Model --out $out
            if ($LASTEXITCODE -ne 0) {
                Write-Host "run_one exited $LASTEXITCODE for $problem arm=$arm repeat=$r"
            }
        }
    }
}

& (Get-PythonExe) (Join-Path $EvalRoot "runner\scorecard.py") $MatrixDir (Join-Path $MatrixDir "scorecard.json") $Model "$Repeats"
Write-Host "matrix complete: $MatrixDir"
exit 0
