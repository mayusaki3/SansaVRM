# governance compatibility matrix governance model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance compatibility matrix governance model を定義する。

compatibility matrix governance model は、package / semantic / migration / rollback / policy / restriction の compatibility matrix を governance object として管理する。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- compatibility matrix taxonomy
- matrix authority
- matrix lifecycle
- matrix freeze
- matrix bridge
- matrix replayability
- matrix invalidation
- cross-project matrix synchronization
```

本 model は以下を行わない。

```text
- compatibility matrix drift を silent acceptance しない
- projection matrix を canonical matrix と扱わない
- replay-incomplete matrix transition を production compatibility に使わない
- temporary bridge matrix を permanent matrix と扱わない
```

---

## 3. matrix positioning

compatibility matrix は以下に属する。

```text
Compatibility Governance Layer
Dependency Governance Layer
Migration Governance Layer
Operational Traceability Layer
```

compatibility matrix は compatibility decision の source artifact である。

---

## 4. compatibility matrix taxonomy

matrix taxonomy：

```text
package_compatibility_matrix
semantic_compatibility_matrix
migration_compatibility_matrix
rollback_compatibility_matrix
policy_compatibility_matrix
restriction_compatibility_matrix
license_compatibility_matrix
```

---

## 5. package_compatibility_matrix

package_compatibility_matrix：

```text
- package version compatibility
- dependency compatibility
- rollout compatibility
```

---

## 6. semantic_compatibility_matrix

semantic_compatibility_matrix：

```text
- vocabulary compatibility
- invariant compatibility
- lifecycle compatibility
- authority compatibility
```

---

## 7. migration_compatibility_matrix

migration_compatibility_matrix：

```text
- migration path compatibility
- bridge requirement
- replay adapter requirement
- migration debt impact
```

---

## 8. rollback_compatibility_matrix

rollback_compatibility_matrix：

```text
- rollback target compatibility
- rollback dependency compatibility
- rollback bridge requirement
- recovery fallback compatibility
```

---

## 9. policy / restriction / license compatibility matrix

policy / restriction / license compatibility matrix は以下を扱う。

```text
- conditional policy compatibility
- restriction merge compatibility
- VN3 / VRM / CC / Booth terms compatibility
- distribution authorization compatibility
```

---

## 10. matrix authority

matrix authority：

```text
compatibility_authority
migration_authority
rollback_authority
policy_authority
restriction_authority
license_authority
```

Authority ambiguity は review_required または blocker。

---

## 11. matrix lifecycle

matrix lifecycle：

```text
matrix_pending
matrix_active
matrix_review_required
matrix_frozen
matrix_invalidated
matrix_superseded
matrix_archived
```

invalidated matrix を active compatibility source として扱ってはならない。

---

## 12. matrix freeze

matrix freeze は compatibility semantics を固定する。

必要：

```text
- matrix version
- freeze baseline refs
- authority refs
- replay refs
- exception refs where applicable
```

---

## 13. matrix bridge

matrix bridge 候補：

```text
migration matrix bridge
rollback matrix bridge
policy matrix bridge
license compatibility bridge
```

Temporary matrix bridge は expiration mandatory。

---

## 14. matrix replayability

replayability 条件：

```text
- matrix refs recorded
- authority refs recorded
- compatibility decision refs recorded
- bridge refs recorded where applicable
- invalidation refs recorded
```

Replay 不可能 matrix transition は production blocker。

---

## 15. matrix invalidation

invalidation trigger：

```text
compatibility rule changed
semantic invariant changed
policy expression changed
restriction merge rule changed
license mapping changed
bridge expired
replayability lost
```

---

## 16. cross-project matrix synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
package compatibility matrix
semantic compatibility matrix
policy compatibility matrix
license compatibility matrix
restriction compatibility matrix
```

---

## 17. matrix report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_compatibility_matrix_report",
  "matrix_taxonomy": "license_compatibility_matrix",
  "matrix_status": "matrix_review_required",
  "authority_refs": [],
  "compatibility_refs": [],
  "source_of_truth_refs": []
}
```

---

## 18. reason codes

```text
matrix_authority_ambiguous
matrix_replayability_missing
matrix_invalidated_but_active
matrix_cross_project_unsynchronized
matrix_bridge_expired
matrix_rule_drift_detected
```

---

## 19. orchestration relation

federation execution orchestration は以下を block する。

```text
- invalidated matrix active in production scope
- replayability missing in compatibility decision
- license compatibility conflict unresolved in distribution scope
- policy/restriction matrix unsynchronized in federation scope
```

---

## 20. HLDocS feedback

```text
- compatibility matrix governance を formalize すべき
- matrix authority / freeze / bridge / invalidation を formal artifact 化すべき
- license compatibility matrix を distribution governance に接続すべき
- cross-project matrix synchronization を governance layer に含めるべき
```

---

## 21. 結論

governance compatibility matrix governance model は、SansaVRM federation における compatibility matrix を replayable governance object として管理する model である。

これにより、package / semantic / policy / restriction / license compatibility を一貫して扱える。
