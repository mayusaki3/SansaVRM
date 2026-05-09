"""Regression tests for lightweight JSON Schema validator.

役割:
    formal JSON Schema に対して fixture / generated artifact を検証する。
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parents[4]
SCHEMA_VALIDATOR = (
    ROOT_DIR / "tools" / "mujoco_schema_validator" / "json_schema_validator.py"
)

SCHEMA_DIR = (
    ROOT_DIR / "schemas" / "mujoco" / "schema_driven_validation"
)

FIXTURE_DIR = (
    ROOT_DIR / "tests" / "convert" / "mujoco" / "schema_driven_validation"
)

VALIDATOR = ROOT_DIR / "tools" / "mujoco_schema_validator" / "validate.py"


class JsonSchemaValidationRegressionTest(unittest.TestCase):
    """JSON Schema validation regression tests."""

    def run_schema_validation(self, schema_path: Path, input_path: Path) -> None:
        """schema validation を実行する。"""

        command = [
            sys.executable,
            str(SCHEMA_VALIDATOR),
            "--schema",
            str(schema_path),
            "--input",
            str(input_path),
        ]

        result = subprocess.run(
            command,
            cwd=str(ROOT_DIR),
            text=True,
            capture_output=True,
            check=False,
        )

        self.assertEqual(
            result.returncode,
            0,
            msg=(
                "schema validation failed\n"
                f"stdout:\n{result.stdout}\n"
                f"stderr:\n{result.stderr}"
            ),
        )

    # trace_id: trace_mujoco_sdv_registry_001
    # responsibility: Validate registry fixture against formal schema.
    def test_registry_fixture_schema_validation(self) -> None:
        """registry fixture が schema に一致すること。"""

        self.run_schema_validation(
            SCHEMA_DIR / "registry.schema.json",
            FIXTURE_DIR / "registry" / "mujoco_schema_registry_2_3.json",
        )

    # trace_id: trace_mujoco_sdv_capability_001
    # responsibility: Validate capability fixture against formal schema.
    def test_capability_fixture_schema_validation(self) -> None:
        """capability fixture が schema に一致すること。"""

        self.run_schema_validation(
            SCHEMA_DIR / "adapter_capability.schema.json",
            FIXTURE_DIR
            / "capability"
            / "sansa_vrm_mujoco_adapter_capability_0_1.json",
        )

    # trace_id: trace_mujoco_sdv_report_001
    # responsibility: Validate generated conversion report against formal schema.
    def test_generated_conversion_report_schema_validation(self) -> None:
        """generated conversion report が schema に一致すること。"""

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

            self.run_schema_validation(
                SCHEMA_DIR / "conversion_report.schema.json",
                report_path,
            )


if __name__ == "__main__":
    unittest.main()
