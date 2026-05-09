"""Diagnostics emitter.

役割:
    Diagnostics を収集し JSON 化可能な形で保持する。
"""

from __future__ import annotations

from typing import Any


class DiagnosticEmitter:
    """Diagnostics emitter.

    注意点:
        skeleton 実装のため diagnostics ID 採番は簡易実装。
    """

    def __init__(self) -> None:
        self._diagnostics: list[dict[str, Any]] = []

    # trace_id: trace_mujoco_sdv_diagnostics_001
    # responsibility: Emit diagnostics with traceability information.
    def emit(
        self,
        *,
        code: str,
        severity: str,
        message: str,
        trace_id: str,
        output_action: str = "allow",
    ) -> None:
        """Diagnostics を追加する。

        Args:
            code:
                Validation Error Code。
            severity:
                diagnostics severity。
            message:
                人間向け短文。
            trace_id:
                traceability unit ID。
            output_action:
                出力への影響。
        """

        diagnostic_id = f"diag-{len(self._diagnostics) + 1:04d}"

        self._diagnostics.append(
            {
                "diagnostic_id": diagnostic_id,
                "code": code,
                "severity": severity,
                "message": message,
                "trace_id": trace_id,
                "output_action": output_action,
            }
        )

    def to_list(self) -> list[dict[str, Any]]:
        """Diagnostics 配列を返す。"""

        return list(self._diagnostics)
