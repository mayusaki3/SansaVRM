# federation MVP first implementation sprint plan

## 1. 目的

本ドキュメントは、SansaVRM federation MVP bootstrap 後に実施する最初の implementation sprint plan を定義する。

本 sprint は Preview Federation MVP の初回実装段階を対象とし、Production Federation readiness を目的としない。

---

## 2. sprint goal

本 sprint の goal：

```text
- federation MVP bootstrap を実装として成立させる
- schema / registry / stub tool / CI artifact を接続する
- dry-run validator pipeline を動作させる
- dashboard snapshot artifact を生成する
- cleanup gate dry-run を可視化する
```

本 sprint は以下を目的としない。

```text
- destructive cleanup
- canonicalization apply
- downstream repository modification
- full federation execution protocol
- production-grade validator implementation
```

---

## 3. sprint scope

対象：

```text
- tools/federation_migration/
- draft schemas
- example JSON
- reconstruction_delta_registry
- external_artifact_registry
- stub tools
- CI dry-run workflow
- dashboard artifact generation
```

非対象：

```text
- real markdown semantic parsing
- full traceability extraction
- full sec_id validation
- cleanup execution
- rollback execution
```

---

## 4. sprint phases

本 sprint は以下の phase で進める。

```text
S1-0 bootstrap verification
S1-1 schema implementation
S1-2 registry implementation
S1-3 stub index builder
S1-4 stub validators
S1-5 dashboard snapshot generator
S1-6 CI workflow integration
S1-7 artifact review
S1-8 sprint acceptance review
```

---

## 5. S1-0 bootstrap verification

目的：

```text
bootstrap repository layout が成立しているか確認する。
```

確認：

```text
- required directories exist
- required schemas exist
- required registries exist
- CI workflow file exists
- reports directory is generated-only
```

完了条件：

```text
bootstrap acceptance criteria が bootstrap_accepted
```

---

## 6. S1-1 schema implementation

対象：

```text
index_bundle.schema.json
validator_report.schema.json
dashboard_snapshot.schema.json
reconstruction_delta_registry.schema.json
external_artifact_registry.schema.json
```

最小要件：

```text
- schema_version
- required fields
- preview_only
- dry_run_only
- source_of_truth_refs
```

レビュー対象：

```text
- draft/canonical boundary
- source_of_truth_refs mandatory
- unknown handling
```

完了条件：

```text
- schema validation succeeds
- example JSON validates
- schema marked as draft
```

---

## 7. S1-2 registry implementation

対象：

```text
reconstruction_delta_registry.json
external_artifact_registry.json
```

最小要件：

```text
- schema_version
- registry_kind
- empty entries allowed
- machine-readable structure
```

レビュー対象：

```text
- reconstruction delta tracking
- artifact freshness tracking
- cleanup impact field
```

完了条件：

```text
- registry JSON validates
- registry readable from stub tools
```

---

## 8. S1-3 stub index builder

対象：

```text
build_index.py
```

初期入力：

```text
- example JSON
- registries
```

初期出力：

```text
reports/federation/index_bundle.json
```

最小要件：

```text
- read-only
- dry-run only
- no repository modification
- generated timestamp
- source_of_truth_refs
```

レビュー対象：

```text
- generated artifact separation
- unknown handling
- skeleton marker usage
```

完了条件：

```text
- index_bundle.json generated
- local dry-run succeeds
- CI artifact upload possible
```

---

## 9. S1-4 stub validators

対象：

```text
run_validators.py
```

対象 validator：

```text
manifest validator
canonicalization validator
rewrite validator
cleanup gate validator
federation validator
```

最小要件：

```text
- validator_report.json generation
- summary markdown generation
- preview_only=true
- dry_run_only=true
- stub=true where applicable
```

レビュー対象：

```text
- unknown-as-pass prevention
- cleanup_ready restrictions
- draft schema misuse detection
```

完了条件：

