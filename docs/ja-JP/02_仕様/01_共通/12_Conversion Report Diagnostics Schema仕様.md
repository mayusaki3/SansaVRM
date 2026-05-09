<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-001200Z-SV0J
lang: ja-JP
canonical_title: Conversion Report Diagnostics Schema仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Conversion Report Diagnostics Schema仕様

# Conversion Report Diagnostics Schema仕様

## 1. 目的

本仕様は、SansaVRM と外部形式または外部 runtime との変換・連携において生成される Conversion Report および Diagnostics の構造、記録規則、識別子、severity、traceability、検証要件を定義する。

Conversion Report は、変換処理全体の結果、使用 schema、使用 Adapter Capability、入出力成果物、fallback、非可逆変換、保持情報を記録する機械可読な報告情報である。

Diagnostics は、変換・検証・照合の各段階で発生した情報、警告、エラーを機械可読に記録する診断情報である。

本仕様により、以下を保証する。

- schema-driven validation の判定根拠を追跡できる
- Adapter Capability 照合結果を追跡できる
- fallback 適用理由を追跡できる
- strict モードでの成果物出力可否を機械判定できる
- CI、回帰テスト、差分確認、障害解析で再利用できる
- SansaVRM 本体が Adapter 内部実装に依存せず診断結果を保持できる

---

## 2. 基本方針

- Conversion Report と Diagnostics は機械可読な構造化データとして出力する
- Diagnostics は Conversion Report から参照可能でなければならない
- Conversion Report は変換処理単位で 1 つ以上生成できる
- Diagnostics は個別判定単位で複数生成できる
- strict モードで `severity = error` が存在する場合、変換成果物の出力を禁止する
- fallback を適用した場合、必ず Diagnostics と Conversion Report の両方に記録する
- 非可逆変換は Diagnostics と Conversion Report の両方に記録する
- `preserve_only`、`unsupported`、`source_raw` は Conversion Report に分類して記録する
- Report / Diagnostics は Adapter が生成し、SansaVRM 本体が保持できる

---

## 3. スコープ

本仕様の対象は以下とする。

- Conversion Report schema
- Diagnostics schema
- severity 定義
- diagnostics code 体系
- fallback 記録形式
- schema 判定根拠の記録形式
- Adapter Capability 照合結果の記録形式
- 成果物出力可否の判定形式
- strict / permissive モードでの扱い
- 検証要件

本仕様の対象外は以下とする。

- UI 表示形式
- ログファイル出力形式
- 外部監視基盤連携
- Adapter 内部アルゴリズム
- 外部 runtime 実行ログの完全保存

---

## 4. Conversion Report構造

Conversion Report は、変換処理全体の結果を表す。

Conversion Report は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `report_id` | 必須 | report の一意識別子 |
| `report_schema_version` | 必須 | report schema version |
| `conversion_id` | 必須 | 変換処理単位の一意識別子 |
| `source_format` | 必須 | 入力形式 |
| `target_format` | 必須 | 出力形式 |
| `mode` | 必須 | 変換モード |
| `strict` | 必須 | strict モード有無 |
| `adapter` | 必須 | 使用 Adapter 情報 |
| `schema_registry` | 任意 | 使用 Schema Registry 情報 |
| `adapter_capability` | 任意 | 使用 Adapter Capability 情報 |
| `input_artifacts` | 任意 | 入力成果物一覧 |
| `output_artifacts` | 任意 | 出力成果物一覧 |
| `parameter_results` | 必須 | パラメータ単位の判定結果 |
| `fallback_results` | 任意 | fallback 適用結果 |
| `preserved_results` | 任意 | preserve_only / source_raw 結果 |
| `lossy_results` | 任意 | 非可逆変換結果 |
| `diagnostics_summary` | 必須 | diagnostics 集計 |
| `diagnostics` | 必須 | diagnostics 配列または参照 |
| `output_allowed` | 必須 | 成果物出力可否 |

---

## 5. conversion_id

`conversion_id` は、1回の変換処理を一意に識別する。

`conversion_id` は、同一入力、同一 schema、同一 Adapter Capability、同一設定で再実行した結果を比較するために使用できなければならない。

`conversion_id` は report 内の Diagnostics と対応付ける。

---

## 6. source_format / target_format

`source_format` と `target_format` は変換元および変換先を表す。

使用可能な値の例は以下とする。

- `sansa_vrm`
- `mujoco_mjcf`
- `urdf`
- `fbx`
- `vrm_1_0`
- `vrm_0_x`
- `mmd_pmx`
- `mmd_pmd`

---

## 7. mode

`mode` は変換処理の目的を表す。

