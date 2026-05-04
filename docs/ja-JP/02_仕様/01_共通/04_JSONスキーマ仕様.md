<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260504-000204Z-SV0E
lang: ja-JP
canonical_title: JSONスキーマ仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > JSONスキーマ仕様

# JSONスキーマ仕様

## 1. 目的

本仕様は、SansaVRM Format の JSON 表現に対する
バリデーション規則を JSON Schema により定義する。

本仕様の目的は以下とする。

- SansaVRM Format の構造妥当性を自動検証可能とする
- 必須項目、型、列挙値、配列構造を機械的に検証可能とする
- [メタモデル仕様](./02_メタモデル仕様.md) および [glTF拡張仕様](./03_glTF拡張仕様.md) の形式仕様を固定する
- Core API、変換処理、Runtime 入出力の前提となるデータ契約を明確化する

---

## 2. 基本方針

- JSON Schema Draft 2020-12 を前提とする
- SansaVRM Format のルート拡張構造を検証対象とする
- glTF 標準構造の全文検証は本仕様の対象外とする
- SansaVRM 固有拡張領域のみを検証対象とする
- 参照整合性のうち、Schema で表現可能な範囲を本仕様で定義する
- 完全な参照存在確認や相互整合の一部は追加バリデータで補完してよい
- 各拡張は独立して検証可能としつつ、全体スキーマでも検証可能とする

---

## 3. 適用範囲

本仕様は以下を対象とする。

- `SansaVRM_model`
- `SansaVRM_modules`
- `SansaVRM_slots`
- `SansaVRM_states`
- `SansaVRM_rights`
- `SansaVRM_revenue`
- `SansaVRM_compatibility`
- `SansaVRM_diagnostics`
- `SansaVRM_extension_layer`
- Property 共通表現
- conditions / actions / connection_rules / constraints の共通表現

以下は対象外とする。

- glTF 標準 JSON 全体の完全バリデーション
- 外部ファイルの実在確認
- UUID の生成方法
- パス形式 ID の意味論的正当性
- 実行時状態遷移の妥当性
- Core API 呼び出し順序
- Runtime のイベント実行順序

---

## 4. スキーマ構成

本仕様では以下のスキーマ群を定義する。

- ルートスキーマ
- 共通定義スキーマ
- 各拡張個別スキーマ

### 4.1 ルートスキーマ

ルートスキーマは、glTF ルート `extensions` に配置される
SansaVRM 拡張群の妥当性を検証する。

### 4.2 共通定義スキーマ

共通定義スキーマは以下を含む。

- ID
- Property
- diagnostics item
- state condition
- state action
- connection rule
- compatibility rule
- source_raw
- glTF binding

### 4.3 個別スキーマ

個別スキーマは以下ごとに定義する。

- model
- modules
- slots
- states
- rights
- revenue
- compatibility
- diagnostics
- extension layer

---

## 5. スキーマ識別子

本仕様で使用する `$id` は以下を基本とする。

```text
https://sansavrm.local/schema/sansavrm/v1/root.schema.json
https://sansavrm.local/schema/sansavrm/v1/defs.schema.json
https://sansavrm.local/schema/sansavrm/v1/model.schema.json
https://sansavrm.local/schema/sansavrm/v1/modules.schema.json
https://sansavrm.local/schema/sansavrm/v1/slots.schema.json
https://sansavrm.local/schema/sansavrm/v1/states.schema.json
https://sansavrm.local/schema/sansavrm/v1/rights.schema.json
https://sansavrm.local/schema/sansavrm/v1/revenue.schema.json
https://sansavrm.local/schema/sansavrm/v1/compatibility.schema.json
https://sansavrm.local/schema/sansavrm/v1/diagnostics.schema.json
https://sansavrm.local/schema/sansavrm/v1/extension-layer.schema.json
```

実運用では、上記は配布 URL またはローカル参照 URI に置き換えてよい。

---

## 6. 共通バリデーション規則

### 6.1 構造検証

sec_id: sec_a8k3m2q1

JSON Schema により、オブジェクト構造および必須項目の存在を検証する。

- 必須項目は各オブジェクトごとに `required` で定義する
- 配列項目は、存在する場合は配列型でなければならない

---

### 6.2 制約検証

sec_id: sec_c6t5v8s3

以下の制約を検証する。

- `additionalProperties` は `false` とする（特に断りがない限り）
- 列挙値は `enum` により制約する

