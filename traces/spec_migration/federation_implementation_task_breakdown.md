# federation implementation task breakdown

## 1. 目的

本ドキュメントは、SansaVRM federation minimal viable implementation set を実装タスク粒度へ分解する。

対象は Preview Federation MVP であり、production cleanup や automatic apply execution は対象外とする。

---

## 2. 基本方針

実装タスクは以下を優先する。

```text
1. read-only index generation
2. validator JSON report generation
3. cleanup gate dry-run
4. dashboard snapshot artifact generation
5. CI dry-run integration
6. cross-project handoff / artifact freshness display
```

以下は MVP では実装しない。

```text
- destructive cleanup execution
- automatic canonicalization apply
- downstream repository modification
- full governance workflow automation
- production release automation
```

---

## 3. 推奨ディレクトリ構成

MVP 用の実装候補ディレクトリ：

```text
tools/federation_migration/
  index_builder/
  validators/
  dashboard/
  schemas/
  ci/
  examples/
  reports/
```

既存構成に合わせる場合は、上記を分割または名称変更してよい。

ただし、以下の責務分離は維持する。

```text
index_builder != validators
validators != dashboard
dashboard != source of truth
CI != destructive executor
```

---

## 4. Task group A: schema / data contracts

### A-1. index schema 定義

作成候補：

```text
tools/federation_migration/schemas/index_bundle.schema.json
```

含める index：

```text
filesystem_index
migration_index
canonical_index
rewrite_index
external_artifact_index
hash_index
```

### A-2. validator report schema 定義

作成候補：

```text
tools/federation_migration/schemas/validator_report.schema.json
```

対象：

```text
manifest_validator_report
canonicalization_validator_report
rewrite_validator_report
cleanup_gate_validator_report
federation_validator_report
```

### A-3. dashboard snapshot schema 定義

作成候補：

```text
tools/federation_migration/schemas/dashboard_snapshot.schema.json
```

必須：

```text
source_of_truth_refs
blocking_reasons
cleanup_readiness_summary
external_artifact_summary
```

---

## 5. Task group B: index builder

### B-1. filesystem_index builder

入力：

```text
repository root
```

出力：

```text
filesystem_index.json
```

最小項目：

```text
path
exists
file_kind
content_hash
```

### B-2. migration_index builder

入力：

```text
migration manifest / traces/spec_migration/*
```

出力：

```text
migration_index.json
```

最小項目：

```text
migration_entry_id
dry_doc_id
source_path
target_path
migration_state
placeholder_state
```

### B-3. canonical_index builder

入力：

```text
canonicalization manifest / related reports
```

出力：

```text
canonical_index.json
```

最小項目：

```text
document_fate
old_doc_id
canonical_doc_id
canonicalization_status
semantic_equivalent
cleanup_allowed
```

### B-4. external_artifact_index builder

入力：

```text
cross-project handoff response documents
manual artifact metadata
```

出力：

```text
external_artifact_index.json
```

最小項目：

```text
artifact_id
source_project
artifact_kind
declared_stage
freshness_status
cleanup_impact
```

---

## 6. Task group C: project-local validators

### C-1. manifest validator MVP

入力：

```text
index_bundle.json
```

出力：

```text
manifest_validator_report.json
manifest_validator_summary.md
```

検査：

```text
manifest loadability
duplicate dry_doc_id
duplicate migration_entry_id
unknown migration_state
placeholder relocation remaining
```

### C-2. canonicalization validator MVP

出力：

```text
canonicalization_validator_report.json
canonicalization_validator_summary.md
```

検査：

```text
unknown document_fate
pending document_fate
canonical_doc_id missing / duplicate
temporary dual canonical unresolved
cleanup_allowed invalid
```

### C-3. rewrite validator MVP

出力：

```text
rewrite_validator_report.json
rewrite_validator_summary.md
```

検査：

```text
transaction_status validity
rewrite_state validated / not_required
operation_kind unknown
affected_files mismatch
rollback scope missing
```

