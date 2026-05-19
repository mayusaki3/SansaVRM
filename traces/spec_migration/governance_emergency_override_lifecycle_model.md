# governance emergency override lifecycle model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance emergency override lifecycle model を定義する。

emergency override lifecycle model は、security / privacy / recovery / rollback / distribution / replay repair などの緊急 override を audit・expiration・follow-up review 付きで扱う。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- emergency override taxonomy
- override lifecycle
- override authority
- override boundary
- override replayability
- override expiration
- override debt
- cross-project override synchronization
```

本 model は以下を行わない。

```text
- override を silent bypass としない
- non-overridable blocker を override しない
- replay-incomplete override を production/distribution decision に使わない
- expired override を active override と扱わない
```

---

## 3. override positioning

emergency override は以下に属する。

```text
Emergency Governance Layer
Audit Governance Layer
Recovery Governance Layer
Release Governance Layer
Distribution Governance Layer
```

override は通常 governance path の例外であり、通常 path の代替ではない。

---

## 4. emergency override taxonomy

override taxonomy：

```text
security_hotfix_override
privacy_boundary_override
critical_replay_repair_override
emergency_recovery_override
rollback_compatibility_override
temporary_preview_distribution_override
critical_distribution_correction_override
```

---

## 5. non-overridable blocker

non-overridable blocker 候補：

```text
unresolved rights holder conflict
unresolved legal blocker
missing provenance chain for public distribution
source-of-truth ambiguity in production scope
replay evidence missing for destructive operation
```

non-overridable blocker を override してはならない。

---

## 6. override lifecycle

override lifecycle：

```text
override_requested
override_review_required
override_approved
override_active
override_expired
override_followup_required
override_resolved
override_invalidated
override_archived
```

override_active は expiration mandatory。

---

## 7. override authority

authority taxonomy：

```text
emergency_authority
security_authority
privacy_authority
recovery_authority
distribution_authority
audit_authority
```

Authority ambiguity は review_required または blocker。

---

## 8. override boundary

override boundary は以下を明示する。

```text
- affected blocker
- affected scope
- affected operation
- allowed duration
- allowed downstream propagation
- rollback plan
```

boundary ambiguity は blocker。

---

## 9. override replayability

replayability 条件：

```text
- override refs recorded
- authority refs recorded
- affected blocker refs recorded
- expiration refs recorded
- audit refs recorded
- follow-up refs recorded
```

Replay 不可能 override は production/distribution decision に使ってはならない。

---

## 10. override expiration

expiration は mandatory。

候補：

```text
absolute timestamp
release milestone
recovery completion
follow-up review completion
```

expired override を active と扱ってはならない。

---

## 11. override debt

override debt 候補：

```text
temporary governance waiver
post-override audit gap
post-override compatibility review
temporary distribution waiver
```

override debt は visible governance artifact とする。

---

## 12. override invalidation

invalidation trigger：

```text
override expired
authority invalidated
affected blocker reclassified as non-overridable
follow-up review failed
replayability lost
```

invalidated override を active decision evidence として扱ってはならない。

---

## 13. cross-project override synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
override taxonomy
override boundary
override authority
non-overridable blocker taxonomy
override expiration policy
```

---

## 14. override report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_emergency_override_report",
  "override_taxonomy": "critical_replay_repair_override",
  "override_status": "override_review_required",
  "authority_refs": [],
  "expiration_refs": [],
  "source_of_truth_refs": []
}
```

---

## 15. reason codes

```text
override_without_audit
override_expired_but_active
override_non_overridable_blocker
override_replayability_missing
override_authority_ambiguous
override_cross_project_unsynchronized
override_followup_missing
```

---

## 16. orchestration relation

federation execution orchestration は以下を block する。

```text
- non-overridable blocker override
- expired override active
- override without audit
- replayability missing in override decision
- follow-up missing after emergency override
```

---

## 17. HLDocS feedback

```text
- emergency override lifecycle を formalize すべき
- override / expiration / follow-up / debt を formal artifact 化すべき
- non-overridable blocker taxonomy を governance layer に含めるべき
- override replayability を mandatory 化すべき
```

---

## 18. 結論

governance emergency override lifecycle model は、SansaVRM federation における緊急 override を replayable かつ audit 可能に管理する model である。

これにより、緊急対応と governance safety を両立できる。
