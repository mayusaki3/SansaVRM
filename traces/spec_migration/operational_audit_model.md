# operational audit model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における operational audit model を定義する。

operational audit model は、validator / comparison / cleanup / provenance / checkpoint / automation / override / rollback を横断する federated governance audit を扱う。

---

## 2. 基本方針

operational audit model は以下を扱う。

```text
- federated audit model
- audit scope taxonomy
- audit lifecycle
- audit invalidation
- audit retention
- audit/source separation
- cross-project audit federation
- replayable governance audit
```

operational audit model は以下を行わない。

```text
- audit projection を source of truth と扱わない
- stale audit を active authorization evidence と扱わない
- audit alone で governance pass を決定しない
- privacy/security boundary を無視しない
```

---

## 3. audit positioning

audit は以下に属する。

```text
Operational Governance Layer
Federated Governance Layer
Operational Traceability Layer
```

Core Semantic Layer ではない。

---

## 4. federated audit model

federated audit は複数 governance scope を横断する。

対象：

```text
- validator audit
- comparison audit
- cleanup audit
- provenance audit
- checkpoint audit
- automation audit
- override audit
- rollback audit
- completion audit
```

federated audit は cross-project audit reference を扱える。

---

## 5. audit scope taxonomy

audit scope taxonomy：

```text
validator_scope
comparison_scope
cleanup_scope
provenance_scope
checkpoint_scope
automation_scope
override_scope
rollback_scope
completion_scope
federation_scope
```

scope ambiguity は audit warning または blocker とする。

---

## 6. audit severity

audit severity：

```text
informational
warning
critical
security_sensitive
distribution_sensitive
```

severity は retention / federation visibility / override policy に影響する。

---

## 7. audit lifecycle

audit lifecycle：

```text
audit_generated
audit_active
audit_stale
audit_superseded
audit_invalidated
audit_archived
audit_retained
audit_expired
```

stale / superseded audit を active governance evidence として扱ってはならない。

---

## 8. audit invalidation

以下は audit invalidation を発生させる。

```text
- validator taxonomy changed
- comparison criteria changed
- provenance graph changed
- cleanup scope changed
- checkpoint invalidated
- authorization superseded
- governance automation policy changed
```

invalidated audit は replay source として扱えても active authorization evidence としては扱えない。

---

## 9. audit/source separation

重要：

```text
audit
```

と：

```text
source evidence
```

を混同してはならない。

例：

```text
audit says checkpoint passed
```

でも：

```text
source evidence stale
```

なら checkpoint valid ではない。

---

## 10. replayable governance audit

audit は replayable であるべき。

再構成対象：

```text
- why checkpoint passed
- why cleanup authorized
- why rollback triggered
- why automation promoted
- why override allowed
```

Replay に必要：

```text
- evidence refs
- policy version
- validator taxonomy version
- orchestration stage refs
- execution context
```

---

## 11. audit retention

Retention policy は audit severity / scope に依存する。

例：

```text
temporary rerun audit
  → short retention

distribution authorization audit
  → long retention

rollback failure audit
  → long retention
```

Retention expiration は audit lifecycle に記録する。

---

## 12. audit archival

archival 条件：

```text
- superseded audit
- expired operational audit
- archived reconstruction phase
- detached legacy audit scope
```

archived audit は replay source として保持できる。

---

## 13. cross-project audit federation

federated audit は cross-project refs を扱える。

対象例：

```text
SansaVRM
SansaVRM Studio AI
SansaXR
HLDocS
```

cross-project audit ref は以下を持つ。

```text
- project_id
- audit_ref
- audit_scope
- federation_visibility
```

---

## 14. federation visibility

federation visibility：

```text
private
internal_federation
cross_project
public_summary
```

security_sensitive audit は public_summary を禁止できる。

---

## 15. privacy/security boundary

以下は restricted visibility 候補。

```text
- sensitive provenance refs
- internal rollback refs
- internal governance override refs
- distribution restricted refs
- internal cleanup execution refs
```

privacy/security boundary violation は critical audit finding。

---

## 16. override audit

override operation は audit mandatory。

記録：

```text
- override reason
- override approver
- override scope
- override restriction
- override expiration
```

override without audit trail は governance violation。

---

## 17. rollback audit

rollback operation は replayable であるべき。

記録：

```text
- rollback trigger
- rollback scope
- rollback execution order
- rollback result
- rollback failures
```

rollback audit missing は operational blocker。

---

## 18. provenance audit

provenance audit は以下を記録する。

```text
- provenance graph version
- restriction merge result
- unresolved provenance blocker
- temporary provenance bridge usage
- editor attribution continuity
```

provenance audit は legal clearance を意味しない。

---

## 19. automation audit

automation decision は replayable であるべき。

記録：

```text
- automation policy version
- automation eligibility reason
- promotion reason
- demotion reason
- override relation
```

---

## 20. checkpoint audit

checkpoint audit は以下を記録する。

```text
- checkpoint kind
- checkpoint transition
- checkpoint invalidation
- checkpoint rollback
- evidence freeze refs
```

checkpoint audit missing は checkpoint validity warning または blocker。

---

## 21. audit consistency

Audit consistency 検査：

```text
- stale audit not active
- superseded audit not active
- audit/source mismatch detection
- replay evidence completeness
```

consistency unresolved は warning または blocker。

---

## 22. audit report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "operational_audit_report",
  "audit_scope": "checkpoint_scope",
  "audit_status": "audit_active",
  "audit_severity": "warning",
  "audit_refs": [],
  "source_evidence_refs": [],
  "replayable": true
}
```

---

## 23. dashboard relation

Dashboard は audit summary を表示できる。

表示対象：

```text
- audit status
- audit severity summary
- stale audit count
- override audit summary
- rollback audit summary
```

Dashboard は audit validity を独自決定しない。

---

## 24. CI mapping

CI fail 条件：

```text
- stale audit used as active authorization evidence
- override without audit trail
- rollback without rollback audit
- audit/source mismatch unresolved in critical scope
- privacy/security boundary violation
```

CI warn 条件：

```text
- replay evidence incomplete
- audit archival pending
- stale audit outside critical scope
```

---

## 25. 禁止事項

以下を禁止する。

```text
- audit alone で governance pass を決定すること
- stale/superseded audit を active evidence と扱うこと
- replay impossible audit を valid governance audit と扱うこと
- audit/source separation を無視すること
- privacy/security restricted audit を unauthorized federation visibility へ出すこと
```

---

## 26. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- federated operational audit model を formalize すべき
- replayable governance audit を formal artifact 化すべき
- audit lifecycle / invalidation / retention を formalize すべき
- audit/source separation を formalize すべき
- cross-project audit federation を扱えるようにすべき
```

---

## 27. 結論

operational audit model は、SansaVRM 再構成における federated governance audit を定義する model である。

これにより、validator / cleanup / provenance / checkpoint / automation / rollback の operational governance を replayable audit として保持しつつ、cross-project federation と privacy/security boundary を両立できる。
