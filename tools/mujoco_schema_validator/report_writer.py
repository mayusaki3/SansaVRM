"""Conversion report builder.

役割:
    Diagnostics と validation 結果から Conversion Report を構築する。
    severity、category、fallback、lossy、preserved、CI exit code を集計する。
"""

from __future__ import annotations

from collections import Counter
from typing import Any


CATEGORY_EXIT_CODE_PRIORITY = [
    ("INTERNAL", 3),
    ("TRACEABILITY", 4),
    ("SCHEMA", 5),
    ("ADAPTER_CAPABILITY", 6),
    ("ARTIFACT", 7),
    ("VALIDATION", 5),
    ("LOSSY", 1),
    ("FALLBACK", 1),
    ("UNSUPPORTED", 1),
    ("SOURCE_RAW", 1),
]


def _count_by_key(items: list[dict[str, Any]], key: str) -> dict[str, int]:
    """dict 配列を指定 key で集計する。

    Args:
        items:
            集計対象。
        key:
            集計 key。

    Returns:
        dict[str, int]:
            key 値ごとの件数。
    """

    return dict(Counter(str(item.get(key)) for item in items if item.get(key)))


def _build_diagnostics_summary(
    diagnostics: list[dict[str, Any]],
    fallback_results: list[dict[str, Any]],
) -> dict[str, Any]:
    """Diagnostics summary を生成する。

    Args:
        diagnostics:
            Diagnostics 配列。
        fallback_results:
            fallback 結果配列。

    Returns:
        dict[str, Any]:
            diagnostics summary。
    """

    severity_counts = _count_by_key(diagnostics, "severity")
    category_counts = _count_by_key(diagnostics, "category")
    strict_block_count = sum(1 for d in diagnostics if d.get("strict_block"))
    blocked_count = sum(
        1 for d in diagnostics if d.get("output_action") == "block_output"
    )
    lossy_count = sum(1 for d in diagnostics if d.get("category") == "LOSSY")
    preserved_count = sum(
        1
        for d in diagnostics
        if d.get("category") in {"SOURCE_RAW", "UNSUPPORTED"}
    )

    return {
        "info_count": severity_counts.get("info", 0),
        "warning_count": severity_counts.get("warning", 0),
        "error_count": severity_counts.get("error", 0),
        "blocked_count": blocked_count,
        "strict_block_count": strict_block_count,
        "fallback_count": len(fallback_results),
        "lossy_count": lossy_count,
        "preserved_count": preserved_count,
        "severity_counts": severity_counts,
        "category_counts": category_counts,
    }


def _determine_ci_exit_code(
    diagnostics: list[dict[str, Any]],
    output_allowed: bool,
) -> int:
    """Diagnostics から CI exit code を決定する。

    Args:
        diagnostics:
            Diagnostics 配列。
        output_allowed:
            成果物出力可否。

    Returns:
        int:
            CI exit code。
    """

    if not diagnostics:
        return 0

    if not output_allowed:
        for category, exit_code in CATEGORY_EXIT_CODE_PRIORITY:
            if any(d.get("category") == category for d in diagnostics):
                return exit_code
        return 2

    if any(d.get("severity") == "warning" for d in diagnostics):
        return 1

    return 0


def _build_preserved_results(diagnostics: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Diagnostics から preserved_results を生成する。"""

    preserved_results: list[dict[str, Any]] = []

    for diagnostic in diagnostics:
        code = diagnostic.get("code")
        category = diagnostic.get("category")

        if category not in {"SOURCE_RAW", "UNSUPPORTED"}:
            continue

        preserve_type = "source_raw" if category == "SOURCE_RAW" else "unsupported_preserved"
        target = diagnostic.get("target", {})

        preserved_results.append(
            {
                "entry_key": target.get("entry_key"),
                "target_type": target.get("target_type"),
                "preserve_type": preserve_type,
                "diagnostic_id": diagnostic.get("diagnostic_id"),
                "code": code,
            }
        )

    return preserved_results


def _build_lossy_results(diagnostics: list[dict[str, Any]]) -> list[dict[str, Any]]:
    """Diagnostics から lossy_results を生成する。"""

    lossy_results: list[dict[str, Any]] = []

    for diagnostic in diagnostics:
        if diagnostic.get("category") != "LOSSY":
            continue

        target = diagnostic.get("target", {})
        lossy_results.append(
            {
                "entry_key": target.get("entry_key"),
                "target_type": target.get("target_type"),
                "loss_type": diagnostic.get("code"),
                "diagnostic_id": diagnostic.get("diagnostic_id"),
            }
        )

    return lossy_results


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
    """Conversion Report を構築する。

    Args:
        execution_config:
            実行設定。
        registry_package:
            Schema Registry package。
        capability_package:
            Adapter Capability package。
        parameter_results:
            parameter 判定結果。
        fallback_results:
            fallback 判定結果。
        diagnostics:
            Diagnostics 配列。
        output_allowed:
            成果物出力可否。

    Returns:
        dict[str, Any]:
            Conversion Report。
    """

    diagnostics_summary = _build_diagnostics_summary(
        diagnostics=diagnostics,
        fallback_results=fallback_results,
    )
    preserved_results = _build_preserved_results(diagnostics)
    lossy_results = _build_lossy_results(diagnostics)
    ci_exit_code = _determine_ci_exit_code(
        diagnostics=diagnostics,
        output_allowed=output_allowed,
    )

    return {
        "report_schema_version": "0.1.0",
        "source_format": execution_config.get("source_format"),
        "target_format": execution_config.get("target_format"),
        "mode": execution_config.get("mode"),
        "strict": execution_config.get("strict", True),
        "adapter": {
            "adapter_id": capability_package.get("adapter_id"),
            "adapter_version": capability_package.get("adapter_version"),
            "runtime": capability_package.get("runtime"),
        },
        "schema_registry": {
            "schema_id": registry_package.get("schema_id"),
            "schema_version": registry_package.get("schema_version"),
            "runtime": registry_package.get("runtime"),
        },
        "adapter_capability": {
            "capability_id": capability_package.get("capability_id"),
            "capability_version": capability_package.get("capability_version"),
            "adapter_id": capability_package.get("adapter_id"),
            "adapter_version": capability_package.get("adapter_version"),
        },
        "parameter_results": parameter_results,
        "fallback_results": fallback_results,
        "preserved_results": preserved_results,
        "lossy_results": lossy_results,
        "diagnostics_summary": diagnostics_summary,
        "diagnostics": diagnostics,
        "output_allowed": output_allowed,
        "ci_exit_code": ci_exit_code,
    }
