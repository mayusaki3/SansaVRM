# governance license reconciliation model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance license reconciliation model を定義する。

license reconciliation model は、VN3 / VRM / CC / Booth terms / 独自ライセンスなどが混在する asset / component / assembly / distribution において、license semantic mapping、restriction merge、conflict escalation、distribution authorization を replayable に扱う。

---

## 2. 基本方針

本 model は以下を扱う。

```text
- license taxonomy
- license semantic mapping
- license compatibility matrix
- license reconciliation lifecycle
- license conflict taxonomy
- license authority
- license replayability
- cross-project license synchronization
```

本 model は以下を行わない。

```text
- 自然言語 license を完全自動解釈済みとして扱わない
- unknown license semantic を allow としない
- unresolved license conflict を warning のみで distribution しない
- projection license summary を canonical license source と扱わない
```

---

## 3. license reconciliation positioning

license reconciliation は以下に属する。

```text
License Governance Layer
Policy Governance Layer
Restriction Governance Layer
Distribution Governance Layer
Operational Traceability Layer
```

license reconciliation は license text と policy expression の間の semantic reconciliation を扱う。

---

## 4. license taxonomy

license taxonomy：

```text
vn3_license
vrm_license
cc_license
booth_terms
custom_license
internal_license
unknown_license
```

unknown_license は allow ではない。

---

## 5. VN3 license

VN3 license では以下を reconciliation 対象とする。

```text
commercial use
corporate use
redistribution
modification
R18 use
political / religious use
AI training
credit / attribution
```

---

## 6. license semantic mapping

semantic mapping は license text を policy expression へ接続する。

必要：

```text
- source license refs
- license snapshot refs
- semantic mapping refs
- policy expression refs
- ambiguity refs
```

silent semantic mapping を禁止する。

---

## 7. license compatibility matrix

license compatibility matrix は以下を扱う。

```text
VN3 ↔ VRM
VN3 ↔ CC
VN3 ↔ Booth terms
custom ↔ VN3
custom ↔ distribution policy
```

compatibility ambiguity は review_required または blocker。

---

## 8. reconciliation lifecycle

reconciliation lifecycle：

```text
reconciliation_pending
reconciliation_review_required
reconciliation_active
reconciliation_conflicted
reconciliation_invalidated
reconciliation_superseded
reconciliation_archived
```

---

## 9. license conflict taxonomy

conflict taxonomy：

```text
restriction_conflict
commercial_conflict
redistribution_conflict
modification_conflict
attribution_conflict
ai_training_conflict
jurisdiction_conflict
source_snapshot_conflict
```

---

## 10. restriction merge policy

restriction merge policy 候補：

```text
most_restrictive_wins
explicit_deny_wins
conflict_requires_review
unknown_requires_review
```

silent restriction weakening を禁止する。

---

## 11. reconciliation result semantics

result taxonomy：

```text
compatible
compatible_with_conditions
review_required
conflicted
incompatible
unknown
```

unknown を compatible として扱ってはならない。

---

## 12. license authority

license authority：

```text
license_authority
policy_authority
restriction_authority
distribution_authority
review_authority
```

Authority ambiguity は review_required または blocker。

---

## 13. license replayability

replayability 条件：

```text
- license refs recorded
- license snapshot refs recorded
- semantic mapping refs recorded
- reconciliation refs recorded
- authority refs recorded
- policy expression refs recorded
```

Replay 不可能 reconciliation は public/commercial distribution に使ってはならない。

---

## 14. source license snapshot

license snapshot は以下を持つ。

```text
- acquisition location
- acquisition timestamp
- license text snapshot
- license version where available
- source_raw refs
```

snapshot missing は review_required または blocker。

---

## 15. reconciliation invalidation

invalidation trigger：

```text
source license changed
license snapshot invalidated
semantic mapping changed
policy expression invalidated
compatibility matrix invalidated
rights holder changed
```

invalidated reconciliation を active distribution evidence として扱ってはならない。

---

## 16. license override

override 候補：

```text
manual legal review override
temporary preview waiver
emergency rollback redistribution waiver
```

override は audit / expiration / follow-up mandatory。

---

## 17. cross-project license synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
distribution pipeline
HLDocS
```

同期対象：

```text
license taxonomy
semantic mapping taxonomy
compatibility matrix
restriction merge policy
reconciliation result semantics
```

---

## 18. license report

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_license_reconciliation_report",
  "license_taxonomy": "vn3_license",
  "reconciliation_status": "reconciliation_review_required",
  "license_refs": [],
  "policy_expression_refs": [],
  "source_of_truth_refs": []
}
```

---

## 19. reason codes

```text
license_unknown_treated_as_allow
license_snapshot_missing
license_semantic_mapping_ambiguous
license_reconciliation_conflicted
license_replayability_missing
license_invalidated_but_active
license_cross_project_unsynchronized
license_override_without_audit
```

---

## 20. orchestration relation

federation execution orchestration は以下を block する。

```text
- unknown license treated as allow
- unresolved license conflict in distribution scope
- replayability missing in license reconciliation
- invalidated reconciliation active in public/commercial distribution
```

---

## 21. HLDocS feedback

```text
- license reconciliation model を formalize すべき
- natural language license と machine-readable policy expression を分離すべき
- license compatibility matrix を governance artifact 化すべき
- VN3 / VRM / CC / Booth terms 混在を reconciliation 対象に含めるべき
```

---

## 22. 結論

governance license reconciliation model は、VN3を含む複数 license / terms の混在を replayable に reconciliation する model である。

これにより、license text、policy expression、restriction merge、distribution authorization を安全に接続できる。
