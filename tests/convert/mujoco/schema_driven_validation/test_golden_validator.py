"""Golden tests for MuJoCo schema-driven validation PoC.

役割:
    PoC Validator の出力 Diagnostics / Conversion Report を golden data と比較する。

注意点:
    - 標準ライブラリ unittest のみを使用する
    - golden data は安定比較のため部分一致で検証する
    - 実装詳細ではなく、仕様上重要な観点を検証する
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from typing import Any


ROOT_DIR = Path(__file__).resolve().parents[4]
FIXTURE_DIR = ROOT_DIR / "tests" / "convert" / "mujoco" / "schema_driven_validation"
VALIDATOR = ROOT_DIR / "tools" / "mujoco_schema_validator" / "validate.py"


def load_json(path: Path) -> Any:
    """JSON ファイルを読み込む。"""

    return json.loads(path.read_text(encoding="utf-8"))


def assert_json_subset(
    testcase: unittest.TestCase,
    expected: Any,
    actual: Any,
) -> None:
    """expected が actual に部分一致することを検証する。"""

    if isinstance(expected, dict):
        testcase.assertIsInstance(actual, dict)
        for key, expected_value in expected.items():
            testcase.assertIn(key, actual)
            assert_json_subset(testcase, expected_value, actual[key])
        return

    if isinstance(expected, list):
        testcase.assertIsInstance(actual, list)
        testcase.assertGreaterEqual(len(actual), len(expected))
        for expected_item, actual_item in zip(expected, actual):
            assert_json_subset(testcase, expected_item, actual_item)
        return

    testcase.assertEqual(expected, actual)


def run_validator(
    *,
    testcase: unittest.TestCase,
    config_name: str,
) -> tuple[list[dict[str, Any]], dict[str, Any], int]:
    """validator を実行する。"""

    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        diagnostics_path = temp_path / "diagnostics.json"
        report_path = temp_path / "conversion_report.json"

        command = [
            sys.executable,
            str(VALIDATOR),
            "--input",
            str(FIXTURE_DIR / "inputs" / "sample_sansavrm.json"),
            "--registry",
            str(FIXTURE_DIR / "registry" / "mujoco_schema_registry_2_3.json"),
            "--capability",
            str(
                FIXTURE_DIR
                / "capability"
                / "sansa_vrm_mujoco_adapter_capability_0_1.json"
            ),
            "--error-codes",
            str(FIXTURE_DIR / "registry" / "validation_error_codes_0_1.json"),
            "--config",
            str(FIXTURE_DIR / "inputs" / config_name),
            "--diagnostics",
            str(diagnostics_path),
            "--report",
            str(report_path),
        ]

        env = {
            "PYTHONPATH": str(ROOT_DIR / "tools"),
        }

        result = subprocess.run(
            command,
            cwd=str(ROOT_DIR),
            env=env,
            text=True,
            capture_output=True,
            check=False,
        )

        testcase.assertTrue(
            diagnostics_path.exists(),
            msg=f"diagnostics not generated\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}",
        )

        testcase.assertTrue(
            report_path.exists(),
            msg=f"report not generated\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}",
        )

        return (
            load_json(diagnostics_path),
            load_json(report_path),
            result.returncode,
        )


class MuJoCoSchemaDrivenValidationGoldenTest(unittest.TestCase):
    """MuJoCo schema-driven validation golden tests."""

    # trace_id: trace_mujoco_sdv_report_001
    # trace_id: trace_mujoco_sdv_diagnostics_001
    # responsibility: Compare validator output with golden data.
    def test_success_fixture_matches_golden_data(self) -> None:
        """正常系 fixture の Diagnostics / Report が golden data と一致すること。"""

        actual_diagnostics, actual_report, return_code = run_validator(
            testcase=self,
            config_name="export_strict_config.json",
        )

        self.assertEqual(return_code, 0)

        expected_diagnostics = load_json(
            FIXTURE_DIR
            / "expected_diagnostics"
            / "expected_success_diagnostics.json"
        )
        expected_report = load_json(
            FIXTURE_DIR / "expected_reports" / "expected_success_report.json"
        )

        assert_json_subset(self, expected_diagnostics, actual_diagnostics)
        assert_json_subset(self, expected_report, actual_report)

    # trace_id: trace_mujoco_sdv_capability_001
    # trace_id: trace_mujoco_sdv_registry_001
    # responsibility: Validate runtime mismatch behavior.
    def test_runtime_mismatch_fixture_matches_golden_data(self) -> None:
        """runtime mismatch fixture が golden data と一致すること。"""

        actual_diagnostics, actual_report, return_code = run_validator(
            testcase=self,
            config_name="export_runtime_mismatch_config.json",
        )

        self.assertEqual(return_code, 5)

        expected_diagnostics = load_json(
            FIXTURE_DIR
            / "expected_diagnostics"
            / "expected_runtime_mismatch_diagnostics.json"
        )
        expected_report = load_json(
            FIXTURE_DIR
            / "expected_reports"
            / "expected_runtime_mismatch_report.json"
        )

        assert_json_subset(self, expected_diagnostics, actual_diagnostics)
        assert_json_subset(self, expected_report, actual_report)


if __name__ == "__main__":
    unittest.main()
