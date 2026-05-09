<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-001400Z-SV0J
lang: ja-JP
canonical_title: PoC Validator Adapter設計仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > PoC Validator Adapter設計仕様

# PoC Validator Adapter設計仕様

## 1. 目的

本仕様は、MuJoCo Schema Registry、Adapter Capability、Conversion Report、Diagnostics、Validation Error Code を用いた PoC Validator および PoC Adapter の設計を定義する。

本仕様の目的は以下とする。

- schema-driven validation の最小実装単位を固定する
- MuJoCo Adapter 実装前に検証パイプラインを成立させる
- registry package、capability package、入力モデル、成果物、diagnostics、conversion report の流れを定義する
- SansaVRM 本体と Adapter の責務境界を実装可能な粒度で固定する
- strict / permissive mode の実行フローを固定する
- PoC で検証すべき成功条件と失敗条件を明確化する

---

## 2. 基本方針

- PoC Validator は schema-driven validation の最小構成を検証する
- PoC Adapter は MuJoCo runtime を実行しない
- PoC Adapter は MJCF 相当の成果物と Adapter 側補助成果物を生成できる最小実装とする
- SansaVRM 本体は MuJoCo runtime に依存しない
- Adapter は SansaVRM 本体の公開 API または同等の入力データ構造のみを使用する
- registry に存在しない項目は推測変換しない
- capability に存在しない機能は使用しない
- Diagnostics と Conversion Report は必ず生成する
- `output_allowed = false` の場合でも Diagnostics と Conversion Report は生成する

---

## 3. スコープ

本仕様の対象は以下とする。

- PoC Validator の責務
- PoC Adapter の責務
- 入力ファイル構成
- registry package 実ファイル形式
- capability package 実ファイル形式
- validation pipeline
- adapter execution pipeline
- diagnostics emission flow
- conversion report generation flow
- strict / permissive mode execution flow
- PoC 成功条件

本仕様の対象外は以下とする。

- MuJoCo runtime の実行
- 物理シミュレーション結果の検証
- MJCF 完全準拠 validator
- 高性能化
- GUI / IDE 連携
- ネットワーク同期

---

## 4. PoC構成

PoC は以下の構成とする。

```text
tools/
  mujoco_schema_validator/
    validate.py
    schema_loader.py
    capability_loader.py
    diagnostics.py
    report_writer.py
    rules/
      registry_rules.py
      capability_rules.py
      io_scope_rules.py
      fallback_rules.py
      error_code_rules.py
  mujoco_poc_adapter/
    export.py
    adapter.py
    artifact_writer.py
    mjcf_writer.py
    controller_config_writer.py
```

テストデータは以下に配置する。

```text
tests/convert/mujoco/schema_driven_validation/
  registry/
  capability/
  inputs/
  expected_reports/
  expected_diagnostics/
  expected_artifacts/
```

---

## 5. 入力成果物

PoC Validator / PoC Adapter は、少なくとも以下を入力として受け取る。

| 入力 | 必須 | 説明 |
|---|---:|---|
| SansaVRM input | 必須 | 変換対象の SansaVRM 相当 JSON |
| registry package | 必須 | MuJoCo Schema Registry |
| capability package | 必須 | Adapter Capability |
| error code catalog | 必須 | Validation Error Code 定義 |
| execution config | 必須 | strict / permissive 等の実行設定 |

---

## 6. 出力成果物

PoC Validator / PoC Adapter は、少なくとも以下を出力する。

| 出力 | 必須 | 説明 |
|---|---:|---|
| Diagnostics | 必須 | 診断情報 |
| Conversion Report | 必須 | 変換報告情報 |
| MJCF artifact | 条件付き | `output_allowed = true` かつ MJCF 出力対象がある場合 |
| Adapter artifact | 条件付き | `output_allowed = true` かつ補助成果物出力対象がある場合 |

`output_allowed = false` の場合、主要成果物は出力してはならない。

ただし、Diagnostics と Conversion Report は常に出力する。

---

## 7. registry package実ファイル形式

registry package は JSON とする。

ファイル名例：

```text
mujoco_schema_registry_2_3.json
```

最小構造は以下とする。

```json
{
  "schema_id": "schema-mujoco-2.3",
  "schema_version": "0.1.0",
  "runtime": "mujoco",
  "runtime_version_range": {
    "min": "2.3.0",
    "max": null
  },
  "entries": [],
  "diagnostics_policy": {
    "unknown_entry_is_warning": true
  }
}
```

---

## 8. capability package実ファイル形式

