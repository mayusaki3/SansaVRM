# reconstruction completion criteria refinement

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における reconstruction completion criteria を精緻化する。

重要：

```text
new structure generated
```

は reconstruction completed ではない。

reconstruction completed は、comparison、cleanup readiness、cleanup execution where required、post-execution validation、audit trail、federation dependency resolution を経て成立する。

---

## 2. 基本方針

reconstruction completion criteria は以下を扱う。

```text
- completion stage
- completion evidence
- comparison completion
- cleanup completion
- legacy detachment
- federation dependency resolution
- provenance / restriction resolution
- audit trail completion
```

reconstruction completion criteria は以下を行わない。

```text
- new structure generated を completed とみなさない
- cleanup_ready を cleanup_completed とみなさない
- comparison completed を cleanup completed とみなさない
- dashboard projection を source of truth とみなさない
```

---

## 3. completion stages

completion stages は以下とする。

```text
not_started
new_structure_generated
comparison_required
comparison_completed
cleanup_candidate_generated
cleanup_ready
cleanup_execution_authorized
cleanup_completed
post_validation_completed
completion_review_required
reconstruction_completed
completion_rejected
completion_superseded
```

---

## 4. new_structure_generated

`new_structure_generated` は以下を示す。

```text
- new documents / indexes / schemas / artifacts が生成された
- validator 対象になった
- comparison の準備ができた
```

ただし、以下を意味しない。

```text
- semantic equivalence confirmed
- traceability equivalence confirmed
- cleanup approved
- old structure removed
- reconstruction completed
```

---

## 5. comparison_completed

comparison_completed 条件：

```text
- comparison scope frozen
- semantic comparison completed
- traceability comparison completed
- rewrite completeness comparison completed
- orphan detection completed
- contamination detection completed
- cleanup blockers generated
```

comparison_completed は cleanup_completed ではない。

---

## 6. cleanup_candidate_generated

cleanup_candidate_generated 条件：

```text
- comparison evidence exists
- cleanup blockers classified
- old structure cleanup candidates identified
- rollback impact analyzed
- federation dependency impact analyzed
```

cleanup_candidate は cleanup approval ではない。

---

## 7. cleanup_ready

cleanup_ready 条件：

```text
- cleanup blockers resolved
- rollback recoverability confirmed
- cleanup hold cleared
- required validators pass or allowed warn
- federation-wide cleanup safety confirmed
```

cleanup_ready は cleanup_execution_authorized ではない。

---

## 8. cleanup_execution_authorized

cleanup_execution_authorized 条件：

```text
- cleanup_ready true
- cleanup scope frozen
- irreversible cleanup review completed where applicable
- authorization report generated
- rollback restore point verified
```

---

## 9. cleanup_completed

cleanup_completed 条件：

```text
- authorized cleanup executed
- cleanup report generated
- old structure detached / archived / removed as planned
- cleanup rollback not required
- cleanup artifact lifecycle updated
```

cleanup_completed 後でも post-validation が必要である。

---

## 10. post_validation_completed

post_validation_completed 条件：

```text
- post-cleanup validator pass
- reference integrity confirmed
- traceability integrity confirmed
- provenance graph integrity confirmed where applicable
- no stale active artifact
- dashboard projection regenerated
```

post-validation fail の場合、reconstruction_completed にしてはならない。

---

## 11. federation dependency resolution

completion 前に以下を確認する。

```text
- no unresolved handoff dependency
- no unresolved external artifact dependency
- no unresolved downstream action request
- no unresolved federation cleanup blocker
- no stale required external artifact
```

unresolved federation dependency がある場合、completion_review_required または completion_rejected とする。

---

## 12. provenance / restriction resolution

provenance / restriction が scope に含まれる場合、以下を確認する。

```text
- no unresolved provenance chain in cleanup / distribution scope
- no unresolved restriction conflict in cleanup / distribution scope
- editor attribution continuity where required
- temporary provenance bridge not used as source of truth
- provenance comparison completed
```

---

## 13. legacy detachment

legacy detachment は旧構成が active canonical evidence から外れた状態である。

条件：

```text
- old structure no longer active canonical source
- legacy alias policy applied where needed
- old references removed or bridged intentionally
- deprecated / obsolete state recorded
- archive / retention policy recorded
```

legacy detachment なしに reconstruction_completed としてはならない。

---

## 14. completion evidence

completion evidence として以下を保持する。

```text
- comparison_report
- cleanup_candidate_report
- cleanup_authorization_report
- cleanup_execution_report
- post_validation_report
- legacy_detachment_report
- completion_review_report
```

completion evidence missing は completion blocker とする。

---

## 15. completion review report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "reconstruction_completion_review_report",
  "completion_stage": "completion_review_required",
  "reconstruction_completed": false,
  "comparison_completed": false,
  "cleanup_completed": false,
  "post_validation_completed": false,
  "legacy_detached": false,
  "completion_blockers": [],
  "source_of_truth_refs": []
}
```

---

## 16. completion blockers

completion blockers は以下とする。

```text
comparison_blocker
cleanup_blocker
post_validation_blocker
federation_blocker
provenance_blocker
restriction_blocker
legacy_detachment_blocker
audit_blocker
```

completion blocker unresolved のまま reconstruction_completed にしてはならない。

---

## 17. completion_superseded

completion review は以下の場合 superseded となる。

```text
- reconstruction delta が発生した
- comparison criteria が変更された
- cleanup criteria が変更された
- validator taxonomy が変更された
- post-validation report が stale になった
```

completion_superseded は再 review を要求する。

---

## 18. dashboard display

Dashboard は completion status を表示する。

表示対象：

```text
- completion stage
- comparison status
- cleanup status
- post-validation status
- legacy detachment status
- completion blockers
- superseded reason
```

Dashboard は completion を独自決定しない。

---

## 19. CI mapping

CI fail 条件：

```text
- reconstruction_completed without comparison evidence
- reconstruction_completed without cleanup evidence where required
- reconstruction_completed with unresolved blocker
- reconstruction_completed with stale active artifact
- reconstruction_completed without legacy detachment
- completion review superseded but still active
```

CI warn 条件：

```text
- completion_review_required
- cleanup_completed but post_validation pending
- legacy detachment pending outside release scope
```

---

## 20. 禁止事項

以下を禁止する。

```text
- new_structure_generated を reconstruction_completed と扱うこと
- comparison_completed を cleanup_completed と扱うこと
- cleanup_ready を cleanup_completed と扱うこと
- cleanup_completed を post_validation_completed と扱うこと
- legacy detachment なしに completion とすること
- completion blocker を warning のみで済ませること
```

---

## 21. HLDocS feedback

本 criteria から、HLDocS 側へ以下をフィードバック候補とする。

```text
- reconstruction completion は multi-stage criteria とすべき
- new structure generated と reconstruction completed を分離すべき
- comparison / cleanup / post-validation / legacy detachment を completion 条件に含めるべき
- completion evidence を artifact 化すべき
- completion review superseded を formal state とすべき
```

---

## 22. 結論

reconstruction completion criteria refinement は、SansaVRM 再構成における completion 判定を精緻化する criteria である。

これにより、新構成生成後に comparison、cleanup、post-validation、legacy detachment、audit trail を経て初めて reconstruction completed と扱える。
