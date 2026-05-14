# federation transaction consistency model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation transaction consistency model を定義する。

本モデルは、複数 project 間で発生する schema change、artifact regeneration、validator rerun、cleanup、rollback に対して、どの整合性を strong consistency とし、どの整合性を eventual consistency として扱うかを整理する。

---

## 2. 基本方針

federation transaction consistency は以下を扱う。

```text
- strong consistency boundary
- eventual consistency boundary
- rollback consistency
- cleanup consistency
- artifact freshness consistency
- handoff contract consistency
- validator consistency
```

federation transaction consistency は、複数 project を単一 repository transaction として扱わない。

project-local ownership を維持しつつ、federation-level consistency を管理する。

---

## 3. consistency classes

consistency class は以下とする。

```text
strong_consistency
eventual_consistency
validation_consistency
cleanup_consistency
rollback_consistency
artifact_freshness_consistency
handoff_contract_consistency
```

---

## 4. strong consistency boundary

strong consistency が必要な対象：

```text
- execution approval scope
- distributed checkpoint references
- handoff contract acceptance
- cleanup approval boundary
- destructive cleanup target list
- rollback package availability
```

これらは apply / cleanup 前に一致していなければならない。

strong consistency が成立しない場合、execution / cleanup を開始してはならない。

---

## 5. eventual consistency boundary

eventual consistency を許容する対象：

```text
- downstream fixture regeneration
- optional external artifact refresh
- dashboard snapshot regeneration
- non-cleanup-scope warning report
- draft schema alignment during PoC
```

ただし、eventual consistency 対象でも cleanup scope に入った時点で strong consistency または validation consistency が必要になる。

---

## 6. validation consistency

validation consistency は、各 project の validator result と federation validator result が矛盾しない状態を指す。

必要条件：

```text
- project-local validator report が存在する
- federation validator が local result を参照している
- stale validator result が active 判定に使われていない
- validator module version が一致または互換である
```

validator result が stale の場合は rerun_required とする。

---

## 7. cleanup consistency

cleanup consistency は、cleanup target が project-local と federation-level の両方で安全である状態を指す。

必要条件：

```text
- project-local cleanup_ready
- federation_cleanup_ready
- downstream old artifact consumption = false
- handoff contract accepted
- required external artifact fresh
- rollback boundary defined
```

project-local cleanup_ready だけでは cleanup consistency は成立しない。

---

## 8. rollback consistency

rollback consistency は、rollback 実行後に project-local と federation-level の整合性が破綻しない状態を指す。

必要条件：

```text
- local rollback package exists
- affected downstream projects are identified
- stale artifact propagation is recorded
- downstream rerun requirement is issued
- rollback status is acknowledged
```

cross-project rollback は eventual consistency を含む場合がある。

ただし、rollback_required を無視して completed としてはならない。

---

## 9. artifact freshness consistency

artifact freshness consistency は、external artifact が参照元 schema / contract / execution に対して古くない状態を指す。

確認項目：

```text
- artifact source hash
- schema version
- source project revision
- generation timestamp
- handoff contract id
- validator report id
```

cleanup scope に必要な artifact freshness は strong consistency 扱いとする。

cleanup scope 外の optional artifact freshness は eventual consistency として扱える。

---

## 10. handoff contract consistency

handoff contract consistency は、producer project と consumer project の責務境界合意が一致している状態を指す。

必要条件：

```text
- contract id が一致する
- accepted assumptions が一致する
- pending decisions が明示されている
- draft / canonical の区別が一致する
- cleanup dependency が明示されている
```

handoff pending のまま destructive cleanup を行ってはならない。

---

## 11. consistency matrix

```text
対象                                 consistency class
execution approval scope              strong_consistency
distributed checkpoint                strong_consistency
project-local validator report         validation_consistency
federation validator report            validation_consistency
cleanup target list                    cleanup_consistency
rollback package                       rollback_consistency
required external artifact             artifact_freshness_consistency / strong_consistency
optional external artifact             eventual_consistency
handoff contract                       handoff_contract_consistency
Dashboard snapshot                     eventual_consistency
```

---

## 12. consistency violation handling

consistency violation は以下に分類する。

```text
strong_consistency_violation
validation_consistency_violation
cleanup_consistency_violation
rollback_consistency_violation
artifact_freshness_violation
handoff_contract_violation
eventual_consistency_pending
```

strong / validation / cleanup / rollback violation は fail または blocked とする。

eventual_consistency_pending は warn として扱える。

---

## 13. reconstruction delta との関係

reconstruction delta は consistency class を変化させる場合がある。

例：

```text
draft schema artifact:
eventual_consistency

same artifact enters cleanup dependency:
strong_consistency
```

reconstruction delta 発生時は consistency class を再評価する。

---

## 14. report schema draft

```json
{
  "schema_version": "1.0",
  "consistency_report_id": "federation-consistency-YYYYMMDD-NNN",
  "status": "warn",
  "targets": [
    {
      "target_id": "artifact-example",
      "consistency_class": "artifact_freshness_consistency",
      "status": "warn",
      "reason": "optional artifact stale outside cleanup scope"
    }
  ],
  "violations": []
}
```

---

## 15. CI mapping

CI fail 条件：

```text
- strong_consistency_violation
- cleanup_consistency_violation
- rollback_consistency_violation in active execution scope
- handoff_contract_violation affecting cleanup scope
- required artifact freshness violation
```

CI warn 条件：

```text
- eventual_consistency_pending
- optional artifact stale
- dashboard snapshot stale
- draft schema alignment pending outside cleanup scope
```

---

## 16. dashboard projection

Dashboard は consistency state を表示する。

表示対象：

```text
- consistency class
- consistency status
- violation type
- affected project
- affected artifact
- cleanup impact
- rollback impact
```

Dashboard は consistency を直接変更しない。

---

## 17. 禁止事項

以下を禁止する。

```text
- eventual consistency 対象を cleanup scope でそのまま使用すること
- handoff pending のまま destructive cleanup を行うこと
- stale validator report を active pass として扱うこと
- rollback consistency violation を warning のみで済ませること
- dashboard snapshot freshness を source of truth として扱うこと
```

---

## 18. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- multi-project reconstruction では consistency class を分けるべき
- project-local cleanup_ready と federation cleanup consistency を分離すべき
- eventual consistency は cleanup scope に入った時点で再評価すべき
- handoff contract consistency と artifact freshness consistency を validator 対象にすべき
- distributed rollback は rollback consistency として扱うべき
```

---

## 19. 結論

federation transaction consistency model は、multi-project reconstruction における strong / eventual / validation / cleanup / rollback / artifact / handoff consistency を分類するモデルである。

これにより、複数 project 間の実行・検証・cleanup・rollback を、単一 repository transaction と誤認せずに安全に管理できる。
