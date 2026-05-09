#!/usr/bin/env python3
"""Traceability matrix generator for MuJoCo schema-driven validation.

役割:
    PoC 実装ファイル内の trace_id コメントを収集し、
    traceability matrix として JSON 出力する。
    併せて traceability coverage を検証する。

注意点:
    - 標準ライブラリのみを使用する
    - 実装コメント内の trace_id を source of truth として抽出する
    - HLDocS 仕様本文の解析は行わない
"""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any


TRACE_ID_PATTERN = re.compile(r"#\s*trace_id:\s*(?P<trace_id>[a-zA-Z0-9_]+)")
RESPONSIBILITY_PATTERN = re.compile(
    r"#\s*responsibility:\s*(?P<responsibility>.+)"
)


def collect_traceability_entries(root_dir: Path) -> list[dict[str, Any]]:
    """実装ファイルから traceability entry を収集する。

    Args:
        root_dir:
            検索対象ルート。

    Returns:
        list[dict[str, Any]]:
            traceability entry 配列。
    """

    entries: list[dict[str, Any]] = []

    for source_path in sorted(root_dir.rglob("*.py")):
        lines = source_path.read_text(encoding="utf-8").splitlines()

        pending_trace_ids: list[str] = []
        pending_responsibility: str | None = None

        for line_number, line in enumerate(lines, start=1):
            trace_match = TRACE_ID_PATTERN.search(line)
            if trace_match:
                pending_trace_ids.append(trace_match.group("trace_id"))
                continue

            responsibility_match = RESPONSIBILITY_PATTERN.search(line)
            if responsibility_match:
                pending_responsibility = responsibility_match.group("responsibility").strip()
                continue

            stripped = line.strip()
            if stripped.startswith("def ") or stripped.startswith("class "):
                symbol = stripped.split("(", 1)[0].replace("def ", "").replace("class ", "")

                for trace_id in pending_trace_ids:
                    entries.append(
                        {
                            "trace_id": trace_id,
                            "path": str(source_path.as_posix()),
                            "line": line_number,
                            "symbol": symbol,
                            "responsibility": pending_responsibility,
                        }
                    )

                pending_trace_ids = []
                pending_responsibility = None

    return entries


def analyze_traceability_coverage(
    entries: list[dict[str, Any]],
) -> dict[str, Any]:
    """traceability coverage を分析する。

    Args:
        entries:
            traceability entry 配列。

    Returns:
        dict[str, Any]:
            coverage analysis。
    """

    trace_ids = [str(entry.get("trace_id")) for entry in entries]
    trace_id_counts = Counter(trace_ids)

    code_targets = [
        (
            str(entry.get("trace_id")),
            str(entry.get("path")),
            str(entry.get("symbol")),
        )
        for entry in entries
    ]
    code_target_counts = Counter(code_targets)

    duplicate_code_targets = [
        {
            "trace_id": trace_id,
            "path": path,
            "symbol": symbol,
        }
        for (trace_id, path, symbol), count in sorted(code_target_counts.items())
        if count > 1
    ]

    entries_without_responsibility = [
        entry
        for entry in entries
        if not entry.get("responsibility")
    ]

    entries_without_symbol = [
        entry
        for entry in entries
        if not entry.get("symbol")
    ]

    return {
        "entry_count": len(entries),
        "unique_trace_id_count": len(trace_id_counts),
        "duplicate_trace_ids": [],
        "duplicate_code_targets": duplicate_code_targets,
        "entries_without_responsibility": entries_without_responsibility,
        "entries_without_symbol": entries_without_symbol,
        "coverage_ok": not duplicate_code_targets
        and not entries_without_responsibility
        and not entries_without_symbol,
    }


def parse_args() -> argparse.Namespace:
    """CLI 引数を解析する。"""

    parser = argparse.ArgumentParser(
        description="Generate MuJoCo schema-driven validation traceability matrix."
    )
    parser.add_argument(
        "--root",
        default="tools",
        help="Search root directory. Default: tools",
    )
    parser.add_argument(
        "--output",
        required=True,
        help="Output JSON path.",
    )
    parser.add_argument(
        "--fail-on-coverage-error",
        action="store_true",
        help="Exit with non-zero status when coverage analysis fails.",
    )

    return parser.parse_args()


def main() -> int:
    """CLI entry point."""

    args = parse_args()
    root_dir = Path(args.root)
    entries = collect_traceability_entries(root_dir)
    coverage = analyze_traceability_coverage(entries)

    output = {
        "matrix_schema_version": "0.1.0",
        "entry_count": len(entries),
        "coverage": coverage,
        "entries": entries,
    }

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(output, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    if args.fail_on_coverage_error and not coverage.get("coverage_ok"):
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
