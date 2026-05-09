"""Validation Error Code consistency rules."""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter


# trace_id: trace_mujoco_sdv_error_code_001
# responsibility: Validate Validation Error Code consistency.
def validate_error_code_consistency(
    *,
    error_code_catalog: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    codes = error_code_catalog.get("codes")

    if not isinstance(codes, list):
        diagnostics_emitter.emit(
            code="SCHEMA_INVALID_VALUE_TYPE",
            severity="error",
            message="codes field must be a list.",
            trace_id="trace_mujoco_sdv_error_code_001",
            output_action="block_output",
        )
