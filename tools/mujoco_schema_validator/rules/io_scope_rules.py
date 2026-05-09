"""io_scope validation rules."""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter
from mujoco_schema_validator.rules.registry_rules import (
    validate_adapter_artifact,
    validate_mjcf_mapping,
)


# trace_id: trace_mujoco_sdv_io_scope_001
# trace_id: trace_mujoco_sdv_io_scope_002
# trace_id: trace_mujoco_sdv_io_scope_003
# trace_id: trace_mujoco_sdv_io_scope_004
# trace_id: trace_mujoco_sdv_io_scope_005
# trace_id: trace_mujoco_sdv_io_scope_006
# trace_id: trace_mujoco_sdv_io_scope_007
# responsibility: Validate io_scope consistency.
def validate_io_scope_consistency(
    *,
    entry: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    io_scope = entry.get("io_scope")

    if io_scope == "mjcf":
        if not validate_mjcf_mapping(entry):
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_IO_SCOPE",
                severity="error",
                message="mjcf_mapping is required.",
                trace_id="trace_mujoco_sdv_io_scope_002",
                output_action="block_output",
            )

    elif io_scope == "adapter_artifact":
        if not validate_adapter_artifact(entry):
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_ARTIFACT_RULE",
                severity="error",
                message="adapter_artifact is required.",
                trace_id="trace_mujoco_sdv_io_scope_003",
                output_action="block_output",
            )

    elif io_scope == "source_raw":
        diagnostics_emitter.emit(
            code="SOURCE_RAW_STORED",
            severity="info",
            message="Source raw value preserved.",
            trace_id="trace_mujoco_sdv_io_scope_007",
        )
