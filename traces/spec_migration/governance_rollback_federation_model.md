# governance rollback federation model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance rollback federation model を定義する。

governance rollback federation model は、federation-wide governance baseline / package / vocabulary / replay / freeze / migration の rollback を、安全かつ replayable に実行するための model である。

---

## 2. 基本方針

governance rollback federation model は以下を扱う。

```text
- rollback scope taxonomy
- rollback trigger taxonomy
- rollback baseline selection
- partial rollback model
- rollback replayability
- rollback freeze interaction
- rollback propagation lifecycle
- rollback debt governance
- rollback stabilization policy
- cross-project rollback synchronization
```

governance rollback federation model は以下を行わない。

```text
- rollback を silent operation としない
- replay-incomplete rollback を production baseline に適用しない
- rollback debt を hidden state としない
- unsynchronized rollback baseline を federation active baseline としない
```

---

## 3. rollback positioning

rollback federation model は以下に属する。

```text
Federated Governance Layer
Release Governance Layer
Operational Traceability Layer
Cross-Project Compatibility Layer
```

rollback は failure recovery だけでなく governance stabilization にも使用する。

---

## 4. rollback scope taxonomy

rollback scope taxonomy：

```text
local_rollback
project_rollback
cross_project_rollback
federation_rollback
distribution_rollback
```

scope により propagation / synchronization / audit 要件が変わる。

---

## 5. local rollback

local rollback：

```text
- project-local rollback
- federation propagation optional
- production federation baseline変更不可
```

---

## 6. project rollback

project rollback：

```text
- project baseline rollback
- federation compatibility review対象
- replay verification mandatory
```

---

## 7. cross-project rollback

cross-project rollback：

```text
- multiple project synchronization必要
- dependency graph review mandatory
- rollback propagation mandatory
```

cross-project rollback は federation risk を伴う。

---

## 8. federation rollback

federation rollback：

```text
- federation baseline rollback
- cross-project synchronization mandatory
- freeze interaction mandatory
- replayability mandatory
```

federation rollback failure は federation blocker。

---

## 9. distribution rollback

distribution rollback：

```text
- distribution baseline rollback
- provenance/restriction governance review mandatory
- release audit mandatory
- security/privacy review mandatory
```

---

## 10. rollback trigger taxonomy

rollback trigger taxonomy：

```text
replay_breaking_regression
semantic_freeze_violation
package_incompatibility
cross_project_incompatibility
distribution_governance_failure
security_boundary_violation
privacy_boundary_violation
production_rollout_failure
```

trigger ambiguity は rollback blocker。

---

## 11. rollback baseline selection

rollback baseline selection は deterministic であるべき。

必要：

```text
- previous approved baseline exists
- replayable baseline refs exist
- compatibility verified baseline exists
- freeze-valid baseline exists
```

stale baseline を rollback target として使ってはならない。

---

## 12. baseline selection policy

selection policy 候補：

```text
latest_stable_baseline
latest_replayable_baseline
latest_freeze_valid_baseline
last_distribution_approved_baseline
```

selection policy は audit mandatory。

---

## 13. partial rollback model

partial rollback 候補：

```text
package_only_rollback
vocabulary_rollback
replay_baseline_rollback
compatibility_matrix_rollback
freeze_baseline_rollback
```

partial rollback は dependency graph と整合する必要がある。

---

## 14. rollback dependency relation

rollback dependency relation：

```text
package rollback
↓ may require
replay baseline rollback

vocabulary rollback
↓ may require
compatibility matrix rollback
```

incomplete dependency rollback は governance risk。

---

## 15. rollback replayability

rollback replayability 条件：

```text
- rollback trigger recorded
- rollback scope recorded
- rollback baseline refs recorded
- rollback dependency refs recorded
- rollback propagation refs recorded
- rollback verification refs recorded
```

Replay 不可能な rollback は production blocker。

---

## 16. rollback freeze interaction

freeze 中 rollback：

```text
- rollback introduces semantic change
- rollback introduces compatibility bridge
- rollback affects replay baseline
```

の場合、freeze exception review を要求しうる。

---

## 17. rollback propagation lifecycle

rollback propagation lifecycle：

```text
rollback_declared
rollback_propagated
rollback_acknowledged
rollback_in_effect
rollback_completed
rollback_verified
rollback_archived
```

cross-project rollback は acknowledgment tracking mandatory。

