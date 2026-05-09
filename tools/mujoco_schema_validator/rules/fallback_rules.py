"""Fallback evaluation rules."""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter


# trace_id: trace_mujoco_sdv_fallback_001
# trace_id: trace_mujoco_sdv_fallback_002
# responsibility: Evaluate fallback behavior.
def evaluate_fallback(
    *,
    entry: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> dict[str, Any] | None:
    fallback = entry.get("fallback")

    if fallback is None:
        return None

    diagnostics_emitter.emit(
        code="FALLBACK_DEFAULT_APPLIED",
        severity="info",
        message="Fallback applied.",
        trace_id="trace_mujoco_sdv_fallback_001",
    )

    namespace = entry.get("namespace")
    name = entry.get("name")
    target_type = entry.get("target_type")

    entry_key = f"{namespace}.{name}.{target_type}"

    return {
        "entry_key": entry_key,
        "fallback_behavior": fallback.get("behavior"),
    }
