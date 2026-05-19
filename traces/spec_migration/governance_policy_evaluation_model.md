# governance policy evaluation model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance policy evaluation model を定義する。

policy evaluation model は、operation-aware / context-aware policy evaluation、conditional restriction evaluation、distribution authorization evaluation を扱う。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- evaluation taxonomy
- evaluation context
- operation-aware evaluation
- condition-aware evaluation
- evaluation result semantics
- evaluation replayability
- evaluation invalidation
- cross-project evaluation synchronization
```

本 model は以下を行わない。

```text
- unknown policy state を allow としない
- replay-incomplete evaluation を production authorization に使わない
- projection evaluation summary を canonical evaluation result と扱わない
- unresolved ambiguity を automatic allow としない
```

---

## 3. evaluation positioning

policy evaluation は以下に属する。

```text
Policy Governance Layer
Restriction Governance Layer
Distribution Governance Layer
Operational Decision Layer
```

policy evaluation は operation-level governance decision を行う。

---

## 4. evaluation taxonomy

evaluation taxonomy：

```text
policy_evaluation
restriction_evaluation
distribution_evaluation
license_evaluation
conditional_evaluation
runtime_evaluation
```

---

## 5. operation-aware evaluation

operation-aware evaluation 候補：

```text
export
upload
distribution
runtime_load
assembly
conversion
ai_training
commercial_use
public_release
```

operation context は replayable mandatory。

---

## 6. condition-aware evaluation

condition-aware evaluation 候補：

```text
commercial_use=true
public_distribution=true
corporate_use=true
r18_use=true
political_use=true
ai_training=true
```

Condition semantic drift は evaluation invalidation trigger。

---

## 7. evaluation context

context 候補：

```text
actor_context
organization_context
asset_context
component_context
distribution_context
runtime_context
jurisdiction_context
```

missing critical context は review_required または deny。

---

## 8. evaluation result semantics

result taxonomy：

```text
allow
conditional_allow
review_required
deny
not_applicable
unknown
```

unknown を allow として扱ってはならない。

---

## 9. evaluation precedence

precedence 候補：

```text
explicit deny
↓
restriction invariant
↓
conditional allow
↓
allow
↓
unknown
```

unknown は deny 側に倒すことが推奨される。

---

## 10. VN3 evaluation examples

VN3系では以下を評価対象とする。

```text
commercial use allowed?
corporate use allowed?
redistribution allowed?
modification allowed?
AI training allowed?
credit required?
```

---

## 11. evaluation authority

authority taxonomy：

```text
policy_authority
restriction_authority
distribution_authority
license_authority
review_authority
```

Authority ambiguity は review_required または blocker。

---

## 12. evaluation replayability

replayability 条件：

```text
- evaluation refs recorded
- policy expression refs recorded
- context refs recorded
- authority refs recorded
- operation refs recorded
```

Replay 不可能 evaluation は production authorization に使ってはならない。

---

## 13. evaluation invalidation

invalidation trigger：

```text
policy expression invalidated
condition semantic changed
operation taxonomy changed
context incomplete discovered
authority invalidated
replayability lost
```

invalidated evaluation result を active authorization として扱ってはならない。

---

## 14. evaluation override

override 候補：

```text
manual legal review override
emergency rollback override
temporary preview distribution override
```

override は audit / expiration / follow-up mandatory。

---

## 15. cross-project evaluation synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
evaluation taxonomy
operation taxonomy
condition taxonomy
result semantics
review escalation policy
```

---

## 16. evaluation lifecycle

```text
evaluation_pending
evaluation_active
evaluation_review_required
evaluation_invalidated
evaluation_superseded
evaluation_archived
```

---

## 17. evaluation report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_policy_evaluation_report",
  "evaluation_taxonomy": "distribution_evaluation",
  "evaluation_status": "evaluation_review_required",
  "operation_refs": [],
  "context_refs": [],
  "source_of_truth_refs": []
}
```

---

## 18. reason codes

```text
evaluation_unknown_treated_as_allow
evaluation_context_missing
evaluation_replayability_missing
evaluation_invalidated_but_active
evaluation_cross_project_unsynchronized
evaluation_ambiguity_unresolved
```

---

## 19. orchestration relation

federation execution orchestration は以下を block する。

```text
- unknown treated as allow in distribution scope
- replayability missing in policy evaluation
- invalidated evaluation active in production scope
- unresolved ambiguity in public/commercial distribution
```

---

## 20. HLDocS feedback

```text
- policy evaluation model を formalize すべき
- operation-aware / condition-aware evaluation を governance layer に含めるべき
- unknown != allow invariant を formal artifact 化すべき
- evaluation context replayability を扱うべき
```

---

## 21. 結論

governance policy evaluation model は、VN3系を含む conditional policy を operation-aware / context-aware に評価する governance model である。

これにより、distribution authorization / restriction propagation / commercial/public operation を replayable に判定できる。
