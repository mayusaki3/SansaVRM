<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-001500Z-SV0J
lang: ja-JP
canonical_title: MuJoCo Schema-Driven Validation Traceability仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > MuJoCo Schema-Driven Validation Traceability仕様

# MuJoCo Schema-Driven Validation Traceability仕様

## 1. 目的

本仕様は、MuJoCo Schema-Driven Validation における仕様、テスト仕様、Validation Error Code、Diagnostics、Conversion Report、PoC Validator / PoC Adapter 実装の traceability 規則を定義する。

本仕様の目的は以下とする。

- spec から testspec、code target、diagnostics code までの対応関係を固定する
- PoC 実装前に検証単位と実装単位の対応を明確化する
- registry entry、Adapter Capability rule、fallback rule、diagnostics code を追跡可能にする
- CI 失敗時に、対応する仕様・テスト・実装対象を特定できるようにする
- schema-driven validation の実装が仕様から逸脱しないようにする

---

## 2. 基本方針

- 検証単位は traceability unit として定義する
- traceability unit は spec section、testcase、code target、diagnostics code を接続する
- 実装は traceability unit に対応する関数またはモジュールを持つ
- Diagnostics は traceability unit を参照可能でなければならない
- Conversion Report は traceability unit ごとの判定結果を集計可能でなければならない
- 実装時に spec と testspec の対応が不明な項目を推測実装してはならない
- traceability が未定義の機能は PoC 実装対象に含めない

---

## 3. スコープ

本仕様の対象は以下とする。

- traceability unit 定義
- spec section と testspec の対応規則
- registry entry と testspec の対応規則
- Adapter Capability rule と testspec の対応規則
- Validation Error Code と testspec の対応規則
- Diagnostics と traceability unit の対応規則
- Conversion Report と traceability unit の対応規則
- code target 命名規則
- PoC 実装ファイル単位の traceability 規則

本仕様の対象外は以下とする。

- HLDocS 全体の traceability 規約の再定義
- UI 表示形式
- 外部 issue tracker 連携
- IDE plugin 実装

---

## 4. traceability unit

traceability unit は、schema-driven validation における最小追跡単位である。

traceability unit は、少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `trace_id` | 必須 | traceability unit の一意識別子 |
| `spec_ref` | 必須 | 対応仕様参照 |
| `test_ref` | 必須 | 対応テスト仕様参照 |
| `code_target` | 必須 | 対応実装対象 |
| `diagnostic_codes` | 任意 | 対応 Validation Error Code |
| `report_fields` | 任意 | 対応 Conversion Report field |
| `validation_scope` | 必須 | 検証範囲 |

`trace_id` は以下の形式とする。

```text
trace_mujoco_sdv_<category>_<serial>
```

例：

```text
trace_mujoco_sdv_io_scope_001
trace_mujoco_sdv_capability_001
trace_mujoco_sdv_report_001
```

---

## 5. validation_scope

`validation_scope` は検証範囲を表す。

使用可能な値は以下とする。

- `registry_structure`
- `io_scope`
- `mjcf_mapping`
- `adapter_artifact`
- `adapter_capability`
- `fallback`
- `diagnostics`
- `conversion_report`
- `error_code`
- `execution_mode`
- `artifact_output`

---

## 6. spec_ref

`spec_ref` は対応する仕様の参照を表す。

`spec_ref` は少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `doc_id` | 必須 | 仕様書 doc_id |
| `section` | 必須 | 仕様章番号または見出し |
| `requirement` | 任意 | 要求事項の要約 |

`doc_id` は HLDocS の LLM-MANAGED block に記録された doc_id を使用する。

---

## 7. test_ref

`test_ref` は対応するテスト仕様の参照を表す。

`test_ref` は少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `doc_id` | 必須 | テスト仕様 doc_id |
| `test_id` | 必須 | テストID |
| `sec_id` | 必須 | テストケース sec_id |

---

## 8. code_target

`code_target` は対応する実装対象を表す。

`code_target` は少なくとも以下を持つ。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `path` | 必須 | 実装ファイルパス |
| `symbol` | 任意 | 関数、クラス、または処理名 |
| `responsibility` | 必須 | 実装責務 |

PoC 実装では、以下の code target を基本とする。

| validation_scope | path | symbol |
|---|---|---|
| `registry_structure` | `tools/mujoco_schema_validator/rules/registry_rules.py` | `validate_registry_structure` |
| `io_scope` | `tools/mujoco_schema_validator/rules/io_scope_rules.py` | `validate_io_scope_consistency` |
| `mjcf_mapping` | `tools/mujoco_schema_validator/rules/registry_rules.py` | `validate_mjcf_mapping` |
| `adapter_artifact` | `tools/mujoco_schema_validator/rules/registry_rules.py` | `validate_adapter_artifact` |
| `adapter_capability` | `tools/mujoco_schema_validator/rules/capability_rules.py` | `check_adapter_capability` |
| `fallback` | `tools/mujoco_schema_validator/rules/fallback_rules.py` | `evaluate_fallback` |
| `diagnostics` | `tools/mujoco_schema_validator/diagnostics.py` | `emit_diagnostic` |
| `conversion_report` | `tools/mujoco_schema_validator/report_writer.py` | `build_conversion_report` |
| `error_code` | `tools/mujoco_schema_validator/rules/error_code_rules.py` | `validate_error_code_consistency` |
| `execution_mode` | `tools/mujoco_schema_validator/validate.py` | `determine_output_allowed` |
| `artifact_output` | `tools/mujoco_poc_adapter/artifact_writer.py` | `write_artifacts` |

