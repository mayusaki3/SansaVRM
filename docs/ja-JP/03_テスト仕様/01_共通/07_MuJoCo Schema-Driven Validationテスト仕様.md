<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-000407Z-SV0S
lang: ja-JP
canonical_title: MuJoCo Schema-Driven Validationテスト仕様
document_type: testspec
canonical_document: true
-->

[目次](../../目次.md) > テスト仕様 > 共通 > MuJoCo Schema-Driven Validationテスト仕様

# MuJoCo Schema-Driven Validationテスト仕様

## 1. 目的

本仕様は、MuJoCo Schema Registry、Adapter Capability、Conversion Report、Diagnostics、Validation Error Code に基づく schema-driven validation のテスト仕様を定義する。

本仕様の目的は以下とする。

- MuJoCo 固有情報の入出力可否が Schema Registry に基づいて判定されることを検証する
- Adapter の対応能力が Adapter Capability に基づいて判定されることを検証する
- fallback、preserve_only、unsupported、source_raw が仕様通り分類されることを検証する
- Diagnostics と Conversion Report が機械可読な形式で一貫して出力されることを検証する
- Validation Error Code に基づき severity、strict_block、CI exit code が判定されることを検証する

---

## 2. 対応仕様

本テスト仕様は、以下の仕様に対応する。

- [MuJoCo連携仕様](../../02_仕様/01_共通/09_MuJoCo連携仕様.md)
- [MuJoCo Schema Registry仕様](../../02_仕様/01_共通/10_MuJoCo Schema Registry仕様.md)
- [Adapter Capability仕様](../../02_仕様/01_共通/11_Adapter Capability仕様.md)
- [Conversion Report Diagnostics Schema仕様](../../02_仕様/01_共通/12_Conversion Report Diagnostics Schema仕様.md)
- [Validation Error Code体系仕様](../../02_仕様/01_共通/13_Validation Error Code体系仕様.md)

---

## 3. テスト対象

本仕様では以下を対象とする。

- MuJoCo Schema Registry の構造検証
- registry entry の `io_scope` 判定
- `mjcf_mapping` 整合性検証
- `adapter_artifact` 整合性検証
- MuJoCo version 判定
- Adapter Capability 照合
- fallback 判定
- Diagnostics 生成
- Conversion Report 生成
- Validation Error Code 整合性検証
- strict / permissive mode 判定
- CI exit code 判定

---

## 4. テスト分類

### 4.1 Schema Registry構造検証

Schema Registry が必須項目を持ち、registry package と registry entry が仕様通り検証されることを確認する。

検証観点：

- `schema_id` が存在すること
- `schema_version` が存在すること
- `runtime` が `mujoco` であること
- `runtime_version_range` が存在すること
- `entries` が存在すること
- entry が `namespace + name + target_type` で識別できること
- 必須項目欠落時に `SCHEMA_MISSING_REQUIRED_FIELD` が出力されること

---

### 4.2 io_scope Matrix検証

`io_scope` の全値について、出力先分類と整合条件を確認する。

対象値：

- `mjcf`
- `adapter_artifact`
- `both`
- `preserve_only`
- `unsupported`
- `source_raw`

検証観点：

- `mjcf` では `mjcf_mapping` が必須であること
- `adapter_artifact` では `adapter_artifact` が必須であること
- `both` では `mjcf_mapping` と `adapter_artifact` の両方が必須であること
- `preserve_only` は外部成果物へ出力されないこと
- `unsupported` は diagnostics に記録されること
- `source_raw` は解釈されず保持されること
- 不整合時に `SCHEMA_INVALID_IO_SCOPE` が出力されること

---

### 4.3 mjcf_mapping検証

`mjcf_mapping` が MJCF 直接入出力対象として正しく機能することを確認する。

検証観点：

- `element` が存在すること
- `path` が存在すること
- `direction` が `import` / `export` / `import_export` のいずれかであること
- `required_mujoco_version` が存在すること
- `direction = import` では import のみ許可されること
- `direction = export` では export のみ許可されること
- `direction = import_export` では双方向が許可されること
- 未対応 mapping では `ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING` が出力されること

---

### 4.4 adapter_artifact検証

`adapter_artifact` が Adapter 側補助成果物の分類に使用されることを確認する。

