#!/usr/bin/env python3
"""MuJoCo PoC Adapter export CLI.

役割:
    validator report を元に成果物を分離出力する。

注意点:
    本 PoC は実際の MJCF export をまだ行わない。
    skeleton として artifact separation を成立させる。
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from mujoco_poc_adapter.artifact_writer import write_artifacts


def parse_args() -> argparse.Namespace:
    """CLI 引数を解析する。"""

    parser = argparse.ArgumentParser(
        description="MuJoCo PoC Adapter export CLI"
    )

    parser.add_argument("--report", required=True)
    parser.add_argument("--diagnostics", required=True)
    parser.add_argument("--output-dir", required=True)

    return parser.parse_args()


def _load_json(path: str) -> dict[str, Any] | list[dict[str, Any]]:
    """JSON を読み込む。"""

    return json.loads(Path(path).read_text(encoding="utf-8"))


# trace_id: trace_mujoco_sdv_execution_001
# responsibility: Export separated artifacts from conversion report.
def main() -> int:
    """PoC Adapter export entry point."""

    args = parse_args()

    conversion_report = _load_json(args.report)
    diagnostics = _load_json(args.diagnostics)

    preserved_results = conversion_report.get("preserved_results", [])

    adapter_artifacts: list[dict[str, Any]] = []

    for parameter_result in conversion_report.get("parameter_results", []):
        io_scope = parameter_result.get("io_scope")
        if io_scope not in {"adapter_artifact", "both"}:
            continue

        entry_key = parameter_result.get("entry_key")
        safe_name = str(entry_key).replace(".", "_")

        adapter_artifacts.append(
            {
                "artifact_name": safe_name,
                "entry_key": entry_key,
                "artifact_type": "controller_config",
                "generated_by": "mujoco_poc_adapter",
            }
        )

    mjcf_content = """<mujoco model=\"sansa_vrm_poc\">
  <!-- PoC MJCF skeleton -->
</mujoco>
"""

    write_artifacts(
        output_dir=args.output_dir,
        mjcf_content=mjcf_content,
        adapter_artifacts=adapter_artifacts,
        preserved_artifacts=preserved_results,
        diagnostics=diagnostics,
        conversion_report=conversion_report,
    )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