---

## 9. registry entry traceability

registry entry は、少なくとも以下と対応付ける。

- Schema Registry仕様の該当 section
- MuJoCo Schema-Driven Validationテスト仕様の該当 test_id
- PoC Validator の code_target
- 発生し得る Validation Error Code
- Conversion Report の `parameter_results`

registry entry の traceability key は以下とする。

```text
<namespace>.<name>.<target_type>
```

例：

```text
mujoco.armature.joint
mujoco.torque_limit_nm.actuator
```

---

## 10. Adapter Capability rule traceability

Adapter Capability rule は、以下と対応付ける。

- Adapter Capability仕様の該当 section
- MuJoCo Schema-Driven Validationテスト仕様の該当 test_id
- PoC Validator の code_target
- 発生し得る Validation Error Code
- Diagnostics の `capability_ref`

Adapter Capability rule の traceability key は以下とする。

```text
adapter_capability.<check_name>
```

例：

```text
adapter_capability.supported_namespaces
adapter_capability.supported_mappings
adapter_capability.supported_value_conversions
```

---

## 11. Validation Error Code traceability

Validation Error Code は、以下と対応付ける。

- Validation Error Code体系仕様の該当 section
- Diagnostics の `code`
- Conversion Report の summary
- testspec の期待 diagnostics
- CI exit code 判定

Validation Error Code の traceability key は code そのものとする。

例：

```text
SCHEMA_INVALID_IO_SCOPE
ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING
FALLBACK_DEFAULT_APPLIED
```

---

## 12. Diagnostics traceability

Diagnostics は、以下を参照可能でなければならない。

- `trace_id`
- `diagnostic_id`
- `conversion_id`
- `code`
- `schema_ref`
- `capability_ref`
- `target.entry_key`
- `target.target_id`

Diagnostics に `trace_id` を記録する場合、対応する traceability unit が存在しなければならない。

---

## 13. Conversion Report traceability

Conversion Report は、以下を追跡可能でなければならない。

- parameter_results から registry entry traceability key を参照できること
- fallback_results から fallback rule と diagnostics を参照できること
- preserved_results から preserve / source_raw 判定を参照できること
- lossy_results から lossy diagnostics を参照できること
- diagnostics_summary から Validation Error Code 集計を参照できること
- output_allowed から strict / permissive mode 判定を参照できること

---

## 14. testspec対応規則

MuJoCo Schema-Driven Validationテスト仕様の各 test_id は、少なくとも1つの traceability unit に対応しなければならない。

1つの traceability unit が複数の test_id に対応してもよい。

ただし、1つの test_id が対応する code_target を持たない場合、PoC 実装対象として扱ってはならない。

---

## 15. 実装対象判定規則

PoC 実装対象とするには、以下をすべて満たさなければならない。

- 対応する spec_ref が存在する
- 対応する test_ref が存在する
- 対応する code_target が存在する
- 期待 diagnostics code が定義済みである
- Conversion Report に記録先が存在する

上記のいずれかが不足する場合、実装前に仕様または testspec を補完しなければならない。

---

## 16. traceability matrix

PoC 初期実装で使用する traceability matrix は以下とする。