---

## 18. rollback propagation

rollback propagation は dependency graph に従う。

例：

```text
SansaVRM rollback
↓
Studio AI compatibility rollback
↓
SansaXR federation rollback
↓
HLDocS governance feedback rollback alignment
```

---

## 19. rollback verification

rollback verification 条件：

```text
- consistency validator pass
- replay validator pass
- freeze validation pass
- compatibility validation pass
- rollback debt recorded
```

verification incomplete は production blocker。

---

## 20. rollback debt

rollback debt 候補：

```text
temporary rollback bridge
partial compatibility rollback
legacy replay fallback
rollback compatibility shim
```

rollback debt は visible governance artifact とする。

---

## 21. rollback debt governance

rollback debt governance：

```text
- debt registration
- debt severity
- debt expiration
- debt owner
- debt replay impact
- debt cleanup target
```

hidden rollback debt を禁止する。

---

## 22. rollback stabilization policy

rollback stabilization policy：

```text
- rollback cooldown window
- replay verification window
- staged re-promotion
- freeze revalidation
- cross-project re-acknowledgment
```

rollback直後の immediate production re-rollout を禁止しうる。

---

## 23. rollback invalidation

以下は rollback invalidation を発生させる。

```text
- rollback baseline stale
- rollback verification failure
- rollback replayability incomplete
- rollback propagation unresolved
- unresolved rollback debt
```

invalidated rollback baseline を active federation baseline として扱ってはならない。

---

## 24. cross-project rollback synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
SansaXR
HLDocS
```

同期対象：

```text
- rollback baseline
- replay baseline
- compatibility baseline
- freeze baseline
- propagation acknowledgment
```

unsynchronized rollback は federation compatibility risk。

---

## 25. rollback lifecycle

rollback lifecycle：

```text
rollback_requested
rollback_review_required
rollback_prepared
rollback_executed
rollback_verified
rollback_active
rollback_superseded
rollback_archived
```

rollback_verified 前に production baseline として扱ってはならない。

---

## 26. rollback report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_rollback_report",
  "rollback_scope": "federation_rollback",
  "rollback_status": "rollback_review_required",
  "rollback_trigger": "replay_breaking_regression",
  "baseline_refs": [],
  "verification_refs": [],
  "rollback_debt_refs": []
}
```

---

## 27. reason codes

候補 reason code：

```text
rollback_baseline_missing
rollback_replayability_incomplete
rollback_dependency_unresolved
rollback_freeze_exception_required
rollback_verification_failed
rollback_propagation_unresolved
rollback_debt_unresolved
rollback_cross_project_unsynchronized
```

---

## 28. orchestration relation

federation execution orchestration は以下を block する。

```text
- rollback verification incomplete
- rollback replayability incomplete
- rollback debt unresolved in production scope
- rollback propagation unresolved
- rollback freeze violation unresolved
```

---

## 29. dashboard relation

Dashboard は rollback summary を表示できる。

表示対象：

```text
- rollback scope
- rollback trigger
- rollback status
- rollback verification status
- rollback propagation status
- rollback debt summary
```

Dashboard は rollback validity を独自決定しない。

---

## 30. CI mapping

CI fail 条件：

```text
- replay-incomplete rollback used as active baseline
- rollback verification failed
- unresolved rollback debt in production scope
- unsynchronized cross-project rollback
- invalidated rollback baseline active
```

CI warn 条件：

```text
- rollback_review_required
- rollback cooldown active
- rollback debt cleanup pending
- rollback acknowledgment pending outside production scope
```

---

## 31. 禁止事項

以下を禁止する。

```text
- replay-incomplete rollback を production baseline に適用すること
- stale baseline へ rollback すること
- hidden rollback debt を持ち込むこと
- rollback propagation unresolved のまま federation active 化すること
- rollback verification missing のまま production rollout すること
```

---

## 32. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance rollback federation model を formalize すべき
- rollback scope / trigger / propagation を分離すべき
- rollback replayability を mandatory 化すべき
- rollback debt governance を formal artifact 化すべき
- cross-project rollback synchronization を governance layer に含めるべき
```

---

## 33. 結論

governance rollback federation model は、SansaVRM federation governance baseline の rollback を replayable に管理する model である。

これにより、local / project / federation / distribution rollback を dependency-aware に実行しつつ、freeze・migration・compatibility・cross-project synchronization を維持できる。
