"""Artifact output writer.

役割:
    validator の parameter_results / diagnostics を元に、
    MJCF / adapter artifact / preserved artifact を分離出力する。
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


# trace_id: trace_mujoco_sdv_execution_001
# responsibility: Write separated output artifacts.
def write_artifacts(
    *,
    output_dir: str,
    mjcf_content: str,
    adapter_artifacts: list[dict[str, Any]],
    preserved_artifacts: list[dict[str, Any]],
    diagnostics: list[dict[str, Any]],
    conversion_report: dict[str, Any],
) -> None:
    """成果物を分離出力する。

    Args:
        output_dir:
            出力先ディレクトリ。
        mjcf_content:
            MJCF 相当文字列。
        adapter_artifacts:
            adapter artifact 一覧。
        preserved_artifacts:
            preserved artifact 一覧。
        diagnostics:
            diagnostics 配列。
        conversion_report:
            conversion report。
    """

    base_dir = Path(output_dir)
    mjcf_dir = base_dir / "mjcf"
    adapter_dir = base_dir / "adapter_artifacts"
    preserved_dir = base_dir / "preserved"
    report_dir = base_dir / "reports"

    mjcf_dir.mkdir(parents=True, exist_ok=True)
    adapter_dir.mkdir(parents=True, exist_ok=True)
    preserved_dir.mkdir(parents=True, exist_ok=True)
    report_dir.mkdir(parents=True, exist_ok=True)

    (mjcf_dir / "scene.xml").write_text(
        mjcf_content,
        encoding="utf-8",
    )

    for artifact in adapter_artifacts:
        artifact_name = artifact.get("artifact_name", "artifact")
        artifact_path = adapter_dir / f"{artifact_name}.json"

        artifact_path.write_text(
            json.dumps(artifact, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    for preserved in preserved_artifacts:
        entry_key = preserved.get("entry_key", "preserved")
        safe_name = entry_key.replace(".", "_")

        preserved_path = preserved_dir / f"{safe_name}.json"

        preserved_path.write_text(
            json.dumps(preserved, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    (report_dir / "diagnostics.json").write_text(
        json.dumps(diagnostics, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    (report_dir / "conversion_report.json").write_text(
        json.dumps(conversion_report, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