| trace_id | validation_scope | spec_ref | test_ref | code_target | diagnostic_codes | report_fields |
|---|---|---|---|---|---|---|
| `trace_mujoco_sdv_registry_001` | `registry_structure` | `10_MuJoCo Schema Registry仕様.md#4` | `MUJOCO-SDV-001` | `validate_registry_structure` | `SCHEMA_MISSING_REQUIRED_FIELD` | `diagnostics`, `output_allowed` |
| `trace_mujoco_sdv_registry_002` | `registry_structure` | `10_MuJoCo Schema Registry仕様.md#5` | `MUJOCO-SDV-002` | `validate_registry_structure` | `SCHEMA_MISSING_REQUIRED_FIELD` | `diagnostics`, `output_allowed` |
| `trace_mujoco_sdv_io_scope_001` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#8` | `MUJOCO-SDV-003` | `validate_io_scope_consistency` | `SCHEMA_INVALID_IO_SCOPE` | `parameter_results`, `diagnostics` |
| `trace_mujoco_sdv_io_scope_002` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#9` | `MUJOCO-SDV-004` | `validate_io_scope_consistency` | `SCHEMA_INVALID_IO_SCOPE` | `diagnostics`, `output_allowed` |
| `trace_mujoco_sdv_io_scope_003` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#9` | `MUJOCO-SDV-005` | `validate_io_scope_consistency` | `SCHEMA_INVALID_ARTIFACT_RULE` | `parameter_results`, `output_artifacts` |
| `trace_mujoco_sdv_io_scope_004` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#9` | `MUJOCO-SDV-006` | `validate_io_scope_consistency` | `SCHEMA_INVALID_MAPPING`, `SCHEMA_INVALID_ARTIFACT_RULE` | `parameter_results`, `output_artifacts` |
| `trace_mujoco_sdv_io_scope_005` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#8` | `MUJOCO-SDV-007` | `validate_io_scope_consistency` | `SOURCE_RAW_STORED` | `preserved_results` |
| `trace_mujoco_sdv_io_scope_006` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#8` | `MUJOCO-SDV-008` | `validate_io_scope_consistency` | `UNSUPPORTED_PARAMETER` | `parameter_results`, `diagnostics` |
| `trace_mujoco_sdv_io_scope_007` | `io_scope` | `10_MuJoCo Schema Registry仕様.md#8` | `MUJOCO-SDV-009` | `validate_io_scope_consistency` | `SOURCE_RAW_STORED` | `preserved_results` |
| `trace_mujoco_sdv_capability_001` | `adapter_capability` | `11_Adapter Capability仕様.md#10` | `MUJOCO-SDV-010` | `check_adapter_capability` | `ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING` | `diagnostics`, `fallback_results` |
| `trace_mujoco_sdv_capability_002` | `adapter_capability` | `11_Adapter Capability仕様.md#13` | `MUJOCO-SDV-011` | `check_adapter_capability` | `ADAPTER_CAPABILITY_EXPLICIT_UNSUPPORTED_ENTRY` | `diagnostics` |
| `trace_mujoco_sdv_fallback_001` | `fallback` | `10_MuJoCo Schema Registry仕様.md#14` | `MUJOCO-SDV-012` | `evaluate_fallback` | `FALLBACK_DEFAULT_APPLIED` | `fallback_results` |
| `trace_mujoco_sdv_fallback_002` | `fallback` | `10_MuJoCo Schema Registry仕様.md#14` | `MUJOCO-SDV-013` | `evaluate_fallback` | `FALLBACK_NOT_AVAILABLE` | `fallback_results`, `output_allowed` |
| `trace_mujoco_sdv_diagnostics_001` | `diagnostics` | `12_Conversion Report Diagnostics Schema仕様.md#17` | `MUJOCO-SDV-014` | `emit_diagnostic` | `SCHEMA_INVALID_IO_SCOPE` | `diagnostics` |
| `trace_mujoco_sdv_report_001` | `conversion_report` | `12_Conversion Report Diagnostics Schema仕様.md#16` | `MUJOCO-SDV-015` | `build_conversion_report` | `FALLBACK_DEFAULT_APPLIED`, `LOSSY_SEMANTIC_LOSS` | `diagnostics_summary`, `fallback_results`, `lossy_results` |
| `trace_mujoco_sdv_error_code_001` | `error_code` | `13_Validation Error Code体系仕様.md#9` | `MUJOCO-SDV-016` | `validate_error_code_consistency` | `SCHEMA_MISSING_REQUIRED_FIELD` | `diagnostics_summary` |
| `trace_mujoco_sdv_execution_001` | `execution_mode` | `12_Conversion Report Diagnostics Schema仕様.md#25` | `MUJOCO-SDV-017` | `determine_output_allowed` | `ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING` | `output_allowed` |
| `trace_mujoco_sdv_execution_002` | `execution_mode` | `12_Conversion Report Diagnostics Schema仕様.md#25` | `MUJOCO-SDV-018` | `determine_output_allowed` | `SCHEMA_INVALID_IO_SCOPE` | `output_allowed` |

---

## 17. CI traceability出力

CI では、失敗時に少なくとも以下を出力する。

- test_id
- trace_id
- diagnostics code
- severity
- code_target
- spec_ref
- report field

これにより、CI 失敗時に修正対象を特定できるようにする。

---

## 18. 実装コメント規則

PoC 実装の主要関数には、対応する trace_id をコメントとして記録する。

例：

```python
# trace_id: trace_mujoco_sdv_io_scope_001
# responsibility: Validate io_scope and mapping/artifact consistency.
def validate_io_scope_consistency(entry):
    pass
```

変更履歴コメントは記録しない。

---

## 19. 失敗条件

以下の場合は traceability 不備として扱う。

- test_id に対応する trace_id が存在しない
- trace_id に code_target が存在しない
- diagnostics code が Validation Error Code 体系に存在しない
- Conversion Report の記録先が未定義
- 実装関数に対応する trace_id が記録されていない
- CI 失敗時に trace_id が出力されない

---

## 20. 結論

本仕様により、MuJoCo Schema-Driven Validation の spec、testspec、diagnostics code、Conversion Report、PoC 実装対象を一貫して追跡できる。

これにより、PoC 実装開始前に実装対象と検証対象が固定され、仕様逸脱、推測実装、未検証実装を防止できる。

---

[目次](../../目次.md) > 仕様 > 共通 > MuJoCo Schema-Driven Validation Traceability仕様
