# governance replay validator

## 1. 目的

本ドキュメントは、SansaVRM federation における governance replay validator を定義する。

governance replay validator は、authorization / checkpoint / cleanup / provenance / response / completion が、後から source evidence、package version、policy version、audit trail に基づいて再現可能かを検証する validator である。

---

## 2. 基本方針

governance replay validator は以下を扱う。

```text
- replay evidence completeness
- package version replayability
- policy version replayability
- checkpoint replayability
- cleanup authorization replayability
- response propagation replayability
- override / rollback replayability
- cross-project replayability
```

governance replay validator は以下を行わない。

```text
- replay 成功を governance approval とみなさない
- stale evidence に基づく replay を active valid とみなさない
- dashboard projection を replay source としない
- replay 不可能な authorization を warning のみで通過させない
```

---

## 3. validator positioning

本 validator は以下に属する。

```text
Federated Governance Layer
Operational Traceability Layer
Validation Layer
```

governance consistency validator の後段または同列で実行できる。

---

## 4. replay targets

replay target：

```text
checkpoint decision
cleanup authorization
cleanup execution
rollback execution
override decision
automation promotion / demotion
cross-project response propagation
completion review
package rollout decision
```

---

## 5. replay evidence

replay evidence は以下を含む。

```text
source evidence refs
validator reports
comparison reports
cleanup reports
provenance reports
audit reports
package version refs
policy version refs
vocabulary version refs
orchestration stage refs
```

Dashboard projection は replay evidence ではない。

---

## 6. replay completeness

Replay completeness 条件：

```text
- required evidence refs exist
- evidence refs are resolvable
- package version set is recorded
- policy version set is recorded
- vocabulary semantic version is recorded
- audit trail exists
- source evidence is not stale / superseded in active replay scope
```

---

## 7. checkpoint replay validation

検査：

```text
- checkpoint kind recorded
- checkpoint status recorded
- evidence freeze refs recorded
- promotion reason recorded
- invalidation / superseded state recorded
```

Fail / blocked 条件：

```text
- checkpoint passed without evidence freeze refs
- invalidated checkpoint replayed as active
- checkpoint projection used as replay source
```

---

## 8. cleanup authorization replay validation

検査：

```text
- cleanup_authorization_report exists
- cleanup_ready evidence exists
- cleanup scope freeze refs exist
- rollback recoverability evidence exists
- review evidence exists where required
- package / policy version recorded
```

Fail / blocked 条件：

```text
- cleanup authorization without replay evidence
- authorization replay uses stale evidence
- irreversible cleanup authorization lacks review evidence
```

---

## 9. cleanup execution replay validation

検査：

```text
- cleanup execution plan recorded
- execution unit list recorded
- executed unit status recorded
- failure / rollback state recorded
- post-cleanup validation refs recorded
```

Fail / blocked 条件：

```text
- cleanup completed without execution unit evidence
- cleanup failure not replayable
- rollback_required without rollback replay evidence
```

---

## 10. override replay validation

検査：

```text
- override reason recorded
- override approver recorded
- override scope recorded
- override expiration recorded where applicable
- override restriction recorded
```

Fail / blocked 条件：

```text
- override without replayable audit
- override allowed for non-overridable violation
- override source evidence missing
```

---

## 11. rollback replay validation

検査：

```text
- rollback trigger recorded
- rollback scope recorded
- rollback execution order recorded
- rollback result recorded
- rollback failure recorded where applicable
```

Fail / blocked 条件：

```text
- rollback report missing
- rollback failure not replayable
- rollback scope unresolved
```

---

## 12. response propagation replay validation

検査：

```text
- response source recorded
- dependency graph refs recorded
- propagation chain recorded
- acknowledgment chain recorded
- stabilization policy recorded
- isolation boundary recorded
```

Fail / blocked 条件：

```text
- federation_breaking response not replayable
- cleanup_hold propagation missing evidence
- acknowledgment state missing in critical scope
```

