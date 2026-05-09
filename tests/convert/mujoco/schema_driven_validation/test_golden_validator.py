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
    """JSON ファイルを読み込む。

    Args:
        path:
            JSON ファイルパス。

    Returns:
        Any:
            JSON データ。
    """

    return json.loads(path.read_text(encoding="utf-8"))


def assert_json_subset(
    testcase: unittest.TestCase,
    expected: Any,
    actual: Any,
) -> None:
    """expected が actual に部分一致することを検証する。

    Args:
        testcase:
            unittest.TestCase インスタンス。
        expected:
            期待値。
        actual:
            実値。
    """

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


class MuJoCoSchemaDrivenValidationGoldenTest(unittest.TestCase):
    """MuJoCo schema-driven validation golden tests."""

    # trace_id: trace_mujoco_sdv_report_001
    # trace_id: trace_mujoco_sdv_diagnostics_001
    # responsibility: Compare validator output with golden data.
    def test_success_fixture_matches_golden_data(self) -> None:
        """正常系 fixture の Diagnostics / Report が golden data と一致すること。"""

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
                str(FIXTURE_DIR / "inputs" / "export_strict_config.json"),
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

            self.assertEqual(
                result.returncode,
                0,
                msg=(
                    "validator failed\n"
                    f"stdout:\n{result.stdout}\n"
                    f"stderr:\n{result.stderr}"
                ),
            )

            actual_diagnostics = load_json(diagnostics_path)
            actual_report = load_json(report_path)

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


if __name__ == "__main__":
    unittest.main()
