use serde_json::Value;

/// JSON normalization for test comparison.
///
/// NOTE:
/// - Format agnostic (VRM / glTF / URDF / MuJoCo)
/// - Used for RoundTrip and semantic equality tests
///
/// @hldocs.ref doc-20260504-000405Z-SV0S#sec_e4y7z6v5
pub fn normalize_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries = map.iter().collect::<Vec<_>>();
            entries.sort_by_key(|(k, _)| *k);

            let mut new_map = serde_json::Map::new();

            for (k, v) in entries {
                let normalized = normalize_json(v);

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
