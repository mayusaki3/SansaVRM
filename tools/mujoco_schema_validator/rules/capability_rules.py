"""Adapter Capability validation rules."""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter


def _entry_key(entry: dict[str, Any]) -> str:
    """registry entry key を生成する。"""

    return f"{entry.get('namespace')}.{entry.get('name')}.{entry.get('target_type')}"


def _target(entry: dict[str, Any]) -> dict[str, Any]:
    """Diagnostics target 情報を生成する。"""

    return {
        "entry_key": _entry_key(entry),
        "target_type": entry.get("target_type"),
    }


def _capability_ref(check_name: str, capability_package: dict[str, Any]) -> dict[str, Any]:
    """Diagnostics capability_ref を生成する。"""

    return {
        "capability_id": capability_package.get("capability_id"),
        "capability_version": capability_package.get("capability_version"),
        "adapter_id": capability_package.get("adapter_id"),
        "capability_check": check_name,
    }


def _is_explicitly_unsupported(
    entry: dict[str, Any],
    capability_package: dict[str, Any],
) -> bool:
    """unsupported_entries に一致するか判定する。"""

    unsupported_entries = capability_package.get("unsupported_entries", [])

    for unsupported_entry in unsupported_entries:
        if not isinstance(unsupported_entry, dict):
            continue

        if (
            unsupported_entry.get("namespace") == entry.get("namespace")
            and unsupported_entry.get("name") == entry.get("name")
            and unsupported_entry.get("target_type") == entry.get("target_type")
        ):
            return True

    return False


def _mapping_key(entry: dict[str, Any]) -> str | None:
    """mjcf_mapping から capability 照合用 key を生成する。"""

    mapping = entry.get("mjcf_mapping")
    if not isinstance(mapping, dict):
        return None

    return mapping.get("path")


def _artifact_key(entry: dict[str, Any]) -> str | None:
    """adapter_artifact から capability 照合用 key を生成する。"""

    artifact = entry.get("adapter_artifact")
    if not isinstance(artifact, dict):
        return None

    return artifact.get("artifact_type")


# trace_id: trace_mujoco_sdv_capability_001
# trace_id: trace_mujoco_sdv_capability_002
# responsibility: Check adapter capability support.
def check_adapter_capability(
    *,
    entry: dict[str, Any],
    capability_package: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    """Adapter Capability と registry entry の整合性を検証する。

    Args:
        entry:
            registry entry。
        capability_package:
            Adapter Capability package。
        diagnostics_emitter:
            Diagnostics emitter。
    """

    target = _target(entry)

    if _is_explicitly_unsupported(entry, capability_package):
        diagnostics_emitter.emit(
            code="ADAPTER_CAPABILITY_EXPLICIT_UNSUPPORTED_ENTRY",
            message="Entry is explicitly unsupported by adapter.",
            trace_id="trace_mujoco_sdv_capability_002",
            target=target,
            capability_ref=_capability_ref("unsupported_entries", capability_package),
        )
        return

    namespace = entry.get("namespace")
    supported_namespaces = capability_package.get("supported_namespaces", [])
    if namespace not in supported_namespaces:
        diagnostics_emitter.emit(
            code="ADAPTER_CAPABILITY_UNSUPPORTED_NAMESPACE",
            message="Namespace is not supported by adapter.",
            trace_id="trace_mujoco_sdv_capability_001",
            target=target,
            capability_ref=_capability_ref("supported_namespaces", capability_package),
        )

    target_type = entry.get("target_type")
    supported_targets = capability_package.get("supported_targets", [])
    if target_type not in supported_targets:
        diagnostics_emitter.emit(
            code="ADAPTER_CAPABILITY_UNSUPPORTED_TARGET",
            message="Target type is not supported by adapter.",
            trace_id="trace_mujoco_sdv_capability_001",
            target=target,
            capability_ref=_capability_ref("supported_targets", capability_package),
        )

    io_scope = entry.get("io_scope")
    supported_io_scopes = capability_package.get("supported_io_scopes", [])
    if io_scope not in supported_io_scopes:
        diagnostics_emitter.emit(
            code="ADAPTER_CAPABILITY_UNSUPPORTED_IO_SCOPE",
            message="io_scope is not supported by adapter.",
            trace_id="trace_mujoco_sdv_capability_001",
            target=target,
            capability_ref=_capability_ref("supported_io_scopes", capability_package),
        )

    if io_scope in {"mjcf", "both"}:
        mapping_key = _mapping_key(entry)
        supported_mappings = capability_package.get("supported_mappings", [])
        if mapping_key not in supported_mappings:
            diagnostics_emitter.emit(
                code="ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING",
                message="MJCF mapping is not supported by adapter.",
                trace_id="trace_mujoco_sdv_capability_001",
                target=target,
                capability_ref=_capability_ref("supported_mappings", capability_package),
            )

    if io_scope in {"adapter_artifact", "both"}:
        artifact_key = _artifact_key(entry)
        supported_artifacts = capability_package.get("supported_artifacts", [])
        if artifact_key not in supported_artifacts:
            diagnostics_emitter.emit(
                code="ADAPTER_CAPABILITY_UNSUPPORTED_ARTIFACT",
                message="Adapter artifact is not supported by adapter.",
                trace_id="trace_mujoco_sdv_capability_001",
                target=target,
                capability_ref=_capability_ref("supported_artifacts", capability_package),
            )

    value_conversion = None
    mapping = entry.get("mjcf_mapping")
    if isinstance(mapping, dict):
        value_conversion = mapping.get("value_conversion")

    if value_conversion is not None:
        supported_value_conversions = capability_package.get(
            "supported_value_conversions",
            [],
        )
        conversion_type = (
            value_conversion.get("type")
            if isinstance(value_conversion, dict)
            else value_conversion
        )
        if conversion_type not in supported_value_conversions:
            diagnostics_emitter.emit(
                code="ADAPTER_CAPABILITY_UNSUPPORTED_VALUE_CONVERSION",
                message="Value conversion is not supported by adapter.",
                trace_id="trace_mujoco_sdv_capability_001",
                target=target,
                capability_ref=_capability_ref(
                    "supported_value_conversions",
                    capability_package,
                ),
            )
