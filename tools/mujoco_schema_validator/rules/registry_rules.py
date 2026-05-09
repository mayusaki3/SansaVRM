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
    """Registry package の必須構造を検証する。

    Args:
        registry_package:
            MuJoCo Schema Registry package。
        diagnostics_emitter:
            Diagnostics emitter。
    """

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
                message=f"Missing required field: {field_name}",
                trace_id="trace_mujoco_sdv_registry_002",
            )

    entries = registry_package.get("entries")
    if "entries" in registry_package and not isinstance(entries, list):
        diagnostics_emitter.emit(
            code="SCHEMA_INVALID_VALUE_TYPE",
            message="entries field must be a list.",
            trace_id="trace_mujoco_sdv_registry_002",
        )


# trace_id: trace_mujoco_sdv_io_scope_003
# trace_id: trace_mujoco_sdv_io_scope_004
# responsibility: Validate MJCF mapping definitions.
def validate_mjcf_mapping(entry: dict[str, Any]) -> bool:
    """mjcf_mapping が最小構造を満たすか検証する。

    Args:
        entry:
            registry entry。

    Returns:
        bool:
            有効なら True。
    """

    mapping = entry.get("mjcf_mapping")

    if not isinstance(mapping, dict):
        return False

    required_fields = ["element", "path", "direction", "required_mujoco_version"]
    if any(field_name not in mapping for field_name in required_fields):
        return False

    return mapping.get("direction") in {"import", "export", "import_export"}


# trace_id: trace_mujoco_sdv_io_scope_003
# responsibility: Validate adapter artifact definitions.
def validate_adapter_artifact(entry: dict[str, Any]) -> bool:
    """adapter_artifact が最小構造を満たすか検証する。

    Args:
        entry:
            registry entry。

    Returns:
        bool:
            有効なら True。
    """

    artifact = entry.get("adapter_artifact")

    if not isinstance(artifact, dict):
        return False

    required_fields = [
        "artifact_type",
        "path",
        "direction",
        "required_adapter_version",
    ]
    if any(field_name not in artifact for field_name in required_fields):
        return False

    return artifact.get("direction") in {"import", "export", "import_export"}