検証観点：

- `artifact_type` が存在すること
- `path` が存在すること
- `direction` が存在すること
- `required_adapter_version` が存在すること
- `controller_config` へ出力されること
- `runtime_config` へ出力されること
- `conversion_report` へ出力されること
- `diagnostics` へ出力されること
- 未対応 artifact では `ADAPTER_CAPABILITY_UNSUPPORTED_ARTIFACT` が出力されること

---

### 4.5 Adapter Capability照合検証

Adapter Capability と Schema Registry の照合結果を確認する。

検証観点：

- `adapter_id` が一致すること
- `runtime` が一致すること
- `runtime_version_range` が一致すること
- namespace が `supported_namespaces` に含まれること
- target_type が `supported_targets` に含まれること
- io_scope が `supported_io_scopes` に含まれること
- mapping が `supported_mappings` に含まれること
- artifact が `supported_artifacts` に含まれること
- value_conversion が `supported_value_conversions` に含まれること
- `unsupported_entries` に一致する場合は未対応として扱われること

---

### 4.6 fallback Matrix検証

fallback 全値について、Diagnostics と Conversion Report への記録を確認する。

対象値：

- `use_default`
- `preserve_only`
- `warn`
- `error`
- `ignore`

検証観点：

- fallback 適用時に Diagnostics が生成されること
- fallback 適用時に Conversion Report の `fallback_results` に記録されること
- `use_default` では既定値が適用されること
- `preserve_only` では出力せず保持されること
- `warn` では warning が出力されること
- `error` では strict モードで出力禁止となること
- `ignore` では無視された理由が記録されること

---

### 4.7 Diagnostics整合性検証

Diagnostics の構造と code 整合性を確認する。

検証観点：

- `diagnostic_id` が存在すること
- `conversion_id` が存在すること
- `code` が Validation Error Code 定義と一致すること
- `severity` が code 定義と一致すること
- `category` が code 定義と一致すること
- `schema_ref` が Schema Registry を参照すること
- `capability_ref` が Adapter Capability を参照すること
- `fallback` 情報が必要時に記録されること
- `output_action` が記録されること

---

### 4.8 Conversion Report整合性検証

Conversion Report の構造と Diagnostics との整合性を確認する。

検証観点：

- `report_id` が存在すること
- `conversion_id` が存在すること
- `adapter` が存在すること
- `schema_registry` が存在すること
- `adapter_capability` が存在すること
- `parameter_results` が存在すること
- `diagnostics_summary` が Diagnostics 配列と一致すること
- `fallback_results` が fallback Diagnostics と対応すること
- `preserved_results` が preserve_only / source_raw と対応すること
- `lossy_results` が lossy Diagnostics と対応すること
- `output_allowed` が strict 判定と一致すること

---

### 4.9 Validation Error Code整合性検証

Validation Error Code 定義と Diagnostics / Conversion Report の対応を確認する。

検証観点：

- code が命名規則に従うこと
- code が category と一致すること
- severity が code 定義と一致すること
- strict_block が code 定義と一致すること
- recoverable が code 定義と一致すること
- fallback_possible が code 定義と一致すること
- lossy が code 定義と一致すること
- requires_manual_review が code 定義と一致すること
- CI exit code が category 優先順位に従うこと

---

### 4.10 strict / permissive mode検証

strict mode と permissive mode の成果物出力可否を確認する。

検証観点：

- strict mode で `severity = error` がある場合、`output_allowed = false` になること
- strict mode で `strict_block = true` がある場合、`output_allowed = false` になること
- strict mode で warning のみの場合、`output_allowed = true` になること
- permissive mode では fallback / preserve_only / source_raw を許可できること
- fatal internal error は permissive mode でも出力禁止にできること
- `output_allowed = false` でも Conversion Report と Diagnostics は出力できること

---

## 5. テストケース設計

各テストケースは以下を持つ。

- テストID
- sec_id
- 分類
- 対応仕様
- 前提条件
- 入力
- 実行内容
- 期待結果
- 期待 diagnostics
- 期待 conversion report
- 期待 CI exit code

---

## 6. テストケース

### MUJOCO-SDV-001

sec_id: sec_msdv_001a

