"""Tests for schema compatibility checker.

役割:
    schema evolution compatibility checker が破壊的変更を検出できることを検証する。
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parents[4]
CHECKER = (
    ROOT_DIR / "tools" / "mujoco_schema_validator" / "schema_compatibility_checker.py"
)


class SchemaCompatibilityCheckerTest(unittest.TestCase):
    """Schema compatibility checker tests."""

    # trace_id: trace_mujoco_sdv_registry_001
    # responsibility: Verify schema evolution compatibility analysis.
    def test_detect_breaking_changes(self) -> None:
        """breaking changes が検出されること。"""

        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)

            old_schema = temp_path / "old_schema.json"
            new_schema = temp_path / "new_schema.json"
            report_path = temp_path / "compatibility_report.json"

            old_schema.write_text(
                json.dumps(
                    {
                        "type": "object",
                        "required": ["name"],
                        "properties": {
                            "name": {
                                "type": "string"
                            },
                            "mode": {
                                "enum": ["a", "b", "c"]
                            }
                        },
                        "additionalProperties": true
                    },
                    ensure_ascii=False,
                    indent=2,
                ),
                encoding="utf-8",
            )

            new_schema.write_text(
                json.dumps(
                    {
                        "type": "object",
                        "required": ["name", "runtime"],
                        "properties": {
                            "name": {
                                "type": "string"
                            },
                            "mode": {
                                "enum": ["a", "b"]
                            }
                        },
                        "additionalProperties": false
                    },
                    ensure_ascii=False,
                    indent=2,
                ),
                encoding="utf-8",
            )

            command = [
                sys.executable,
                str(CHECKER),
                "--old",
                str(old_schema),
                "--new",
                str(new_schema),
                "--output",
                str(report_path),
                "--fail-on-breaking-change",
            ]

            result = subprocess.run(
                command,
                cwd=str(ROOT_DIR),
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(result.returncode, 1)
            self.assertTrue(report_path.exists())

            report = json.loads(report_path.read_text(encoding="utf-8"))

            self.assertFalse(report.get("compatible"))
            self.assertGreater(report.get("breaking_change_count", 0), 0)

            change_types = {
                change.get("type")
                for change in report.get("breaking_changes", [])
            }

            self.assertIn("required_added", change_types)
            self.assertIn("enum_values_removed", change_types)
            self.assertIn(
                "additional_properties_restricted",
                change_types,
            )


if __name__ == "__main__":
    unittest.main()
