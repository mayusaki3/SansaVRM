# SansaVRM coverage runner.
# TODO(trace): CI仕様 / Coverage

$ErrorActionPreference = "Stop"

Write-Host "SansaVRM coverage start"

cargo llvm-cov --workspace --html

Write-Host "SansaVRM coverage complete"
Write-Host "HTML report: target/llvm-cov/html/index.html"
