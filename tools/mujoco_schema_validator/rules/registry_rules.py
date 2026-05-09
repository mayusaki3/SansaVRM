"""Registry validation rules.

役割:
    Schema Registry の構造と mapping / artifact を検証する。
"""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter


# trace_id: trace_mujoco_sdv_registry_001
# trace_id: trace_mujoco_sdv_registry_002
# responsibility: Validate registry package structure.
def validate_registry_structure(
    *,
    registry_package: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    required_fields = [
        "schema_id",
        "schema_version",
        "runtime",
        "entries",
    ]

    for field_name in required_fields:
        if field_name not in registry_package:
            diagnostics_emitter.emit(
                code="SCHEMA_MISSING_REQUIRED_FIELD",
                severity="error",
                message=f"Missing required field: {field_name}",
                trace_id="trace_mujoco_sdv_registry_002",
                output_action="block_output",
            )


# trace_id: trace_mujoco_sdv_io_scope_003
# trace_id: trace_mujoco_sdv_io_scope_004
# responsibility: Validate MJCF mapping definitions.
def validate_mjcf_mapping(entry: dict[str, Any]) -> bool:
    return "mjcf_mapping" in entry


# trace_id: trace_mujoco_sdv_io_scope_003
# responsibility: Validate adapter artifact definitions.
def validate_adapter_artifact(entry: dict[str, Any]) -> bool:
    return "adapter_artifact" in entry
