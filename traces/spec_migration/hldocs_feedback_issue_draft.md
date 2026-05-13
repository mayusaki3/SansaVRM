# HLDocS feedback issue draft

## 1. 目的

本ドラフトは、SansaVRM の大規模仕様再構成 dry-run で得られた実地検証結果を、HLDocS 側へフィードバックするための Issue 案である。

本ドラフトは、まだ HLDocS リポジトリへ投稿する Issue 本文ではない。

---

## 2. 背景

SansaVRM では、仕様群を以下のような Layer 構造へ dry-run relocation した。

```text
Core Semantic Layer
Preservation Compatibility Layer
Data Model Layer
Runtime Integration Layer
Validation Layer
Import Export Layer
Roadmap Layer
```

Layer 名は責務を表す。

Layer 番号は filesystem ordering であり、dependency 意味論を持たない。

Dependency は Layer Index、dependency diagram、migration manifest、および本文定義により表現する。

---

## 3. 実地検証で確認できた課題

### 3.1 dry-run relocation

大規模再構成では、旧pathを削除せず、新pathを先行作成して検証する dry-run relocation が有効だった。

必要になった概念：

```text
- old path
- new path
- migration entry
- semantic_equivalent
- mapping_status
- placeholder relocation
- verification state
```

---

### 3.2 placeholder relocation

全文移行がすぐにできない巨大文書や複雑文書では、placeholder relocation が必要だった。

ただし、placeholder relocation は migration complete を意味しない。

必要になった分類：

```text
placeholder_only
migrated_partial
migrated_complete
verified
```

---

### 3.3 legacy alias phase

旧pathを即削除すると、旧リンク・旧参照・作業中の検証が壊れる。

そのため、旧pathを一定期間 legacy alias として残す phase が必要だった。

legacy alias は単なる redirect ではなく、以下を担う。

```text
- 旧リンク互換
- canonical path 案内
- migration continuity の可視化
- cleanup safety
```

---

### 3.4 dual canonical state

dry-run 中は、旧pathと新pathの両方が canonical_document=true に見える一時状態が発生した。

恒久状態としては危険だが、dry-run relocation 中の検証状態としては実用上必要だった。

HLDocS 側で以下の扱いを定義したい。

```text
- temporary dual canonical state
- canonical transition state
- legacy alias canonical_document=false
- verified 後の canonical switch
```

---

### 3.5 manifest federation

大規模 migration では single manifest が大きくなり、競合・保守性・レビュー性の問題が発生する。

SansaVRM では Validation Layer 用に sub-manifest を分離した。

必要になった概念：

```text
- root manifest
- sub-manifest
- manifest federation
- entry_id uniqueness
- path collision detection
- doc_id collision detection
- mapping_status summary
```

---

### 3.6 migration integrity check

再構成では、ファイル移動後に integrity check が必要だった。

検査対象：

```text
- orphan specification
- duplicate path
- semantic duplicate
- duplicate doc_id
- sec_id continuity
- placeholder relocation
- cleanup block list
```

---

### 3.7 cleanup gate

cleanup は relocation とは別フェーズにすべきだった。

cleanup は filesystem ordering ではなく verification-driven で判断する。

cleanup gate の例：

```text
- migration manifest 登録済み
- semantic_equivalent verified
- mapping_status complete / verified
- sec_id continuity 確認済み
- placeholder relocation 解消済み
- legacy alias policy 適用済み
- CI validation 通過済み
```

---

### 3.8 filesystem ordering と dependency ordering の分離

HLDocS 側の番号付き表記更新により、SansaVRM 側でも以下を明確化した。

```text
filesystem ordering:
Layer番号やファイル番号

dependency ordering:
Layer Index / dependency diagram / manifest / 本文定義

migration ordering:
migration manifest / manifest federation

cleanup ordering:
verification condition / cleanup gate
```

番号は意味論ではなく、並び順制御として扱う必要がある。

---

## 4. HLDocS 共通仕様へ追加したい候補

### 4.1 Relocation Operation Model

大規模再構成の operation として、以下を定義したい。

```text
new_document
path_relocation
full_copy_relocation
placeholder_relocation
split
merge
legacy_alias
cleanup
```

---

### 4.2 Migration Manifest Model

migration manifest の基本構造を共通仕様化したい。

最小要素案：

```text
migration_id
entry_id
migration_type
old.path
old.doc_id
new.path
new.doc_id
semantic_equivalent
mapping_status
sec_mappings
reason
```

---

### 4.3 Manifest Federation Model

大規模再構成では manifest federation を許可したい。

必要要件：

```text
- root manifest
- sub-manifest list
- migration_id uniqueness
- entry_id uniqueness
- path collision detection
- doc_id collision detection
- mapping_status summary
```

---

### 4.4 Placeholder Relocation State

placeholder relocation を正式な migration state として扱いたい。

禁止事項：

```text
- placeholder_only を migrated_complete と扱わない
- placeholder_only の旧pathを削除しない
- placeholder_only の canonical switch を行わない
```

---

### 4.5 Legacy Alias State

legacy alias を正式な post relocation state として扱いたい。

要件案：

```text
- old_doc_id を保持
- canonical_document=false
- canonical path を案内
- migration_entry_id を記録
- semantic_equivalent verified 後にのみ適用
```

---

### 4.6 Cleanup Gate

cleanup は migration verification 後の別フェーズとして扱いたい。

cleanup gate は verification-driven とする。

---

### 4.7 Numbering Semantics Separation

番号付き表記について、以下の分離を共通仕様として明示したい。

```text
番号:
filesystem ordering

依存:
dependency diagram / manifest / 本文定義

migration順:
migration manifest

cleanup順:
cleanup gate
```

---

## 5. SansaVRM で作成した検証文書

今回の実地検証では以下を作成した。

```text
post relocation policy
旧path cleanup計画
旧path cleanup分類表
migration integrity check report
Layer reorder最終計画
Validation Layer Index
Import Export Layer Index
Roadmap Layer Index
```

これらは HLDocS 側へ仕様化する際の材料として利用できる。

---

## 6. 提案するHLDocS側の次アクション

```text
1. 大規模再構成規約を新規作成する
2. migration manifest 規約を追加する
3. placeholder relocation / legacy alias / cleanup gate を定義する
4. manifest federation を扱うか検討する
5. 番号付き表記規定と relocation dependency を整合させる
6. SansaVRM の実地検証文書を参考資料として取り込む
```

---

## 7. Issue投稿時のタイトル案

```text
大規模仕様再構成における dry-run relocation / manifest federation / cleanup gate の共通仕様化
```

---

## 8. Issue投稿時のラベル案

```text
spec
traceability
migration
feedback
```

---

## 9. 結論

SansaVRM の大規模再構成 dry-run により、HLDocS 共通仕様として以下を定義する必要性が確認できた。

```text
- dry-run relocation
- placeholder relocation
- staged relocation
- legacy alias phase
- dual canonical state
- manifest federation
- migration integrity check
- cleanup gate
- verification-driven cleanup
- filesystem ordering / dependency ordering separation
```

これらは、HLDocS における大規模文書再構成の再現性と安全性を高めるための共通仕様候補である。
