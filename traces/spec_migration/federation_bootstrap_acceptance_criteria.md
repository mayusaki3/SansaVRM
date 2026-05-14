# federation bootstrap acceptance criteria

## 1. 目的

本ドキュメントは、SansaVRM federation MVP bootstrap を正式に成立と判定するための acceptance criteria を定義する。

本 criteria は Preview Federation MVP bootstrap を対象とし、Production Federation readiness 判定ではない。

---

## 2. 基本方針

bootstrap acceptance は以下を確認する。

```text
- bootstrap artifacts が存在する
- dry-run toolchain が動作する
- validator reports が生成される
- cleanup gate dry-run が動作する
- dashboard projection が生成される
- CI artifact upload が動作する
- risk guardrails が有効である
- preview_only / dry_run_only が明示されている
```

bootstrap acceptance は以下を意味しない。

```text
- production readiness
- cleanup safety 完了
- canonical release readiness
- governance completeness
- full validator implementation completion
```

---

## 3. acceptance stages

acceptance stage は以下とする。

```text
bootstrap_incomplete
bootstrap_candidate
bootstrap_accepted
bootstrap_rejected
bootstrap_superseded
```

---

## 4. bootstrap_incomplete

以下のいずれかを満たす場合：

```text
- required schema missing
- required registry missing
- required validator report missing
- CI workflow missing
- dashboard artifact missing
- local dry-run failure
```

bootstrap_incomplete は Preview Federation として扱ってはならない。

---

## 5. bootstrap_candidate

以下を満たす場合：

```text
- directory skeleton exists
- draft schemas exist
- example JSON exists
- registries exist
- stub tools execute locally
- CI dry-run succeeds once
```

ただし：

```text
- risk review 未完了
- acceptance checklist 未完了
- dashboard / validator review 未完了
```

の場合は bootstrap_candidate とする。

---

## 6. bootstrap_accepted

以下を満たす場合：

```text
- bootstrap execution checklist 完了
- required artifacts generated
- required validator reports generated
- dashboard artifacts generated
- cleanup gate dry-run operational
- source_of_truth_refs present
- risk review completed
- critical open risks が accepted されていない
- preview_only=true
- dry_run_only=true
```

さらに：

```text
- unknown / skeleton を pass 扱いしていない
- generated reports を source of truth として扱っていない
- draft schema を canonical として扱っていない
```

必要がある。

---

## 7. bootstrap_rejected

以下の場合：

```text
- cleanup_ready が unsafe に出ている
- unknown-as-pass を検出
- dashboard が source of truth として扱われている
- generated report が source file と混同されている
- draft schema が canonical boundary に混入
- preview_only flag missing
- dry_run_only flag missing
```

bootstrap_rejected の場合、cleanup / release scope に進めてはならない。

---

## 8. bootstrap_superseded

以下の場合：

```text
- reconstruction delta 後に再 bootstrap が必要
- bootstrap schema が obsolete
- bootstrap layout が reference architecture と乖離
- bootstrap validator reports が stale
```

bootstrap_superseded は再 acceptance を必要とする。

---

## 9. required artifacts

required artifacts：

```text
reports/federation/index_bundle.json
reports/federation/manifest_validator_report.json
reports/federation/canonicalization_validator_report.json
reports/federation/rewrite_validator_report.json
reports/federation/cleanup_gate_validator_report.json
reports/federation/federation_validator_report.json
reports/federation/dashboard_snapshot.json
reports/federation/dashboard_summary.md
```

artifact missing は bootstrap_incomplete または bootstrap_rejected とする。

---

## 10. required flags

以下の flag は必須：

```text
preview_only=true
dry_run_only=true
projection_only=true
```

optional：

```text
stub=true
skeleton=true
```

required flag missing は bootstrap_rejected とする。

---

## 11. required CI results

CI acceptance 条件：

```text
- workflow success
- required reports generated
- dashboard artifacts uploaded
- non-zero exit on validator failure
- no destructive operation executed
```

CI warning は acceptance reject 条件ではない。

ただし、critical risk を warning に downgrade してはならない。

---

## 12. required dashboard checks

dashboard review で確認：

```text
- cleanup_ready / blocked / pending summary
- blocking reason list
- source_of_truth_refs
- reconstruction delta summary
- external artifact freshness summary
- preview_only / dry_run_only flags
- open critical risks
```

Dashboard は acceptance authority を持たない。

---

## 13. risk review acceptance

risk review で確認：

```text
- R-001 stub validator overtrust mitigation
- R-002 unknown-as-pass mitigation
- R-003 dashboard projection misuse mitigation
- R-004 draft schema misuse mitigation
- R-006 artifact freshness unknown mitigation
- R-010 preview/production confusion mitigation
```

critical risk が open のまま accepted としてはならない。

---

## 14. acceptance record schema draft

```json
{
  "schema_version": "0.1-draft",
  "acceptance_record_id": "bootstrap-acceptance-YYYYMMDD-NNN",
  "acceptance_stage": "bootstrap_accepted",
  "preview_only": true,
  "dry_run_only": true,
  "required_artifacts_verified": true,
  "risk_review_completed": true,
  "open_critical_risks": []
}
```

---

## 15. acceptance authority

bootstrap acceptance authority は以下を確認する。

```text
- acceptance checklist 完了
- CI result review 完了
- dashboard review 完了
- risk review 完了
- generated artifact misuse がない
```

bootstrap acceptance authority は以下を承認しない。

```text
- production cleanup
- canonical release
- governance completeness
- production readiness
```

---

## 16. bootstrap acceptance flow

```text
bootstrap_candidate
  ↓
artifact verification
  ↓
CI review
  ↓
dashboard review
  ↓
risk review
  ↓
bootstrap_accepted
```

critical blocking issue がある場合：

```text
bootstrap_rejected
```

reconstruction delta 後：

```text
bootstrap_superseded
```

---

## 17. 禁止事項

以下を禁止する。

```text
- bootstrap_accepted を production readiness と誤認すること
- dashboard review のみで acceptance を通すこと
- critical open risks を accepted に downgrade すること
- unknown-as-pass を known limitation として放置すること
- generated reports を source of truth として扱うこと
```

---

## 18. HLDocS feedback

本 acceptance criteria から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction bootstrap には acceptance gate が必要
- Preview Federation acceptance と Production Federation readiness を分離すべき
- required flags / required artifacts を formalize すべき
- risk review を acceptance 条件に含めるべき
- unknown-as-pass を acceptance reject 条件にすべき
```

---

## 19. 結論

federation bootstrap acceptance criteria は、SansaVRM federation MVP bootstrap を正式に成立と判定するための acceptance criteria である。

これにより、Preview Federation bootstrap を、artifact、validator、dashboard、CI、risk review に基づいて安全に成立判定できる。
