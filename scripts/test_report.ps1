# scripts/test_report.ps1

param(
    [switch]$Full
)

if ($Full) {
    cargo test --workspace --color always
    exit $LASTEXITCODE
}

$ErrorActionPreference = "Continue"

Write-Host "=== SansaVRM Test Report ==="
Write-Host ""

$startTime = Get-Date

$cargoOutput = cargo test --workspace --color always 2>&1
$exitCode = $LASTEXITCODE

$endTime = Get-Date
$duration = $endTime - $startTime

$passed = ($cargoOutput | Select-String "test result: ok").Count
$failed = ($cargoOutput | Select-String "test result: FAILED").Count
$compileErrors = ($cargoOutput | Select-String "error\[E[0-9]+\]").Count
$warnings = ($cargoOutput | Select-String "warning:").Count

Write-Host "Summary"
Write-Host "-------"
Write-Host "ExitCode      : $exitCode"
Write-Host "Duration      : $($duration.ToString())"
Write-Host "Passed groups : $passed"
Write-Host "Failed groups : $failed"
Write-Host "Compile errors: $compileErrors"
Write-Host "Warnings      : $warnings"
Write-Host ""

$testLines = $cargoOutput |
    Select-String "^test .+ \.\.\. (ok|FAILED)$" |
    ForEach-Object {
        $line = $_.Line

        if ($line -match "^test (.+) \.\.\. ok$") {
            [PSCustomObject]@{
                Name = $Matches[1]
                Status = "ok"
            }
        }
        elseif ($line -match "^test (.+) \.\.\. FAILED$") {
            [PSCustomObject]@{
                Name = $Matches[1]
                Status = "failed"
            }
        }
    } |
    Sort-Object Name

Write-Host "Tests"
Write-Host "-----"

foreach ($test in $testLines) {
    if ($test.Status -eq "ok") {
        Write-Host "[OK]" -NoNewline -ForegroundColor Green
    }
    else {
        Write-Host "[NG]" -NoNewline -ForegroundColor Red
    }
    Write-Host " $($test.Name)"
}

Write-Host ""

if ($exitCode -ne 0) {
    Write-Host "Failed / Error Lines"
    Write-Host "--------------------"

    $cargoOutput |
        Select-String "FAILED|error\[E[0-9]+\]|panicked at|failures:" |
        ForEach-Object {
            Write-Host $_.Line
        }

    Write-Host ""
}

exit $exitCode
