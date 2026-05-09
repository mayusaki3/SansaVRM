#!/usr/bin/env python3
"""MuJoCo schema-driven validation PoC validator.

役割:
    Schema Registry、Adapter Capability、Validation Error Code を読み込み、
    schema-driven validation を実行する PoC CLI。

注意点:
    - MuJoCo runtime は実行しない
    - skeleton のため実ロジックは未実装部分を含む
    - traceability unit ごとに責務分離する
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from mujoco_schema_validator.schema_loader import load_json_file
from mujoco_schema_validator.capability_loader import load_capability_package
from mujoco_schema_validator.diagnostics import DiagnosticEmitter
from mujoco_schema_validator.report_writer import build_conversion_report
from mujoco_schema_validator.rules.registry_rules import validate_registry_structure
from mujoco_schema_validator.rules.io_scope_rules import validate_io_scope_consistency
from mujoco_schema_validator.rules.capability_rules import check_adapter_capability
from mujoco_schema_validator.rules.fallback_rules import evaluate_fallback
from mujoco_schema_validator.rules.error_code_rules import validate_error_code_consistency


# trace_id: trace_mujoco_sdv_execution_001
# trace_id: trace_mujoco_sdv_execution_002
# responsibility: Determine output_allowed and orchestrate validation flow.
def determine_output_allowed(diagnostics: list[dict[str, Any]], strict: bool) -> bool:
    """Diagnostics から output_allowed を判定する。

    Args:
        diagnostics:
            diagnostics 配列。
        strict:
            strict mode の有無。

    Returns:
        bool:
            成果物出力可能なら True。
    """

    if not strict:
        return True

    for diagnostic in diagnostics:
        severity = diagnostic.get("severity")
        output_action = diagnostic.get("output_action")

        if severity == "error":
            return False

        if output_action == "block_output":
            return False

    return True


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="MuJoCo schema-driven validation PoC validator"
    )

    parser.add_argument("--input", required=True)
    parser.add_argument("--registry", required=True)
    parser.add_argument("--capability", required=True)
    parser.add_argument("--error-codes", required=True)
    parser.add_argument("--config", required=True)
    parser.add_argument("--report", required=True)
    parser.add_argument("--diagnostics", required=True)

    return parser.parse_args()


def main() -> int:
    args = parse_args()

    execution_config = load_json_file(args.config)
    registry_package = load_json_file(args.registry)
    capability_package = load_capability_package(args.capability)
    error_code_catalog = load_json_file(args.error_codes)
    sansa_input = load_json_file(args.input)

    diagnostics_emitter = DiagnosticEmitter()

    validate_registry_structure(
        registry_package=registry_package,
        diagnostics_emitter=diagnostics_emitter,
    )

    validate_error_code_consistency(
        error_code_catalog=error_code_catalog,
        diagnostics_emitter=diagnostics_emitter,
    )

    parameter_results: list[dict[str, Any]] = []
    fallback_results: list[dict[str, Any]] = []

    entries = registry_package.get("entries", [])

    for entry in entries:
        validate_io_scope_consistency(
            entry=entry,
            diagnostics_emitter=diagnostics_emitter,
        )

        check_adapter_capability(
            entry=entry,
            capability_package=capability_package,
            diagnostics_emitter=diagnostics_emitter,
        )

        fallback_result = evaluate_fallback(
            entry=entry,
            diagnostics_emitter=diagnostics_emitter,
        )

        if fallback_result is not None:
            fallback_results.append(fallback_result)

        parameter_results.append(
            {
                "entry_key": (
                    f"{entry.get('namespace')}."
                    f"{entry.get('name')}."
                    f"{entry.get('target_type')}"
                ),
                "result": "validated",
            }
        )

    diagnostics = diagnostics_emitter.to_list()

    output_allowed = determine_output_allowed(
        diagnostics=diagnostics,
        strict=execution_config.get("strict", True),
    )

    report = build_conversion_report(
        execution_config=execution_config,
        registry_package=registry_package,
        capability_package=capability_package,
        parameter_results=parameter_results,
        fallback_results=fallback_results,
        diagnostics=diagnostics,
        output_allowed=output_allowed,
    )

    Path(args.diagnostics).write_text(
        json.dumps(diagnostics, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    Path(args.report).write_text(
        json.dumps(report, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )

    _ = sansa_input

    return 0 if output_allowed else 2


if __name__ == "__main__":
    raise SystemExit(main())
