"""Tests for traceability matrix generator.

役割:
    traceability matrix generator が trace_id を収集できることを検証する。
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parents[4]
TRACEABILITY_GENERATOR = (
    ROOT_DIR / "tools" / "mujoco_schema_validator" / "traceability_matrix.py"
)


class TraceabilityMatrixGeneratorTest(unittest.TestCase):
    """Traceability matrix generator tests."""

    # trace_id: trace_mujoco_sdv_execution_001
    # responsibility: Verify traceability matrix generation.
    def test_traceability_matrix_generation(self) -> None:
        """traceability matrix が生成されること。"""

        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            output_path = temp_path / "traceability_matrix.json"

            command = [
                sys.executable,
                str(TRACEABILITY_GENERATOR),
                "--root",
                str(ROOT_DIR / "tools"),
                "--output",
                str(output_path),
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
                    "traceability generator failed\n"
                    f"stdout:\n{result.stdout}\n"
                    f"stderr:\n{result.stderr}"
                ),
            )

            self.assertTrue(output_path.exists())

            matrix = json.loads(output_path.read_text(encoding="utf-8"))

            self.assertEqual(matrix.get("matrix_schema_version"), "0.1.0")
            self.assertGreater(matrix.get("entry_count", 0), 0)

            entries = matrix.get("entries", [])
            trace_ids = {entry.get("trace_id") for entry in entries}

            self.assertIn("trace_mujoco_sdv_execution_001", trace_ids)
            self.assertIn("trace_mujoco_sdv_report_001", trace_ids)
            self.assertIn("trace_mujoco_sdv_capability_001", trace_ids)


if __name__ == "__main__":
    unittest.main()
