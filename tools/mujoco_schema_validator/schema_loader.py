"""Schema and JSON loader utilities.

役割:
    registry package、execution config、error code catalog 等を
    JSON として読み込む。
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def load_json_file(path: str) -> dict[str, Any]:
    """JSON ファイルを読み込む。

    Args:
        path:
            JSON ファイルパス。

    Returns:
        dict[str, Any]:
            読み込み結果。
    """

    file_path = Path(path)

    with file_path.open("r", encoding="utf-8") as file:
        return json.load(file)
