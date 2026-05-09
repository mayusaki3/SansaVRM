<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260509-001300Z-SV0J
lang: ja-JP
canonical_title: Validation Error Code体系仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Validation Error Code体系仕様

# Validation Error Code体系仕様

## 1. 目的

本仕様は、SansaVRM および関連 Adapter、Validator、Schema Registry、Conversion Report、Diagnostics において使用する Validation Error Code の命名規則、分類、severity、strict block 条件、CI 連携、machine-readable 判定規則を定義する。

Validation Error Code は、schema-driven validation、Adapter Capability 照合、fallback、非可逆変換、runtime version 判定、成果物出力可否判定を一貫した形式で機械可読に識別するために使用する。

本仕様により、以下を保証する。

- diagnostics code の安定した識別子体系を提供する
- Validator、CI、IDE、UI、regression test 間で共通判定できる
- strict モードにおける成果物出力可否を機械判定できる
- Adapter ごとの差異を共通分類へ正規化できる
- 非可逆変換や fallback を分類可能にする
- code から severity、category、blocking 条件を導出できる

---

## 2. 基本方針

- Validation Error Code は machine-readable な安定識別子とする
- Validation Error Code は人間向けメッセージと分離する
- 同一意味の diagnostics は同一 code を使用しなければならない
- Validation Error Code は後方互換性を重視し、意味変更してはならない
- severity は code ごとに定義する
- strict block 条件は code ごとに定義する
- Adapter 固有エラーも共通 category に正規化しなければならない
- diagnostics message はローカライズ可能とするが、code は固定とする

---

## 3. スコープ

本仕様の対象は以下とする。

- Validation Error Code 命名規則
- category 分類
- severity 定義
- strict block 定義
- CI exit code 連携
- machine-readable classification
- fallback 分類
- lossy conversion 分類
- Adapter Capability 分類
- Schema Registry 分類
- diagnostics との対応

本仕様の対象外は以下とする。

- 人間向け UI 文言
- localization 実装
- logging backend
- monitoring system 実装
- 外部 runtime 自体のエラーコード

---

## 4. Validation Error Code構造

Validation Error Code は以下の形式とする。

```text
<CATEGORY>_<DETAIL>
```

例：

```text
SCHEMA_MISSING_REQUIRED_FIELD
ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING
FALLBACK_DEFAULT_APPLIED
LOSSY_PRECISION_LOSS
```

Validation Error Code は ASCII 大文字、数字、アンダースコアのみを使用する。

---

## 5. category

category は diagnostics の大分類を表す。

使用可能な category は以下とする。

| category | 用途 |
|---|---|
| `SCHEMA` | Schema Registry 検証 |
| `ADAPTER_CAPABILITY` | Adapter Capability 照合 |
| `VERSION` | runtime / schema / adapter version 判定 |
| `FALLBACK` | fallback 適用 |
| `LOSSY` | 非可逆変換 |
| `UNSUPPORTED` | 未対応 |
| `SOURCE_RAW` | source_raw 保持 |
| `ARTIFACT` | 成果物入出力 |
| `VALIDATION` | 一般検証 |
| `TRACEABILITY` | traceability |
| `INTERNAL` | 内部エラー |

---

## 6. severity

severity は diagnostics の重要度を表す。

使用可能な値は以下とする。

- `info`
- `warning`
- `error`

各値の意味は以下とする。

| severity | 意味 |
|---|---|
| `info` | 処理継続可能で影響が限定的 |
| `warning` | 処理継続可能だが注意が必要 |
| `error` | strict モードでは成果物出力を禁止する |

severity は code ごとに固定定義する。

同一 code に対して severity を変更してはならない。

---

## 7. strict_block

`strict_block` は strict モードで成果物出力を禁止するかを表す。

使用可能な値は以下とする。

- `true`
- `false`

`severity = error` の場合でも、必ずしも `strict_block = true` とは限らない。

例：

- `LOSSY_PRECISION_LOSS`
  - severity = warning
  - strict_block = false

- `SCHEMA_MISSING_REQUIRED_FIELD`
  - severity = error
  - strict_block = true

---

## 8. machine-readable classification

Validation Error Code は、少なくとも以下の machine-readable 情報へ変換可能でなければならない。

| 項目 | 説明 |
|---|---|
| `category` | diagnostics category |
| `severity` | diagnostics severity |
| `strict_block` | strict モード blocking |
| `ci_exit_code` | CI 用 exit code |
| `recoverable` | recoverable かどうか |
| `fallback_possible` | fallback 可否 |
| `lossy` | 非可逆変換かどうか |
| `requires_manual_review` | 手動確認が必要か |

---

## 9. CI exit code

CI exit code は Validation Error Code 群から導出できなければならない。

標準 exit code は以下とする。

