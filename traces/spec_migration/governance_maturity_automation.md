# governance maturity automation

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における governance maturity automation を定義する。

governance maturity automation は、M0〜M3 reconstruction maturity に応じて、どの governance を automation 可能とし、どの governance を human-required に維持するかを整理する。

---

## 2. 基本方針

governance maturity automation は以下を扱う。

```text
- automation eligibility
- maturity promotion criteria
- automation-safe governance
- human-required governance
- automation rollback
- human override
- automation audit trail
- orchestration automation boundary
```

governance maturity automation は以下を行わない。

```text
- unstable governance を自動昇格しない
- restriction conflict resolution を自動化しない
- irreversible cleanup を unattended execution しない
- distribution-ready authorization を自動承認しない
- dashboard projection を source of truth として扱わない
```

---

## 3. maturity model

maturity は以下とする。

```text
M0 exploratory reconstruction
M1 structured reconstruction
M2 operational reconstruction
M3 production reconstruction
```

---

## 4. M0 exploratory reconstruction

特徴：

```text
- terminology unstable
- validator taxonomy unstable
- reconstruction lifecycle unstable
- cleanup semantics unstable
- provenance governance unstable
```

automation policy：

```text
- manual-first
- no unattended orchestration
- no grouped cleanup execution
- micro-step confirmation required
```

automation promotion は禁止。

---

## 5. M1 structured reconstruction

特徴：

```text
- validator taxonomy mostly stable
- lifecycle mostly stable
- comparison semantics mostly stable
- rollback semantics mostly stable
```

許可候補：

```text
- grouped validator rerun
- grouped registry regeneration
- grouped comparison rerun
- summarized reporting
```

ただし cleanup authorization は human-required を維持する。

---

## 6. M2 operational reconstruction

特徴：

```text
- orchestration stabilized
- propagation engine stabilized
- cleanup governance stabilized
- provenance validation stabilized
```

許可候補：

```text
- checkpoint automation
- orchestration-driven rerun
- grouped projection regeneration
- grouped stale artifact handling
```

ただし distribution-ready authorization は human-required を維持する。

---

## 7. M3 production reconstruction

特徴：

```text
- federation orchestration stabilized
- rollback orchestration stabilized
- audit model stabilized
- operational governance stabilized
```

許可候補：

```text
- orchestration-driven execution
- automated rerun scheduling
- automated stale invalidation
- automated propagation execution
```

ただし irreversible cleanup は review-required を維持する。

---

## 8. automation-safe governance

automation-safe 候補：

```text
- validator rerun
- registry regeneration
- projection regeneration
- stale artifact invalidation
- comparison rerun
- grouped dashboard regeneration
- stale cache cleanup
```

これらは orchestration automation 候補にできる。

---

## 9. human-required governance

human-required 候補：

```text
- restriction conflict resolution
- provenance ambiguity resolution
- irreversible cleanup approval
- distribution-ready authorization
- source_of_truth transition
- mixed reconstruction overwrite
- rollback boundary modification
```

これらは unattended automation 禁止。

---

## 10. automation eligibility

automation eligibility 条件：

```text
- validator taxonomy stable
- lifecycle stable
- rollback recoverability verified
- comparison criteria stable
- reason code taxonomy stable
- no unresolved blocker in execution scope
```

eligibility fail の場合 automation promotion 禁止。

---

## 11. automation promotion criteria

Maturity promotion 条件候補：

```text
- validator churn below threshold
- rerun success ratio above threshold
- rollback success ratio above threshold
- no unresolved critical contamination
- no unresolved critical provenance blocker
- checkpoint review stable over time
```

promotion evidence を保持する。

---

## 12. automation demotion

以下の場合 automation demotion を行う。

```text
- validator taxonomy churn increased
- repeated rollback failure
- repeated cleanup failure
- provenance corruption detected
- stale active artifact leak detected
- orchestration inconsistency detected
```

Demotion は manual governance を増やす。

---

