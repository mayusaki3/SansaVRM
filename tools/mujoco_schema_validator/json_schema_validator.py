#!/usr/bin/env python3
"""Lightweight JSON Schema validator for MuJoCo schema-driven validation PoC.

役割:
    外部依存を追加せず、PoC で使用する JSON Schema の主要 subset を検証する。

注意点:
    - 完全な JSON Schema Draft 2020-12 実装ではない
    - PoC schema で使用している subset のみを対象とする
    - 対応 subset: type, required, properties, additionalProperties,
      enum, const, items, $defs, $ref, oneOf, minLength, pattern, minimum
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


class JsonSchemaValidationError(Exception):
    """JSON Schema validation error."""


class LightweightJsonSchemaValidator:
    """PoC 用 lightweight JSON Schema validator。"""

    def __init__(self, schema: dict[str, Any]) -> None:
        """validator を初期化する。

        Args:
            schema:
                JSON Schema。
        """

        self.schema = schema

    # trace_id: trace_mujoco_sdv_registry_001
    # responsibility: Validate JSON documents against formal schemas.
    def validate(self, data: Any) -> None:
        """schema に対して data を検証する。

        Args:
            data:
                検証対象 JSON data。

        Raises:
            JsonSchemaValidationError:
                schema validation に失敗した場合。
        """

        self._validate_node(data, self.schema, path="$")

    def _resolve_ref(self, ref: str) -> dict[str, Any]:
        """local $ref を解決する。"""

        if not ref.startswith("#/"):
            raise JsonSchemaValidationError(f"Unsupported ref: {ref}")

        current: Any = self.schema
        for part in ref[2:].split("/"):
            if not isinstance(current, dict) or part not in current:
                raise JsonSchemaValidationError(f"Unresolved ref: {ref}")
            current = current[part]

        if not isinstance(current, dict):
            raise JsonSchemaValidationError(f"Ref does not point to schema object: {ref}")

        return current

    def _validate_node(self, data: Any, schema: dict[str, Any], path: str) -> None:
        """schema node に対して data node を検証する。"""

        if "$ref" in schema:
            self._validate_node(data, self._resolve_ref(schema["$ref"]), path)
            return

        if "oneOf" in schema:
            errors: list[str] = []
            for candidate_schema in schema["oneOf"]:
                try:
                    self._validate_node(data, candidate_schema, path)
                    return
                except JsonSchemaValidationError as exc:
                    errors.append(str(exc))
            raise JsonSchemaValidationError(
                f"{path}: none of oneOf schemas matched: {errors}"
            )

        if "const" in schema and data != schema["const"]:
            raise JsonSchemaValidationError(
                f"{path}: expected const {schema['const']!r}, got {data!r}"
            )

        if "enum" in schema and data not in schema["enum"]:
            raise JsonSchemaValidationError(
                f"{path}: expected one of {schema['enum']!r}, got {data!r}"
            )

        if "type" in schema:
            self._validate_type(data, schema["type"], path)

        if isinstance(data, dict):
            self._validate_object(data, schema, path)

        if isinstance(data, list):
            self._validate_array(data, schema, path)

        if isinstance(data, str):
            self._validate_string(data, schema, path)

        if isinstance(data, int | float) and not isinstance(data, bool):
            self._validate_number(data, schema, path)

    def _validate_type(self, data: Any, expected_type: Any, path: str) -> None:
        """type を検証する。"""

        expected_types = expected_type if isinstance(expected_type, list) else [expected_type]

        for type_name in expected_types:
            if type_name == "object" and isinstance(data, dict):
                return
            if type_name == "array" and isinstance(data, list):
                return
            if type_name == "string" and isinstance(data, str):
                return
            if type_name == "integer" and isinstance(data, int) and not isinstance(data, bool):
                return
            if type_name == "number" and isinstance(data, int | float) and not isinstance(data, bool):
                return
            if type_name == "boolean" and isinstance(data, bool):
                return
            if type_name == "null" and data is None:
                return

        raise JsonSchemaValidationError(
            f"{path}: expected type {expected_type!r}, got {type(data).__name__}"
        )

    def _validate_object(self, data: dict[str, Any], schema: dict[str, Any], path: str) -> None:
        """object schema を検証する。"""

        required = schema.get("required", [])
        for required_key in required:
            if required_key not in data:
                raise JsonSchemaValidationError(f"{path}: missing required key {required_key!r}")

        properties = schema.get("properties", {})
        additional_properties = schema.get("additionalProperties", True)

        if additional_properties is False:
            extra_keys = set(data) - set(properties)
            if extra_keys:
                raise JsonSchemaValidationError(
                    f"{path}: additional properties are not allowed: {sorted(extra_keys)!r}"
                )

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

    def _validate_string(self, data: str, schema: dict[str, Any], path: str) -> None:
        """string schema を検証する。"""

        min_length = schema.get("minLength")
        if isinstance(min_length, int) and len(data) < min_length:
            raise JsonSchemaValidationError(
                f"{path}: string length must be >= {min_length}"
            )

        pattern = schema.get("pattern")
        if isinstance(pattern, str) and not re.match(pattern, data):
            raise JsonSchemaValidationError(
                f"{path}: string does not match pattern {pattern!r}"
            )

    def _validate_number(self, data: int | float, schema: dict[str, Any], path: str) -> None:
        """number schema を検証する。"""

        minimum = schema.get("minimum")
        if isinstance(minimum, int | float) and data < minimum:
            raise JsonSchemaValidationError(f"{path}: number must be >= {minimum}")


def load_json(path: str) -> Any:
    """JSON file を読み込む。"""

    return json.loads(Path(path).read_text(encoding="utf-8"))


def parse_args() -> argparse.Namespace:
    """CLI 引数を解析する。"""

    parser = argparse.ArgumentParser(description="Lightweight JSON Schema validator")
    parser.add_argument("--schema", required=True)
    parser.add_argument("--input", required=True)
    return parser.parse_args()


def main() -> int:
    """CLI entry point."""

    args = parse_args()
    schema = load_json(args.schema)
    data = load_json(args.input)

    validator = LightweightJsonSchemaValidator(schema)
    validator.validate(data)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