### C-4. cleanup gate validator MVP

出力：

```text
cleanup_gate_validator_report.json
cleanup_gate_summary.md
```

判定：

```text
cleanup_ready
cleanup_blocked
cleanup_pending
```

unknown は pass にしない。

---

## 7. Task group D: federation validator MVP

### D-1. handoff response validator

検査：

```text
MuJoCo Adapter response exists
Studio AI response exists
response stage known
pending decision が cleanup scope にない
```

### D-2. artifact freshness validator

検査：

```text
external artifact freshness_status
required artifact unknown/stale
cleanup_impact
```

### D-3. draft/canonical boundary validator

検査：

```text
draft artifact inside canonical boundary
experimental artifact used for cleanup dependency
```

---

## 8. Task group E: dashboard snapshot

### E-1. dashboard snapshot generator

入力：

```text
validator reports
cleanup gate report
external_artifact_index
```

出力：

```text
dashboard_snapshot.json
dashboard_summary.md
```

表示：

```text
validator summary
cleanup readiness
blocking reasons
reconstruction delta list
external artifact freshness
handoff status
```

### E-2. projection validation

検査：

```text
source_of_truth_refs present
cleanup_state matches cleanup gate report
blocking reason source_domain present
```

---

## 9. Task group F: CI dry-run workflow

### F-1. GitHub Actions workflow

作成候補：

```text
.github/workflows/federation-migration-preview.yml
```

実行内容：

```text
1. index builder
2. manifest validator
3. canonicalization validator
4. rewrite validator
5. cleanup gate validator
6. federation validator
7. dashboard snapshot generator
8. upload artifacts
```

### F-2. CI artifacts

出力候補：

```text
reports/federation/index_bundle.json
reports/federation/*_validator_report.json
reports/federation/dashboard_snapshot.json
reports/federation/dashboard_summary.md
```

CI では destructive operation を行わない。

---

## 10. Task group G: reconstruction delta registry

### G-1. registry schema

作成候補：

```text
tools/federation_migration/schemas/reconstruction_delta_registry.schema.json
```

### G-2. registry file

作成候補：

```text
traces/spec_migration/reconstruction_delta_registry.json
```

最小項目：

```text
reconstruction_delta_id
delta_kind
source
affected_scope
status
rerun_required
cleanup_ready_invalidated
```

---

## 11. 実装順序

推奨順：

```text
A-1 → A-2 → B-1 → B-2 → C-1 → E-1 → F-1
  ↓
B-3 → C-2
  ↓
C-3 → C-4
  ↓
A-3 → E-2
  ↓
B-4 → D-1 → D-2 → D-3
  ↓
G-1 → G-2
```

---

## 12. MVP 完了条件

MVP 完了条件：

```text
- CI が index_bundle.json を生成する
- CI が validator reports を生成する
- cleanup gate dry-run が cleanup_ready / blocked / pending を出す
- dashboard_summary.md が生成される
- cross-project handoff status が dashboard に表示される
- external artifact freshness が unknown / stale / fresh で表示される
- unknown を pass として扱っていない
```

---

## 13. 禁止事項

以下を禁止する。

```text
- MVP で cleanup execution を実装すること
- MVP で apply execution を実装すること
- dashboard を source of truth にすること
- skeleton index を pass として扱うこと
- CI で downstream repository を変更すること
```

---

## 14. HLDocS feedback

本 task breakdown から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction MVP は schema / index / validator / dashboard / CI の順で分解すべき
- cleanup gate dry-run を MVP に含めるべき
- destructive operation は MVP から除外すべき
- unknown / skeleton / stale を pass にしてはならない
- CI artifacts と dashboard summary を早期に導入すべき
```

---

## 15. 結論

federation implementation task breakdown は、Preview Federation MVP を実装するための具体的な作業分解である。

これにより、SansaVRM federation を read-only / dry-run / validator-first の最小構成から実装開始できる。