capability package は JSON とする。

ファイル名例：

```text
sansa_vrm_mujoco_adapter_capability_0_1.json
```

最小構造は以下とする。

```json
{
  "capability_id": "capability-sansa-vrm-mujoco-adapter-0.1.0",
  "capability_version": "0.1.0",
  "adapter_id": "sansa-vrm-mujoco-adapter",
  "adapter_version": "0.1.0",
  "runtime": "mujoco",
  "runtime_version_range": {
    "min": "2.3.0",
    "max": null
  },
  "supported_namespaces": [],
  "supported_targets": [],
  "supported_io_scopes": [],
  "supported_mappings": [],
  "supported_artifacts": [],
  "supported_value_conversions": [],
  "unsupported_entries": [],
  "diagnostics_policy": {
    "unsupported_is_error_in_strict": true,
    "fallback_is_warning": true
  }
}
```

---

## 9. error code catalog実ファイル形式

error code catalog は JSON とする。

ファイル名例：

```text
validation_error_codes_0_1.json
```

最小構造は以下とする。

```json
{
  "catalog_id": "validation-error-codes-0.1.0",
  "catalog_version": "0.1.0",
  "codes": []
}
```

各 code は以下を持つ。

```json
{
  "code": "SCHEMA_MISSING_REQUIRED_FIELD",
  "category": "SCHEMA",
  "severity": "error",
  "strict_block": true,
  "ci_exit_code": 5,
  "recoverable": false,
  "fallback_possible": false,
  "lossy": false,
  "requires_manual_review": false
}
```

---

## 10. execution config

execution config は JSON とする。

最小構造は以下とする。

```json
{
  "mode": "export",
  "strict": true,
  "source_format": "sansa_vrm",
  "target_format": "mujoco_mjcf",
  "runtime": "mujoco",
  "runtime_version": "2.3.0",
  "output_dir": "out/mujoco"
}
```

---

## 11. validation pipeline

PoC Validator は以下の順で実行する。

1. execution config を読み込む
2. registry package を読み込む
3. capability package を読み込む
4. error code catalog を読み込む
5. SansaVRM input を読み込む
6. registry package 構造を検証する
7. capability package 構造を検証する
8. error code catalog 構造を検証する
9. registry entry ごとに `io_scope` 整合性を検証する
10. registry entry ごとに Adapter Capability と照合する
11. runtime version を照合する
12. fallback を評価する
13. Diagnostics を生成する
14. Conversion Report を生成する
15. `output_allowed` を判定する

---

## 12. adapter execution pipeline

PoC Adapter は以下の順で実行する。

1. PoC Validator を実行する
2. `output_allowed` を確認する
3. `output_allowed = false` の場合、主要成果物生成を停止する
4. `output_allowed = true` の場合、parameter_results を参照する
5. `io_scope = mjcf` または `both` の項目を MJCF artifact へ出力する
6. `io_scope = adapter_artifact` または `both` の項目を補助成果物へ出力する
7. `preserve_only` / `source_raw` / `unsupported` は Conversion Report に記録する
8. Diagnostics と Conversion Report を出力する

---

## 13. io_scope処理フロー

`io_scope` ごとの処理は以下とする。

| io_scope | PoC Validator | PoC Adapter |
|---|---|---|
| `mjcf` | `mjcf_mapping` 必須検証 | MJCF artifact へ出力 |
| `adapter_artifact` | `adapter_artifact` 必須検証 | 補助成果物へ出力 |
| `both` | 両方必須検証 | MJCF と補助成果物へ出力 |
| `preserve_only` | mapping 不在検証 | 外部成果物へ出力しない |
| `unsupported` | diagnostics 生成 | 外部成果物へ出力しない |
| `source_raw` | 非解釈保持検証 | 外部成果物へ出力しない |

---

## 14. fallback処理フロー

fallback は以下の順で評価する。

1. registry entry の fallback を取得する
2. 未対応理由を特定する
3. fallback behavior を判定する
4. Diagnostics を生成する
5. Conversion Report の `fallback_results` へ記録する
6. fallback behavior が `error` の場合、strict mode では `output_allowed = false` とする

---

## 15. Diagnostics生成フロー

Diagnostics は以下の入力から生成する。

- error code catalog
- registry entry
- capability check result
- fallback result
- target information
- execution config

Diagnostics は、Validation Error Code 定義と以下を一致させなければならない。

- `code`
- `category`
- `severity`
- `strict_block`

---

## 16. Conversion Report生成フロー

