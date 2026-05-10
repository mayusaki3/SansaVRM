#!/usr/bin/env python3
"""Migration manifest validator for SansaVRM traceability migration.

役割:
    migration_manifest.json / migration_manifest.dry-run.json を検証する。

注意点:
    - 外部依存を追加しない
    - JSON Schema subset validator を内蔵する
    - schema validation に加えて migration 固有の整合性を検査する

引数:
    --schema: migration_manifest.schema.json のパス
    --input: migration manifest JSON のパス

戻り値:
    0: 検証成功
    1: 検証失敗
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any


class ValidationError(Exception):
    """Validation error for migration manifest."""


class LightweightJsonSchemaValidator:
    """外部依存なしの lightweight JSON Schema subset validator。"""

    def __init__(self, schema: dict[str, Any]) -> None:
        """validator を初期化する。

        Args:
            schema: JSON Schema。
        """
        self.schema = schema

    def validate(self, data: Any) -> None:
        """schema に対して data を検証する。

        Args:
            data: 検証対象 JSON。

        Raises:
            ValidationError: schema validation に失敗した場合。
        """
        self._validate_node(data, self.schema, "$")

    def _resolve_ref(self, ref: str) -> dict[str, Any]:
        """local $ref を解決する。

        Args:
            ref: local JSON Pointer。

        Returns:
            解決された schema node。

        Raises:
            ValidationError: 未対応または未解決の場合。
        """
        if not ref.startswith("#/"):
            raise ValidationError(f"Unsupported ref: {ref}")

        current: Any = self.schema
        for part in ref[2:].split("/"):
            if not isinstance(current, dict) or part not in current:
                raise ValidationError(f"Unresolved ref: {ref}")
            current = current[part]

        if not isinstance(current, dict):
            raise ValidationError(f"Ref does not point to schema object: {ref}")

        return current

    def _validate_node(self, data: Any, schema: dict[str, Any], path: str) -> None:
        """schema node に対して data node を検証する。"""
        if "$ref" in schema:
            self._validate_node(data, self._resolve_ref(schema["$ref"]), path)
            return

        if "oneOf" in schema:
            errors: list[str] = []
            for candidate in schema["oneOf"]:
                try:
                    self._validate_node(data, candidate, path)
                    return
                except ValidationError as exc:
                    errors.append(str(exc))
            raise ValidationError(f"{path}: none of oneOf schemas matched: {errors}")

        if "enum" in schema and data not in schema["enum"]:
            raise ValidationError(f"{path}: expected one of {schema['enum']!r}, got {data!r}")

        if "type" in schema:
            self._validate_type(data, schema["type"], path)

        if isinstance(data, dict):
            self._validate_object(data, schema, path)
        elif isinstance(data, list):
            self._validate_array(data, schema, path)
        elif isinstance(data, str):
            self._validate_string(data, schema, path)

    def _validate_type(self, data: Any, expected_type: Any, path: str) -> None:
        """JSON Schema type を検証する。"""
        expected_types = expected_type if isinstance(expected_type, list) else [expected_type]

        for type_name in expected_types:
            if type_name == "object" and isinstance(data, dict):
                return
            if type_name == "array" and isinstance(data, list):
                return
            if type_name == "string" and isinstance(data, str):
                return
            if type_name == "boolean" and isinstance(data, bool):
                return
            if type_name == "integer" and isinstance(data, int) and not isinstance(data, bool):
                return
            if type_name == "number" and isinstance(data, int | float) and not isinstance(data, bool):
                return
            if type_name == "null" and data is None:
                return

        raise ValidationError(f"{path}: expected type {expected_type!r}, got {type(data).__name__}")

    def _validate_object(self, data: dict[str, Any], schema: dict[str, Any], path: str) -> None:
        """object schema を検証する。"""
        for required_key in schema.get("required", []):
            if required_key not in data:
                raise ValidationError(f"{path}: missing required key {required_key!r}")

        properties = schema.get("properties", {})
        additional_properties = schema.get("additionalProperties", True)

        if additional_properties is False:
            extra_keys = set(data) - set(properties)
            if extra_keys:
                raise ValidationError(f"{path}: additional properties are not allowed: {sorted(extra_keys)!r}")

        for key, value in data.items():
            if key in properties:
                self._validate_node(value, properties[key], f"{path}.{key}")

    def _validate_array(self, data: list[Any], schema: dict[str, Any], path: str) -> None:
        """array schema を検証する。"""
        item_schema = schema.get("items")
        if not isinstance(item_schema, dict):
            return

        for index, item in enumerate(data):
            self._validate_node(item, item_schema, f"{path}[{index}]")

        min_items = schema.get("minItems")
        if isinstance(min_items, int) and len(data) < min_items:
            raise ValidationError(f"{path}: array length must be >= {min_items}")

    def _validate_string(self, data: str, schema: dict[str, Any], path: str) -> None:
        """string schema を検証する。"""
        pattern = schema.get("pattern")
        if isinstance(pattern, str) and not re.match(pattern, data):
            raise ValidationError(f"{path}: string does not match pattern {pattern!r}")


def load_json(path: Path) -> Any:
    """JSON file を読み込む。

    Args:
        path: JSON file path。

    Returns:
        JSON data。
    """
    return json.loads(path.read_text(encoding="utf-8"))


def iter_doc_refs(value: Any) -> list[dict[str, Any]]:
    """docRef または docRef 配列を list として返す。"""
    if isinstance(value, list):
        return [item for item in value if isinstance(item, dict)]
    if isinstance(value, dict):
        return [value]
    return []


def validate_manifest_consistency(manifest: dict[str, Any]) -> None:
    """migration manifest 固有の整合性を検査する。

    Args:
        manifest: migration manifest。

    Raises:
        ValidationError: 整合性検査に失敗した場合。
    """
    entries = manifest.get("entries", [])
    if not isinstance(entries, list):
        raise ValidationError("entries must be an array")

    entry_ids: set[str] = set()
    old_paths: set[str] = set()
    new_paths: set[str] = set()
    new_doc_ids: set[str] = set()

    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise ValidationError(f"entries[{index}] must be an object")

        entry_id = entry.get("entry_id")
        if not isinstance(entry_id, str):
            raise ValidationError(f"entries[{index}].entry_id must be a string")
        if entry_id in entry_ids:
            raise ValidationError(f"duplicate entry_id: {entry_id}")
        entry_ids.add(entry_id)

        if entry.get("mapping_status") == "pending":
            # dry-run では許可する。正式 manifest の pending 禁止は CI 側で扱う。
            pass

        for old_ref in iter_doc_refs(entry.get("old")):
            path = old_ref.get("path")
            if isinstance(path, str):
                if path in old_paths:
                    raise ValidationError(f"duplicate old path: {path}")
                old_paths.add(path)

        for new_ref in iter_doc_refs(entry.get("new")):
            path = new_ref.get("path")
            if isinstance(path, str):
                if path in new_paths:
                    raise ValidationError(f"duplicate new path: {path}")
                new_paths.add(path)

            doc_id = new_ref.get("doc_id")
            if isinstance(doc_id, str):
                if doc_id in new_doc_ids:
                    raise ValidationError(f"duplicate new doc_id: {doc_id}")
                new_doc_ids.add(doc_id)

        sec_mappings = entry.get("sec_mappings", [])
        if isinstance(sec_mappings, list):
            seen_sec_pairs: set[tuple[str | None, str | None]] = set()
            for sec_index, sec_mapping in enumerate(sec_mappings):
                if not isinstance(sec_mapping, dict):
                    raise ValidationError(f"{entry_id}.sec_mappings[{sec_index}] must be an object")
                pair = (sec_mapping.get("old_sec_id"), sec_mapping.get("new_sec_id"))
                if pair in seen_sec_pairs:
                    raise ValidationError(f"duplicate sec mapping in {entry_id}: {pair}")
                seen_sec_pairs.add(pair)


def parse_args() -> argparse.Namespace:
    """CLI 引数を解析する。"""
    parser = argparse.ArgumentParser(description="Validate SansaVRM migration manifest")
    parser.add_argument("--schema", required=True, help="migration_manifest.schema.json path")
    parser.add_argument("--input", required=True, help="migration manifest JSON path")
    return parser.parse_args()


def main() -> int:
    """CLI entry point。

    Returns:
        process exit code。
    """
    args = parse_args()

    try:
        schema = load_json(Path(args.schema))
        manifest = load_json(Path(args.input))
        LightweightJsonSchemaValidator(schema).validate(manifest)
        validate_manifest_consistency(manifest)
    except Exception as exc:
        print(f"migration manifest validation failed: {exc}", file=sys.stderr)
        return 1

    print("migration manifest validation succeeded")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
