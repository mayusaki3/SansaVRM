# verified migration criteria

## 1. 目的

本ドキュメントは、SansaVRM 仕様再配置における `verified migration` の成立条件を定義する。

`verified` は単なる目視確認ではなく、migration graph 全体の整合性が安定化した状態を意味する。

---

## 2. 基本方針

verified migration は cleanup / legacy alias / release migration の前提状態として扱う。

以下を原則とする。

```text
1. verified は federation 全体で判定する
2. verified は sec_id continuity を含む
3. verified は alias graph integrity を含む
4. verified は placeholder relocation を許容しない
5. verified は CI validation 成功を要求する
```

---

## 3. migration state

migration entry は以下の状態を持つ。

| 状態 | 意味 |
|---|---|
| draft | 作成直後 |
| placeholder | 仮移行 |
| partial | 一部移行 |
| semantic_verified | semantic equivalence 確認済み |
| verified | migration graph 全体で成立 |
| aliased | legacy alias 化済み |
| archived | cleanup完了 |

---

## 4. verified 条件

verified には以下が必要。

```text
- semantic_equivalent = true
- mapping_status = verified
- federation validator success
- cleanup gate pass
- sec_id state resolved
- placeholder entry なし
- unresolved alias なし
- duplicate path なし
- duplicate formal_doc_id なし
- migration manifest consistency pass
```

---

## 5. semantic equivalence

semantic_equivalent は以下のいずれか。

```text
true
false
unknown
```

verified では `true` 必須。

---

## 6. mapping_status

mapping_status は以下を持つ。

```text
draft
placeholder
partial
complete
verified
```

verified migration では、cleanup対象 entry は `verified` 必須。

---

## 7. sec_id state resolved

cleanup対象文書は以下のいずれかである必要がある。

```text
preserved
remapped
newly_assigned
not_required
```

以下は禁止。

```text
pending
unknown
```

---

## 8. placeholder prohibition

verified migration では placeholder relocation を禁止する。

対象例：

```text
4002_MuJoCo連携仕様.md
5001_JSONスキーマ仕様.md
```

これらが placeholder のままでは verified 不成立。

---

## 9. alias graph integrity

verified migration では alias graph が整合している必要がある。

検査内容：

```text
- alias_target_path exists
- alias_target_doc_id exists
- alias loopなし
- unresolved aliasなし
- duplicate formal_doc_idなし
```

---

## 10. federation validator

verified migration では federation validator success を要求する。

最低限以下を検査する。

```text
- duplicate entry_id
- duplicate path
- orphan manifest
- unresolved doc_id alias
- sec_id conflict
- missing required manifest
```

---

## 11. cleanup gate

verified migration は cleanup gate pass を含む。

cleanup gate：

```text
- sec_id resolved
- no placeholder
- no pending migration
- alias graph valid
- federation validation success
```

---

## 12. release migration

release migration では verified migration 必須。

release 禁止条件：

```text
- placeholder exists
- dry-doc remains as formal_doc_id
- pending sec_id state exists
- unresolved alias exists
- migration manifest inconsistency exists
```

---

## 13. verified と legacy alias

legacy alias 化は verified migration 後にのみ許可する。

理由：

```text
- alias は cleanup準備状態
- placeholder 状態では alias 化できない
- sec_id continuity 確認前に alias 化してはならない
```

---

## 14. verified と archive

archive は verified migration 後にのみ許可する。

archive 条件：

```text
- verified migration success
- legacy alias maintenance period completed
- cleanup decision approved
```

---

## 15. CI Requirements

CI は verified migration を検査できる。

fail 条件例：

```text
- placeholder remains
- mapping_status != verified
- pending sec_id state exists
- unresolved alias exists
- duplicate formal_doc_id exists
- federation validator failed
```

---

## 16. HLDocS feedback

本基準で得られた知見：

```text
- verified は graph integrity 状態である
- placeholder relocation は verified に含めないべき
- sec_id continuity は verified gate に必要
- federation validator が必要
- alias graph validation が必要
- migration は state machine として扱うべき
```

---

## 17. 結論

verified migration は、単なる semantic equivalence ではなく、federation validator、alias graph、sec_id continuity、cleanup gate を含む migration graph 全体の安定化状態である。

placeholder relocation や unresolved alias を含む状態では verified を成立させない。