使用可能な値は以下とする。

- `import`
- `export`
- `roundtrip`
- `validate_only`
- `capability_check`

---

## 8. adapter

`adapter` は使用した Adapter 情報を表す。

`adapter` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `adapter_id` | 必須 | Adapter ID |
| `adapter_version` | 必須 | Adapter version |
| `runtime` | 任意 | 対象 runtime |
| `runtime_version` | 任意 | 対象 runtime version |

---

## 9. schema_registry

`schema_registry` は使用した Schema Registry 情報を表す。

`schema_registry` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `schema_id` | 必須 | Schema Registry ID |
| `schema_version` | 必須 | Schema Registry version |
| `runtime` | 必須 | 対象 runtime |
| `runtime_version_range` | 任意 | runtime version 範囲 |

---

## 10. adapter_capability

`adapter_capability` は使用した Adapter Capability 情報を表す。

`adapter_capability` は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `capability_id` | 必須 | Adapter Capability ID |
| `capability_version` | 必須 | Adapter Capability version |
| `adapter_id` | 必須 | Adapter ID |
| `adapter_version` | 必須 | Adapter version |

---

## 11. input_artifacts / output_artifacts

`input_artifacts` と `output_artifacts` は、変換で使用した入力成果物と生成した出力成果物を表す。

各 artifact は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `artifact_id` | 必須 | artifact の一意識別子 |
| `artifact_type` | 必須 | artifact 種別 |
| `format` | 必須 | artifact 形式 |
| `path` | 任意 | 保存先または論理パス |
| `generated` | 任意 | 生成有無 |
| `blocked` | 任意 | 出力禁止有無 |
| `reason` | 任意 | blocked の理由 |

---

## 12. parameter_results

`parameter_results` は、registry entry または変換対象パラメータごとの判定結果を表す。

`parameter_results` の各要素は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `entry_key` | 必須 | `namespace.name.target_type` |
| `namespace` | 必須 | namespace |
| `name` | 必須 | parameter name |
| `target_type` | 必須 | target_type |
| `target_id` | 任意 | 対象 SansaVRM 要素 ID |
| `io_scope` | 必須 | registry 上の io_scope |
| `result` | 必須 | 判定結果 |
| `output_scope` | 任意 | 実際の出力先 |
| `diagnostic_ids` | 任意 | 関連 Diagnostics ID 配列 |

`result` は以下のいずれかとする。

- `exported`
- `imported`
- `preserved`
- `source_raw`
- `unsupported`
- `fallback_applied`
- `blocked`
- `error`

---

## 13. fallback_results

`fallback_results` は fallback を適用した結果を表す。

各要素は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `entry_key` | 必須 | 対象 entry key |
| `target_id` | 任意 | 対象 ID |
| `fallback_behavior` | 必須 | fallback 挙動 |
| `fallback_value` | 任意 | fallback 値 |
| `reason` | 必須 | fallback 理由 |
| `diagnostic_id` | 必須 | 対応 Diagnostics ID |

`fallback_behavior` は以下のいずれかとする。

- `use_default`
- `preserve_only`
- `warn`
- `error`
- `ignore`

---

## 14. preserved_results

`preserved_results` は、出力せず保持した情報を表す。

対象は以下とする。

- `preserve_only`
- `source_raw`
- `unsupported` だが保持可能な情報

各要素は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `entry_key` | 任意 | 対象 entry key |
| `target_id` | 任意 | 対象 ID |
| `preserve_type` | 必須 | preserve 分類 |
| `reason` | 必須 | 保持理由 |
| `source_path` | 任意 | 元情報の論理パス |
| `diagnostic_id` | 任意 | 対応 Diagnostics ID |

`preserve_type` は以下のいずれかとする。

- `preserve_only`
- `source_raw`
- `unsupported_preserved`

---

## 15. lossy_results

`lossy_results` は、非可逆変換または情報欠落リスクを記録する。

各要素は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `entry_key` | 任意 | 対象 entry key |
| `target_id` | 任意 | 対象 ID |
| `loss_type` | 必須 | 非可逆分類 |
| `reason` | 必須 | 理由 |
| `source_value_summary` | 任意 | 元値の要約 |
| `converted_value_summary` | 任意 | 変換後値の要約 |
| `diagnostic_id` | 必須 | 対応 Diagnostics ID |

`loss_type` は以下のいずれかとする。

- `precision_loss`
- `semantic_loss`
- `unsupported_feature`
- `approximation`
- `dropped`
- `preserved_raw`

---

## 16. diagnostics_summary

`diagnostics_summary` は Diagnostics の集計を表す。

