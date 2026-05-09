"""Conversion report builder.

役割:
    Diagnostics と validation 結果から Conversion Report を構築する。
"""

from __future__ import annotations

from typing import Any


# trace_id: trace_mujoco_sdv_report_001
# responsibility: Build conversion report from validation results.
def build_conversion_report(
    *,
    execution_config: dict[str, Any],
    registry_package: dict[str, Any],
    capability_package: dict[str, Any],
    parameter_results: list[dict[str, Any]],
    fallback_results: list[dict[str, Any]],
    diagnostics: list[dict[str, Any]],
    output_allowed: bool,
) -> dict[str, Any]:
    """Conversion Report を構築する。"""

    info_count = sum(1 for d in diagnostics if d.get("severity") == "info")
    warning_count = sum(
        1 for d in diagnostics if d.get("severity") == "warning"
    )
    error_count = sum(1 for d in diagnostics if d.get("severity") == "error")

    return {
        "report_schema_version": "0.1.0",
        "source_format": execution_config.get("source_format"),
        "target_format": execution_config.get("target_format"),
        "strict": execution_config.get("strict", True),
        "adapter": {
            "adapter_id": capability_package.get("adapter_id"),
            "adapter_version": capability_package.get("adapter_version"),
            "runtime": capability_package.get("runtime"),
        },
        "schema_registry": {
            "schema_id": registry_package.get("schema_id"),
            "schema_version": registry_package.get("schema_version"),
        },
        "parameter_results": parameter_results,
        "fallback_results": fallback_results,
        "diagnostics_summary": {
            "info_count": info_count,
            "warning_count": warning_count,
            "error_count": error_count,
        },
        "diagnostics": diagnostics,
        "output_allowed": output_allowed,
    }
