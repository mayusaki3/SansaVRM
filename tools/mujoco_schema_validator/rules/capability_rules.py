"""Adapter Capability validation rules."""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.diagnostics import DiagnosticEmitter


# trace_id: trace_mujoco_sdv_capability_001
# trace_id: trace_mujoco_sdv_capability_002
# responsibility: Check adapter capability support.
def check_adapter_capability(
    *,
    entry: dict[str, Any],
    capability_package: dict[str, Any],
    diagnostics_emitter: DiagnosticEmitter,
) -> None:
    supported_namespaces = capability_package.get(
        "supported_namespaces",
        [],
    )

    namespace = entry.get("namespace")

    if namespace not in supported_namespaces:
        diagnostics_emitter.emit(
            code="ADAPTER_CAPABILITY_UNSUPPORTED_NAMESPACE",
            severity="warning",
            message="Namespace is not supported by adapter.",
            trace_id="trace_mujoco_sdv_capability_001",
        )
