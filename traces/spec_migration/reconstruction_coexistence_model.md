# reconstruction coexistence model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における reconstruction coexistence model を定義する。

reconstruction coexistence model は、再構成前の旧構成と再構成後の新構成が同時に存在する期間の扱いを定義する。

特に、旧構成と新構成を分離できる separated reconstruction と、分離できない mixed reconstruction の両方を扱う。

---

## 2. 基本方針

reconstruction coexistence は以下を扱う。

```text
- old structure と new structure の共存
- separated reconstruction
- mixed reconstruction
- coexistence boundary
- contamination detection
- comparison before cleanup
- cleanup hold
- reconstruction completion 判定
```

reconstruction coexistence は以下を行わない。

```text
- 比較調査前に旧構成を削除しない
- new structure generated を reconstruction completed と扱わない
- cleanup_ready を独自判定しない
- mixed reconstruction を separated reconstruction と誤認しない
```

---

## 3. reconstruction coexistence state

coexistence state は以下とする。

```text
old_only
new_draft_created
coexisting_separated
coexisting_mixed
comparison_required
comparison_in_progress
cleanup_hold
cleanup_ready_candidate
cleanup_executed
reconstruction_completed
```

---

## 4. separated reconstruction

separated reconstruction は、旧構成と新構成を比較的明確に分離できる再構成である。

特徴：

```text
- old path と new path が分かれている
- old document と new document が分かれている
- migration manifest で対応関係を追跡できる
- cleanup candidate を比較的明確に生成できる
```

今回の SansaVRM 再構成は、主にこの形に近い。

---

## 5. mixed reconstruction

mixed reconstruction は、旧構成と新構成が同一 namespace / path / document set 内で混在する再構成である。

特徴：

```text
- old/new が同じ directory に混在する
- partial overwrite が発生する
- shared namespace を使う
- staged replacement が発生する
- old reference と new reference が一時的に共存する
```

mixed reconstruction は separated reconstruction より contamination risk が高い。

---

## 6. coexistence boundary

coexistence boundary は以下を明示する。

```text
- old structure scope
- new structure scope
- shared scope
- temporary overlap scope
- cleanup hold scope
- comparison scope
```

boundary が不明な場合、cleanup_ready に進めてはならない。

---

## 7. contamination types

coexistence contamination は以下とする。

```text
old_reference_inside_new_structure
new_reference_inside_old_structure
draft_dependency_inside_canonical_boundary
partial_rewrite_leak
mixed_namespace_collision
legacy_alias_conflict
stale_old_document_used_as_active
new_document_missing_old_semantics
```

---

## 8. contamination detection

検出対象：

```text
- old path reference が new structure に残っている
- new draft reference が old structure に入り込んでいる
- canonical boundary に draft artifact が混入している
- partial rewrite state が active として扱われている
- old/new doc_id が同一意味で二重 active になっている
```

contamination がある場合、comparison_required または cleanup_hold とする。

---

## 9. comparison before cleanup

再構成後は cleanup 前に比較調査を行う。

比較対象：

```text
- old document set
- new document set
- semantic content
- traceability links
- references
- sec_id mapping
- external artifact dependency
- handoff dependency
```

比較調査なしで旧構成を削除してはならない。

---

## 10. cleanup hold

cleanup hold は、旧構成を残す必要がある状態である。

cleanup hold 条件：

```text
- comparison not completed
- unresolved contamination
- unresolved traceability mismatch
- unresolved external artifact dependency
- handoff pending
- legacy alias missing
- rollback scope missing
```

cleanup hold 中は old structure を active cleanup target にしてはならない。

---

## 11. coexistence registry

coexistence registry は old/new の共存状態を記録する。

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "reconstruction_coexistence_registry",
  "coexistence_state": "coexisting_separated",
  "old_scope": [],
  "new_scope": [],
  "shared_scope": [],
  "comparison_required": true,
  "cleanup_hold": true
}
```

---

## 12. cleanup readiness との関係

cleanup_ready は以下を満たすまで出してはならない。

```text
- coexistence boundary resolved
- comparison completed
- contamination unresolved = 0
- traceability mismatch unresolved = 0
- cleanup hold cleared
- rollback scope exists
```

new structure generated だけでは cleanup_ready ではない。

---

## 13. reconstruction completion との関係

reconstruction completed は以下を満たす必要がある。

```text
- new structure validated
- old/new comparison completed
- required semantic equivalence confirmed
- traceability equivalence confirmed
- cleanup execution completed where required
- old structure detached or archived
- audit trail recorded
```

new structure generated は reconstruction completed ではない。

---

## 14. dashboard display

Dashboard は coexistence state を表示する。

表示対象：

```text
- coexistence_state
- old scope
- new scope
- shared scope
- comparison status
- cleanup hold status
- contamination findings
- cleanup readiness impact
```

Dashboard は coexistence state を独自変更しない。

---

## 15. CI mapping

CI fail 条件：

```text
- old structure deleted before comparison completed
- cleanup_ready issued while cleanup_hold=true
- mixed reconstruction treated as separated without boundary
- contamination finding ignored in cleanup scope
- stale old document used as active canonical evidence
```

CI warn 条件：

```text
- coexistence boundary partially defined
- comparison_required=true
- cleanup_hold=true outside cleanup execution scope
```

---

## 16. 禁止事項

以下を禁止する。

```text
- new structure generated を reconstruction completed と扱うこと
- comparison 前に old structure を削除すること
- mixed reconstruction を separated reconstruction として扱うこと
- cleanup_hold 中に cleanup execution へ進むこと
- contamination を known limitation のみで cleanup 通過させること
```

---

## 17. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction には coexistence model が必要
- separated reconstruction と mixed reconstruction を分離すべき
- new structure generated と reconstruction completed を分離すべき
- cleanup 前に old/new comparison を必須にすべき
- cleanup_hold を formal state として扱うべき
```

---

## 18. 結論

reconstruction coexistence model は、旧構成と新構成が共存する期間の境界、汚染検出、比較調査、cleanup hold、完了条件を定義する model である。

これにより、再構成後の新構成生成だけで旧構成を削除せず、比較調査と cleanup readiness を経て安全に reconstruction completed へ進められる。