```text
- validator reports generated
- cleanup_ready / blocked / pending visible
- validator failure returns non-zero exit
```

---

## 10. S1-5 dashboard snapshot generator

対象：

```text
build_dashboard.py
```

入力：

```text
validator reports
registries
```

出力：

```text
dashboard_snapshot.json
dashboard_summary.md
```

最小要件：

```text
- projection_only=true
- source_of_truth_refs
- blocking reason summary
- reconstruction delta summary
- artifact freshness summary
```

レビュー対象：

```text
- dashboard/source separation
- cleanup_ready rendering correctness
- preview_only rendering
```

完了条件：

```text
- dashboard artifacts generated
- local review possible
- CI artifact upload possible
```

---

## 11. S1-6 CI workflow integration

対象：

```text
.github/workflows/federation-migration-preview.yml
```

実行順：

```text
1. build_index.py
2. run_validators.py
3. build_dashboard.py
4. upload reports/federation
```

最小要件：

```text
- workflow success
- artifact upload
- required reports verified
- validator failure -> workflow fail
```

レビュー対象：

```text
- destructive operation absence
- generated artifact separation
- dry-run-only guarantee
```

完了条件：

```text
- GitHub Actions success
- artifact downloadable
- dashboard artifact visible
```

---

## 12. S1-7 artifact review

レビュー対象：

```text
index_bundle.json
validator reports
dashboard_snapshot.json
dashboard_summary.md
```

確認：

```text
- preview_only present
- dry_run_only present
- projection_only present
- source_of_truth_refs present
- cleanup_ready not overissued
- unknown not treated as pass
```

Reject 条件：

```text
- dashboard acting as source of truth
- draft schema inside canonical boundary
- generated reports committed as source
```

---

## 13. S1-8 sprint acceptance review

Acceptance 条件：

```text
- all required artifacts generated
- CI workflow operational
- risk review passed
- bootstrap risks mitigated where required
- cleanup gate dry-run operational
- dashboard projection operational
```

Reject 条件：

```text
- unknown-as-pass
- cleanup_ready unsafe overissue
- preview/prod confusion
- generated artifact misuse
```

---

## 14. sprint deliverables

本 sprint の deliverables：

```text
- draft schemas
- registries
- stub tools
- validator reports
- dashboard artifacts
- CI workflow
- sprint review notes
```

---

## 15. sprint known limitations

本 sprint の known limitations：

```text
- semantic parsing incomplete
- traceability extraction incomplete
- sec_id validation incomplete
- external artifact freshness mostly manual
- federation execution protocol not implemented
- cleanup execution not implemented
```

これらは accepted limitation として扱う。

ただし、production readiness と誤認してはならない。

---

## 16. sprint review checklist

review checklist：

```text
- preview_only=true exists
- dry_run_only=true exists
- projection_only=true exists
- stub=true exists where applicable
- unknown-as-pass absent
- generated reports excluded from source tracking
- source_of_truth_refs exists
- CI artifacts downloadable
- dashboard review possible
```

---

## 17. 次 sprint への接続

次 sprint 候補：

```text
Sprint 2:
filesystem_index real scan
migration_index real extraction
real duplicate detection
real artifact freshness handling

Sprint 3:
reference extraction
traceability extraction
cleanup gate strengthening

Sprint 4:
federation validator strengthening
cross-project delta propagation
```

---

## 18. HLDocS feedback

本 sprint plan から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction framework は sprint-level planning を持つべき
- bootstrap 後は schema / registry / stub tool / dashboard / CI を順番に接続すべき
- sprint acceptance criteria を formalize すべき
- preview limitations を explicit に記録すべき
- generated artifact misuse を sprint review 対象にすべき
```

---

## 19. 結論

federation MVP first implementation sprint plan は、SansaVRM federation MVP bootstrap 後の初回 implementation sprint を定義する計画である。

これにより、Preview Federation MVP を read-only / dry-run / validator-first 原則のまま、安全に実装開始できる。
