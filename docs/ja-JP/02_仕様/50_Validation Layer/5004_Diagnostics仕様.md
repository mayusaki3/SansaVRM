<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5004-diagnostics-specification
lang: ja-JP
canonical_title: Diagnostics仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > Diagnostics仕様

# Diagnostics仕様

## 1. 目的

本仕様は、SansaVRM における diagnostics の構造、分類、重大度、出力責務を定義する。

Diagnostics は、以下の処理で発生した問題、警告、情報を可視化するために使用する。

```text
- JSON Schema validation
- Validator validation
- Import
- Export
- RoundTrip
- Conversion Profile application
- Adapter execution
- Migration validation
```

---

## 2. 基本方針

Diagnostics は silent loss を防止するための観測可能性レイヤである。

以下を原則とする。

```text
1. semantic loss を記録する
2. approximation を記録する
3. unsupported を記録する
4. preservation_only を記録する
5. validator / adapter / migration が同一形式で出力できる
6. CI で検査可能にする
```

---

## 3. Diagnostics Item

Diagnostics Item は、単一の検出結果を表す。

最低限以下を持つ。

```text
diagnostic_id
code
message
severity
category
source
path
related_ref
```

---

## 4. severity

severity は以下を許可する。

```text
info
warning
error
critical
```

### 4.1 info

処理継続に影響しない参考情報。

### 4.2 warning

処理継続可能だが、注意が必要な状態。

### 4.3 error

仕様違反または変換不能状態。

### 4.4 critical

出力禁止または処理停止を伴う重大エラー。

---

## 5. category

category は以下を許可する。

```text
schema_validation
semantic_validation
reference_integrity
conversion
roundtrip
preservation
adapter
runtime
migration
traceability
compatibility
```

---

## 6. code

code は機械判定可能な識別子とする。

例：

```text
REF_NOT_FOUND
DUPLICATE_DOC_ID
UNSUPPORTED_EXPORT
SEMANTIC_LOSS
APPROXIMATION_APPLIED
PRESERVATION_ONLY
MIGRATION_MAPPING_PENDING
```

---

## 7. source

source は diagnostics を生成した主体を示す。

例：

```text
json_schema_validator
sansavrm_validator
conversion_adapter
mujoco_adapter
migration_manifest_validator
ci
```

---

## 8. path

path は問題箇所を示す。

形式は以下を許可する。

```text
json_pointer
file_path
hldocs_sec_id
logical_path
```

---

## 9. related_ref

related_ref は関連する仕様、テスト、コード、変換対象を参照する。

例：

```text
doc_id
sec_id
test_id
code_path
source_format_path
target_format_path
```

---

## 10. Diagnostics と Loss Report の関係

Diagnostics と Loss Report は分離する。

```text
Diagnostics:
検出された問題や警告を記録する

Loss Report:
変換時に失われた意味や近似を記録する
```

ただし、semantic loss が発生した場合は、Loss Report と Diagnostics の両方に記録してよい。

---

## 11. Validator Requirements

Validator は以下を diagnostics として出力できなければならない。

```text
- schema validation error
- reference integrity error
- semantic contradiction
- unsupported mapping
- warning_as_error escalation
```

---

## 12. Adapter Requirements

Adapter は以下を diagnostics として出力できなければならない。

```text
- unsupported source feature
- unsupported target feature
- approximation applied
- preservation_only data retained
- adapter_artifact separation
- fallback applied
```

---

## 13. Migration Requirements

Migration validator は以下を diagnostics として出力できなければならない。

```text
- missing migration manifest
- duplicate entry_id
- duplicate path
- orphan doc_id
- orphan sec_id
- pending migration entry
- invalid split mapping
- invalid merge mapping
```

---

## 14. CI Requirements

CI は diagnostics を検査対象に含めることができる。

例：

```text
- critical が存在する場合は fail
- error が存在する場合は fail
- warning_as_error 時は warning で fail
- pending migration が残る場合は release 不可
```

---

## 15. 出力例

```json
{
  "diagnostic_id": "diag-0001",
  "code": "UNSUPPORTED_EXPORT",
  "message": "Runtime semantic cannot be exported to VRM 0.x.",
  "severity": "warning",
  "category": "conversion",
  "source": "vrm_export_adapter",
  "path": "/extensions/SansaVRM_runtime",
  "related_ref": {
    "doc_id": "dry-doc-1003-roundtrip-semantic-criteria",
    "sec_id": null
  }
}
```

---

## 16. 関連仕様

本仕様は以下と連携する。

```text
JSONスキーマ仕様
Validator実装仕様
変換仕様
RoundTrip Semantic Criteria
Migration Manifest Specification
Traceability Migration Specification
```

---

## 17. 結論

Diagnostics は、SansaVRM の validation / conversion / migration における観測可能性の中核である。

Diagnostics により、semantic loss、unsupported、approximation、preservation_only、migration pending を可視化し、silent loss を防止する。

---

[目次](../../目次.md) > 仕様 > Validation Layer > Diagnostics仕様