Conversion Report は以下を集約して生成する。

- execution config
- Adapter 情報
- Schema Registry 情報
- Adapter Capability 情報
- parameter_results
- fallback_results
- preserved_results
- lossy_results
- diagnostics_summary
- diagnostics
- output_allowed

Conversion Report は、主要成果物が出力されない場合でも生成する。

---

## 17. output_allowed判定

`output_allowed` は以下の条件で `false` とする。

- strict mode で `severity = error` の Diagnostics が存在する
- strict mode で `strict_block = true` の Diagnostics が存在する
- `output_action = block_output` が存在する
- fatal internal error が存在する

上記以外は `true` としてよい。

---

## 18. CLI設計

PoC Validator の CLI は以下とする。

```powershell
python tools/mujoco_schema_validator/validate.py `
  --input tests/convert/mujoco/schema_driven_validation/inputs/sample_sansavrm.json `
  --registry tests/convert/mujoco/schema_driven_validation/registry/mujoco_schema_registry_2_3.json `
  --capability tests/convert/mujoco/schema_driven_validation/capability/sansa_vrm_mujoco_adapter_capability_0_1.json `
  --error-codes tests/convert/mujoco/schema_driven_validation/registry/validation_error_codes_0_1.json `
  --config tests/convert/mujoco/schema_driven_validation/inputs/export_strict_config.json `
  --report out/conversion_report.json `
  --diagnostics out/diagnostics.json
```

PoC Adapter の CLI は以下とする。

```powershell
python tools/mujoco_poc_adapter/export.py `
  --input tests/convert/mujoco/schema_driven_validation/inputs/sample_sansavrm.json `
  --registry tests/convert/mujoco/schema_driven_validation/registry/mujoco_schema_registry_2_3.json `
  --capability tests/convert/mujoco/schema_driven_validation/capability/sansa_vrm_mujoco_adapter_capability_0_1.json `
  --error-codes tests/convert/mujoco/schema_driven_validation/registry/validation_error_codes_0_1.json `
  --config tests/convert/mujoco/schema_driven_validation/inputs/export_strict_config.json `
  --out out/mujoco
```

---

## 19. PoC成功条件

PoC は以下を満たす場合に成功とする。

- registry package を読み込めること
- capability package を読み込めること
- error code catalog を読み込めること
- `io_scope` 全値を分類できること
- Adapter Capability と照合できること
- fallback を評価できること
- Diagnostics を生成できること
- Conversion Report を生成できること
- strict mode で `output_allowed = false` の場合、主要成果物を出力しないこと
- permissive mode で許容可能な fallback を処理できること
- テスト仕様 `MUJOCO-SDV-001` から `MUJOCO-SDV-018` の検証に着手可能であること

---

## 20. PoC失敗条件

以下の場合は失敗とする。

- registry に存在しない項目を推測変換している
- capability に存在しない機能を使用している
- `io_scope` と mapping / artifact の整合性を検証していない
- fallback 結果が Diagnostics に記録されない
- fallback 結果が Conversion Report に記録されない
- Diagnostics が Validation Error Code と不整合
- Conversion Report の summary が Diagnostics と不整合
- `output_allowed = false` で主要成果物を出力する
- SansaVRM 本体が MuJoCo runtime に依存する

---

## 21. 実装順序

PoC 実装は以下の順で行う。

1. error code catalog 読み込み
2. registry package 読み込み
3. capability package 読み込み
4. registry 構造検証
5. capability 構造検証
6. `io_scope` 整合性検証
7. Adapter Capability 照合
8. fallback 評価
9. Diagnostics 生成
10. Conversion Report 生成
11. `output_allowed` 判定
12. PoC Adapter による成果物分離
13. testspec に基づくテストデータ作成
14. CI 組み込み

---

## 22. 非スコープ

PoC では以下を行わない。

- MuJoCo runtime 実行
- MuJoCo API 呼び出し
- 完全な MJCF schema validation
- 物理挙動の妥当性評価
- 性能最適化
- 外部 GUI 連携

---

## 23. 結論

本設計により、MuJoCo Schema Registry、Adapter Capability、Diagnostics、Conversion Report、Validation Error Code を用いた schema-driven validation の最小実装単位を固定する。

これにより、SansaVRM 本体を MuJoCo runtime に依存させず、Adapter 境界を保ったまま、MuJoCo 連携の PoC 実装と自動検証へ進むことができる。

---

[目次](../../目次.md) > 仕様 > 共通 > PoC Validator Adapter設計仕様
