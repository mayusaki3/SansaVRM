#!/usr/bin/env python3
"""JSON Schema compatibility checker.

役割:
    MuJoCo schema-driven validation で使用する JSON Schema の変更互換性を検査する。

注意点:
    - 完全な JSON Schema 差分解析ではない
    - PoC では破壊的変更になりやすい代表パターンを検出する
    - 対象: required 追加、enum 削除、const 変更、type 変更、additionalProperties 締め付け
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


BreakingChange = dict[str, Any]


def load_json(path: str) -> Any:
    """JSON file を読み込む。

    Args:
        path:
            JSON file path。

    Returns:
        Any:
            JSON data。
    """

    return json.loads(Path(path).read_text(encoding="utf-8"))


def _join_path(base: str, key: str) -> str:
    """schema path を連結する。"""

    if base == "$":
        return f"$.{key}"
    return f"{base}.{key}"


def _as_set(value: Any) -> set[Any]:
    """list-like value を set に変換する。"""

    if isinstance(value, list):
        return set(value)
    if value is None:
        return set()
    return {value}


def detect_breaking_changes(
    old_schema: dict[str, Any],
    new_schema: dict[str, Any],
) -> list[BreakingChange]:
    """JSON Schema の破壊的変更候補を検出する。

    Args:
        old_schema:
            旧 schema。
        new_schema:
            新 schema。

    Returns:
        list[BreakingChange]:
            破壊的変更候補。
    """

    changes: list[BreakingChange] = []
    _compare_schema_node(old_schema, new_schema, "$", changes)
    return changes


# trace_id: trace_mujoco_sdv_registry_001
# responsibility: Detect schema evolution breaking changes.
def _compare_schema_node(
    old_node: Any,
    new_node: Any,
    path: str,
    changes: list[BreakingChange],
) -> None:
    """schema node を再帰比較する。"""

    if not isinstance(old_node, dict) or not isinstance(new_node, dict):
        return

    _compare_required(old_node, new_node, path, changes)
    _compare_enum(old_node, new_node, path, changes)
    _compare_const(old_node, new_node, path, changes)
    _compare_type(old_node, new_node, path, changes)
    _compare_additional_properties(old_node, new_node, path, changes)

    old_properties = old_node.get("properties", {})
    new_properties = new_node.get("properties", {})
    if isinstance(old_properties, dict) and isinstance(new_properties, dict):
        for key in sorted(set(old_properties) & set(new_properties)):
            _compare_schema_node(
                old_properties[key],
                new_properties[key],
                _join_path(path, key),
                changes,
            )

    old_defs = old_node.get("$defs", {})
    new_defs = new_node.get("$defs", {})
    if isinstance(old_defs, dict) and isinstance(new_defs, dict):
        for key in sorted(set(old_defs) & set(new_defs)):
            _compare_schema_node(
                old_defs[key],
                new_defs[key],
                f"{path}.$defs.{key}",
                changes,
            )


def _compare_required(
    old_node: dict[str, Any],
    new_node: dict[str, Any],
    path: str,
    changes: list[BreakingChange],
) -> None:
    """required 追加を検出する。"""

    old_required = _as_set(old_node.get("required"))
    new_required = _as_set(new_node.get("required"))
    added_required = sorted(new_required - old_required)

    if added_required:
        changes.append(
            {
                "type": "required_added",
                "path": path,
                "fields": added_required,
            }
        )


def _compare_enum(
    old_node: dict[str, Any],
    new_node: dict[str, Any],
    path: str,
    changes: list[BreakingChange],
) -> None:
    """enum 値削除を検出する。"""

    if "enum" not in old_node or "enum" not in new_node:
        return

    old_enum = _as_set(old_node.get("enum"))
    new_enum = _as_set(new_node.get("enum"))
    removed_values = sorted(old_enum - new_enum)

    if removed_values:
        changes.append(
            {
                "type": "enum_values_removed",
                "path": path,
                "values": removed_values,
            }
        )


def _compare_const(
    old_node: dict[str, Any],
    new_node: dict[str, Any],
    path: str,
    changes: list[BreakingChange],
) -> None:
    """const 変更を検出する。"""

    if "const" in old_node and "const" in new_node and old_node["const"] != new_node["const"]:
        changes.append(
            {
                "type": "const_changed",
                "path": path,
                "old": old_node["const"],
                "new": new_node["const"],
            }
        )


def _compare_type(
    old_node: dict[str, Any],
    new_node: dict[str, Any],
    path: str,
    changes: list[BreakingChange],
) -> None:
    """type 変更を検出する。"""

    if "type" in old_node and "type" in new_node and old_node["type"] != new_node["type"]:
        changes.append(
            {
                "type": "type_changed",
                "path": path,
                "old": old_node["type"],
                "new": new_node["type"],
            }
        )


def _compare_additional_properties(
    old_node: dict[str, Any],
    new_node: dict[str, Any],
    path: str,
    changes: list[BreakingChange],
) -> None:
    """additionalProperties の締め付けを検出する。"""

    old_value = old_node.get("additionalProperties", True)
    new_value = new_node.get("additionalProperties", True)

    if old_value is not False and new_value is False:
        changes.append(
            {
                "type": "additional_properties_restricted",
                "path": path,
                "old": old_value,
                "new": new_value,
            }
        )


def parse_args() -> argparse.Namespace:
    """CLI 引数を解析する。"""

    parser = argparse.ArgumentParser(description="JSON Schema compatibility checker")
    parser.add_argument("--old", required=True, help="Old schema path")
    parser.add_argument("--new", required=True, help="New schema path")
    parser.add_argument("--output", required=True, help="Output report path")
    parser.add_argument(
        "--fail-on-breaking-change",
        action="store_true",
        help="Exit with non-zero status when breaking changes are detected.",
    )
    return parser.parse_args()


def main() -> int:
    """CLI entry point."""

    args = parse_args()
    old_schema = load_json(args.old)
    new_schema = load_json(args.new)
    changes = detect_breaking_changes(old_schema, new_schema)

    report = {
        "schema_compatibility_report_version": "0.1.0",
        "breaking_change_count": len(changes),
        "breaking_changes": changes,
        "compatible": len(changes) == 0,
    }

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(report, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    if args.fail_on_breaking_change and changes:
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
