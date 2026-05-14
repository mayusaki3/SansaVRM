# reconstruction delta handling

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成中に発生する要件追加・方針変更・cross-project feedback の取り扱いを定義する。

reconstruction 中の変更要求は、通常運用の patch / hotfix / maintenance update とは分離して扱う。

本ドキュメントでは、これらを reconstruction delta と呼ぶ。

---

## 2. 基本方針

reconstruction delta handling は以下を行う。

```text
- reconstruction delta を通常 patch と分離する
- reconstruction scope への影響範囲を特定する
- freeze 前 / freeze 後で扱いを分離する
- superseded execution を管理する
- reconstruction delta traceability を保持する
- cross-project feedback を reconstruction delta として統合する
- cleanup_ready 前に再 validation を要求する
```

reconstruction delta handling は以下を行わない。

```text
- reconstruction delta を自動適用しない
- validator result を無視して execution を継続しない
- cleanup_ready を維持したまま semantic delta を適用しない
- reconstruction delta を通常 maintenance patch と誤分類しない
```

---

## 3. reconstruction delta 定義

reconstruction delta とは以下を指す。

```text
- reconstruction 中の要件追加
- semantic rule change
- canonicalization policy change
- validation policy change
- traceability policy change
- sec_id policy change
- cleanup policy change
- cross-project feedback による構造変更
- external artifact schema change
```

通常 patch は reconstruction delta に含めない。

---

## 4. reconstruction phase と通常運用の違い

通常運用：

```text
stable canonical identity
↓
localized patch
↓
limited validation
↓
release
```

reconstruction phase：

```text
canonical identity rebuilding
↓
migration / rewrite / validation graph 更新
↓
cross-layer revalidation
↓
cleanup gate reevaluation
```

reconstruction phase では localized patch 前提にしてはならない。

---

## 5. reconstruction delta taxonomy

reconstruction delta は以下に分類する。

```text
semantic_delta
representation_delta
validation_delta
traceability_delta
execution_delta
cleanup_delta
cross_project_delta
external_artifact_delta
```

### semantic_delta

Core semantic identity に影響する変更。

### representation_delta

representation のみを変更する変更。

### validation_delta

validator rule / validator scope の変更。

### traceability_delta

traceability / sec_id / testspec mapping の変更。

### execution_delta

execution lifecycle / rollback / checkpoint の変更。

### cleanup_delta

cleanup gate / cleanup execution の変更。

### cross_project_delta

MuJoCo Adapter / Studio AI 等からの feedback。

### external_artifact_delta

external artifact schema / fixture / freshness rule の変更。

---

## 6. freeze 前 delta

scope freeze 前の reconstruction delta は active planning scope に統合可能。

条件：

```text
- execution batch planning 前
- checkpoint creation 前
- rewrite transaction freeze 前
- cleanup scope freeze 前
```

freeze 前 delta の処理：

```text
- orchestration graph を更新する
- validator scope を更新する
- rewrite transaction plan を更新する
- dashboard projection を更新する
- affected dependency hash を更新する
```

---

## 7. freeze 後 delta

scope freeze 後の reconstruction delta は、現在 execution に直接混入してはならない。

freeze 後 delta の処理：

```text
- current execution を superseded 候補にする
- delta impact analysis を行う
- new execution scope を生成する
- new canonicalization_id / execution_id を発行する
- validator rerun scope を決定する
- cleanup_ready を再評価する
```

freeze 後 delta を既存 apply execution に混入してはならない。

---

## 8. superseded execution

reconstruction delta により置換された execution は superseded とする。

superseded 条件：

```text
- canonicalization policy changed
- rewrite transaction ordering changed
- semantic integrity rule changed
- traceability rewrite changed
- cleanup gate dependency changed
- cross-project artifact dependency changed
```

superseded execution は cleanup_ready 判定に使用してはならない。

---

## 9. delta impact analysis

reconstruction delta 発生時は impact analysis を行う。

対象：

```text
- affected canonical_doc_ids
- affected rewrite transactions
- affected validator scopes
- affected traceability links
- affected sec_id mappings
- affected cleanup scopes
- affected external artifacts
- affected cross-project handoffs
```

impact analysis は validator orchestration と dashboard projection に渡す。

