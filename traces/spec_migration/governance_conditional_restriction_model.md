# governance conditional restriction model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance conditional restriction model を定義する。

conditional restriction model は、VN3系を含む conditional restriction、operation-aware restriction propagation、distribution blocker、component/assembly/derived restriction を扱う。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- restriction taxonomy
- conditional restriction
- operation-aware restriction
- restriction propagation
- restriction merge
- restriction authority
- restriction replayability
- cross-project restriction synchronization
```

本 model は以下を行わない。

```text
- unresolved restriction ambiguity を allow としない
- restriction weakening を silent mutation しない
- replay-incomplete restriction propagation を distribution authorization に使わない
- projection restriction summary を canonical restriction source と扱わない
```

---

## 3. restriction positioning

conditional restriction は以下に属する。

```text
Restriction Governance Layer
Policy Governance Layer
Distribution Governance Layer
Operational Traceability Layer
```

restriction は operation-level governance constraint を表す。

---

## 4. restriction taxonomy

restriction taxonomy：

```text
commercial_restriction
corporate_restriction
redistribution_restriction
modification_restriction
r18_restriction
political_restriction
religious_restriction
ai_training_restriction
runtime_distribution_restriction
jurisdiction_restriction
```

---

## 5. conditional restriction

conditional restriction 候補：

```text
commercial_use=true の場合 restriction 有効
public_distribution=true の場合 restriction 有効
ai_training=true の場合 restriction 有効
corporate_use=true の場合 restriction 有効
```

condition semantic drift は invalidation trigger。

---

## 6. operation-aware restriction

operation-aware restriction 候補：

```text
export restriction
upload restriction
distribution restriction
runtime restriction
assembly restriction
conversion restriction
AI training restriction
```

operation context は replayable mandatory。

---

## 7. restriction scope

scope taxonomy：

```text
asset_scope
component_scope
assembly_scope
derived_scope
distribution_scope
runtime_scope
operation_scope
```

scope ambiguity は review_required または blocker。

---

## 8. restriction propagation

propagation 例：

```text
component restriction
↓
assembly restriction
↓
derived restriction
↓
distribution restriction
```

restriction propagation は replayable mandatory。

---

## 9. restriction merge

merge policy 候補：

```text
most_restrictive_wins
explicit_deny_wins
conflict_requires_review
unknown_requires_review
```

silent restriction weakening を禁止する。

---

## 10. VN3 restriction examples

VN3系では以下を restriction として扱いうる。

```text
commercial restriction
corporate restriction
redistribution restriction
modification restriction
R18 restriction
political/religious restriction
AI training restriction
credit requirement
```

---

## 11. restriction authority

restriction authority：

```text
restriction_authority
policy_authority
distribution_authority
review_authority
```

Authority ambiguity は review_required または blocker。

---

## 12. restriction replayability

replayability 条件：

```text
- restriction refs recorded
- policy expression refs recorded
- propagation refs recorded
- merge refs recorded
- authority refs recorded
```

Replay 不可能 restriction propagation は public/commercial distribution に使ってはならない。

---

## 13. restriction invalidation

invalidation trigger：

```text
condition semantic changed
policy expression invalidated
restriction merge policy changed
replayability lost
scope interpretation changed
compatibility matrix invalidated
```

invalidated restriction propagation を active distribution evidence として扱ってはならない。

---

## 14. restriction blockers

blocker taxonomy：

```text
commercial_distribution_blocker
public_distribution_blocker
redistribution_blocker
ai_training_blocker
runtime_distribution_blocker
jurisdiction_blocker
```

blocker unresolved の場合 distribution を停止する。

---

## 15. restriction override

override 候補：

```text
manual legal review override
temporary preview waiver
emergency rollback redistribution waiver
```

override は audit / expiration / follow-up mandatory。

---

## 16. cross-project restriction synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
distribution pipeline
HLDocS
```

同期対象：

```text
restriction taxonomy
merge policy
condition taxonomy
scope taxonomy
blocker taxonomy
```

---

## 17. restriction lifecycle

```text
restriction_pending
restriction_active
restriction_review_required
restriction_invalidated
restriction_superseded
restriction_archived
```

---

## 18. restriction report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_conditional_restriction_report",
  "restriction_taxonomy": "commercial_restriction",
  "restriction_status": "restriction_review_required",
  "restriction_refs": [],
  "propagation_refs": [],
  "source_of_truth_refs": []
}
```

---

## 19. reason codes

```text
restriction_unknown_treated_as_allow
restriction_scope_ambiguous
restriction_replayability_missing
restriction_invalidated_but_active
restriction_cross_project_unsynchronized
restriction_merge_conflicted
restriction_override_without_audit
```

---

## 20. orchestration relation

federation execution orchestration は以下を block する。

```text
- unresolved distribution blocker
- replayability missing in restriction propagation
- invalidated restriction active in public/commercial distribution
- unknown restriction treated as allow
```

---

## 21. HLDocS feedback

```text
- conditional restriction model を formalize すべき
- restriction propagation / merge / blocker を governance artifact 化すべき
- operation-aware restriction を governance layer に含めるべき
- unknown restriction != allow invariant を formalize すべき
```

---

## 22. 結論

governance conditional restriction model は、VN3系を含む conditional restriction を operation-aware / replayable に管理する model である。

これにより、component/assembly/derived/distribution に跨る restriction propagation と distribution blocker を安全に扱える。
