# federation dashboard projection refinement

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation dashboard projection refinement を定義する。

federation dashboard projection は、validator / comparison / cleanup / provenance / completion / orchestration の状態を operational view として可視化する projection layer を扱う。

重要：

```text
Dashboard projection
```

は：

```text
source of truth
```

ではない。

---

## 2. 基本方針

federation dashboard projection は以下を扱う。

```text
- operational projection
- summarized reporting
- orchestration visibility
- provenance visibility
- cleanup visibility
- completion visibility
- projection invalidation
- stale projection handling
```

federation dashboard projection は以下を行わない。

```text
- source of truth を置き換えない
- governance approval を自動決定しない
- validator result を上書きしない
- stale projection を active evidence として扱わない
```

---

## 3. projection positioning

Dashboard projection は以下に属する。

```text
Operational Projection Layer
Operational Reporting Layer
```

Core Semantic Layer ではない。

---

## 4. projection sources

Projection source 候補：

```text
validator reports
comparison reports
cleanup reports
provenance graph reports
completion review reports
orchestration reports
batch execution reports
```

projection は source reports の projection に過ぎない。

---

## 5. projection categories

Projection category：

```text
validator_projection
comparison_projection
cleanup_projection
provenance_projection
completion_projection
orchestration_projection
batch_projection
risk_projection
```

---

## 6. summarized reporting

Dashboard は summarized reporting を行う。

summary 対象：

```text
- blocker summary
- rerun summary
- cleanup summary
- provenance issue summary
- restriction conflict summary
- rollback summary
- completion readiness summary
```

per-document detail は source artifact 側に保持する。

---

## 7. projection lifecycle

projection lifecycle：

```text
projection_generated
projection_stale
projection_superseded
projection_invalidated
projection_regenerated
projection_archived
```

stale / superseded projection を active operational evidence として扱ってはならない。

---

## 8. projection invalidation

以下は projection invalidation を発生させる。

```text
- validator rerun
- comparison rerun
- cleanup execution
- provenance graph change
- orchestration stage change
- completion review change
- governance automation policy change
```

projection invalidation 後は regeneration required。

---

## 9. stale projection handling

stale projection は以下を持つ。

```text
- stale_reason
- stale_since
- superseded_by
- regeneration_required
```

stale projection を authorization source に使用してはならない。

---

## 10. orchestration projection

表示対象：

```text
- current orchestration stage
- completed stages
- blocked stages
- rerun_required
- authorization status
- completion readiness
```

Dashboard は orchestration state を直接変更しない。

---

## 11. cleanup projection

表示対象：

```text
- cleanup readiness
- cleanup blockers
- cleanup authorization
- cleanup execution status
- rollback status
- post-validation status
```

cleanup projection は cleanup execution の代替ではない。

---

## 12. provenance projection

表示対象：

```text
- provenance graph status
- unresolved provenance count
- restriction conflict count
- editor attribution status
- tool provenance status
- distribution readiness impact
```

Dashboard は legal clearance を決定しない。

---

## 13. completion projection

表示対象：

```text
- completion stage
- comparison completion
- cleanup completion
- post-validation completion
- legacy detachment status
- completion blockers
```

new_structure_generated を reconstruction_completed と誤表示してはならない。

---

## 14. batch projection

表示対象：

```text
- batch execution scope
- checkpoint status
- grouped rerun status
- rollback readiness
- automation status
```

batch projection は execution authorization を意味しない。

---

## 15. projection consistency

Projection consistency 検査：

```text
- stale projection not active
- superseded projection not active
- projection source refs resolved
- projection/source mismatch detection
```

projection/source mismatch unresolved は warning または blocker とする。

---

## 16. projection/source separation

重要：

```text
projection
```

と：

```text
source artifact
```

を混同してはならない。

例：

```text
Dashboard says cleanup_ready
```

でも：

```text
cleanup_authorization_report missing
```

なら execution 不可。

---

## 17. projection regeneration

projection regeneration 条件：

```text
- source artifact changed
- projection invalidated
- stale threshold exceeded
- rerun completed
- cleanup completed
- completion review updated
```

regeneration failure は stale projection を残す。

---

## 18. projection audit trail

Projection operation を audit する。

記録：

```text
- projection source refs
- generation timestamp
- invalidation reason
- regeneration reason
- projection version
```

projection audit missing は operational warning とする。

---

## 19. projection report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "dashboard_projection_report",
  "projection_status": "projection_stale",
  "projection_categories": [],
  "source_report_refs": [],
  "stale_reason": null,
  "regeneration_required": true
}
```

---

## 20. CI mapping

CI fail 条件：

```text
- stale projection used as authorization source
- superseded projection treated as active evidence
- projection/source mismatch unresolved in critical scope
- reconstruction_completed shown from stale projection
```

CI warn 条件：

```text
- projection regeneration pending
- projection audit missing
- projection stale outside critical scope
```

---

## 21. 禁止事項

以下を禁止する。

```text
- Dashboard projection を source of truth と扱うこと
- stale projection を active operational evidence と扱うこと
- projection mismatch を silent ignore すること
- projection が authorization を決定すること
- new_structure_generated を completion projection で completed 扱いすること
```

---

## 22. HLDocS feedback

本 refinement から、HLDocS 側へ以下をフィードバック候補とする。

```text
- operational projection layer を formalize すべき
- projection/source separation を formalize すべき
- stale/superseded projection lifecycle を持つべき
- summarized operational reporting を projection layer で扱うべき
- projection invalidation / regeneration governance を formalize すべき
```

---

## 23. 結論

federation dashboard projection refinement は、SansaVRM 再構成における operational dashboard projection governance を定義する refinement である。

これにより、validator / comparison / cleanup / provenance / completion の operational visibility を提供しつつ、projection と source of truth を厳密に分離できる。
