# governance vocabulary normalization

## 1. 目的

本ドキュメントは、SansaVRM federation における governance vocabulary normalization を定義する。

governance vocabulary normalization は、validator / cleanup / checkpoint / projection / audit / provenance / orchestration / federation response に跨る terminology を正規化し、cross-project vocabulary drift を防止する。

---

## 2. 基本方針

governance vocabulary normalization は以下を扱う。

```text
- governance vocabulary registry
- normalized state semantics
- severity taxonomy normalization
- scope taxonomy normalization
- transition semantics
- reserved governance vocabulary
- cross-project vocabulary compatibility
- vocabulary drift prevention
```

governance vocabulary normalization は以下を行わない。

```text
- ambiguous shorthand を source of truth にしない
- project-local meaning drift を federation standard にしない
- projection wording を source evidence wording と混同しない
- authorization / readiness / completion を同義扱いしない
```

---

## 3. vocabulary positioning

vocabulary normalization は以下に属する。

```text
Federated Governance Layer
Operational Traceability Layer
Cross-Project Compatibility Layer
```

Core Semantic Layer とは分離する。

---

## 4. governance vocabulary registry

federation-wide vocabulary を registry 化する。

registry 対象：

```text
- state vocabulary
- severity vocabulary
- scope vocabulary
- lifecycle vocabulary
- orchestration vocabulary
- audit vocabulary
- projection vocabulary
- provenance vocabulary
```

registry ambiguity は governance warning または blocker。

---

## 5. normalized state semantics

normalized state 候補：

```text
pending
review_required
passed
blocked
invalidated
superseded
resolved
archived
stale
expired
```

各 state は federation-wide semantic を持つ。

---

## 6. readiness / authorization / completion separation

重要：

```text
ready
authorized
completed
```

を同義扱いしてはならない。

例：

```text
cleanup_ready
```

は：

```text
cleanup_execution_authorized
```

ではない。

また：

```text
checkpoint_passed
```

は：

```text
reconstruction_completed
```

ではない。

---

## 7. projection/source separation vocabulary

重要：

```text
projection_valid
```

は：

```text
source_valid
```

ではない。

また：

```text
audit_active
```

は：

```text
source evidence active
```

を意味しない。

---

## 8. severity taxonomy normalization

severity taxonomy：

```text
informational
warning
moderate
high
critical
federation_critical
security_sensitive
distribution_sensitive
```

severity alias を uncontrolled に増やしてはならない。

---

## 9. severity semantics

severity semantics：

```text
informational
  → operational visibility only

warning
  → review recommended

moderate
  → rerun/review likely required

high
  → progression gating likely required

critical
  → local execution blocking

federation_critical
  → federation-wide blocking
```

security_sensitive / distribution_sensitive は visibility restriction を伴いうる。

---

## 10. scope taxonomy normalization

scope taxonomy：

```text
project_local
cross_project
federation_wide
distribution_wide
validator_scope
comparison_scope
cleanup_scope
provenance_scope
checkpoint_scope
federation_scope
```

scope vocabulary は execution scope と governance scope を混同してはならない。

---

## 11. scope relation

例：

```text
cleanup_scope
```

は：

```text
what cleanup affects
```

であり、

```text
federation_wide
```

は：

```text
how far governance response propagates
```

を意味する。

---

## 12. lifecycle normalization

lifecycle vocabulary：

```text
generated
active
stale
superseded
invalidated
resolved
archived
expired
```

各 lifecycle state は replayable semantic を持つ。

---

## 13. transition semantics

transition semantic を formalize する。

許可例：

```text
active → stale
stale → superseded
resolved → archived
archived → expired
```

review_required → passed は evidence mandatory。

---

## 14. forbidden transition ambiguity

以下を禁止する。

```text
passed == completed
ready == authorized
projection_valid == source_valid
stale == archived
resolved == deleted
```

semantic collapse は federation governance risk とする。

---

## 15. reserved governance vocabulary

reserved vocabulary 候補：

```text
pass
valid
safe
ready
authorized
completed
resolved
stable
```

reserved vocabulary は registry semantic に従う。

project-local shorthand を federation semantic として扱ってはならない。

---

## 16. vocabulary alias governance

alias を許可する場合：

```text
- canonical vocabulary ref
- alias scope
- alias expiration
- compatibility mapping
```

を保持する。

alias drift は governance warning または blocker。

---

## 17. cross-project vocabulary compatibility

対象例：

```text
SansaVRM
SansaVRM Studio AI
SansaXR
HLDocS
```

cross-project vocabulary compatibility は以下を確認する。

```text
- same state semantic
- same severity semantic
- same scope semantic
- same lifecycle semantic
```

semantic mismatch は federation compatibility issue。

---

## 18. vocabulary drift prevention

drift prevention policy 候補：

```text
- registry-first terminology
- compatibility review
- deprecated alias tracking
- semantic freeze during release
- drift detection validator
```

vocabulary drift は replay consistency を壊しうる。

---

## 19. vocabulary invalidation

以下は vocabulary invalidation を発生させる。

```text
- semantic meaning changed
- lifecycle semantics changed
- severity semantics changed
- compatibility mapping removed
- reserved vocabulary misuse detected
```

invalidated vocabulary mapping は active federation governance に使用してはならない。

---

## 20. vocabulary replayability

vocabulary semantic は replayable であるべき。

必要：

```text
- vocabulary version
- semantic mapping version
- alias compatibility refs
- transition semantic refs
```

Replay 不可能な vocabulary drift は governance risk。

---

## 21. vocabulary audit

vocabulary change は audit mandatory。

記録：

```text
- changed vocabulary
- semantic diff
- compatibility impact
- federation impact
- alias impact
```

semantic change without audit は governance violation。

---

## 22. vocabulary report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_vocabulary_report",
  "vocabulary_version": "0.1-draft",
  "normalized_states": [],
  "severity_taxonomy": [],
  "scope_taxonomy": [],
  "reserved_vocabulary": []
}
```

---

## 23. dashboard relation

Dashboard は vocabulary compatibility summary を表示できる。

表示対象：

```text
- vocabulary drift warnings
- compatibility mismatches
- deprecated alias usage
- invalidated semantic mappings
```

Dashboard は semantic meaning を独自決定しない。

---

## 24. CI mapping

CI fail 条件：

```text
- ready treated as authorized
- passed treated as completed
- projection_valid treated as source_valid
- incompatible cross-project semantic mapping
- reserved vocabulary misuse in critical scope
```

CI warn 条件：

```text
- deprecated alias usage
- vocabulary drift pending review
- semantic mapping replay incomplete
```

---

## 25. 禁止事項

以下を禁止する。

```text
- authorization / readiness / completion を同義扱いすること
- projection/source semantic を混同すること
- stale/superseded semantic を collapse すること
- project-local shorthand を federation semantic に昇格すること
- replay impossible semantic drift を許容すること
```

---

## 26. HLDocS feedback

本 normalization から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance vocabulary registry を formalize すべき
- normalized lifecycle/state semantic を持つべき
- reserved governance vocabulary を定義すべき
- cross-project semantic compatibility を formalize すべき
- vocabulary drift prevention を governance layer に含めるべき
```

---

## 27. 結論

governance vocabulary normalization は、SansaVRM federation における governance terminology を正規化する model である。

これにより、checkpoint / cleanup / projection / audit / provenance / federation response の semantic drift を防止し、cross-project replayable governance を維持できる。