## 13. automation rollback

automation policy 自体も rollback 対象とする。

例：

```text
checkpoint automation disabled
grouped rerun disabled
automated propagation disabled
```

automation rollback 条件：

```text
- unstable validator behavior
- unstable lifecycle behavior
- rollback failure trend
- unresolved governance ambiguity
```

---

## 14. human override governance

human override は automation decision を override できる。

例：

```text
automation_blocked → manual approval
checkpoint_required → emergency override
rerun_required → freeze override
```

ただし override は audit trail mandatory。

---

## 15. override restrictions

override 禁止候補：

```text
- unknown-as-pass
- unresolved provenance corruption
- unresolved restriction conflict in distribution scope
- missing rollback recoverability
- missing authorization evidence
```

critical governance violation は override 不可。

---

## 16. automation audit trail

automation decision は trace 可能でなければならない。

記録対象：

```text
- automation reason
- promotion reason
- demotion reason
- override reason
- automation policy version
- validator taxonomy version
- execution context
```

automation audit missing は governance warning または blocker とする。

---

## 17. automation confidence

automation confidence は以下を評価する。

```text
- validator stability
- rerun stability
- rollback success stability
- comparison consistency
- contamination recurrence
- provenance validation consistency
```

confidence low の場合 automation promotion 禁止。

---

## 18. automation invalidation

以下は automation invalidation を発生させる。

```text
- validator taxonomy changed
- comparison criteria changed
- provenance graph model changed
- cleanup lifecycle changed
- rollback semantics changed
- federation orchestration changed
```

automation invalidation 後は re-review required。

---

## 19. orchestration relation

governance maturity automation は federation execution orchestration と接続される。

対象：

```text
- grouped rerun
- checkpoint automation
- propagation automation
- cleanup execution gating
- completion review gating
```

orchestration automation は maturity-aware policy に従う。

---

## 20. provenance relation

provenance governance は automation-safe とは限らない。

特に：

```text
- restriction conflict resolution
- temporary provenance bridge resolution
- distribution-ready authorization
```

は human-required を維持する。

---

## 21. automation report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_automation_report",
  "current_maturity": "M2 operational reconstruction",
  "automation_status": "semi_automatic",
  "automation_safe_scopes": [],
  "human_required_scopes": [],
  "automation_invalidated": false,
  "override_used": false,
  "source_of_truth_refs": []
}
```

---

## 22. dashboard display

Dashboard は automation status を表示する。

表示対象：

```text
- current maturity
- automation status
- automation-safe scope
- human-required scope
- override usage
- automation invalidation
- promotion / demotion history
```

Dashboard は automation approval を独自決定しない。

---

## 23. CI mapping

CI fail 条件：

```text
- irreversible cleanup automated without review
- restriction conflict resolution automated
- distribution-ready authorization automated
- automation invalidated but still active
- override executed without audit trail
```

CI warn 条件：

```text
- automation confidence low
- automation demotion recommended
- repeated rerun instability
- repeated rollback instability
```

---

## 24. 禁止事項

以下を禁止する。

```text
- unstable governance を automation promotion すること
- restriction conflict resolution を unattended automation すること
- distribution-ready authorization を unattended automation すること
- override without audit trail
- automation-safe と human-required を混同すること
```

---

## 25. HLDocS feedback

本 governance から、HLDocS 側へ以下をフィードバック候補とする。

```text
- maturity-aware automation policy が必要
- automation-safe governance と human-required governance を分離すべき
- automation invalidation / demotion を formal state にすべき
- override governance と audit trail を formalize すべき
- automation promotion criteria を evidence-driven にすべき
```

---

## 26. 結論

governance maturity automation は、SansaVRM 再構成における automation boundary を定義する governance である。

これにより、M0〜M3 の maturity に応じて automation-safe governance のみを段階的に昇格しつつ、restriction conflict、irreversible cleanup、distribution-ready authorization のような high-risk governance を human-required として維持できる。
