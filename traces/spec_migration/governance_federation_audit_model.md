# governance federation audit model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance federation audit model を定義する。

governance federation audit model は、local / project / federation / distribution / compliance に跨る audit scope、evidence chain、replayability、severity、propagation、authority、cross-project synchronization を扱う。

---

## 2. 基本方針

governance federation audit model は以下を扱う。

```text
- federation audit taxonomy
- audit evidence chain
- audit replayability
- audit severity / escalation
- audit propagation lifecycle
- audit invalidation
- audit debt governance
- audit authority model
- cross-project audit synchronization
```

governance federation audit model は以下を行わない。

```text
- audit projection を source of truth としない
- stale audit evidence を active authorization evidence としない
- audit waiver を hidden state としない
- audit authority 不明の blocker を federation blocker として確定しない
```

---

## 3. audit positioning

federation audit は以下に属する。

```text
Federated Governance Layer
Operational Traceability Layer
Release Governance Layer
Distribution Governance Layer
Compliance Support Layer
```

federation audit は governance decision の replayability と accountability を支える。

---

## 4. federation audit taxonomy

audit taxonomy：

```text
local_audit
project_audit
cross_project_audit
federation_audit
distribution_audit
compliance_audit
emergency_audit
```

scope により authority / retention / visibility が変わる。

---

## 5. local audit

local_audit：

```text
- project-local evidence
- federation propagation optional
- production / distribution authorization の根拠には単独で使わない
```

---

## 6. project audit

project_audit：

```text
- project baseline / release candidate の audit
- cross-project compatibility review 対象
- federation audit の input になりうる
```

---

## 7. cross-project audit

cross_project_audit：

```text
- project 間依存の audit
- acknowledgment chain mandatory
- dependency graph refs mandatory
```

---

## 8. federation audit

federation_audit：

```text
- federation baseline / gate / rollback / freeze の audit
- federation blocker 判定に影響する
- replayability mandatory
```

---

## 9. distribution audit

distribution_audit：

```text
- distribution authorization の audit
- provenance / restriction / rights inheritance refs mandatory
- public / commercial distribution では mandatory
```

---

## 10. compliance audit

compliance_audit：

```text
- legal-sensitive / policy-sensitive governance の audit
- distribution / commercial scope で使用
- visibility restriction を伴いうる
```

---

## 11. emergency audit

emergency_audit：

```text
- emergency gate / emergency override / hotfix の audit
- post-action review mandatory
- expiration / follow-up mandatory
```

---

## 12. audit evidence chain

audit evidence chain は governance decision の根拠 chain を表す。

例：

```text
baseline report
↓
release gate report
↓
semantic freeze report
↓
distribution authorization report
↓
distribution audit report
```

chain は replayable でなければならない。

---

## 13. audit evidence requirements

必要 evidence：

```text
- source evidence refs
- baseline refs
- gate refs
- freeze refs
- replay refs
- consistency refs
- authorization refs
- override refs where applicable
- rollback refs where applicable
```

Projection は audit evidence source ではない。

---

## 14. audit replayability

audit replayability 条件：

```text
- audit taxonomy recorded
- audit scope recorded
- evidence chain recorded
- policy / package / vocabulary versions recorded
- authority recorded
- propagation chain recorded where applicable
```

Replay 不可能 audit は federation / distribution authorization evidence として扱わない。

---

## 15. audit severity

audit severity：

```text
informational
warning
review_required
audit_blocker
federation_audit_blocker
distribution_audit_blocker
legal_audit_blocker
```

blocker severity は gate / readiness / authorization を停止しうる。

---

## 16. audit escalation

audit escalation は severity / scope に基づく。

例：

```text
local warning
↓
project review_required
↓
cross-project audit_blocker
↓
federation_audit_blocker
```

Escalation chain は audit trail mandatory。

---

## 17. audit propagation lifecycle

audit propagation lifecycle：

```text
audit_generated
audit_propagated
audit_acknowledged
audit_in_effect
audit_escalated
audit_resolved
audit_archived
```

cross-project / federation audit は acknowledgment tracking を要求する。

---

## 18. audit propagation

Propagation は dependency graph に従う。

例：

```text
SansaVRM distribution audit finding
↓
Studio AI workflow review
↓
Distribution pipeline authorization hold
↓
HLDocS feedback candidate
```

---

## 19. audit invalidation

以下は audit invalidation を発生させる。

```text
- source evidence stale
- source_of_truth invalidated
- replayability lost
- policy version superseded
- package baseline invalidated
- authority invalidated
```

