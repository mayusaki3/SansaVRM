"""Runtime and adapter version validation rules.

役割:
    execution config、Schema Registry、Adapter Capability、registry entry の
    version compatibility を検証する。
"""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter


def _parse_version(version: str | None) -> tuple[int, ...] | None:
    """バージョン文字列を比較可能な tuple に変換する。

    Args:
        version:
            バージョン文字列。

    Returns:
        tuple[int, ...] | None:
            比較可能な tuple。変換不能な場合は None。
    """

    if not version:
        return None

    try:
        return tuple(int(part) for part in str(version).split("."))
    except ValueError:
        return None


def _is_version_in_range(
    version: str | None,
    version_range: dict[str, Any] | None,
) -> bool:
    """version が version_range に含まれるか判定する。

    Args:
        version:
            対象バージョン。
        version_range:
            {"min": ..., "max": ...} 形式の範囲。

    Returns:
        bool:
            範囲内なら True。
    """

    if version_range is None:
        return True

    parsed_version = _parse_version(version)
    if parsed_version is None:
        return False

    min_version = _parse_version(version_range.get("min"))
    max_version = _parse_version(version_range.get("max"))

    if min_version is not None and parsed_version < min_version:
        return False

    if max_version is not None and parsed_version > max_version:
        return False

    return True


def _entry_key(entry: dict[str, Any]) -> str:
    """registry entry key を生成する。"""

    return f"{entry.get('namespace')}.{entry.get('name')}.{entry.get('target_type')}"


# trace_id: trace_mujoco_sdv_registry_001
# responsibility: Validate registry runtime version range.
def validate_registry_runtime_version(
    *,
    execution_config: dict[str, Any],
    registry_package: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    """Registry package の runtime version 範囲を検証する。"""

    runtime_version = execution_config.get("runtime_version")
    version_range = registry_package.get("runtime_version_range")

    if not _is_version_in_range(runtime_version, version_range):
        diagnostics_emitter.emit(
            code="SCHEMA_VERSION_OUT_OF_RANGE",
            message="Runtime version is out of registry range.",
            trace_id="trace_mujoco_sdv_registry_001",
            schema_ref={
                "schema_id": registry_package.get("schema_id"),
                "schema_version": registry_package.get("schema_version"),
            },
        )


# trace_id: trace_mujoco_sdv_capability_001
# responsibility: Validate adapter capability runtime version range.
def validate_capability_runtime_version(
    *,
    execution_config: dict[str, Any],
    capability_package: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    """Adapter Capability の runtime version 範囲を検証する。"""

    runtime_version = execution_config.get("runtime_version")
    version_range = capability_package.get("runtime_version_range")

    if not _is_version_in_range(runtime_version, version_range):
        diagnostics_emitter.emit(
            code="ADAPTER_CAPABILITY_VERSION_MISMATCH",
            message="Runtime version is out of adapter capability range.",
            trace_id="trace_mujoco_sdv_capability_001",
            capability_ref={
                "capability_id": capability_package.get("capability_id"),
                "capability_version": capability_package.get("capability_version"),
                "adapter_id": capability_package.get("adapter_id"),
                "capability_check": "runtime_version_range",
            },
        )


# trace_id: trace_mujoco_sdv_capability_001
# responsibility: Validate mapping and artifact required versions.
def validate_entry_required_versions(
    *,
    execution_config: dict[str, Any],
    entry: dict[str, Any],
    capability_package: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    """registry entry の required version を検証する。"""

    runtime_version = execution_config.get("runtime_version")
    adapter_version = capability_package.get("adapter_version")
    target = {
        "entry_key": _entry_key(entry),
        "target_type": entry.get("target_type"),
    }

    mapping = entry.get("mjcf_mapping")
    if isinstance(mapping, dict):
        required_mujoco_version = mapping.get("required_mujoco_version")
        if isinstance(required_mujoco_version, str):
            required_mujoco_version = {
                "min": required_mujoco_version,
                "max": None,
            }

        if not _is_version_in_range(runtime_version, required_mujoco_version):
            diagnostics_emitter.emit(
                code="VERSION_RUNTIME_MISMATCH",
                message="Runtime version does not satisfy mjcf_mapping requirement.",
                trace_id="trace_mujoco_sdv_capability_001",
                target=target,
            )

    artifact = entry.get("adapter_artifact")
    if isinstance(artifact, dict):
        required_adapter_version = artifact.get("required_adapter_version")
        if isinstance(required_adapter_version, str):
            required_adapter_version = {
                "min": required_adapter_version,
                "max": None,
            }

        if not _is_version_in_range(adapter_version, required_adapter_version):
            diagnostics_emitter.emit(
                code="ADAPTER_CAPABILITY_VERSION_MISMATCH",
                message="Adapter version does not satisfy artifact requirement.",
                trace_id="trace_mujoco_sdv_capability_001",
                target=target,
                capability_ref={
                    "capability_id": capability_package.get("capability_id"),
                    "capability_version": capability_package.get("capability_version"),
                    "adapter_id": capability_package.get("adapter_id"),
                    "capability_check": "required_adapter_version",
                },
            )