ただし以下は例外とする。

- `extensions`
- `source_raw.raw`
- フォーマット固有拡張保持領域
- 将来拡張を前提とした custom 領域

---

### 6.3 型検証

sec_id: sec_b7n4p9r2

値の型は Schema 定義と一致しなければならない。

- `null` を許容する場合は `type: ["string", "null"]` 等で明示する

---

### 6.4 参照構造検証

sec_id: sec_d5w6x7u4

`$ref` によるスキーマ参照は正しく解決可能でなければならない。

---

### 6.5 文字列規則

文字列値に対して以下を適用する。

- ID や識別子は空文字列を許容しない

---

## 7. 共通定義スキーマ

以下は `defs.schema.json` 相当の共通定義である。

### 7.1 ID

```json
{
  "$defs": {
    "Id": {
      "type": "string",
      "minLength": 1
    }
  }
}
```

### 7.2 StringOrNull

```json
{
  "$defs": {
    "StringOrNull": {
      "type": ["string", "null"]
    }
  }
}
```

### 7.3 NumberOrNull

```json
{
  "$defs": {
    "NumberOrNull": {
      "type": ["number", "null"]
    }
  }
}
```

### 7.4 PropertyConstraints

```json
{
  "$defs": {
    "PropertyConstraints": {
      "type": "object",
      "properties": {
        "required": { "type": "boolean" },
        "enum": {
          "type": "array",
          "items": {}
        },
        "min": { "type": "number" },
        "max": { "type": "number" },
        "min_length": { "type": "integer", "minimum": 0 },
        "max_length": { "type": "integer", "minimum": 0 },
        "pattern": { "type": "string" }
      },
      "additionalProperties": false
    }
  }
}
```

### 7.5 PropertyMetadata

```json
{
  "$defs": {
    "PropertyMetadata": {
      "type": "object",
      "properties": {
        "description": { "$ref": "#/$defs/StringOrNull" },
        "unit": { "$ref": "#/$defs/StringOrNull" },
        "source": { "$ref": "#/$defs/StringOrNull" },
        "notes": { "$ref": "#/$defs/StringOrNull" }
      },
      "required": ["description", "unit", "source", "notes"],
      "additionalProperties": false
    }
  }
}
```

### 7.6 Property

```json
{
  "$defs": {
    "PropertyValue": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "type": { "const": "String" },
            "data": { "type": "string" }
          },
          "required": ["type", "data"],
          "additionalProperties": false
        },
        {
          "type": "object",
          "properties": {
            "type": { "const": "Number" },
            "data": { "type": "number" }
          },
          "required": ["type", "data"],
          "additionalProperties": false
        },
        {
          "type": "object",
          "properties": {
            "type": { "const": "Bool" },
            "data": { "type": "boolean" }
          },
          "required": ["type", "data"],
          "additionalProperties": false
        }
      ]
    },
    "Property": {
      "type": "object",
      "properties": {
        "property_id": { "$ref": "#/$defs/Id" },
        "key": { "type": "string", "minLength": 1 },
        "value": { "$ref": "#/$defs/PropertyValue" },
        "property_type": {
          "type": "string",
          "enum": [
            "Metadata",
            "Physics",
            "Geometry",
            "Material",
            "Texture",
            "Rig",
            "Animation",
            "Expression",
            "Control",
            "Sensor",
            "Actuator",
            "Constraint",
            "Compatibility",
            "Rights",
            "Revenue",
            "Custom"
          ]
        },
        "context": {
          "type": "string",
          "enum": [
            "Description",
            "Simulation",
            "Rendering",
            "IO",
            "Validation",
            "Conversion",
            "Execution",
            "Binding",
            "Authoring",
            "Custom"
          ]
        }
      },
      "required": ["property_id", "key", "value", "property_type", "context"],
      "additionalProperties": false
    }
  }
}
```

### 7.7 ConnectionRule

```json
{
  "$defs": {
    "ConnectionRule": {
      "type": "object",
      "properties": {
        "min_connections": { "type": "integer", "minimum": 0 },
        "max_connections": { "type": "integer", "minimum": 0 },
        "exclusive": { "type": "boolean" },
        "replace_mode": { "type": "string", "minLength": 1 },
        "priority": { "type": "integer" }
      },
      "required": ["min_connections", "max_connections", "exclusive", "replace_mode", "priority"],
      "additionalProperties": false
    }
  }
}
```

