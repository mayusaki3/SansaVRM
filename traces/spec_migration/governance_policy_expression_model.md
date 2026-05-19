# governance policy expression model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance policy expression model を定義する。

policy expression model は、VN3系ライセンスや独自ライセンスの conditional policy を machine-readable に表現し、operation-level evaluation / restriction propagation / license reconciliation へ接続する。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- policy expression taxonomy
- policy rule structure
- condition expression
- effect expression
- scope expression
- policy authority
- policy replayability
- cross-project policy expression synchronization
```

本 model は以下を行わない。

```text
- natural language license を完全自動解釈済みとして扱わない
- unresolved policy ambiguity を allow としない
- projection policy summary を canonical policy expression と扱わない
- replay-incomplete policy expression を distribution authorization に使わない
```

---

## 3. policy expression positioning

policy expression は以下に属する。

```text
Policy Governance Layer
Restriction Governance Layer
Distribution Governance Layer
Operational Traceability Layer
```

policy expression は license text の semantic projection であり、source license text そのものではない。

---

## 4. policy expression taxonomy

policy expression taxonomy：

```text
license_policy_expression
restriction_policy_expression
distribution_policy_expression
operation_policy_expression
conditional_policy_expression
exception_policy_expression
```

---

## 5. policy rule structure

policy rule は以下を持つ。

```text
rule_id
effect
condition
scope
authority_refs
source_license_refs
replay_refs
```

rule_id は replayable であるべき。

---

## 6. effect expression

effect 候補：

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

## 7. condition expression

condition は operation / actor / distribution / asset / context を表せる。

候補：

```text
commercial_use
public_distribution
third_party_distribution
corporate_use
ai_training
r18_use
political_use
redistribution
modification
derived_asset
```

---

## 8. scope expression

scope 候補：

```text
asset_scope
component_scope
assembly_scope
derived_scope
distribution_scope
runtime_scope
operation_scope
```

scope ambiguity は review_required。

---

## 9. VN3 policy expression

VN3系では以下を表現対象とする。

```text
commercial use condition
corporate use condition
redistribution condition
modification condition
R18 condition
political / religious use condition
AI training condition
credit / attribution condition
```

---

## 10. source license relation

policy expression は source license refs を持つ。

必要：

```text
- source license text refs
- license snapshot refs
- acquisition refs
- source_raw refs where applicable
```

source license が不明な expression を canonical expression として扱ってはならない。

---

## 11. policy authority

policy authority：

```text
policy_authority
license_authority
restriction_authority
distribution_authority
```

Authority ambiguity は review_required または blocker。

---

## 12. policy replayability

replayability 条件：

```text
- policy expression refs recorded
- source license refs recorded
- authority refs recorded
- condition refs recorded
- evaluation refs recorded where applicable
```

Replay 不可能 policy expression は distribution authorization に使ってはならない。

---

## 13. policy ambiguity

policy ambiguity 候補：

```text
ambiguous license text
missing source snapshot
conflicting condition
unknown operation scope
unknown actor scope
```

policy ambiguity は review_required または deny 側に倒す。

---

## 14. policy override

policy override 候補：

```text
temporary preview waiver
manual legal review override
emergency rollback redistribution waiver
```

override は audit / expiration / follow-up mandatory。

---

## 15. policy invalidation

invalidation trigger：

```text
source license changed
license snapshot invalidated
policy authority invalidated
condition semantic changed
replayability lost
evaluation model changed
```

invalidated policy expression を active distribution policy として扱ってはならない。

---

## 16. cross-project policy synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
HLDocS
distribution pipeline
```

同期対象：

```text
policy expression taxonomy
condition taxonomy
effect taxonomy
scope taxonomy
policy authority
```

---

## 17. policy lifecycle

```text
policy_pending
policy_active
policy_review_required
policy_invalidated
policy_superseded
policy_archived
```

---

## 18. policy expression report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_policy_expression_report",
  "policy_expression_taxonomy": "conditional_policy_expression",
  "policy_status": "policy_review_required",
  "rule_refs": [],
  "source_license_refs": [],
  "source_of_truth_refs": []
}
```

---

## 19. reason codes

```text
policy_expression_ambiguous
policy_source_license_missing
policy_replayability_missing
policy_invalidated_but_active
policy_cross_project_unsynchronized
policy_unknown_treated_as_allow
policy_override_without_audit
```

---

## 20. orchestration relation

federation execution orchestration は以下を block する。

```text
- unknown treated as allow
- replayability missing in distribution policy
- invalidated policy expression active in distribution scope
- unresolved policy ambiguity in public/commercial distribution
```

---

## 21. HLDocS feedback

```text
- policy expression model を formalize すべき
- condition / effect / scope taxonomy を formal artifact 化すべき
- source license refs と policy expression を分離すべき
- unknown を allow と扱わない invariant を導入すべき
```

---

## 22. 結論

governance policy expression model は、VN3系を含む conditional policy を machine-readable governance object として扱う model である。

これにより、license text 由来の条件付き許諾を replayable に表現し、policy evaluation / distribution authorization / restriction propagation へ接続できる。