- テストID：MUJOCO-SDV-001
- 分類：Schema Registry構造
- 内容：正常な registry package を検証する
- 期待結果：
  - schema validation が成功する
  - diagnostics error が発生しない
  - CI exit code は `0`

---

### MUJOCO-SDV-002

sec_id: sec_msdv_002b

- テストID：MUJOCO-SDV-002
- 分類：Schema Registry構造
- 内容：必須項目が欠落した registry entry を検証する
- 期待結果：
  - `SCHEMA_MISSING_REQUIRED_FIELD` が出力される
  - severity は `error`
  - strict_block は `true`
  - CI exit code は `5`

---

### MUJOCO-SDV-003

sec_id: sec_msdv_003c

- テストID：MUJOCO-SDV-003
- 分類：io_scope Matrix
- 内容：`io_scope = mjcf` で `mjcf_mapping` が存在する
- 期待結果：
  - MJCF 出力対象として分類される
  - `parameter_results.result = exported`
  - diagnostics error が発生しない

---

### MUJOCO-SDV-004

sec_id: sec_msdv_004d

- テストID：MUJOCO-SDV-004
- 分類：io_scope Matrix
- 内容：`io_scope = mjcf` で `mjcf_mapping` が欠落している
- 期待結果：
  - `SCHEMA_INVALID_IO_SCOPE` が出力される
  - `output_allowed = false`
  - CI exit code は `5`

---

### MUJOCO-SDV-005

sec_id: sec_msdv_005e

- テストID：MUJOCO-SDV-005
- 分類：io_scope Matrix
- 内容：`io_scope = adapter_artifact` で `adapter_artifact` が存在する
- 期待結果：
  - Adapter 側補助成果物へ分類される
  - `output_artifacts` に補助成果物が記録される
  - diagnostics error が発生しない

---

### MUJOCO-SDV-006

sec_id: sec_msdv_006f

- テストID：MUJOCO-SDV-006
- 分類：io_scope Matrix
- 内容：`io_scope = both` で MJCF と補助成果物へ出力する
- 期待結果：
  - MJCF と Adapter 側補助成果物の両方へ分類される
  - `parameter_results.output_scope` に両出力が記録される

---

### MUJOCO-SDV-007

sec_id: sec_msdv_007g

- テストID：MUJOCO-SDV-007
- 分類：io_scope Matrix
- 内容：`io_scope = preserve_only` を検証する
- 期待結果：
  - 外部成果物へ出力されない
  - `preserved_results.preserve_type = preserve_only`
  - diagnostics error が発生しない

---

### MUJOCO-SDV-008

sec_id: sec_msdv_008h

- テストID：MUJOCO-SDV-008
- 分類：io_scope Matrix
- 内容：`io_scope = unsupported` を検証する
- 期待結果：
  - `UNSUPPORTED_PARAMETER` が出力される
  - `parameter_results.result = unsupported`
  - strict mode では fallback 有無に従う

---

### MUJOCO-SDV-009

sec_id: sec_msdv_009i

- テストID：MUJOCO-SDV-009
- 分類：io_scope Matrix
- 内容：`io_scope = source_raw` を検証する
- 期待結果：
  - 値は解釈されない
  - `SOURCE_RAW_STORED` が出力される
  - `preserved_results.preserve_type = source_raw`

---

### MUJOCO-SDV-010

sec_id: sec_msdv_010j

- テストID：MUJOCO-SDV-010
- 分類：Adapter Capability
- 内容：Adapter Capability が mapping 未対応の entry を検証する
- 期待結果：
  - `ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING` が出力される
  - fallback が評価される
  - Conversion Report に capability_ref が記録される

---

### MUJOCO-SDV-011

sec_id: sec_msdv_011k

- テストID：MUJOCO-SDV-011
- 分類：Adapter Capability
- 内容：`unsupported_entries` に一致する entry を検証する
- 期待結果：
  - `ADAPTER_CAPABILITY_EXPLICIT_UNSUPPORTED_ENTRY` が出力される
  - 他の supported 条件を満たしていても未対応となる

---

### MUJOCO-SDV-012

sec_id: sec_msdv_012l

- テストID：MUJOCO-SDV-012
- 分類：fallback Matrix
- 内容：`fallback = use_default` を検証する
- 期待結果：
  - `FALLBACK_DEFAULT_APPLIED` が出力される
  - default 値が適用される
  - `fallback_results` に記録される

