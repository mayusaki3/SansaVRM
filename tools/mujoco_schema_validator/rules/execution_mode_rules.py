"""Execution mode rules for strict / permissive validation.

役割:
    strict / permissive mode に応じた成果物出力可否を判定する。
"""

from __future__ import annotations

from typing import Any


RECOVERABLE_CATEGORIES = {
    "FALLBACK",
    "LOSSY",
    "UNSUPPORTED",
    "SOURCE_RAW",
    "ADAPTER_CAPABILITY",
}

FATAL_CATEGORIES = {
    "INTERNAL",
    "TRACEABILITY",
}


# trace_id: trace_mujoco_sdv_execution_001
# trace_id: trace_mujoco_sdv_execution_002
# responsibility: Determine output_allowed with strict/permissive behavior.
def determine_output_allowed(
    *,
    diagnostics: list[dict[str, Any]],
    strict: bool,
) -> bool:
    """Diagnostics から成果物出力可否を判定する。

    Args:
        diagnostics:
            Diagnostics 配列。
        strict:
            strict mode の有無。

    Returns:
        bool:
            成果物出力可能なら True。
    """

    for diagnostic in diagnostics:
        category = diagnostic.get("category")
        if category in FATAL_CATEGORIES:
            return False

        if diagnostic.get("output_action") == "block_output":
            return False

    if strict:
        return _determine_strict_output_allowed(diagnostics)

    return _determine_permissive_output_allowed(diagnostics)


def _determine_strict_output_allowed(diagnostics: list[dict[str, Any]]) -> bool:
    """strict mode の成果物出力可否を判定する。"""

    for diagnostic in diagnostics:
        if diagnostic.get("severity") == "error":
            return False

        if diagnostic.get("strict_block", False):
            return False

    return True


def _determine_permissive_output_allowed(diagnostics: list[dict[str, Any]]) -> bool:
    """permissive mode の成果物出力可否を判定する。

    permissive mode では recoverable category の error を許容できる。
    unrecoverable schema / artifact / validation error は禁止する。
    """

    for diagnostic in diagnostics:
        severity = diagnostic.get("severity")
        category = diagnostic.get("category")
        strict_block = diagnostic.get("strict_block", False)

        if severity != "error" and not strict_block:
            continue

        if category in RECOVERABLE_CATEGORIES:
            continue

        return False

    return True
