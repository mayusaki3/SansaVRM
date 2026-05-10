<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000015Z-SV02
lang: ja-JP
canonical_title: Traceability Migration Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > トレーサビリティ > 共通 > Traceability Migration Specification

# Traceability Migration Specification

## 1. 目的

本仕様は、SansaVRM における仕様体系再構築時の Traceability migration を定義する。

本仕様は以下を対象とする。

```text
- doc_id migration
- sec_id migration
- Layer migration
- specification split
- specification merge
- semantic continuity
- validator migration
```

---

## 2. 背景

SansaVRM は現在、仕様体系の Layer 化と dependency 正規化を進めている。

この過程では以下が発生する。

```text
- 文書番号再配置
- Layer再配置
- specification split
- specification merge
- section責務変更
- sec_id再生成
```

そのため：

```text
旧doc_id / sec_id を固定維持する
```

方式では、semantic構造と Traceability が乖離する危険がある。

---

## 3. 基本方針

本仕様では以下を原則とする。

```text
1. semantic continuity を最優先する
2. doc_id continuity は必須としない
3. sec_id continuity は必須としない
4. migration map を保持する
5. validator が migration を追跡可能であること
6. split / merge migration を許可する
```

---

## 4. 用語定義

## 4.1 semantic continuity

以下を満たすこと。

```text
仕様意味が継承されている
```

完全一致は要求しない。

---

## 4.2 migration

以下を含む。

```text
- doc_id変更
- sec_id変更
- section split
- section merge
- Layer移動
```

---

## 4.3 migration map

旧仕様と新仕様の対応情報。

---

## 5. migration対象

最低限以下を対象とする。

```text
doc_id
sec_id
layer_id
spec_path
validator_scope
runtime_scope
```

---

## 6. doc_id Migration

## 6.1 基本方針

doc_id は再生成可能とする。

以下は許可する。

```text
- Layer再編
- specification split
- specification merge
- semantic再整理
```

---

## 6.2 migration map 必須

doc_id を変更する場合、migration map を保持しなければならない。

最低限以下を持つ。

```text
old_doc_id
new_doc_id
migration_reason
semantic_equivalent
migration_version
```

---

## 7. sec_id Migration

## 7.1 基本方針

sec_id は再生成可能とする。

以下は許可する。

```text
- section split
- section merge
- Layer移動
- dependency整理
```

---

## 7.2 semantic continuity 優先

以下を禁止する。

```text
sec_id continuity を優先して semantic構造を破壊すること
```

semantic continuity を優先する。

---

## 7.3 sec mapping 必須

sec_id を変更する場合、mapping を保持しなければならない。

最低限以下を持つ。

```text
old_sec_id
new_sec_id
mapping_type
semantic_equivalent
```

---

## 8. specification split

仕様 split を許可する。

例：

```text
旧:
Physics仕様

新:
Physics Semantic
Runtime Physics
MuJoCo Physics
```

split 時は以下を保持する。

```text
split_source_doc_id
split_target_doc_list
semantic_mapping
```

---

## 9. specification merge

仕様 merge を許可する。

例：

```text
旧:
Morph仕様
Expression仕様

新:
Morph Semantic仕様
```

merge 時は以下を保持する。

```text
merge_source_doc_list
merge_target_doc_id
semantic_mapping
```

---

## 10. Layer Migration

Layer migration を許可する。

例：

```text
Validation Layer
↓
Compatibility Layer
```

Layer migration 時は以下を保持する。

```text
old_layer_id
new_layer_id
migration_reason
```

---

## 11. migration map structure

推奨 structure：

```json
{
  "migration_version": "1.0",
  "migration_type": "layer_refactor",
  "old_doc_id": "doc-001",
  "new_doc_id": "doc-101",
  "semantic_equivalent": true,
  "sec_mapping": [
    {
      "old_sec_id": "sec_a1",
      "new_sec_id": "sec_b2",
      "mapping_type": "split"
    }
  ]
}
```

---

## 12. semantic equivalence

semantic equivalence は以下を許可する。

```text
- section split
- section merge
- Layer変更
- wording変更
- structure変更
```

ただし以下は禁止する。

```text
semantic meaning loss
```

---

## 13. validator Requirements

validator は以下を検査可能でなければならない。

```text
- migration map validity
- semantic continuity
- orphan sec_id
- orphan doc_id
- invalid split mapping
- invalid merge mapping
```

---

## 14. Traceability Requirements

Traceability は以下を保持できなければならない。

```text
- old specification reference
- migration chain
- semantic continuity
- Layer continuity
```

---

## 15. CI Requirements

CI は以下を検査する。

```text
- missing migration map
- orphan doc_id
- orphan sec_id
- invalid semantic mapping
- invalid migration reference
```

---

## 16. 実施時の禁止事項

以下は禁止する。

```text
- migration map無しのdoc_id変更
- migration map無しのsec_id変更
- semantic lossを伴うmigration
- orphan traceability生成
```

---

## 17. 今後の適用対象

本仕様は以下へ適用する。

```text
- Layer再編
- validator refactor
- specification split
- specification merge
- Runtime分離
- Compatibility分離
```

---

## 18. 関連仕様

本仕様は以下と連携する。

```text
トレーサビリティ運用方針
正式仕様セット
仕様依存マップ
仕様再配置計画
RoundTrip Semantic Criteria
Semantic Preservation Matrix
```

---

## 19. 結論

SansaVRM の仕様体系再構築では、doc_id / sec_id continuity より semantic continuity を優先する。

そのため、migration map を正式管理し、Traceability migration を validator / CI から追跡可能にする必要がある。

これにより、Layer再編や specification split / merge を許可しつつ、長期的な semantic continuity を維持できる。

---

[目次](../../目次.md) > トレーサビリティ > 共通 > Traceability Migration Specification
