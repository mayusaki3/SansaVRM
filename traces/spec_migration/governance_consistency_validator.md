# governance consistency validator

## 1. 目的

本ドキュメントは、SansaVRM federation における governance consistency validator を定義する。

governance consistency validator は、vocabulary / package / checkpoint / audit / projection / response / provenance / cleanup / completion の governance artifacts 間の不整合を横断的に検出する validator である。

---

## 2. 基本方針

governance consistency validator は以下を扱う。

```text
- vocabulary consistency
- package consistency
- checkpoint consistency
- audit consistency
- projection/source consistency
- response consistency
- provenance consistency
- cleanup/completion consistency
```

governance consistency validator は以下を行わない。

```text
- governance decision を自動承認しない
- dashboard projection を source of truth としない
- legal rights decision を確定しない
- inconsistent state を warning のみで通過させない
```

---

## 3. validator positioning

本 validator は以下に属する。

```text
Federated Governance Layer
Validation Layer
Cross-Project Compatibility Layer
```

risk_guard_validator の後段または一部として実行できる。

---

## 4. validation targets

validation targets：

```text
governance vocabulary registry
governance package reports
execution checkpoint reports
operational audit reports
dashboard projection reports
cross-project response reports
provenance graph reports
cleanup authorization reports
reconstruction completion reports
```

---

## 5. consistency dimensions

consistency dimensions：

```text
semantic_consistency
state_consistency
lifecycle_consistency
scope_consistency
severity_consistency
source_ref_consistency
replay_consistency
compatibility_consistency
```

---

## 6. vocabulary consistency

検査：

```text
- reserved vocabulary misuse
- readiness / authorization / completion collapse
- projection_valid / source_valid collapse
- stale / superseded semantic collapse
- cross-project vocabulary mismatch
```

Fail / blocked 条件：

```text
- ready treated as authorized
- passed treated as completed
- projection_valid treated as source_valid
```

---

## 7. package consistency

検査：

```text
- package dependency unresolved
- incompatible package mix
- invalidated package active
- cyclic dependency
- replay-incompatible package baseline
```

Fail / blocked 条件：

```text
- hard dependency unresolved
- invalidated package used in federation baseline
- incompatible package mix in active baseline
```

---

## 8. checkpoint consistency

検査：

```text
- checkpoint progression without evidence freeze
- invalidated checkpoint treated as passed
- superseded checkpoint used as authorization source
- rollback checkpoint missing after rollback_required
```

Fail / blocked 条件：

```text
- stale checkpoint used as active checkpoint
- completion checkpoint without audit trail
- cleanup checkpoint without comparison checkpoint
```

---

## 9. audit consistency

検査：

```text
- stale audit used as active evidence
- audit/source mismatch
- replay evidence incomplete
- override without audit
- rollback without audit
```

Fail / blocked 条件：

```text
- override without audit trail
- rollback without rollback audit
- privacy/security boundary violation
```

---

## 10. projection/source consistency

検査：

```text
- stale projection used as authorization source
- projection/source mismatch
- projection showing completed while source says pending
- dashboard projection deciding governance state
```

Fail / blocked 条件：

```text
- stale projection used as authorization source
- reconstruction_completed shown from stale projection
- projection used as source of truth
```

---

## 11. response consistency

検査：

```text
- stale response active
- unresolved acknowledgment ignored
- isolation boundary violation
- response/source mismatch
- federation_breaking ignored
```

Fail / blocked 条件：

```text
- federation_breaking ignored in affected scope
- isolation boundary violation
- unresolved acknowledgment in critical scope
```

---

## 12. provenance consistency

検査：

```text
- temporary bridge used as canonical provenance
- superseded provenance used as active
- unresolved provenance chain in distribution scope
- restriction conflict unresolved
- editor attribution missing in required scope
```

Fail / blocked 条件：

```text
- distribution_ready_overissued
- temporary bridge used as source of truth
- unknown_origin used as cleanup evidence
```

---

## 13. cleanup/completion consistency

検査：

```text
- cleanup_ready treated as cleanup_execution_authorized
- cleanup_completed without post-validation
- reconstruction_completed without legacy detachment
- completion review superseded but still active
```

Fail / blocked 条件：

```text
- cleanup execution without authorization
- reconstruction_completed with unresolved blocker
- new_structure_generated treated as completed
```

---

## 14. consistency report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_consistency_report",
  "validator_module": "governance_consistency_validator",
  "status": "blocked",
  "consistency_dimensions": [],
  "findings": [],
  "source_of_truth_refs": []
}
```

---

## 15. reason codes

候補 reason code：

```text
ready_treated_as_authorized
passed_treated_as_completed
projection_valid_treated_as_source_valid
invalidated_package_active
incompatible_package_mix
checkpoint_without_evidence_freeze
stale_checkpoint_used_as_active
stale_audit_used_as_active
projection_used_as_source_of_truth
federation_breaking_ignored
isolation_boundary_violation
temporary_bridge_used_as_source_of_truth
distribution_ready_overissued
cleanup_ready_used_as_authorized
completion_without_legacy_detachment
```

---

## 16. severity mapping

Blocked 条件：

```text
- semantic collapse in critical scope
- invalidated artifact used as active evidence
- authorization/completion overissued
- isolation boundary violation
- privacy/security boundary violation
```

Warn 条件：

```text
- deprecated alias usage outside critical scope
- stale projection outside authorization scope
- replay evidence incomplete outside completion scope
```

---

## 17. orchestration relation

federation execution orchestration は、本 validator の fail / blocked を受けた場合、次 stage へ進めてはならない。

特に以下を block する。

```text
- cleanup execution
- authorization
- completion review
- production rollout
```

---

## 18. CI mapping

CI fail 条件：

```text
- governance consistency status fail / blocked
- semantic collapse in critical scope
- active baseline has incompatible package mix
- active checkpoint is stale / invalidated
- projection/source mismatch in authorization scope
```

CI warn 条件：

```text
- non-critical replay evidence incomplete
- deprecated vocabulary alias
- stale dashboard outside authorization scope
```

---

## 19. dashboard relation

Dashboard は consistency findings を表示できる。

表示対象：

```text
- consistency status
- semantic collapse findings
- active invalidated evidence
- projection/source mismatch
- package compatibility mismatch
```

Dashboard は consistency finding を独自修正しない。

---

## 20. 禁止事項

以下を禁止する。

```text
- consistency validator fail を warning のみで通過させること
- semantic collapse を known limitation として authorization へ進めること
- invalidated active evidence を projection 側で隠すこと
- package incompatibility を local-only issue として扱うこと
```

---

## 21. HLDocS feedback

本 validator から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance consistency validator を formalize すべき
- vocabulary / package / checkpoint / audit / projection の横断整合を検査すべき
- semantic collapse detection を validator reason code 化すべき
- consistency fail は cleanup / authorization / completion を block すべき
```

---

## 22. 結論

governance consistency validator は、SansaVRM federation governance artifacts 間の不整合を横断的に検出する validator である。

これにより、ready/authorized/completed の semantic collapse、projection/source 混同、invalidated evidence 使用、package incompatibility を検出し、unsafe federation progression を防止できる。
