"""Diagnostics emitter.

役割:
    Diagnostics を収集し JSON 化可能な形で保持する。
    Validation Error Code catalog を参照し、code / severity / category / strict_block の
    整合性を維持する。
"""

from __future__ import annotations

from typing import Any


class DiagnosticEmitter:
    """Diagnostics emitter.

    注意点:
        diagnostics ID 採番は PoC 用の連番とする。
    """

    def __init__(self, error_code_catalog: dict[str, Any] | None = None) -> None:
        """Diagnostics emitter を初期化する。

        Args:
            error_code_catalog:
                Validation Error Code catalog。
        """

        self._diagnostics: list[dict[str, Any]] = []
        self._error_code_map = self._build_error_code_map(error_code_catalog)

    @staticmethod
    def _build_error_code_map(
        error_code_catalog: dict[str, Any] | None,
    ) -> dict[str, dict[str, Any]]:
        """Validation Error Code catalog を code lookup 用 dict に変換する。

        Args:
            error_code_catalog:
                Validation Error Code catalog。

        Returns:
            dict[str, dict[str, Any]]:
                code を key とした catalog entry。
        """

        if error_code_catalog is None:
            return {}

        codes = error_code_catalog.get("codes", [])
        if not isinstance(codes, list):
            return {}

        return {
            code_entry["code"]: code_entry
            for code_entry in codes
            if isinstance(code_entry, dict) and "code" in code_entry
        }

    # trace_id: trace_mujoco_sdv_diagnostics_001
    # responsibility: Emit diagnostics with traceability information.
    def emit(
        self,
        *,
        code: str,
        severity: str | None = None,
        message: str,
        trace_id: str,
        output_action: str | None = None,
        category: str | None = None,
        target: dict[str, Any] | None = None,
        schema_ref: dict[str, Any] | None = None,
        capability_ref: dict[str, Any] | None = None,
        fallback: dict[str, Any] | None = None,
    ) -> None:
        """Diagnostics を追加する。

        Args:
            code:
                Validation Error Code。
            severity:
                diagnostics severity。None の場合は catalog 定義を使用する。
            message:
                人間向け短文。
            trace_id:
                traceability unit ID。
            output_action:
                出力への影響。None の場合は severity / strict_block から決定する。
            category:
                diagnostics category。None の場合は catalog 定義を使用する。
            target:
                対象情報。
            schema_ref:
                Schema Registry 参照。
            capability_ref:
                Adapter Capability 参照。
            fallback:
                fallback 情報。
        """

        code_entry = self._error_code_map.get(code, {})
        resolved_severity = severity or code_entry.get("severity", "warning")
        resolved_category = category or code_entry.get("category")
        strict_block = bool(code_entry.get("strict_block", False))

        if output_action is None:
            output_action = "block_output" if strict_block else "allow"

        diagnostic_id = f"diag-{len(self._diagnostics) + 1:04d}"

        diagnostic = {
            "diagnostic_id": diagnostic_id,
            "code": code,
            "severity": resolved_severity,
            "category": resolved_category,
            "message": message,
            "trace_id": trace_id,
            "strict_block": strict_block,
            "output_action": output_action,
        }

        if target is not None:
            diagnostic["target"] = target

        if schema_ref is not None:
            diagnostic["schema_ref"] = schema_ref

        if capability_ref is not None:
            diagnostic["capability_ref"] = capability_ref

        if fallback is not None:
            diagnostic["fallback"] = fallback

        self._diagnostics.append(diagnostic)

    def to_list(self) -> list[dict[str, Any]]:
        """Diagnostics 配列を返す。"""

        return list(self._diagnostics)
