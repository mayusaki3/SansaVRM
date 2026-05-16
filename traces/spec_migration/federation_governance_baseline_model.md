# federation governance baseline model

## 1. 目的

本ドキュメントは、SansaVRM federation における federation governance baseline model を定義する。

federation governance baseline model は、vocabulary / package / freeze / replay / compatibility / response / audit を federation-wide baseline として成立させる条件を扱う。

---

## 2. 基本方針

federation governance baseline model は以下を扱う。

```text
- baseline composition
- baseline dependency
- baseline stability
- baseline compatibility
- baseline replayability
- baseline freeze relation
- baseline drift / migration handling
- baseline rollout / rollback
```

federation governance baseline model は以下を行わない。

```text
- local draft governance を federation baseline と扱わない
- unstable package mix を federation stable と扱わない
- replay-incomplete baseline を production baseline と扱わない
- unsynchronized cross-project baseline を active federation baseline と扱わない
```

---

## 3. baseline positioning

federation governance baseline は以下に属する。

```text
Federated Governance Layer
Release Governance Layer
Cross-Project Compatibility Layer
Operational Traceability Layer
```

baseline は source artifact ではなく、governance package / semantic / compatibility の approved set である。

---

## 4. baseline composition

baseline は以下で構成する。

```text
- governance package set
- vocabulary registry version
- compatibility matrix version
- replay baseline version
- semantic freeze baseline
- audit policy version
- response policy version
```

すべて replayable でなければならない。

---

## 5. baseline kinds

baseline kind：

```text
local_baseline
project_baseline
preview_federation_baseline
production_federation_baseline
distribution_baseline
```

baseline kind により stability / rollout / CI 条件が変わる。

---

## 6. local baseline

local_baseline は project-local governance 検証用 baseline である。

特徴：

```text
- project local only
- cross-project propagation optional
- production rollout 不可
```

---

## 7. project baseline

project_baseline は単一 project で有効な governance baseline である。

特徴：

```text
- project release candidate
- cross-project compatibility review 対象
- federation baseline ではない
```

---

## 8. preview federation baseline

preview_federation_baseline は限定 federation で検証する baseline である。

特徴：

```text
- federation preview scope
- stabilization required
- production rollout 前段
```

---

## 9. production federation baseline

production_federation_baseline は federation-wide operational baseline である。

必要条件：

```text
- package compatibility verified
- replayability verified
- semantic freeze satisfied
- cross-project synchronization completed
- no unresolved critical blocker
```

---

## 10. distribution baseline

distribution_baseline は distribution readiness を伴う baseline である。

追加条件：

```text
- distribution-sensitive governance resolved
- provenance / restriction governance reviewed
- security / privacy boundary satisfied
- release audit completed
```

---

## 11. baseline dependency

baseline dependency は package dependency と cross-project dependency を含む。

依存例：

```text
production_federation_baseline
↓ depends on
Core Governance Package
Operational Governance Package
Federation Governance Package
Compatibility Governance Package
```

hard dependency unresolved は baseline blocker。

---

## 12. baseline compatibility

baseline compatibility は以下を検査する。

```text
- package compatibility
- vocabulary compatibility
- replay compatibility
- migration compatibility
- cross-project compatibility
```

compatibility ambiguity は review_required または blocker。

---

## 13. baseline stability

baseline stability：

```text
unstable
stabilizing
preview_stable
federation_stable
production_stable
```

production_stable は release / distribution に使える候補である。

---

## 14. baseline replayability

baseline replayability 条件：

```text
- package set version recorded
- vocabulary version recorded
- compatibility matrix version recorded
- freeze baseline refs recorded
- replay validator pass
- audit policy version recorded
```

replayability missing は production baseline blocker。

---

## 15. baseline freeze relation

production / distribution baseline は semantic freeze と接続される。

必要：

```text
- freeze baseline exists
- freeze boundary resolved
- freeze violations resolved
- freeze exceptions audited
- freeze synchronization completed
```

---

## 16. baseline drift handling

baseline drift は governance drift detection と接続する。

以下は baseline invalidation 候補：

```text
- semantic drift
- replay-breaking drift
- package compatibility drift
- unsynchronized cross-project drift
- semantic freeze violation
```

---

## 17. baseline migration handling

baseline migration は governance migration compatibility model と接続する。

必要：

```text
- migration compatibility classified
- bridge lifecycle recorded where required
- migration debt visible
- rollback verified
- cross-project migration synchronized
```

---

## 18. baseline rollout

baseline rollout path：

```text
local_baseline
↓
project_baseline
↓
preview_federation_baseline
↓
production_federation_baseline
↓
distribution_baseline
```

各 stage は gate と audit を要求する。

---

## 19. baseline rollback

baseline rollback 対象：

```text
- package set
- compatibility matrix
- semantic freeze baseline
- replay baseline
- response policy
```

rollback verification missing は federation blocker。

---

## 20. baseline lifecycle

baseline lifecycle：

```text
baseline_draft
baseline_review_required
baseline_preview
baseline_approved
baseline_active
baseline_stale
baseline_invalidated
baseline_superseded
baseline_rolled_back
baseline_archived
```

invalidated baseline を active federation baseline として扱ってはならない。

---

## 21. baseline report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "federation_governance_baseline_report",
  "baseline_kind": "preview_federation_baseline",
  "baseline_status": "baseline_review_required",
  "package_set_refs": [],
  "vocabulary_refs": [],
  "compatibility_refs": [],
  "freeze_refs": [],
  "replay_refs": []
}
```

---

## 22. reason codes

候補 reason code：

```text
baseline_dependency_unresolved
baseline_compatibility_ambiguous
baseline_replayability_missing
baseline_freeze_missing
baseline_drift_detected
baseline_migration_required
baseline_cross_project_unsynchronized
baseline_invalidated_but_active
baseline_rollback_unverified
```

---

## 23. orchestration relation

federation execution orchestration は以下を block する。

```text
- invalidated baseline active
- production rollout without replayable baseline
- distribution rollout without distribution baseline
- unsynchronized baseline in federation scope
```

---

## 24. dashboard relation

Dashboard は baseline summary を表示できる。

表示対象：

```text
- active baseline kind
- baseline lifecycle status
- compatibility status
- replayability status
- freeze status
- rollout status
- rollback status
```

Dashboard は baseline approval を独自決定しない。

---

## 25. CI mapping

CI fail 条件：

```text
- invalidated baseline used as active
- production federation baseline without replayability
- distribution baseline without freeze satisfaction
- incompatible package mix in active baseline
- unsynchronized cross-project baseline in production scope
```

CI warn 条件：

```text
- baseline_review_required
- preview baseline stabilization pending
- migration debt exists outside production scope
```

---

## 26. 禁止事項

以下を禁止する。

```text
- local_baseline を federation baseline と扱うこと
- project_baseline を production_federation_baseline と扱うこと
- replay-incomplete baseline を production rollout に使うこと
- invalidated baseline を active と扱うこと
- dashboard projection を baseline source と扱うこと
```

---

## 27. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- governance baseline model を formalize すべき
- baseline kind / lifecycle / rollout を分離すべき
- baseline replayability を release governance に含めるべき
- semantic freeze と baseline を接続すべき
- cross-project baseline synchronization を扱うべき
```

---

## 28. 結論

federation governance baseline model は、SansaVRM federation における governance semantic / package / replay / compatibility の approved baseline を定義する model である。

これにより、local / project / preview / production / distribution baseline を分離し、replayability・freeze・compatibility・cross-project synchronization を満たす baseline のみを federation progression に使用できる。
