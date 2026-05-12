# relocation lifecycle definition

## 1. 目的

本ドキュメントは、SansaVRM 仕様再配置における relocation lifecycle を定義する。

relocation lifecycle は、仕様移行を単発変換ではなく、状態遷移を持つ evolution process として扱うための基準である。

---

## 2. 基本方針

relocation は stateful process として扱う。

以下を原則とする。

```text
1. relocation は lifecycle state を持つ
2. placeholder / partial を正式 state として扱う
3. verified は graph stabilized state として扱う
4. alias は deprecation compatibility state として扱う
5. archive 後も migration trace を保持する
```

---

## 3. lifecycle state 一覧

| state | 意味 |
|---|---|
| draft | relocation planning 中 |
| placeholder | path relocation のみ存在 |
| partial | 一部のみ移行済み |
| semantic_verified | semantic equivalence 確認済み |
| verified | migration graph 安定化済み |
| aliased | legacy alias 化済み |
| cleanup_candidate | cleanup 判定待ち |
| archived | migration 完了・履歴保持状態 |

---

## 4. draft

### 状態

relocation planning 中。

### 許可

```text
- dry-doc
- temporary manifest
- incomplete mapping
```

### 禁止

```text
- cleanup
- release migration
- alias化
```

---

## 5. placeholder

### 状態

新path が存在するが、本文移行が未完了。

### 例

```text
4002_MuJoCo連携仕様.md
5001_JSONスキーマ仕様.md
```

### 許可

```text
- temporary verification
- TOC registration
- dependency normalization
```

### 禁止

```text
- verified migration
- cleanup
- sec_id assignment
- alias化
```

---

## 6. partial

### 状態

一部のみ relocation 済み。

### 特徴

```text
- semantic duplicate coexistence
- old/new path coexistence
- split relocation ongoing
```

### 禁止

```text
- release migration
- cleanup
```

---

## 7. semantic_verified

### 状態

semantic equivalence が確認済み。

### 条件

```text
- semantic_equivalent = true
- mapping_status = complete
```

### 注意

semantic_verified は verified ではない。

以下が未解決の可能性がある。

```text
- sec_id continuity
- alias graph
- federation validation
- cleanup gate
```

---

## 8. verified

### 状態

migration graph 全体が安定化済み。

### 条件

```text
- federation validator success
- sec_id resolved
- no placeholder
- alias graph resolved
- verified migration criteria pass
```

### 許可

```text
- release migration
- legacy alias化
- cleanup candidate化
```

---

## 9. aliased

### 状態

旧path を legacy alias 化済み。

### 特徴

```text
- compatibility maintained
- old body removed
- migration target stabilized
```

### 注意

aliased は archived ではない。

互換維持期間中とする。

---

## 10. cleanup_candidate

### 状態

cleanup 判定待ち。

### 条件

```text
- verified migration completed
- legacy alias maintenance period completed
- no active compatibility dependency
```

### 許可

```text
- deletion review
- obsolete判定
- archive preparation
```

---

## 11. archived

### 状態

migration 完了状態。

### 保持対象

```text
- manifest federation
- alias graph
- sec_id mappings
- verified migration report
- cleanup classification
- lifecycle history
```

### 禁止

```text
- migration trace deletion
- audit trail removal
```

---

## 12. lifecycle transition

許可される基本遷移：

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

## 13. rollback

rollback は前状態へ戻すことを許可する。

例：

```text
verified → partial
aliased → verified
cleanup_candidate → aliased
```

ただし archive rollback は audit log を必要とする。

---

## 14. split relocation

split relocation では、文書ごとに異なる lifecycle state を持つことを許可する。

---

## 15. merge relocation

merge relocation では、複数旧文書の lifecycle state を統合判定する。

最も未成熟な state を優先する。

例：

```text
verified + partial
→ partial
```

---

## 16. release と lifecycle

release 対象は verified 以上でなければならない。

placeholder / partial は release scope に含めない。

---

## 17. cleanup と lifecycle

cleanup は cleanup_candidate 以降でのみ許可する。

verified 直後に即削除してはならない。

---

## 18. archive と lifecycle

archive 後も migration trace を保持する。

archive は migration history preservation state として扱う。

---

## 19. CI Requirements

CI は lifecycle state を検査できる。

fail 条件例：

```text
- invalid transition
- placeholder marked as verified
- cleanup before cleanup_candidate
- archive without verified migration
```

---

## 20. HLDocS feedback

本定義で得られた知見：

```text
- migration は lifecycle を持つ
- placeholder / partial は正式 state として扱うべき
- verified は graph stabilized state である
- alias は deprecation compatibility state である
- archive は audit preservation state である
```

---

## 21. 結論

SansaVRM の relocation は、draft から archived までの lifecycle state を持つ stateful evolution process として扱う。

verified migration、legacy alias、cleanup、archive は、それぞれ relocation lifecycle 上の distinct state として管理する。