### 7.8 ConditionExpression

```json
{
  "$defs": {
    "ConditionExpression": {
      "type": "object",
      "properties": {
        "left": {},
        "comparator": {
          "type": "string",
          "enum": ["eq", "neq", "gt", "gte", "lt", "lte", "in", "not_in"]
        },
        "right": {}
      },
      "required": ["left", "comparator", "right"],
      "additionalProperties": false
    }
  }
}
```

### 7.9 Conditions

```json
{
  "$defs": {
    "Conditions": {
      "type": "object",
      "properties": {
        "operator": {
          "type": "string",
          "enum": ["AND", "OR", "NOT"]
        },
        "expressions": {
          "type": "array",
          "items": { "$ref": "#/$defs/ConditionExpression" }
        }
      },
      "required": ["operator", "expressions"],
      "additionalProperties": false
    }
  }
}
```

### 7.10 StateAction

```json
{
  "$defs": {
    "StateAction": {
      "type": "object",
      "properties": {
        "action": {
          "type": "string",
          "enum": [
            "module_enable",
            "module_disable",
            "slot_bind",
            "slot_unbind",
            "expression_change",
            "property_override",
            "visibility_change",
            "connection_enable",
            "connection_disable"
          ]
        },
        "module_id": { "$ref": "#/$defs/Id" },
        "slot_id": { "$ref": "#/$defs/Id" },
        "target_slot_id": { "$ref": "#/$defs/Id" },
        "expression_id": { "$ref": "#/$defs/Id" },
        "property_id": { "$ref": "#/$defs/Id" },
        "value": {},
        "target_id": { "$ref": "#/$defs/Id" },
        "visible": { "type": "boolean" },
        "connection_id": { "$ref": "#/$defs/Id" }
      },
      "required": ["action"],
      "additionalProperties": false,
      "allOf": [
        {
          "if": {
            "properties": { "action": { "const": "module_enable" } },
            "required": ["action"]
          },
          "then": { "required": ["module_id"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "module_disable" } },
            "required": ["action"]
          },
          "then": { "required": ["module_id"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "slot_bind" } },
            "required": ["action"]
          },
          "then": { "required": ["slot_id", "target_slot_id"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "slot_unbind" } },
            "required": ["action"]
          },
          "then": { "required": ["slot_id", "target_slot_id"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "expression_change" } },
            "required": ["action"]
          },
          "then": { "required": ["expression_id"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "property_override" } },
            "required": ["action"]
          },
          "then": { "required": ["property_id", "value"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "visibility_change" } },
            "required": ["action"]
          },
          "then": { "required": ["target_id", "visible"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "connection_enable" } },
            "required": ["action"]
          },
          "then": { "required": ["connection_id"] }
        },
        {
          "if": {
            "properties": { "action": { "const": "connection_disable" } },
            "required": ["action"]
          },
          "then": { "required": ["connection_id"] }
        }
      ]
    }
  }
}
```

### 7.11 CompatibilityRule

```json
{
  "$defs": {
    "CompatibilityRule": {
      "type": "object",
      "properties": {
        "rule_id": { "$ref": "#/$defs/Id" },
        "condition": { "type": "string", "minLength": 1 },
        "result": {
          "type": "string",
          "enum": ["ALLOW", "CONDITIONAL", "DENY"]
        },
        "severity": {
          "type": "string",
          "enum": ["info", "normal", "warning", "critical"]
        }
      },
      "required": ["rule_id", "condition", "result", "severity"],
      "additionalProperties": false
    }
  }
}
```

### 7.12 DiagnosticsItem

```json
{
  "$defs": {
    "DiagnosticsItem": {
      "type": "object",
      "properties": {
        "type": {
          "type": "string",
          "enum": [
            "non_reversible_conversion",
            "validation_error",
            "rights_warning",
            "structure_error",
            "compatibility_error",
            "runtime_warning",
            "custom"
          ]
        },
        "message": { "type": "string", "minLength": 1 },
        "severity": {
          "type": "string",
          "enum": ["info", "normal", "warning", "critical"]
        },
        "source": { "type": "string", "minLength": 1 }
      },
      "required": ["type", "message", "severity", "source"],
      "additionalProperties": false
    }
  }
}
```

### 7.13 SourceRaw

