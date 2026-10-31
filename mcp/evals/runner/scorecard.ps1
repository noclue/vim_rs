$ErrorActionPreference = "Stop"
. "$PSScriptRoot\lib\EvalRunner.ps1"
$EvalRoot = Get-EvalRoot
& (Get-PythonExe) (Join-Path $EvalRoot "runner\scorecard.py") @args
exit $LASTEXITCODE
