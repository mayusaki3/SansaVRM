<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5006-coverage-analysis-specification
lang: ja-JP
canonical_title: Coverage Analysis仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > Coverage Analysis仕様

# Coverage Analysis仕様

## 1. 目的

本仕様は、SansaVRM における traceability coverage および validation coverage の分析方法を定義する。

Coverage Analysis は以下を目的とする。

```text
- spec → schema coverage の分析
- spec → validator coverage の分析
- spec → tests coverage の分析
- spec → implementation coverage の分析
- migration coverage の分析
- orphan detection
- silent unvalidated area の検出
```

---

## 2. 基本方針

Coverage Analysis は validation observability の一部として扱う。

以下を原則とする。

```text
1. coverage は traceability graph 上で分析する
2. sec_id 単位で coverage を分析する
3. migration 中の coverage continuity を維持する
4. orphan coverage を検出する
5. unvalidated semantic を検出する
6. CI で coverage gate を設定可能にする
```

---

## 3. Coverage Unit

Coverage Unit は coverage の最小単位である。

最低限以下を許可する。

```text
doc_id
sec_id
schema_id
validator_rule_id
test_id
implementation_ref
migration_entry_id
```

---

## 4. coverage_type

coverage_type は以下を許可する。

```text
schema_coverage
validator_coverage
test_coverage
implementation_coverage
migration_coverage
roundtrip_coverage
compatibility_coverage
```

---

## 5. schema_coverage

schema_coverage は、仕様が JSON Schema により検査されている割合を表す。

例：

```text
sec_id
↓
schema property
```

---

## 6. validator_coverage

validator_coverage は、仕様が Validator rule により検査されている割合を表す。

例：

```text
sec_id
↓
validator_rule_id
```

---

## 7. test_coverage

test_coverage は、仕様が test により検証されている割合を表す。

例：

```text
sec_id
↓
test_id
```

---

## 8. implementation_coverage

implementation_coverage は、仕様が implementation に接続されている割合を表す。

例：

```text
sec_id
↓
code_path
```

---

## 9. migration_coverage

migration_coverage は relocation 後の traceability continuity を表す。

最低限以下を分析する。

```text
- migrated sec_id
- orphan sec_id
- unresolved mapping
- semantic_equivalent unknown
- pending migration
```

---

## 10. roundtrip_coverage

roundtrip_coverage は semantic preservation verification の coverage を表す。

例：

```text
VRM → SansaVRM → VRM
MMD → SansaVRM → MMD
FBX → SansaVRM → FBX
```

---

## 11. compatibility_coverage

compatibility_coverage は format compatibility の coverage を表す。

例：

```text
VRM 0.x
VRM 1.0
MMD
FBX
URDF
MuJoCo
```

---

## 12. orphan detection

Coverage Analyzer は以下を orphan として検出する。

```text
- spec without schema
- spec without validator
- spec without tests
- spec without implementation
- orphan sec_id
- orphan migration entry
```

---

## 13. silent unvalidated area

Coverage Analyzer は silent unvalidated semantic を検出しなければならない。

例：

```text
- schema coverage only
- validator coverage missing
- migration coverage missing
- roundtrip verification missing
```

---

## 14. coverage status

coverage_status は以下を許可する。

```text
covered
partial
missing
unknown
orphan
```

---

## 15. coverage diagnostics

Coverage Analyzer は diagnostics と連携する。

例：

```text
SCHEMA_COVERAGE_MISSING
VALIDATOR_COVERAGE_MISSING
TEST_COVERAGE_MISSING
IMPLEMENTATION_COVERAGE_MISSING
ORPHAN_SEC_ID
UNVALIDATED_SEMANTIC
```

---

## 16. migration continuity

Coverage Analysis は migration continuity を分析対象に含める。

以下を確認する。

```text
- old_sec_id continuity
- new_sec_id continuity
- split mapping continuity
- merge mapping continuity
- semantic_equivalent continuity
```

---

## 17. coverage graph

Coverage graph は traceability graph 上に構築する。

例：

```text
spec
↓
schema
↓
validator
↓
tests
↓
implementation
↓
runtime verification
```

---

## 18. CI Requirements

CI は coverage gate を設定可能とする。

例：

```text
- validator coverage 100%
- migration coverage missing 禁止
- orphan sec_id 禁止
- semantic_equivalent unknown 禁止
```

---

## 19. HLDocS feedback

本仕様で得られた知見：

```text
- traceability は graph として扱う方が強い
- sec_id 単位 coverage が重要
- migration coverage が必要
- orphan semantic の検出が重要
- validation observability が必要
```

---

## 20. 関連仕様

本仕様は以下と連携する。

```text
Traceability Migration仕様
Diagnostics仕様
Validator実装仕様
JSONスキーマ仕様
```

---

## 21. 結論

Coverage Analysis は、SansaVRM validation graph における coverage continuity と validation observability を保証するための分析レイヤである。

これにより、silent unvalidated semantic、orphan sec_id、migration coverage missing を検出できる。

---

[目次](../../目次.md) > 仕様 > Validation Layer > Coverage Analysis仕様
