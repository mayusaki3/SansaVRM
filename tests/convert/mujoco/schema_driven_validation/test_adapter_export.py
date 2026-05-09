"""Golden tests for MuJoCo PoC Adapter artifact separation.

役割:
    Validator が生成した Conversion Report / Diagnostics を入力にして、
    PoC Adapter が MJCF / adapter artifact / preserved / reports を分離出力することを検証する。

注意点:
    - 標準ライブラリ unittest のみを使用する
    - MuJoCo runtime は実行しない
    - artifact の存在と代表的内容のみを検証する
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
ADAPTER_EXPORT = ROOT_DIR / "tools" / "mujoco_poc_adapter" / "export.py"


def load_json(path: Path) -> Any:
    """JSON ファイルを読み込む。"""

    return json.loads(path.read_text(encoding="utf-8"))


def run_validator_for_adapter(temp_path: Path) -> tuple[Path, Path]:
    """Adapter test 用に validator を実行する。

    Args:
        temp_path:
            一時ディレクトリ。

    Returns:
        tuple[Path, Path]:
            diagnostics path と report path。
    """

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

    if result.returncode != 0:
        raise AssertionError(
            "validator failed\n"
            f"stdout:\n{result.stdout}\n"
            f"stderr:\n{result.stderr}"
        )

    return diagnostics_path, report_path


class MuJoCoPoCAdapterExportGoldenTest(unittest.TestCase):
    """MuJoCo PoC Adapter export golden tests."""

    # trace_id: trace_mujoco_sdv_execution_001
    # responsibility: Verify separated artifact output from PoC adapter.
    def test_adapter_export_outputs_separated_artifacts(self) -> None:
        """PoC Adapter が成果物を分離出力すること。"""

        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            diagnostics_path, report_path = run_validator_for_adapter(temp_path)
            output_dir = temp_path / "adapter_output"

            command = [
                sys.executable,
                str(ADAPTER_EXPORT),
                "--diagnostics",
                str(diagnostics_path),
                "--report",
                str(report_path),
                "--output-dir",
                str(output_dir),
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
                    "adapter export failed\n"
                    f"stdout:\n{result.stdout}\n"
                    f"stderr:\n{result.stderr}"
                ),
            )

            mjcf_path = output_dir / "mjcf" / "scene.xml"
            adapter_artifact_path = (
                output_dir
                / "adapter_artifacts"
                / "mujoco_controller_blob_actuator.json"
            )
            preserved_path = (
                output_dir
                / "preserved"
                / "mujoco_legacy_raw_data_body.json"
            )
            report_copy_path = output_dir / "reports" / "conversion_report.json"
            diagnostics_copy_path = output_dir / "reports" / "diagnostics.json"

            self.assertTrue(mjcf_path.exists())
            self.assertTrue(adapter_artifact_path.exists())
            self.assertTrue(preserved_path.exists())
            self.assertTrue(report_copy_path.exists())
            self.assertTrue(diagnostics_copy_path.exists())

            self.assertIn("<mujoco", mjcf_path.read_text(encoding="utf-8"))

            adapter_artifact = load_json(adapter_artifact_path)
            self.assertEqual(
                adapter_artifact.get("entry_key"),
                "mujoco.controller_blob.actuator",
            )
            self.assertEqual(
                adapter_artifact.get("artifact_type"),
                "controller_config",
            )

            preserved_artifact = load_json(preserved_path)
            self.assertEqual(
                preserved_artifact.get("entry_key"),
                "mujoco.legacy_raw_data.body",
            )
            self.assertEqual(
                preserved_artifact.get("preserve_type"),
                "source_raw",
            )


if __name__ == "__main__":
    unittest.main()
