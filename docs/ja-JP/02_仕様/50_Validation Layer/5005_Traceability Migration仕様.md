<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5005-traceability-migration-specification
lang: ja-JP
canonical_title: Traceability Migration仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > Traceability Migration仕様

# Traceability Migration仕様

## 1. 目的

本仕様は、SansaVRM および HLDocS ベース仕様群における traceability migration の管理方法を定義する。

本仕様の目的は以下とする。

```text
- doc_id relocation の追跡
- sec_id continuity の維持
- split / merge migration の管理
- semantic equivalence の記録
- staged relocation の管理
- migration observability の提供
- dry-run relocation の安全化
```

---

## 2. 基本方針

Traceability Migration は、単なる path rename ではなく、traceability graph の変換として扱う。

以下を原則とする。

```text
1. migration は graph transformation として扱う
2. doc_id continuity と sec_id continuity を分離する
3. split / merge を正式に許可する
4. semantic equivalence を明示する
5. placeholder relocation を許可する
6. staged relocation を許可する
7. migration state を observability 対象とする
```

---

## 3. Migration Entry

Migration Entry は単一 migration 単位を表す。

最低限以下を持つ。

```text
entry_id
migration_type
mapping_status
semantic_equivalent
old
new
sec_mappings
migration_reason
```

---

## 4. migration_type

migration_type は以下を許可する。

```text
path_relocation
split_relocation
merge_relocation
placeholder_relocation
staged_relocation
sec_id_remap
semantic_restructure
```

---

## 5. mapping_status

mapping_status は以下を許可する。

```text
pending
partial
complete
verified
rejected
obsolete
```

---

## 6. doc_id continuity

doc_id continuity は document-level traceability continuity を表す。

以下を許可する。

```text
- old_doc_id → new_doc_id
- old_doc_id → multiple new_doc_id
- multiple old_doc_id → single new_doc_id
```

---

## 7. sec_id continuity

sec_id continuity は section-level traceability continuity を表す。

sec_id continuity は doc_id continuity と独立して管理する。

最低限以下を持つ。

```text
old_sec_id
new_sec_id
semantic_equivalent
mapping_reason
```

---

## 8. split relocation

split relocation は、1文書を複数文書へ分離する migration を表す。

例：

```text
01_共通
↓
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
```

split relocation では以下を保持する。

```text
- split boundary
- semantic scope
- sec_id mapping
- orphan detection
```

---

## 9. merge relocation

merge relocation は、複数文書を単一文書へ統合する migration を表す。

merge relocation では以下を保持する。

```text
- merged source list
- semantic equivalence
- duplicate sec_id resolution
- traceability merge policy
```

---

## 10. placeholder relocation

placeholder relocation は、巨大仕様 migration を段階化するための仮配置 migration を表す。

以下を許可する。

```text
- path reservation
- temporary doc_id
- partial hierarchy link migration
- staged finalize
```

placeholder relocation は semantic complete を意味しない。

---

## 11. staged relocation

staged relocation は複数段階 migration を表す。

例：

```text
create
↓
placeholder
↓
partial
↓
complete
↓
verified
```

各段階は diagnostics 対象とする。

---

## 12. semantic equivalence

semantic_equivalent は migration 前後で意味保持されているかを示す。

以下を許可する。

```text
true
false
partial
unknown
```

---

## 13. orphan detection

Migration validator は以下を orphan として検出する。

```text
- orphan doc_id
- orphan sec_id
- orphan hierarchy link
- orphan reference
- orphan migration entry
```

---

## 14. duplicate detection

Migration validator は以下を検出する。

```text
- duplicate entry_id
- duplicate doc_id
- duplicate sec_id
- duplicate path
- duplicate mapping
```

---

## 15. migration diagnostics

Migration validator は diagnostics と連携する。

例：

```text
MIGRATION_MAPPING_PENDING
SEC_ID_MAPPING_MISSING
SEMANTIC_EQUIVALENCE_UNKNOWN
INVALID_SPLIT_MAPPING
INVALID_MERGE_MAPPING
```

---

## 16. migration graph

Migration graph は migration 全体の traceability continuity を表す。

最低限以下を持つ。

```text
migration_id
source_revision
target_revision
entry_list
state_summary
```

---

## 17. migration manifest

migration manifest は migration graph のシリアライズ表現である。

最低限以下を持つ。

```text
manifest_version
migration_id
migration_type
entries
```

---

## 18. dry-run relocation

dry-run relocation は、本適用前に migration graph を検証するモードである。

以下を目的とする。

```text
- path collision detection
- duplicate detection
- orphan detection
- hierarchy validation
- sec_id continuity validation
```

---

## 19. CI Requirements

CI は migration manifest を検査対象に含めることができる。

例：

```text
- pending migration が残る場合は fail
- semantic_equivalent = unknown が残る場合は fail
- orphan sec_id が存在する場合は fail
- duplicate path が存在する場合は fail
```

---

## 20. HLDocS feedback

本仕様で得られた知見：

```text
- Traceability には静的 Traceability と Migration Traceability が存在する
- doc_id continuity だけでは不十分
- sec_id continuity が長期的に重要
- relocation は graph migration として扱うべき
- placeholder relocation が巨大仕様 migration に有効
- migration observability が必要
```

---

## 21. 関連仕様

本仕様は以下と連携する。

```text
Diagnostics仕様
Validator実装仕様
Migration Manifest Specification
RoundTrip Semantic Criteria
```

---

## 22. 結論

Traceability Migration は、仕様進化に伴う traceability continuity を維持するための migration graph 管理機構である。

これにより、split relocation、merge relocation、placeholder relocation、staged relocation を安全に実施し、sec_id continuity を維持できる。

---

[目次](../../目次.md) > 仕様 > Validation Layer > Traceability Migration仕様