少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `info_count` | 必須 | info 件数 |
| `warning_count` | 必須 | warning 件数 |
| `error_count` | 必須 | error 件数 |
| `blocked_count` | 必須 | blocked 件数 |
| `fallback_count` | 必須 | fallback 件数 |
| `lossy_count` | 必須 | 非可逆件数 |

---

## 17. Diagnostics構造

Diagnostics は、個別の診断情報を表す。

Diagnostics の各要素は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `diagnostic_id` | 必須 | diagnostics の一意識別子 |
| `conversion_id` | 必須 | 対応 conversion_id |
| `code` | 必須 | diagnostics code |
| `severity` | 必須 | severity |
| `category` | 必須 | 診断分類 |
| `message` | 必須 | 人間向け短文 |
| `reason` | 必須 | 機械判定可能な理由 |
| `target` | 任意 | 対象情報 |
| `schema_ref` | 任意 | Schema Registry 参照 |
| `capability_ref` | 任意 | Adapter Capability 参照 |
| `fallback` | 任意 | fallback 情報 |
| `output_action` | 必須 | 出力処理への影響 |

---

## 18. severity

`severity` は以下のいずれかとする。

- `info`
- `warning`
- `error`

各値の意味は以下とする。

| severity | 意味 |
|---|---|
| `info` | 処理継続可能な情報 |
| `warning` | 処理継続可能だが注意が必要 |
| `error` | strict モードでは成果物出力を禁止する |

---

## 19. category

`category` は Diagnostics の分類を表す。

使用可能な値は以下とする。

- `schema_validation`
- `adapter_capability`
- `version_mismatch`
- `fallback`
- `lossy_conversion`
- `unsupported`
- `source_raw`
- `artifact_output`
- `internal_error`

---

## 20. target

`target` は Diagnostics の対象を表す。

`target` は、少なくとも以下を持てる。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `target_type` | 任意 | target_type |
| `target_id` | 任意 | SansaVRM 要素 ID |
| `entry_key` | 任意 | registry entry key |
| `path` | 任意 | 入力または出力内の論理パス |

---

## 21. schema_ref

`schema_ref` は Diagnostics の判定に使用した Schema Registry 参照を表す。

`schema_ref` は、少なくとも以下を持てる。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `schema_id` | 任意 | Schema Registry ID |
| `schema_version` | 任意 | Schema Registry version |
| `entry_key` | 任意 | registry entry key |
| `io_scope` | 任意 | io_scope |

---

## 22. capability_ref

`capability_ref` は Diagnostics の判定に使用した Adapter Capability 参照を表す。

`capability_ref` は、少なくとも以下を持てる。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `capability_id` | 任意 | Capability ID |
| `capability_version` | 任意 | Capability version |
| `adapter_id` | 任意 | Adapter ID |
| `capability_check` | 任意 | 照合項目 |

---

## 23. output_action

`output_action` は Diagnostics が成果物出力へ与える影響を表す。

使用可能な値は以下とする。

- `allow`
- `allow_with_warning`
- `preserve_only`
- `source_raw`
- `block_output`

strict モードで `output_action = block_output` が存在する場合、`output_allowed` は `false` とする。

---

## 24. diagnostics code体系

Diagnostics code は分類しやすい接頭辞を持つ。

基本接頭辞は以下とする。

| 接頭辞 | 用途 |
|---|---|
| `SCHEMA_` | Schema Registry 検証 |
| `ADAPTER_CAPABILITY_` | Adapter Capability 照合 |
| `VERSION_` | runtime / schema / adapter version 判定 |
| `FALLBACK_` | fallback 適用 |
| `LOSSY_` | 非可逆変換 |
| `UNSUPPORTED_` | 未対応 |
| `SOURCE_RAW_` | source_raw 保持 |
| `ARTIFACT_` | 成果物入出力 |
| `INTERNAL_` | 内部エラー |

---

## 25. output_allowed判定

`output_allowed` は以下の規則で判定する。

| 条件 | output_allowed |
|---|---:|
| strict = true かつ severity = error が存在 | false |
| strict = true かつ output_action = block_output が存在 | false |
| strict = false かつ fatal internal error が存在 | false |
| 上記以外 | true |

`output_allowed = false` の場合、Adapter は主要成果物を出力してはならない。

ただし、Conversion Report と Diagnostics は出力できる。

---

## 26. JSON例

Conversion Report の例を以下に示す。

