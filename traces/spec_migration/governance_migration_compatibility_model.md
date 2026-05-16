# governance migration compatibility model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance migration compatibility model を定義する。

governance migration compatibility model は、旧 governance semantic / package / replay model から新 governance model への migration compatibility を管理し、bridge・rollback・replayability・cross-project synchronization を扱う。

---

## 2. 基本方針

governance migration compatibility model は以下を扱う。

```text
- migration compatibility taxonomy
- migration bridge lifecycle
- bridge replayability
- migration propagation lifecycle
- migration freeze interaction
- cross-project migration synchronization
- migration rollback
- migration debt governance
```

governance migration compatibility model は以下を行わない。

```text
- replay-incompatible migration を silent migration としない
- temporary bridge を permanent semantic baseline としない
- stale migration bridge を active federation baseline としない
- migration debt を hidden state としない
```

---

## 3. migration positioning

migration compatibility は以下に属する。

```text
Federated Governance Layer
Cross-Project Compatibility Layer
Operational Traceability Layer
```

consistency validator / replay validator / drift detection と連携する。

---

## 4. migration compatibility taxonomy

migration compatibility taxonomy：

```text
fully_compatible
replay_compatible
migration_required
bridge_required
incompatible
```

compatibility ambiguity は governance warning または blocker。

---

## 5. fully_compatible

fully_compatible：

```text
- semantic driftなし
- replay driftなし
- package compatibility維持
- vocabulary semantic維持
```

federation baseline を直接継続できる。

---

## 6. replay_compatible

replay_compatible：

```text
- semantic refinementあり
- replay reconstruction可能
- compatibility mappingあり
```

Replay validator により replayability を維持する。

---

## 7. migration_required

migration_required：

```text
- package baseline migration必要
- semantic mapping migration必要
- replay evidence adaptation必要
```

migration completion 前は federation baseline 更新を制限しうる。

---

## 8. bridge_required

bridge_required：

```text
- temporary semantic bridge必要
- legacy replay adapter必要
- vocabulary alias bridge必要
```

bridge lifecycle を mandatory とする。

---

## 9. incompatible

incompatible：

```text
- replay reconstruction不可能
- semantic collapse発生
- federation baseline互換性喪失
```

production federation rollout を禁止する。

---

## 10. migration bridge

migration bridge は旧 semantic と新 semantic を接続する。

例：

```text
checkpoint semantic v1
↓ bridge
checkpoint semantic v2
```

bridge は replayable でなければならない。

---

## 11. bridge lifecycle

bridge lifecycle：

```text
bridge_registered
bridge_active
bridge_review_required
bridge_deprecated
bridge_expired
bridge_removed
```

expired bridge を active federation baseline に使用してはならない。

---

## 12. bridge replayability

bridge replayability 条件：

```text
- previous semantic refs exist
- new semantic refs exist
- mapping rule exists
- replay impact refs exist
- package compatibility refs exist
```

Replay 不可能な bridge は federation governance risk。

---

## 13. migration propagation lifecycle

migration propagation lifecycle：

```text
migration_detected
migration_review_required
migration_bridge_generated
migration_propagated
migration_stabilized
migration_completed
migration_archived
```

migration propagation は replayable であるべき。

---

## 14. migration propagation

例：

```text
vocabulary migration
↓
package migration
↓
replay migration
↓
federation baseline migration
```

Propagation chain は dependency graph と整合する必要がある。

---

## 15. migration freeze interaction

semantic freeze 中：

```text
- bridge_required migration
- replay-breaking migration
- incompatible migration
```

は federation blocker としうる。

freeze 中の replay_compatible migration は review_required としうる。

---

## 16. cross-project migration synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
SansaXR
HLDocS
```

同期対象：

```text
- migration baseline
- compatibility mapping
- replay adapter baseline
- semantic bridge baseline
```

unsynchronized migration は federation compatibility risk。

---

## 17. migration rollback

migration failure 時：

```text
- previous package baseline
- previous semantic baseline
- previous replay baseline
```

へ rollback できる必要がある。

rollback replayability は mandatory。

---

## 18. migration rollback lifecycle

rollback lifecycle：

```text
rollback_required
rollback_prepared
rollback_executed
rollback_verified
rollback_archived
```

rollback verification missing は federation blocker。

---

## 19. migration debt

migration debt 候補：

```text
temporary bridge
deprecated semantic alias
legacy replay adapter
legacy package compatibility shim
```

migration debt は visible governance artifact とする。

---

## 20. migration debt governance

migration debt governance：

```text
- debt registration
- debt severity
- debt expiration
- debt replay impact
- debt cleanup target
```

hidden migration debt を禁止する。

---

## 21. migration invalidation

以下は migration invalidation を発生させる。

```text
- stale bridge
- replay-incompatible mapping
- superseded compatibility baseline
- unresolved migration debt
- rollback verification failure
```

invalidated migration baseline を active federation baseline に使用してはならない。

---

## 22. migration report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_migration_report",
  "migration_status": "bridge_required",
  "compatibility_taxonomy": "migration_required",
  "bridge_refs": [],
  "replay_refs": [],
  "migration_debt_refs": [],
  "source_of_truth_refs": []
}
```

---

## 23. reason codes

候補 reason code：

```text
replay_incompatible_migration
semantic_bridge_required
legacy_replay_adapter_required
migration_baseline_unsynchronized
semantic_freeze_violation
migration_rollback_required
migration_debt_unresolved
bridge_replay_incomplete
```

---

## 24. orchestration relation

federation execution orchestration は以下を block する。

```text
- incompatible migration
- replay-breaking migration unresolved
- rollback verification missing
- unsynchronized migration baseline in production scope
```

---

## 25. dashboard relation

Dashboard は migration summary を表示できる。

表示対象：

```text
- migration compatibility status
- bridge lifecycle status
- migration debt summary
- replay compatibility summary
- rollback verification summary
```

Dashboard は migration semantic を独自決定しない。

---

## 26. CI mapping

CI fail 条件：

```text
- incompatible migration unresolved
- replay-incompatible migration unresolved
- semantic freeze violation by migration
- rollback verification missing
- hidden migration debt detected
```

CI warn 条件：

```text
- bridge_required pending review
- deprecated bridge still active
- migration debt cleanup pending
```

---

## 27. 禁止事項

以下を禁止する。

```text
- replay-incompatible migration を silent migration とすること
- temporary bridge を permanent baseline とすること
- stale bridge を active federation baseline に使うこと
- hidden migration debt を持ち込むこと
- rollback verification missing のまま production rollout すること
```

---

## 28. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance migration compatibility model を formalize すべき
- migration bridge lifecycle を formal artifact 化すべき
- migration debt governance を導入すべき
- replay-compatible migration を validator 対象にすべき
- cross-project migration synchronization を governance layer に含めるべき
```

---

## 29. 結論

governance migration compatibility model は、SansaVRM federation governance semantic の migration compatibility を管理する model である。

これにより、旧 semantic / package / replay baseline から新 federation baseline への移行を replayable に管理しつつ、bridge・rollback・migration debt を統制できる。