---

### MUJOCO-SDV-013

sec_id: sec_msdv_013m

- テストID：MUJOCO-SDV-013
- 分類：fallback Matrix
- 内容：`fallback = error` を strict mode で検証する
- 期待結果：
  - `FALLBACK_NOT_AVAILABLE` または対応する error code が出力される
  - `output_allowed = false`
  - 主要成果物は出力されない
  - Conversion Report と Diagnostics は出力される

---

### MUJOCO-SDV-014

sec_id: sec_msdv_014n

- テストID：MUJOCO-SDV-014
- 分類：Diagnostics整合性
- 内容：Diagnostics の code / severity / category が code 定義と一致する
- 期待結果：
  - Diagnostics の `severity` が Validation Error Code 定義と一致する
  - Diagnostics の `category` が Validation Error Code 定義と一致する

---

### MUJOCO-SDV-015

sec_id: sec_msdv_015o

- テストID：MUJOCO-SDV-015
- 分類：Conversion Report整合性
- 内容：Diagnostics summary と Diagnostics 配列の件数を照合する
- 期待結果：
  - `info_count` / `warning_count` / `error_count` が実数と一致する
  - `fallback_count` が fallback_results と一致する
  - `lossy_count` が lossy_results と一致する

---

### MUJOCO-SDV-016

sec_id: sec_msdv_016p

- テストID：MUJOCO-SDV-016
- 分類：Validation Error Code
- 内容：CI exit code 優先順位を検証する
- 期待結果：
  - 複数 category が存在する場合、優先順位に従って CI exit code が決定される

---

### MUJOCO-SDV-017

sec_id: sec_msdv_017q

- テストID：MUJOCO-SDV-017
- 分類：strict / permissive mode
- 内容：strict mode で warning のみの場合を検証する
- 期待結果：
  - `output_allowed = true`
  - 主要成果物が出力される
  - warning は Conversion Report に記録される

---

### MUJOCO-SDV-018

sec_id: sec_msdv_018r

- テストID：MUJOCO-SDV-018
- 分類：strict / permissive mode
- 内容：strict mode で error が存在する場合を検証する
- 期待結果：
  - `output_allowed = false`
  - 主要成果物は出力されない
  - Conversion Report と Diagnostics は出力される

---

## 7. 成功条件

以下を満たすこと。

- Schema Registry が仕様通り検証される
- Adapter Capability が仕様通り照合される
- `io_scope` 全値が期待通り分類される
- fallback 全値が期待通り処理される
- Diagnostics が Validation Error Code と整合する
- Conversion Report が Diagnostics と整合する
- strict / permissive mode の成果物出力可否が仕様と一致する
- CI exit code が仕様と一致する

---

## 8. 失敗条件

以下の場合は失敗とする。

- Schema Registry に反して推測で変換している
- Adapter Capability に存在しない機能を使用している
- `io_scope` と mapping / artifact の整合性が崩れている
- fallback が Diagnostics または Conversion Report に記録されない
- Diagnostics の code / severity / category が不一致
- Conversion Report の summary と実データが不一致
- strict mode で block すべき成果物が出力される
- CI exit code が仕様と一致しない

---

## 9. テストデータ管理

テストデータは以下の構成で管理する。

```text
tests/convert/mujoco/schema_driven_validation/
  registry/
  capability/
  inputs/
  expected_reports/
  expected_diagnostics/
  expected_artifacts/
```

各テストケースは、入力 registry、Adapter Capability、SansaVRM 入力、期待 Conversion Report、期待 Diagnostics を分離して管理する。

---

## 10. 自動化

- 本テストは CI に組み込む
- Schema Registry 変更時に必ず実行する
- Adapter Capability 変更時に必ず実行する
- Validation Error Code 定義変更時に必ず実行する
- Conversion Report / Diagnostics schema 変更時に必ず実行する

---

## 11. 結論

本テスト仕様により、MuJoCo 連携における schema-driven validation、Adapter Capability 照合、fallback、Diagnostics、Conversion Report、Validation Error Code の一貫性を保証する。

---

[目次](../../目次.md) > テスト仕様 > 共通 > MuJoCo Schema-Driven Validationテスト仕様
