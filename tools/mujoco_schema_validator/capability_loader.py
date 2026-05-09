"""Adapter Capability loader.

役割:
    Adapter Capability package を読み込む。
"""

from __future__ import annotations

from typing import Any

from mujoco_schema_validator.schema_loader import load_json_file


def load_capability_package(path: str) -> dict[str, Any]:
    """Capability package を読み込む。

    Args:
        path:
            capability package JSON パス。

    Returns:
        dict[str, Any]:
            capability package。
    """

    return load_json_file(path)