| exit code | 意味 |
|---:|---|
| `0` | 成功 |
| `1` | warning のみ |
| `2` | strict block error |
| `3` | internal error |
| `4` | traceability error |
| `5` | schema validation error |
| `6` | adapter capability error |
| `7` | artifact generation error |

複数 category が存在する場合、最優先 category を使用する。

優先順位は以下とする。

```text
INTERNAL
TRACEABILITY
SCHEMA
ADAPTER_CAPABILITY
ARTIFACT
VALIDATION
LOSSY
FALLBACK
UNSUPPORTED
SOURCE_RAW
```

---

## 10. recoverable

`recoverable` は、自動 fallback または preserve により処理継続可能かを表す。

使用可能な値は以下とする。

- `true`
- `false`

例：

| code | recoverable |
|---|---:|
| `FALLBACK_DEFAULT_APPLIED` | true |
| `SOURCE_RAW_STORED` | true |
| `SCHEMA_INVALID_IO_SCOPE` | false |
| `INTERNAL_UNEXPECTED_EXCEPTION` | false |

---

## 11. fallback_possible

`fallback_possible` は fallback 適用可能かを表す。

使用可能な値は以下とする。

- `true`
- `false`

fallback 不可能な場合、strict モードでは成果物出力を禁止してよい。

---

## 12. lossy

`lossy` は非可逆変換を伴うかを表す。

使用可能な値は以下とする。

- `true`
- `false`

`lossy = true` の場合、Conversion Report の `lossy_results` へ記録しなければならない。

---

## 13. requires_manual_review

`requires_manual_review` は、人間による確認を推奨するかを表す。

使用可能な値は以下とする。

- `true`
- `false`

以下は原則 `true` とする。

- semantic loss
- approximation
- unsupported runtime feature
- custom value conversion
- Adapter 固有拡張

---

## 14. code安定性

Validation Error Code は stable identifier として扱う。

以下は禁止する。

- 同一 code の意味変更
- 同一 code の severity 変更
- 同一 code の strict_block 変更
- 同一 code の category 変更

意味変更が必要な場合は新規 code を作成しなければならない。

---

## 15. code廃止

Validation Error Code を廃止する場合、以下を定義しなければならない。

| 項目 | 必須 | 説明 |
|---|---:|---|
| `deprecated_since` | 必須 | 廃止開始 version |
| `replacement_code` | 任意 | 代替 code |
| `reason` | 必須 | 廃止理由 |

廃止済み code は後方互換性のため読み取り可能でなければならない。

---

## 16. 標準code定義

### 16.1 SCHEMA

| code | severity | strict_block |
|---|---|---:|
| `SCHEMA_MISSING_REQUIRED_FIELD` | error | true |
| `SCHEMA_INVALID_IO_SCOPE` | error | true |
| `SCHEMA_INVALID_MAPPING` | error | true |
| `SCHEMA_INVALID_ARTIFACT_RULE` | error | true |
| `SCHEMA_INVALID_VALUE_TYPE` | error | true |
| `SCHEMA_VERSION_OUT_OF_RANGE` | error | true |
| `SCHEMA_UNKNOWN_ENTRY` | warning | false |

### 16.2 ADAPTER_CAPABILITY

| code | severity | strict_block |
|---|---|---:|
| `ADAPTER_CAPABILITY_UNSUPPORTED_NAMESPACE` | warning | false |
| `ADAPTER_CAPABILITY_UNSUPPORTED_TARGET` | warning | false |
| `ADAPTER_CAPABILITY_UNSUPPORTED_IO_SCOPE` | warning | false |
| `ADAPTER_CAPABILITY_UNSUPPORTED_MAPPING` | warning | false |
| `ADAPTER_CAPABILITY_UNSUPPORTED_ARTIFACT` | warning | false |
| `ADAPTER_CAPABILITY_UNSUPPORTED_VALUE_CONVERSION` | warning | false |
| `ADAPTER_CAPABILITY_EXPLICIT_UNSUPPORTED_ENTRY` | warning | false |
| `ADAPTER_CAPABILITY_CONFLICT_WITH_REGISTRY` | error | true |

### 16.3 FALLBACK

| code | severity | strict_block |
|---|---|---:|
| `FALLBACK_DEFAULT_APPLIED` | info | false |
| `FALLBACK_PRESERVE_ONLY_APPLIED` | info | false |
| `FALLBACK_IGNORE_APPLIED` | warning | false |
| `FALLBACK_NOT_AVAILABLE` | error | true |

### 16.4 LOSSY

| code | severity | strict_block |
|---|---|---:|
| `LOSSY_PRECISION_LOSS` | warning | false |
| `LOSSY_SEMANTIC_LOSS` | warning | false |
| `LOSSY_APPROXIMATION_APPLIED` | warning | false |
| `LOSSY_UNSUPPORTED_FEATURE_DROPPED` | warning | false |