---

## 10. reconstruction delta traceability

reconstruction delta は traceable でなければならない。

記録対象：

```text
- reconstruction_delta_id
- delta_kind
- source_project
- source_feedback_reference
- affected_scope
- superseded_execution_ids
- replacement_execution_ids
- validator_rerun_scope
- cleanup_impact
```

reconstruction delta を undocumented state change として扱ってはならない。

---

## 11. cross-project feedback handling

MuJoCo Adapter / Studio AI 等からの feedback は reconstruction delta として扱う。

対象例：

```text
- adapter JSON schema change
- diagnostics classification change
- updated_extension_properties policy
- export profile requirement change
- validation dependency change
```

cross-project feedback を通常 patch として apply してはならない。

---

## 12. cleanup_ready との関係

reconstruction delta 発生後は cleanup_ready を再評価する。

以下の場合は cleanup_ready を無効化する。

```text
- semantic_delta
- validation_delta
- traceability_delta
- cleanup_delta
- cross_project_delta affecting cleanup scope
```

representation_delta のみで semantic_integrity=pass の場合は、cleanup_ready を維持できる場合がある。

---

## 13. validator rerun policy

reconstruction delta 後は validator rerun を行う。

rerun scope：

```text
semantic_delta:
full canonicalization / rewrite / cleanup validation

representation_delta:
representation / integrity validation

validation_delta:
validator orchestration rerun

traceability_delta:
traceability / rewrite rerun

cleanup_delta:
cleanup gate rerun

cross_project_delta:
external artifact / federation rerun
```

rerun scope は impact analysis に基づいて決定する。

---

## 14. execution status との関係

reconstruction delta により execution status は変化する。

例：

```text
approved
→ superseded

executing
→ blocked / rollback_requested

validator_handoff_ready
→ rerun_required

cleanup_ready
→ cleanup_pending / cleanup_blocked
```

reconstruction delta 発生後に stale execution を completed と扱ってはならない。

---

## 15. report schema draft

```json
{
  "schema_version": "1.0",
  "reconstruction_delta_id": "reconstruction-delta-YYYYMMDD-NNN",
  "delta_kind": "cross_project_delta",
  "source_project": "SansaVRM-MuJoCo-Adapter",
  "affected_scope": {
    "canonical_doc_ids": [],
    "rewrite_transactions": [],
    "cleanup_scopes": []
  },
  "superseded_execution_ids": [],
  "replacement_execution_ids": [],
  "validator_rerun_scope": "full_cleanup_validation",
  "cleanup_impact": "cleanup_pending"
}
```

---

## 16. CI integration

CI では reconstruction delta を検出した場合、以下を行う。

```text
- stale execution detection
- rerun_required marking
- cleanup_ready invalidation
- dependency hash update
- dry-run execution rerun
```

CI は reconstruction delta を通常 patch として扱ってはならない。

---

## 17. dashboard との関係

Dashboard は reconstruction delta を表示する。

表示対象：

```text
- reconstruction_delta_id
- delta_kind
- superseded executions
- rerun_required state
- cleanup_ready invalidation
- affected cross-project artifacts
- affected validator scopes
```

Dashboard は reconstruction delta を apply しない。

---

## 18. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction phase の変更要求は通常 patch と分離すべき
- freeze 後の reconstruction delta は current execution に直接混入してはならない
- reconstruction delta は traceable な execution replacement として扱うべき
- cleanup_ready は reconstruction delta 発生後に再評価すべき
- cross-project feedback は reconstruction delta として validator rerun を要求すべき
```

---

## 19. 禁止事項

以下を禁止する。

```text
- freeze 後 delta を既存 apply execution に直接混入すること
- superseded execution を cleanup_ready 判定に使用すること
- reconstruction delta を undocumented patch として適用すること
- semantic_delta 発生後に validator rerun を省略すること
- cross-project delta を localized patch として扱うこと
```

---

## 20. 結論

reconstruction delta handling は、再構築中に発生する要件追加・方針変更・cross-project feedback を、通常運用 patch と分離して扱う execution / validation model である。

これにより、freeze 後 delta の unsafe mixing を防ぎ、superseded execution、rerun_required、cleanup_ready reevaluation を traceable に管理できる。