```json
{
  "$defs": {
    "SourceRaw": {
      "type": "object",
      "properties": {
        "format_type": {
          "type": "string",
          "enum": ["glTF", "VRM", "URDF", "custom"]
        },
        "raw": {
          "type": "object"
        }
      },
      "required": ["format_type", "raw"],
      "additionalProperties": false
    }
  }
}
```

### 7.14 GlTFBinding

```json
{
  "$defs": {
    "GlTFBinding": {
      "type": "object",
      "properties": {
        "node_index": { "type": "integer", "minimum": 0 },
        "mesh_index": { "type": "integer", "minimum": 0 },
        "material_indices": {
          "type": "array",
          "items": { "type": "integer", "minimum": 0 }
        },
        "animation_indices": {
          "type": "array",
          "items": { "type": "integer", "minimum": 0 }
        }
      },
      "additionalProperties": false
    }
  }
}
```

---

## 8. ルートスキーマ

ルートスキーマは glTF ルート `extensions` に存在する
SansaVRM 拡張群を検証する。

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/root.schema.json",
  "type": "object",
  "properties": {
    "extensions": {
      "type": "object",
      "properties": {
        "SansaVRM_model": { "$ref": "model.schema.json" },
        "SansaVRM_modules": { "$ref": "modules.schema.json" },
        "SansaVRM_slots": { "$ref": "slots.schema.json" },
        "SansaVRM_states": { "$ref": "states.schema.json" },
        "SansaVRM_rights": { "$ref": "rights.schema.json" },
        "SansaVRM_revenue": { "$ref": "revenue.schema.json" },
        "SansaVRM_compatibility": { "$ref": "compatibility.schema.json" },
        "SansaVRM_diagnostics": { "$ref": "diagnostics.schema.json" },
        "SansaVRM_extension_layer": { "$ref": "extension-layer.schema.json" }
      },
      "required": [
        "SansaVRM_model",
        "SansaVRM_modules",
        "SansaVRM_slots"
      ],
      "additionalProperties": true
    },
    "extensionsUsed": {
      "type": "array",
      "items": { "type": "string" }
    },
    "extensionsRequired": {
      "type": "array",
      "items": { "type": "string" }
    }
  },
  "required": ["extensions"],
  "additionalProperties": true
}
```

---

## 9. model.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/model.schema.json",
  "type": "object",
  "properties": {
    "model_id": { "$ref": "defs.schema.json#/$defs/Id" },
    "modules": {
      "type": "array",
      "items": { "$ref": "defs.schema.json#/$defs/Id" }
    },
    "slots": {
      "type": "array",
      "items": { "$ref": "defs.schema.json#/$defs/Id" }
    },
    "states": {
      "type": "array",
      "items": { "$ref": "defs.schema.json#/$defs/Id" }
    },
    "rights_ref": { "$ref": "defs.schema.json#/$defs/Id" },
    "revenue_ref": { "$ref": "defs.schema.json#/$defs/Id" },
    "diagnostics_ref": { "$ref": "defs.schema.json#/$defs/Id" },
    "connections": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "connection_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "from_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "to_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "connection_type": {
            "type": "string",
            "enum": ["attach", "joint", "logical"]
          },
          "enabled": { "type": "boolean" },
          "conditions": {
            "anyOf": [
              { "$ref": "defs.schema.json#/$defs/Conditions" },
              { "type": "null" }
            ]
          }
        },
        "required": [
          "connection_id",
          "from_id",
          "to_id",
          "connection_type",
          "enabled"
        ],
        "additionalProperties": false
      }
    }
  },
  "required": ["model_id"],
  "additionalProperties": false
}
```

---

## 10. modules.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/modules.schema.json",
  "type": "object",
  "properties": {
    "modules": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "module_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "type": {
            "type": "string",
            "enum": [
              "Body",
              "Clothing",
              "Accessory",
              "Equipment",
              "Prop",
              "Module",
              "WorldObject",
              "CompositeRoot",
              "Custom"
            ]
          },
          "slots": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Id" }
          },
          "properties": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Property" }
          },
          "state_refs": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Id" }
          },
          "glTF_binding": { "$ref": "defs.schema.json#/$defs/GlTFBinding" }
        },
        "required": ["module_id", "type"],
        "additionalProperties": false
      }
    }
  },
  "required": ["modules"],
  "additionalProperties": false
}
```

---

## 11. slots.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/slots.schema.json",
  "type": "object",
  "properties": {
    "slots": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "slot_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "slot_type": {
            "type": "string",
            "enum": [
              "Structure",
              "Equipment",
              "State",
              "Rights",
              "Revenue",
              "Physics",
              "Control",
              "Sensor",
              "Actuator",
              "Compatibility",
              "SemanticTag",
              "Morph",
              "Animation",
              "Custom"
            ]
          },
          "owner_module_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "target_slot_types": {
            "type": "array",
            "items": { "type": "string" }
          },
          "current_connections": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Id" }
          },
          "connection_rules": { "$ref": "defs.schema.json#/$defs/ConnectionRule" },
          "constraints": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Property" }
          },
          "properties": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Property" }
          }
        },
        "required": ["slot_id", "slot_type", "owner_module_id"],
        "additionalProperties": false
      }
    }
  },
  "required": ["slots"],
  "additionalProperties": false
}
```

