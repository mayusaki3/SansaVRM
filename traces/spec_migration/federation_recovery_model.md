# federation recovery model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation recovery model を定義する。

federation recovery model は、multi-project reconstruction において partial federation failure、stale acknowledgement、orphaned execution、diverged cleanup state、rollback failure が発生した場合の復旧方針を整理する。

---

## 2. 基本方針

federation recovery は以下を行う。

```text
- failure state を分類する
- recovery scope を決定する
- participant ごとの recovery responsibility を明示する
- federation validator rerun を要求する
- cleanup_ready / federation_cleanup_ready を再評価する
- manual recovery required を traceable に扱う
```

federation recovery は以下を行わない。

```text
- 他 project の repository を直接修正しない
- participant の local rollback を代替しない
- stale acknowledgement を valid とみなさない
- diverged cleanup state を warning のみで済ませない
- failed execution を completed として扱わない
```

---

## 3. recovery target failures

recovery 対象となる failure は以下とする。

```text
partial_federation_failure
stale_acknowledgement
orphaned_execution
diverged_cleanup_state
rollback_failure
stale_artifact_after_execution
handoff_contract_divergence
federation_validator_failure_after_execution
manual_recovery_required
```

---

## 4. partial federation failure

partial federation failure は、一部 participant の execution / validation / acknowledgement が失敗した状態である。

例：

```text
- SansaVRM は execution completed
- MuJoCo Adapter の validator が fail
- Studio AI fixture regeneration が未完了
```

処理：

```text
- federation execution を completed にしない
- failed participant を特定する
- affected downstream dependency を特定する
- recovery action request を生成する
- federation validator rerun を要求する
```

---

## 5. stale acknowledgement

stale acknowledgement は、participant の acknowledgement が古い contract / artifact / execution scope に基づいている状態である。

判定条件：

```text
- acknowledgement contract id が current contract と一致しない
- checkpoint reference が superseded
- artifact hash が現在の hash と一致しない
- reconstruction delta 後に再 acknowledgement されていない
```

stale acknowledgement は valid acknowledgement として扱ってはならない。

---

## 6. orphaned execution

orphaned execution は、federation orchestration / execution protocol から参照されなくなった execution が project-local に残っている状態である。

例：

```text
- federation execution は superseded
- project-local branch / artifact / report は残存
- dashboard から active と誤認される可能性がある
```

処理：

```text
- orphaned_execution として分類する
- active execution から除外する
- cleanup / archive / rollback の方針を participant に要求する
- dashboard に orphaned state を表示する
```

---

## 7. diverged cleanup state

diverged cleanup state は、project-local cleanup state と federation cleanup state が矛盾した状態である。

例：

```text
project-local cleanup_completed
federation_cleanup_blocked
```

これは重大状態として扱う。

処理：

```text
- federation consistency violation とする
- affected artifacts / references を検査する
- downstream project rerun を要求する
- 必要に応じて rollback_required とする
```

---

## 8. rollback failure

rollback failure は、project-local rollback または federation rollback coordination が失敗した状態である。

処理：

```text
- rollback_failed として記録する
- manual_recovery_required に遷移する
- rollback boundary を再評価する
- stale artifact propagation を記録する
- cleanup_ready を無効化する
```

rollback failure を warning のみで扱ってはならない。

---

## 9. stale artifact after execution

execution 後に artifact が stale と判定された場合、以下を確認する。

```text
- artifact が cleanup scope に必要か
- artifact が downstream validator に必要か
- artifact が handoff contract に含まれているか
- artifact regeneration が可能か
```

cleanup scope に必要な stale artifact は recovery blocking とする。

---

## 10. handoff contract divergence

handoff contract divergence は、producer / consumer project で contract 前提がずれた状態である。

例：

```text
- producer は draft schema として扱う
- consumer は canonical schema として扱う
```

処理：

```text
- handoff_contract_divergence として分類する
- affected downstream action request を生成する
- schema drift validation を再実行する
- cleanup_ready を再評価する
```

---

## 11. recovery action request

recovery action request は、participant に要求する復旧作業を表す。

種類：

```text
rerun_validator
regenerate_artifact
refresh_acknowledgement
rollback_local_execution
archive_orphaned_execution
resolve_handoff_contract
manual_recovery
```

schema draft：

```json
{
  "recovery_action_request_id": "recovery-action-YYYYMMDD-NNN",
  "target_project": "SansaVRM-MuJoCo-Adapter",
  "request_kind": "refresh_acknowledgement",
  "reason": "stale_acknowledgement",
  "required_before": "federation_execution_completion"
}
```

---

## 12. recovery status

recovery status の許容値：

```text
recovery_planned
recovery_requested
recovery_in_progress
recovery_validating
recovery_completed
recovery_failed
manual_recovery_required
superseded
```

---

## 13. recovery validation

recovery 後は以下を再検証する。

```text
- affected participant local validator
- federation validator
- federation consistency validator
- cleanup gate validator
- dashboard projection validator
```

recovery validation が pass するまで federation execution completed として扱ってはならない。

---

## 14. dashboard projection

Dashboard は recovery 状態を表示する。

表示対象：

```text
- failure kind
- affected projects
- recovery action requests
- recovery status
- manual recovery required
- cleanup_ready invalidation
- federation_cleanup_state
```

Dashboard は recovery を実行しない。

---

## 15. CI mapping

CI fail 条件：

```text
- diverged cleanup state
- rollback failure
- stale acknowledgement in required scope
- orphaned execution used as active
- handoff contract divergence affecting cleanup
```

CI warn 条件：

```text
- orphaned execution outside active scope
- stale optional artifact
- recovery action pending outside cleanup scope
```

---

## 16. 禁止事項

以下を禁止する。

```text
- stale acknowledgement を accepted として扱うこと
- orphaned execution を active execution として扱うこと
- diverged cleanup state を warning のみで処理すること
- rollback_failed のまま cleanup_ready を維持すること
- recovery validation なしで federation execution completed とすること
```

---

## 17. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- multi-project reconstruction には recovery model が必要
- stale acknowledgement / orphaned execution / diverged cleanup state を明示状態として扱うべき
- rollback failure は manual_recovery_required として扱うべき
- recovery action request を traceable artifact として扱うべき
- recovery 後は federation validator / cleanup gate の再実行が必要
```

---

## 18. 結論

federation recovery model は、multi-project reconstruction における partial failure、stale acknowledgement、orphaned execution、diverged cleanup state、rollback failure を安全に扱うための復旧モデルである。

これにより、分散した project-local 実行結果を、federation-level consistency と cleanup safety に再接続できる。