```json
{
  "report_id": "report-20260509-001200Z-0001",
  "report_schema_version": "0.1.0",
  "conversion_id": "conversion-20260509-001200Z-0001",
  "source_format": "sansa_vrm",
  "target_format": "mujoco_mjcf",
  "mode": "export",
  "strict": true,
  "adapter": {
    "adapter_id": "sansa-vrm-mujoco-adapter",
    "adapter_version": "0.1.0",
    "runtime": "mujoco",
    "runtime_version": "2.3.0"
  },
  "schema_registry": {
    "schema_id": "schema-mujoco-2.3",
    "schema_version": "0.1.0",
    "runtime": "mujoco",
    "runtime_version_range": {
      "min": "2.3.0",
      "max": null
    }
  },
  "adapter_capability": {
    "capability_id": "capability-sansa-vrm-mujoco-adapter-0.1.0",
    "capability_version": "0.1.0",
    "adapter_id": "sansa-vrm-mujoco-adapter",
    "adapter_version": "0.1.0"
  },
  "input_artifacts": [],
  "output_artifacts": [
    {
      "artifact_id": "artifact-mjcf-0001",
      "artifact_type": "mjcf",
      "format": "xml",
      "path": "model.xml",
      "generated": true,
      "blocked": false,
      "reason": null
    }
  ],
  "parameter_results": [
    {
      "entry_key": "mujoco.armature.joint",
      "namespace": "mujoco",
      "name": "armature",
      "target_type": "joint",
      "target_id": "joint-left-knee",
      "io_scope": "mjcf",
      "result": "exported",
      "output_scope": "mjcf",
      "diagnostic_ids": []
    }
  ],
  "fallback_results": [],
  "preserved_results": [],
  "lossy_results": [],
  "diagnostics_summary": {
    "info_count": 0,
    "warning_count": 0,
    "error_count": 0,
    "blocked_count": 0,
    "fallback_count": 0,
    "lossy_count": 0
  },
  "diagnostics": [],
  "output_allowed": true
}
```

Diagnostics の例を以下に示す。

```json
{
  "diagnostic_id": "diag-20260509-001200Z-0001",
  "conversion_id": "conversion-20260509-001200Z-0001",
  "code": "ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING",
  "severity": "warning",
  "category": "adapter_capability",
  "message": "Adapter does not support the requested MJCF mapping.",
  "reason": "mapping_not_declared_in_adapter_capability",
  "target": {
    "target_type": "actuator",
    "target_id": "actuator-left-knee",
    "entry_key": "mujoco.torque_limit_nm.actuator",
    "path": "actuator.*.@forcerange"
  },
  "schema_ref": {
    "schema_id": "schema-mujoco-2.3",
    "schema_version": "0.1.0",
    "entry_key": "mujoco.torque_limit_nm.actuator",
    "io_scope": "both"
  },
  "capability_ref": {
    "capability_id": "capability-sansa-vrm-mujoco-adapter-0.1.0",
    "capability_version": "0.1.0",
    "adapter_id": "sansa-vrm-mujoco-adapter",
    "capability_check": "supported_mappings"
  },
  "fallback": {
    "applied": true,
    "behavior": "preserve_only",
    "value": null
  },
  "output_action": "allow_with_warning"
}
```

---

## 27. 実装責務分離

### 27.1 SansaVRM本体

SansaVRM 本体は以下を担当する。

- Conversion Report の保持
- Diagnostics の保持
- Conversion Report / Diagnostics の構造検証
- Adapter が返却した report を SansaVRM モデルに関連付けること
- report から成果物出力可否を参照できる API の提供

### 27.2 Adapter

Adapter は以下を担当する。

- Conversion Report の生成
- Diagnostics の生成
- fallback 適用結果の記録
- 非可逆変換結果の記録
- output_allowed の判定
- strict モードで `output_allowed = false` の場合に主要成果物を出力しないこと

---

## 28. 検証要件

Conversion Report / Diagnostics Schema に対するテスト仕様では、少なくとも以下を検証する。

- Conversion Report に必須項目が存在すること
- Diagnostics に必須項目が存在すること
- Diagnostics summary と Diagnostics 配列の件数が一致すること
- fallback 適用時に `fallback_results` と Diagnostics の両方へ記録されること
- 非可逆変換時に `lossy_results` と Diagnostics の両方へ記録されること
- preserve_only / source_raw が `preserved_results` に記録されること
- strict モードで error が存在する場合 `output_allowed = false` になること
- `output_allowed = false` の場合でも Conversion Report と Diagnostics は出力できること
- Schema Registry 参照が `schema_ref` に記録されること
- Adapter Capability 参照が `capability_ref` に記録されること
- parameter_results から関連 Diagnostics を追跡できること

---

[目次](../../目次.md) > 仕様 > 共通 > Conversion Report Diagnostics Schema仕様