invalidated audit を active evidence として扱ってはならない。

---

## 20. audit debt

audit debt 候補：

```text
temporary audit waiver
delayed evidence registration
legacy replay evidence gap
post-action audit follow-up
```

audit debt は visible governance artifact とする。

---

## 21. audit debt governance

audit debt governance：

```text
- debt registration
- debt severity
- debt expiration
- debt owner
- debt evidence gap
- debt cleanup target
```

hidden audit debt を禁止する。

---

## 22. audit authority model

audit authority は audit finding / blocker / escalation を確定できる主体を定義する。

候補：

```text
project_audit_authority
federation_audit_authority
distribution_audit_authority
compliance_audit_authority
emergency_audit_authority
```

Authority missing の blocker は review_required とし、確定 blocker とは分ける。

---

## 23. audit authority requirements

Authority に必要：

```text
- authority id
- authority scope
- decision capability
- expiration where applicable
- delegation rule
- audit refs
```

Authority decision は replayable でなければならない。

---

## 24. cross-project audit synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
distribution pipeline
HLDocS
```

同期対象：

```text
- audit taxonomy
- audit evidence policy
- audit severity
- audit authority
- audit visibility
- audit retention
```

unsynchronized audit policy は federation audit risk。

---

## 25. audit visibility

audit visibility：

```text
private
project_internal
internal_federation
cross_project
public_summary
restricted_compliance
```

privacy / security / compliance sensitive audit は visibility restriction を持つ。

---

## 26. audit retention

retention は scope / severity / visibility に依存する。

例：

```text
local_audit warning
  → short retention

distribution_audit blocker
  → long retention

compliance_audit
  → policy-defined retention
```

---

## 27. audit lifecycle

audit lifecycle：

```text
audit_pending
audit_review_required
audit_active
audit_blocked
audit_invalidated
audit_superseded
audit_resolved
audit_archived
audit_expired
```

invalidated audit を active evidence として扱ってはならない。

---

## 28. audit report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_federation_audit_report",
  "audit_taxonomy": "federation_audit",
  "audit_status": "audit_review_required",
  "audit_severity": "review_required",
  "evidence_chain_refs": [],
  "authority_refs": [],
  "source_of_truth_refs": []
}
```

---

## 29. reason codes

候補 reason code：

```text
audit_evidence_chain_missing
audit_replayability_missing
audit_authority_missing
audit_authority_invalid
audit_stale_evidence_used
audit_projection_used_as_source
audit_debt_hidden
audit_cross_project_unsynchronized
audit_visibility_violation
audit_invalidated_but_active
```

---

## 30. orchestration relation

federation execution orchestration は以下を block する。

```text
- federation_audit_blocker unresolved
- distribution_audit_blocker unresolved
- legal_audit_blocker unresolved
- audit replayability missing in production/distribution scope
- audit authority invalid in blocker scope
```

---

## 31. dashboard relation

Dashboard は audit summary を表示できる。

表示対象：

```text
- audit taxonomy
- audit lifecycle status
- audit severity summary
- blocker summary
- authority summary
- evidence chain completeness
- debt summary
```

Dashboard は audit decision を独自決定しない。

---

## 32. CI mapping

CI fail 条件：

```text
- federation/distribution audit blocker unresolved
- audit authority invalid in blocker scope
- invalidated audit used as active evidence
- audit evidence chain missing in production scope
- hidden audit debt detected
- audit visibility violation
```

CI warn 条件：

```text
- audit_review_required
- audit evidence delayed outside production scope
- audit debt cleanup pending
- audit acknowledgment pending outside production scope
```

---

## 33. 禁止事項

以下を禁止する。

```text
- audit projection を source of truth と扱うこと
- replay-incomplete audit を federation/distribution authorization evidence とすること
- hidden audit debt を持ち込むこと
- authority missing の finding を確定 blocker として扱うこと
- invalidated audit を active evidence として扱うこと
```

---

## 34. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- federation audit model を formalize すべき
- audit evidence chain / replayability / authority を formal artifact 化すべき
- audit severity / escalation / propagation を governance layer に含めるべき
- audit debt / invalidation / visibility を formal lifecycle 化すべき
- cross-project audit synchronization を扱うべき
```

---

## 35. 結論

governance federation audit model は、SansaVRM federation における audit evidence / authority / propagation / replayability を管理する model である。

これにより、release・rollback・distribution authorization・freeze・baseline の governance decision を replayable に監査し、cross-project federation における accountability を維持できる。
