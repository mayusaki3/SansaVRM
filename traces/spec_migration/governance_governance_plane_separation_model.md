# governance governance-plane separation model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance governance-plane separation model を定義する。

governance governance-plane separation model は、source-of-truth / orchestration / projection / audit / authorization / recovery / distribution の責務境界と authority boundary を分離する model である。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- governance plane taxonomy
- plane responsibility boundary
- authority boundary
- projection separation
- orchestration separation
- audit separation
- recovery separation
- cross-plane synchronization
- plane invalidation
```

本 model は以下を行わない。

```text
- projection を source-of-truth と扱わない
- orchestration を authority source と扱わない
- audit projection を operational source と扱わない
- recovery bridge を permanent baseline と扱わない
```

---

## 3. governance plane positioning

governance plane separation は以下に属する。

```text
Federated Governance Layer
Operational Traceability Layer
Source-of-Truth Governance Layer
Execution Governance Layer
```

plane separation は governance ambiguity を防ぐために存在する。

---

## 4. governance plane taxonomy

plane taxonomy：

```text
source_of_truth_plane
orchestration_plane
projection_plane
audit_plane
authorization_plane
recovery_plane
distribution_plane
compatibility_plane
```

各 plane は authority / replayability / lifecycle を持つ。

---

## 5. source_of_truth_plane

source_of_truth_plane：

```text
- canonical governance evidence
- canonical baseline refs
- canonical authority refs
- replayable semantic source
```

Projection は source_of_truth_plane の substitute ではない。

---

## 6. orchestration_plane

orchestration_plane：

```text
- execution coordination
- dependency ordering
- propagation handling
- synchronization handling
```

orchestration_plane は authority source ではない。

---

## 7. projection_plane

projection_plane：

```text
- dashboard
- visualization
- summarized status
- operational overview
```

projection_plane は convenience layer である。

Projection stale は source corruption と同義ではない。

---

## 8. audit_plane

audit_plane：

```text
- audit evidence chain
- replayability verification
- escalation tracking
- authority traceability
```

audit_plane は execution orchestration を直接制御しない。

---

## 9. authorization_plane

authorization_plane：

```text
- distribution authorization
- restriction propagation
- rights inheritance
- provenance governance
```

authorization_plane は semantic source-of-truth を直接変更しない。

---

## 10. recovery_plane

recovery_plane：

```text
- emergency recovery
- baseline reconstruction
- evidence reconstruction
- rollback fallback
```

recovery_plane は temporary bridge を持ちうる。

---

## 11. distribution_plane

distribution_plane：

```text
- release packaging
- distribution export
- downstream propagation
- distribution restriction handling
```

distribution_plane は canonical authority source ではない。

---

## 12. compatibility_plane

compatibility_plane：

```text
- compatibility matrix
- migration bridge
- dependency compatibility
- semantic compatibility
```

compatibility_plane は replayable evidence mandatory。

---

## 13. authority boundary

authority boundary：

```text
source authority
release authority
audit authority
distribution authority
recovery authority
compatibility authority
```

Authority crossing は replayable でなければならない。

---

## 14. projection separation

Projection separation：

```text
projection != source_of_truth
projection != authority
projection != replay evidence
```

projection ambiguity は governance risk。

---

## 15. orchestration separation

orchestration separation：

```text
orchestration != semantic authority
orchestration != audit authority
orchestration != authorization authority
```

Execution ordering は authority ownership と独立。

---

## 16. audit separation

audit separation：

```text
audit != execution source
audit != distribution source
audit != semantic authority
```

Audit は replayability/accountability を扱う。

---

## 17. authorization separation

authorization separation：

```text
authorization != semantic mutation
authorization != orchestration ownership
authorization != compatibility ownership
```

Authorization は distribution permission boundary を扱う。

---

## 18. recovery separation

recovery separation：

```text
recovery != stable baseline
recovery bridge != permanent compatibility
emergency recovery != normal release path
```

Recovery ambiguity は operational risk。

---

## 19. cross-plane synchronization

同期対象：

```text
baseline refs
authority refs
compatibility refs
freeze refs
replay refs
```

cross-plane unsynchronized state は governance risk。

---

## 20. plane replayability

各 plane は以下を replayable にする。

```text
- plane lifecycle
- authority transition
- synchronization refs
- invalidation refs
- escalation refs
```

Replay 不可能 plane transition は production blocker。

---

## 21. plane invalidation

以下は invalidation trigger：

```text
source_of_truth invalidated
projection stale in critical scope
authority invalidated
replayability lost
unsynchronized cross-plane state
```

invalidated plane evidence を active source として扱ってはならない。

---

## 22. plane lifecycle

plane lifecycle：

```text
plane_pending
plane_active
plane_review_required
plane_invalidated
plane_superseded
plane_archived
```

---

## 23. cross-project synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
plane taxonomy
authority boundary
projection policy
source-of-truth policy
compatibility ownership
```

unsynchronized plane model は federation ambiguity risk。

---

## 24. plane report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_plane_report",
  "plane_taxonomy": "projection_plane",
  "plane_status": "plane_review_required",
  "authority_refs": [],
  "synchronization_refs": [],
  "source_of_truth_refs": []
}
```

---

## 25. reason codes

候補 reason code：

```text
plane_projection_used_as_source
plane_authority_ambiguous
plane_cross_sync_missing
plane_replayability_missing
plane_invalidated_but_active
plane_boundary_violation
```

---

## 26. orchestration relation

federation execution orchestration は以下を block する。

```text
- projection used as source_of_truth
- authority ambiguity in production scope
- invalidated plane active
- replayability missing in authority transition
```

---

## 27. dashboard relation

Dashboard は plane summary を表示できる。

表示対象：

```text
- active planes
- invalidated planes
- authority ownership
- synchronization status
- replayability status
```

Dashboard は authority を独自決定しない。

---

## 28. CI mapping

CI fail 条件：

```text
- projection used as source_of_truth
- invalidated plane used as active authority
- unsynchronized cross-plane authority in production scope
- replayability missing in plane transition
```

CI warn 条件：

```text
- plane_review_required
- projection stale outside critical scope
- cross-plane acknowledgment pending outside production scope
```

---

## 29. 禁止事項

以下を禁止する。

```text
- projection を source-of-truth と扱うこと
- orchestration を semantic authority と扱うこと
- emergency recovery bridge を permanent baseline と扱うこと
- audit projection を operational source と扱うこと
- invalidated plane を active authority と扱うこと
```

---

## 30. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance plane separation を formalize すべき
- source-of-truth / projection / orchestration / audit を分離すべき
- authority boundary を formal artifact 化すべき
- cross-plane synchronization を governance layer に含めるべき
- projection != source-of-truth を formal invariant 化すべき
```

---

## 31. 結論

governance governance-plane separation model は、SansaVRM federation における governance responsibility / authority / replayability boundary を分離する model である。

これにより、source-of-truth、projection、orchestration、audit、authorization、recovery の責務混線を防ぎ、federation governance の replayability と accountability を維持できる。
