#!/usr/bin/env python3
"""Traceability manifest validator.

役割:
    traceability manifest と implementation traceability matrix の整合性を検証する。

注意点:
    - manifest は仕様上要求される traceability link を表現する
    - implementation matrix は実装コメントから自動生成される
    - 両者の差分を CI で検出する
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ValidationIssue = dict[str, Any]


def load_json(path: str) -> Any:
    """JSON file を読み込む。"""

    return json.loads(Path(path).read_text(encoding="utf-8"))


# trace_id: trace_mujoco_sdv_traceability_001
# responsibility: Validate manifest ↔ implementation traceability linkage.
def validate_traceability_manifest(
    manifest: dict[str, Any],
    matrix: dict[str, Any],
) -> list[ValidationIssue]:
    """manifest と implementation matrix の整合性を検証する。

    Args:
        manifest:
            traceability manifest。
        matrix:
            implementation traceability matrix。

    Returns:
        list[ValidationIssue]:
            validation issue 一覧。
    """

    issues: list[ValidationIssue] = []

    manifest_entries = manifest.get("entries", [])
    matrix_entries = matrix.get("entries", [])

    matrix_trace_ids = {
        entry.get("trace_id")
        for entry in matrix_entries
    }

    manifest_trace_ids = {
        entry.get("trace_id")
        for entry in manifest_entries
    }

    for trace_id in sorted(manifest_trace_ids - matrix_trace_ids):
        issues.append(
            {
                "type": "missing_implementation_trace",
                "trace_id": trace_id,
            }
        )

    for trace_id in sorted(matrix_trace_ids - manifest_trace_ids):
        issues.append(
            {
                "type": "untracked_implementation_trace",
                "trace_id": trace_id,
            }
        )

    return issues


def parse_args() -> argparse.Namespace:
    """CLI 引数を解析する。"""

    parser = argparse.ArgumentParser(description="Traceability manifest validator")
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--matrix", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument(
        "--fail-on-issue",
        action="store_true",
        help="Exit with non-zero status when validation issues exist.",
    )
    return parser.parse_args()


def main() -> int:
    """CLI entry point."""

    args = parse_args()
    manifest = load_json(args.manifest)
    matrix = load_json(args.matrix)

    issues = validate_traceability_manifest(manifest, matrix)

    report = {
        "traceability_validation_report_version": "0.1.0",
        "issue_count": len(issues),
        "issues": issues,
        "valid": len(issues) == 0,
    }

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(report, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    if args.fail_on_issue and issues:
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