### 16.5 UNSUPPORTED

| code | severity | strict_block |
|---|---|---:|
| `UNSUPPORTED_RUNTIME_FEATURE` | warning | false |
| `UNSUPPORTED_ADAPTER_FEATURE` | warning | false |
| `UNSUPPORTED_PARAMETER` | warning | false |

### 16.6 SOURCE_RAW

| code | severity | strict_block |
|---|---|---:|
| `SOURCE_RAW_STORED` | info | false |
| `SOURCE_RAW_IMPORT_ONLY` | warning | false |

### 16.7 ARTIFACT

| code | severity | strict_block |
|---|---|---:|
| `ARTIFACT_OUTPUT_BLOCKED` | error | true |
| `ARTIFACT_GENERATION_FAILED` | error | true |
| `ARTIFACT_PARTIAL_OUTPUT` | warning | false |

### 16.8 TRACEABILITY

| code | severity | strict_block |
|---|---|---:|
| `TRACEABILITY_MISSING_SEC_ID` | error | true |
| `TRACEABILITY_INVALID_REF_ID` | error | true |
| `TRACEABILITY_DUPLICATE_IDENTIFIER` | error | true |

### 16.9 INTERNAL

| code | severity | strict_block |
|---|---|---:|
| `INTERNAL_UNEXPECTED_EXCEPTION` | error | true |
| `INTERNAL_STATE_CORRUPTION` | error | true |
| `INTERNAL_NOT_IMPLEMENTED` | error | true |

---

## 17. diagnosticsとの対応

Diagnostics は Validation Error Code を必ず持たなければならない。

Diagnostics の `severity` は code 定義と一致しなければならない。

Diagnostics の `category` は code の category と一致しなければならない。

---

## 18. Conversion Reportとの対応

Conversion Report は Validation Error Code を集計可能でなければならない。

少なくとも以下を集計する。

- category 別件数
- severity 別件数
- strict_block 件数
- recoverable 件数
- fallback 件数
- lossy 件数
- manual review 推奨件数

---

## 19. strict mode判定

strict モードでは、以下を満たす場合に成果物出力を禁止する。

- `strict_block = true`
- または `output_action = block_output`

warning のみの場合、成果物出力を継続してよい。

---

## 20. permissive mode判定

permissive モードでは、以下を許可してよい。

- fallback 適用
- preserve_only
- source_raw
- unsupported の保持
- lossy conversion

ただし、以下は出力禁止としてよい。

- internal error
- state corruption
- unrecoverable schema corruption

---

## 21. JSON例

Validation Error Code 定義例を以下に示す。

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
  "requires_manual_review": false,
  "description": "A required schema field is missing."
}
```

fallback 系 code の例を以下に示す。

```json
{
  "code": "FALLBACK_DEFAULT_APPLIED",
  "category": "FALLBACK",
  "severity": "info",
  "strict_block": false,
  "ci_exit_code": 1,
  "recoverable": true,
  "fallback_possible": true,
  "lossy": false,
  "requires_manual_review": false,
  "description": "Fallback default value was applied."
}
```

lossy 系 code の例を以下に示す。

```json
{
  "code": "LOSSY_SEMANTIC_LOSS",
  "category": "LOSSY",
  "severity": "warning",
  "strict_block": false,
  "ci_exit_code": 1,
  "recoverable": true,
  "fallback_possible": false,
  "lossy": true,
  "requires_manual_review": true,
  "description": "Semantic meaning could not be fully preserved."
}
```

---

## 22. 実装責務分離

### 22.1 SansaVRM本体

SansaVRM 本体は以下を担当する。

- Validation Error Code 定義の保持または参照
- diagnostics code の整合性検証
- Conversion Report 集計 API の提供
- strict_block 判定 API の提供

### 22.2 Adapter / Validator

Adapter および Validator は以下を担当する。

- Validation Error Code に従った diagnostics 生成
- code と severity の整合性維持
- strict_block 判定
- CI exit code 生成
- fallback / lossy / unsupported 分類

---

## 23. 検証要件

Validation Error Code体系に対するテスト仕様では、少なくとも以下を検証する。

- code が命名規則に従うこと
- code が category と整合すること
- Diagnostics の severity が code 定義と一致すること
- strict_block 判定が code 定義と一致すること
- CI exit code が category 優先順位に従うこと
- lossy code が Conversion Report の `lossy_results` に記録されること
- fallback code が `fallback_results` に記録されること
- unsupported code が preserve_only または source_raw と整合すること
- deprecated code が後方互換で読み取り可能であること
- strict モードで strict_block = true の場合に output_allowed = false となること

---

[目次](../../目次.md) > 仕様 > 共通 > Validation Error Code体系仕様