---

## 12. states.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/states.schema.json",
  "type": "object",
  "properties": {
    "states": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "state_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "category": {
            "type": "string",
            "enum": [
              "Expression",
              "Configuration",
              "Equipment",
              "Visibility",
              "Control",
              "Physics",
              "Actuator"
            ]
          },
          "conditions": { "$ref": "defs.schema.json#/$defs/Conditions" },
          "actions": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/StateAction" }
          },
          "priority": { "type": "integer" },
          "enabled": { "type": "boolean" }
        },
        "required": ["state_id", "category", "conditions", "actions", "priority", "enabled"],
        "additionalProperties": false
      }
    }
  },
  "required": ["states"],
  "additionalProperties": false
}
```

---

## 13. rights.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/rights.schema.json",
  "type": "object",
  "properties": {
    "rights": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "rights_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "owner_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "normalized": {
            "type": "object",
            "properties": {
              "authors": {
                "type": "array",
                "items": { "type": "string" }
              },
              "license_name": { "type": "string" },
              "commercial_use_allowed": { "type": "boolean" },
              "redistribution_allowed": { "type": "boolean" },
              "modification_allowed": { "type": "boolean" },
              "attribution_required": { "type": "boolean" },
              "violent_expression_allowed": { "type": "boolean" },
              "sexual_expression_allowed": { "type": "boolean" },
              "political_or_religious_use_allowed": { "type": "boolean" },
              "prohibited_uses": {
                "type": "array",
                "items": { "type": "string" }
              },
              "additional_notes": { "type": "string" }
            },
            "required": [
              "authors",
              "license_name",
              "commercial_use_allowed",
              "redistribution_allowed",
              "modification_allowed",
              "attribution_required",
              "violent_expression_allowed",
              "sexual_expression_allowed",
              "political_or_religious_use_allowed",
              "prohibited_uses",
              "additional_notes"
            ],
            "additionalProperties": false
          },
          "source_raw": { "$ref": "defs.schema.json#/$defs/SourceRaw" },
          "diagnostics": {
            "type": "array",
            "items": { "type": "string" }
          }
        },
        "required": ["rights_id", "owner_id", "normalized", "source_raw", "diagnostics"],
        "additionalProperties": false
      }
    }
  },
  "required": ["rights"],
  "additionalProperties": false
}
```

---

## 14. revenue.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/revenue.schema.json",
  "type": "object",
  "properties": {
    "revenue": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "revenue_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "owner_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "distribution_rules": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "rule_id": { "$ref": "defs.schema.json#/$defs/Id" },
                "policy": {
                  "type": "string",
                  "enum": ["proportional", "fixed_priority", "minimum_guarantee", "exclusive"]
                }
              },
              "required": ["rule_id", "policy"],
              "additionalProperties": false
            }
          },
          "stakeholders": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "contributor_id": { "$ref": "defs.schema.json#/$defs/Id" },
                "contributor_type": {
                  "type": "string",
                  "enum": ["individual", "organization", "system", "marketplace"]
                },
                "contribution_role": {
                  "type": "string",
                  "enum": ["creator", "modifier", "distributor", "platform", "integrator"]
                },
                "weight": { "type": "number" },
                "royalty_rate": { "type": ["number", "null"] },
                "fixed_fee": { "type": ["number", "null"] },
                "external_account": { "type": ["string", "null"] }
              },
              "required": [
                "contributor_id",
                "contributor_type",
                "contribution_role",
                "weight",
                "royalty_rate",
                "fixed_fee",
                "external_account"
              ],
              "additionalProperties": false
            }
          },
          "logs": {
            "type": "array",
            "items": {}
          }
        },
        "required": ["revenue_id", "owner_id", "distribution_rules", "stakeholders", "logs"],
        "additionalProperties": false
      }
    }
  },
  "required": ["revenue"],
  "additionalProperties": false
}
```

---

## 15. compatibility.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/compatibility.schema.json",
  "type": "object",
  "properties": {
    "compatibility": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "compatibility_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "owner_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "required_slots": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Id" }
          },
          "forbidden_slots": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/Id" }
          },
          "required_tags": {
            "type": "array",
            "items": { "type": "string" }
          },
          "forbidden_tags": {
            "type": "array",
            "items": { "type": "string" }
          },
          "constraint_rules": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/CompatibilityRule" }
          },
          "target_module_types": {
            "type": "array",
            "items": { "type": "string" }
          }
        },
        "required": [
          "compatibility_id",
          "owner_id",
          "required_slots",
          "forbidden_slots",
          "required_tags",
          "forbidden_tags",
          "constraint_rules",
          "target_module_types"
        ],
        "additionalProperties": false
      }
    }
  },
  "required": ["compatibility"],
  "additionalProperties": false
}
```

