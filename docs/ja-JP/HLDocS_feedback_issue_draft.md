# HLDocS feedback issue draft

## 状態

```text
DRAFT
最終的に削除または正式issueへ転記予定
```

---

# 目的

SansaVRM の大規模仕様再配置（Layer再構成・Validation Layer分離・migration trace導入）を通して得られた知見を、HLDocS へ feedback する。

本書は現時点の draft であり、formal migration / code migration / validator 実運用後に更新予定。

---

# 現状

## 完了

```text
- Layer再構成
- Validation Layer分離
- migration traces 作成
- manifest federation 設計
- federation validator 設計
- release/archive lifecycle 設計
- verified migration criteria 設計
- alias graph / sec_id continuity 設計
```

## 未完了

```text
- dry-doc formalization
- placeholder全文移行
- formal_doc_id 確定
- code migration
- test migration
- federation validator 実装/運用
- cleanup 実運用
- archive 実運用
```

---

# 現時点で得られた重要知見

## 1. reorder と cleanup は分離すべき

### 従来想定

```text
reorder = cleanup を含む
```

### 実際

```text
reorder:
構造正規化

cleanup:
互換終了・削除・archive
```

### 理由

```text
- placeholder relocation
- partial migration
- legacy alias lifecycle
```

が存在するため。

---

## 2. placeholder relocation state が必要

新pathだけ作成され、本文移行が未完了の状態が必要。

例：

```text
4002_MuJoCo連携仕様
5001_JSONスキーマ仕様
```

### 重要点

```text
placeholder は failure ではない
migration intermediate state
```

---

## 3. partial migration state が必要

一部のみ移行済みの状態が必要。

### 特徴

```text
- old/new coexistence
- semantic duplicate coexistence
- split relocation ongoing
```

---

## 4. semantic_verified と verified は別

### semantic_verified

```text
semantic equivalence のみ確認
```

### verified

```text
migration graph stabilized
```

verified には以下が必要：

```text
- federation validator
- alias graph
- sec_id continuity
- cleanup gate
- no placeholder
```

---

## 5. migration は graph problem

migration は path rename ではなく graph integrity 問題。

### graph node

```text
- document
- manifest
- section
- alias
- migration entry
```

### graph edge

```text
- relocation
- alias
- sec_id mapping
- split
- merge
```

---

## 6. manifest federation が必要

巨大 relocation では single manifest が merge hotspot になる。

### 必要構造

```text
root manifest
↓
Layer別 sub-manifest
```

---

## 7. federation validator が必要

validator は単なる manifest checker では足りない。

必要：

```text
- graph validation
- alias validation
- sec_id validation
- lifecycle validation
- release/archive mode
```

---

## 8. sec_id continuity が最重要

traceability の実体は document ではなく section semantic continuity。

### 結論

```text
doc_id continuity より
sec_id continuity を優先
```

---

## 9. alias graph が必要

legacy alias は temporary redirect ではない。

### 実際

```text
historical migration edge
```

として扱う必要がある。

---

## 10. migration lifecycle が必要

migration は state machine として扱う必要がある。

現在見えている lifecycle：

```text
draft
→ placeholder
→ partial
→ semantic_verified
→ verified
→ aliased
→ cleanup_candidate
→ archived
```

---

## 11. release mode と archive mode は分離すべき

### release

```text
operational stabilized state
```

### archive

```text
historical preserved state
```

---

## 12. migration manifest は audit trail

migration manifest は temporary tooling artifact ではない。

### 実際

```text
historical traceability database
```

として扱われ始めた。

---

## 13. cleanup 後も trace を保持すべき

保持対象：

```text
- manifest federation
- alias graph
- sec_id mappings
- verified reports
- lifecycle history
```

---

# 現時点の仮説

HLDocS は以下へ拡張し始めている可能性がある。

```text
document generation framework
↓
document evolution / migration governance framework
```

---

# 今後の予定

## 次段階

```text
1. dry-doc formalization
2. formal_doc_id 確定
3. placeholder全文移行
4. code migration
5. test migration
6. federation validator 実運用
7. cleanup / archive 実運用
```

## その後

以下を issue 完成版へ反映予定。

```text
- operational findings
- actual cleanup lifecycle
- validator CI findings
- archive運用結果
- rollback運用結果
- split/merge relocation 実運用知見
```

---

# 現時点結論

SansaVRM の大規模仕様再配置では、migration は単なる path rename ではなく、graph integrity / lifecycle / governance 問題として振る舞い始めた。

その結果、HLDocS は document generation framework から、document migration governance framework へ拡張され始めている可能性がある。
