"""io_scope validation rules."""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter
from mujoco_schema_validator.rules.registry_rules import (
    validate_adapter_artifact,
    validate_mjcf_mapping,
)


VALID_IO_SCOPES = {
    "mjcf",
    "adapter_artifact",
    "both",
    "preserve_only",
    "unsupported",
    "source_raw",
}


def _build_target(entry: dict[str, Any]) -> dict[str, Any]:
    """Diagnostics target 情報を生成する。"""

    namespace = entry.get("namespace")
    name = entry.get("name")
    target_type = entry.get("target_type")

    return {
        "entry_key": f"{namespace}.{name}.{target_type}",
        "target_type": target_type,
    }


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
    """registry entry の io_scope 整合性を検証する。

    Args:
        entry:
            registry entry。
        diagnostics_emitter:
            Diagnostics emitter。
    """

    io_scope = entry.get("io_scope")
    target = _build_target(entry)

    if io_scope not in VALID_IO_SCOPES:
        diagnostics_emitter.emit(
            code="SCHEMA_INVALID_IO_SCOPE",
            message=f"Unsupported io_scope: {io_scope}",
            trace_id="trace_mujoco_sdv_io_scope_001",
            target=target,
        )
        return

    if io_scope == "mjcf":
        if not validate_mjcf_mapping(entry):
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_MAPPING",
                message="mjcf_mapping is required and must be valid.",
                trace_id="trace_mujoco_sdv_io_scope_002",
                target=target,
            )

        if "adapter_artifact" in entry:
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_ARTIFACT_RULE",
                severity="warning",
                message="adapter_artifact should not exist for mjcf scope.",
                trace_id="trace_mujoco_sdv_io_scope_003",
                target=target,
            )

    elif io_scope == "adapter_artifact":
        if not validate_adapter_artifact(entry):
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_ARTIFACT_RULE",
                message="adapter_artifact is required and must be valid.",
                trace_id="trace_mujoco_sdv_io_scope_003",
                target=target,
            )

        if "mjcf_mapping" in entry:
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_MAPPING",
                severity="warning",
                message="mjcf_mapping should not exist for adapter_artifact scope.",
                trace_id="trace_mujoco_sdv_io_scope_004",
                target=target,
            )

    elif io_scope == "both":
        mapping_valid = validate_mjcf_mapping(entry)
        artifact_valid = validate_adapter_artifact(entry)

        if not mapping_valid:
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_MAPPING",
                message="both scope requires valid mjcf_mapping.",
                trace_id="trace_mujoco_sdv_io_scope_004",
                target=target,
            )

        if not artifact_valid:
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_ARTIFACT_RULE",
                message="both scope requires valid adapter_artifact.",
                trace_id="trace_mujoco_sdv_io_scope_004",
                target=target,
            )

    elif io_scope == "preserve_only":
        if "mjcf_mapping" in entry or "adapter_artifact" in entry:
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_IO_SCOPE",
                severity="warning",
                message="preserve_only must not define export mappings.",
                trace_id="trace_mujoco_sdv_io_scope_005",
                target=target,
            )

    elif io_scope == "unsupported":
        diagnostics_emitter.emit(
            code="UNSUPPORTED_PARAMETER",
            severity="warning",
            message="Parameter is explicitly unsupported.",
            trace_id="trace_mujoco_sdv_io_scope_006",
            target=target,
        )

    elif io_scope == "source_raw":
        if "mjcf_mapping" in entry or "adapter_artifact" in entry:
            diagnostics_emitter.emit(
                code="SCHEMA_INVALID_IO_SCOPE",
                severity="warning",
                message="source_raw must not define export mappings.",
                trace_id="trace_mujoco_sdv_io_scope_007",
                target=target,
            )

        diagnostics_emitter.emit(
            code="SOURCE_RAW_STORED",
            severity="info",
            message="Source raw value preserved.",
            trace_id="trace_mujoco_sdv_io_scope_007",
            target=target,
        )