---

## 16. diagnostics.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/diagnostics.schema.json",
  "type": "object",
  "properties": {
    "diagnostics": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "diagnostics_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "owner_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "items": {
            "type": "array",
            "items": { "$ref": "defs.schema.json#/$defs/DiagnosticsItem" }
          }
        },
        "required": ["diagnostics_id", "owner_id", "items"],
        "additionalProperties": false
      }
    }
  },
  "required": ["diagnostics"],
  "additionalProperties": false
}
```

---

## 17. extension-layer.schema.json

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://sansavrm.local/schema/sansavrm/v1/extension-layer.schema.json",
  "type": "object",
  "properties": {
    "extension_layers": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "extension_layer_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "owner_id": { "$ref": "defs.schema.json#/$defs/Id" },
          "format_type": {
            "type": "string",
            "enum": ["glTF", "VRM", "URDF", "custom"]
          },
          "extensions": {
            "type": "object"
          },
          "source_raw": { "$ref": "defs.schema.json#/$defs/SourceRaw" },
          "diagnostics": {
            "type": "array",
            "items": { "type": "string" }
          }
        },
        "required": ["extension_layer_id", "owner_id", "format_type", "extensions", "source_raw", "diagnostics"],
        "additionalProperties": false
      }
    }
  },
  "required": ["extension_layers"],
  "additionalProperties": false
}
```

---

## 18. バリデーション責務分担

### 18.1 JSON Schema で検証するもの

- 型
- 必須項目
- 列挙値
- オブジェクト構造
- 配列構造
- action ごとの必要引数
- diagnostics item の構造
- connection_rules の構造
- source_raw の基本構造

### 18.2 追加バリデータで検証するもの

以下は JSON Schema だけでは完全に検証できないため、
追加バリデータで補完する。

- 参照先 ID の実在確認
- `current_connections` と `connections` の完全一致
- `slot_bind` の対象関係妥当性
- module type と slot_type の意味論的整合
- 条件式 DSL の意味評価
- glTF 標準構造との実インデックス整合
- rights / revenue / compatibility の owner 実在確認
- `property_type` / `context` / `key` の意味論的整合
- MuJoCo 変換時の Property 分類妥当性

---

## 19. 運用方針

- ルートスキーマをエントリポイントとする
- 共通定義は `defs.schema.json` に集約する
- 個別スキーマは単独テスト可能とする
- バージョン更新時は `$id` のパスにバージョンを反映する
- 下位互換を破る変更はメジャーバージョンアップとする

---

## 20. 非可逆変換

変換時に失われる情報は以下に保持する。

- `SansaVRM_diagnostics`
- `SansaVRM_extension_layer.source_raw`
- `SansaVRM_rights.source_raw`

---

## 21. 非スコープ

- glTF 標準仕様全文
- VRM 標準仕様全文
- URDF 標準仕様全文
- Runtime API の詳細
- C ABI の詳細

---

## 22. 将来拡張

- AI 属性保持
- 条件式言語拡張
- 分散状態同期
- 外部ライセンスサービス連携
- マーケットプレイス固有拡張
- Schema 自動生成とコード生成の連携

---

[目次](../../目次.md) > 仕様 > 共通 > JSONスキーマ仕様
