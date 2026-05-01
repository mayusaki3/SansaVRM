use serde_json::Value;

/// JSON正規化（キー順・不要値削除）
/// TODO(trace): RoundTrip仕様 / JSON normalization
pub fn normalize_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries = map.iter().collect::<Vec<_>>();
            entries.sort_by_key(|(k, _)| *k);

            let mut new_map = serde_json::Map::new();

            for (k, v) in entries {
                let normalized = normalize_json(v);

                // 空配列 / null は除外
                if normalized.is_null() {
                    continue;
                }

                if let Value::Array(arr) = &normalized {
                    if arr.is_empty() {
                        continue;
                    }
                }

                new_map.insert(k.clone(), normalized);
            }

            Value::Object(new_map)
        }

        Value::Array(arr) => {
            Value::Array(arr.iter().map(normalize_json).collect())
        }

        _ => value.clone(),
    }
}