---

## 13. package replay validation

検査：

```text
- package set version recorded
- dependency graph version recorded
- compatibility matrix version recorded
- semantic mapping version recorded
- rollout stage recorded
```

Fail / blocked 条件：

```text
- package rollout decision not replayable
- incompatible package mix lacks replay evidence
- package baseline missing version set
```

---

## 14. provenance replay validation

検査：

```text
- provenance graph version recorded
- restriction merge result recorded
- review_required decision recorded
- editor attribution continuity recorded
- temporary bridge resolution recorded
```

Fail / blocked 条件：

```text
- distribution-ready decision not replayable
- restriction conflict resolution not replayable
- temporary bridge source decision not replayable
```

---

## 15. replay status

Replay status：

```text
replayable
replayable_with_warning
replay_incomplete
replay_blocked
replay_invalidated
replay_superseded
```

`replayable` は legal clearance や governance approval を意味しない。

---

## 16. replay invalidation

以下は replay invalidation を発生させる。

```text
- source evidence stale
- package version superseded
- policy version superseded
- vocabulary semantic changed
- audit invalidated
- dependency graph changed
```

invalidated replay を active authorization evidence として扱ってはならない。

---

## 17. replay report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_replay_report",
  "validator_module": "governance_replay_validator",
  "replay_status": "replay_incomplete",
  "replay_targets": [],
  "missing_evidence": [],
  "findings": [],
  "source_of_truth_refs": []
}
```

---

## 18. reason codes

候補 reason code：

```text
replay_evidence_missing
replay_evidence_unresolvable
replay_uses_stale_evidence
checkpoint_replay_missing_evidence_freeze
cleanup_authorization_not_replayable
cleanup_execution_not_replayable
rollback_not_replayable
override_not_replayable
response_propagation_not_replayable
package_rollout_not_replayable
provenance_decision_not_replayable
projection_used_as_replay_source
```

---

## 19. severity mapping

Blocked 条件：

```text
- authorization decision not replayable
- cleanup execution not replayable
- rollback not replayable after rollback_required
- federation_breaking response not replayable
- package rollout not replayable in production scope
```

Warn 条件：

```text
- replay evidence incomplete outside critical scope
- optional evidence unresolvable
- replayable_with_warning
```

---

## 20. orchestration relation

federation execution orchestration は replay validator の fail / blocked を受けた場合、以下を停止する。

```text
- cleanup execution
- authorization
- completion review
- production rollout
```

---

## 21. dashboard relation

Dashboard は replay status を表示できる。

表示対象：

```text
- replay status
- missing evidence
- stale evidence
- replay-blocked targets
- replay warning summary
```

Dashboard は replay source ではない。

---

## 22. CI mapping

CI fail 条件：

```text
- governance replay status fail / blocked
- projection used as replay source
- authorization decision not replayable
- cleanup execution not replayable
- package rollout not replayable in production scope
```

CI warn 条件：

```text
- replayable_with_warning
- optional evidence missing
- replay incomplete outside critical scope
```

---

## 23. 禁止事項

以下を禁止する。

```text
- replay 不可能な authorization を active とすること
- dashboard projection を replay evidence として使うこと
- stale evidence に基づく replay を valid とすること
- replayable を governance approved と同義扱いすること
- replay_blocked を known limitation として production rollout へ進めること
```

---

## 24. HLDocS feedback

本 validator から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance replay validator を formalize すべき
- authorization / checkpoint / cleanup / response の replayability を検証すべき
- replay evidence refs と package/policy version refs を mandatory 化すべき
- dashboard projection を replay source から除外すべき
- replay_blocked は cleanup / authorization / completion を block すべき
```

---

## 25. 結論

governance replay validator は、SansaVRM federation governance decision が後から再現可能かを検証する validator である。

これにより、authorization / cleanup / checkpoint / response / package rollout が、source evidence と governance package version に基づいて replayable であることを確認し、unsafe non-replayable governance を防止できる。
