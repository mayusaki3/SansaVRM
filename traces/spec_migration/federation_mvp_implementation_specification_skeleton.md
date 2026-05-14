# federation MVP implementation specification skeleton

## 1. 目的

本ドキュメントは、SansaVRM federation MVP を実装するための implementation specification skeleton を定義する。

本 skeleton は、`tools/federation_migration/` 配下の draft schema、stub tool、validator report、dashboard artifact、CI artifact の実装前提を揃えるための最小仕様である。

本仕様は Preview Federation MVP 用であり、Production Federation の正本仕様ではない。

---

## 2. 基本方針

MVP implementation specification は以下を定義する。

```text
- CLI entrypoint
- input / output paths
- common metadata fields
- index bundle structure
- validator report structure
- cleanup gate result structure
- dashboard snapshot structure
- external artifact registry structure
- reconstruction delta registry structure
- status taxonomy
```

MVP implementation specification は以下を行わない。

```text
- canonical schema を確定しない
- production validator を定義しない
- cleanup execution を許可しない
- apply execution を許可しない
- downstream repository modification を許可しない
```

---

## 3. common metadata

すべての MVP JSON artifact は以下の common metadata を持つ。

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "example",
  "preview_only": true,
  "dry_run_only": true,
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_of_truth_refs": []
}
```

### 必須方針

```text
preview_only=true:
Preview Federation MVP 用であることを示す

dry_run_only=true:
destructive operation を含まないことを示す

source_of_truth_refs:
参照元 registry / schema / validator report / input artifact を記録する
```

---

## 4. CLI entrypoints

MVP の CLI entrypoint は以下とする。

```text
python tools/federation_migration/index_builder/build_index.py
python tools/federation_migration/validators/run_validators.py
python tools/federation_migration/dashboard/build_dashboard.py
```

---

## 5. common CLI args

全 tool は以下の引数を受け取れるようにする。

```text
--repo-root
--output-dir
--strict
--format
```

初期既定値：

```text
--repo-root .
--output-dir reports/federation
--strict false
--format json
```

---

## 6. build_index.py specification

### 入力

```text
traces/spec_migration/reconstruction_delta_registry.json
traces/spec_migration/external_artifact_registry.json
tools/federation_migration/examples/*.json
```

### 出力

```text
reports/federation/index_bundle.json
```

### exit code

```text
0:
index_bundle generated

1:
required input missing or malformed

2:
unsafe condition detected
```

---

## 7. run_validators.py specification

### 入力

```text
reports/federation/index_bundle.json
```

### 出力

```text
reports/federation/manifest_validator_report.json
reports/federation/canonicalization_validator_report.json
reports/federation/rewrite_validator_report.json
reports/federation/cleanup_gate_validator_report.json
reports/federation/federation_validator_report.json
```

### exit code

```text
0:
no fail / blocked in required MVP scope

1:
validator execution error

2:
fail / blocked detected in required MVP scope
```

---

## 8. build_dashboard.py specification

### 入力

```text
reports/federation/*_validator_report.json
traces/spec_migration/reconstruction_delta_registry.json
traces/spec_migration/external_artifact_registry.json
```

### 出力

```text
reports/federation/dashboard_snapshot.json
reports/federation/dashboard_summary.md
```

### exit code

```text
0:
dashboard artifacts generated

1:
required validator report missing

2:
projection validation failed
```

---

## 9. index_bundle structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "index_bundle",
  "preview_only": true,
  "dry_run_only": true,
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_of_truth_refs": [],
  "indexes": {
    "filesystem_index": [],
    "migration_index": [],
    "canonical_index": [],
    "rewrite_index": [],
    "external_artifact_index": [],
    "hash_index": []
  },
  "skeleton_indexes": [
    "reference_index",
    "traceability_index",
    "sec_id_index"
  ]
}
```

`skeleton_indexes` に含まれる index は cleanup_ready の pass 根拠にしてはならない。

---

## 10. validator_report structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "validator_report",
  "validator_module": "manifest_validator",
  "preview_only": true,
  "dry_run_only": true,
  "stub": true,
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_of_truth_refs": [],
  "status": "warn",
  "summary": {
    "checked": 0,
    "pass": 0,
    "warn": 0,
    "fail": 0,
    "blocked": 0
  },
  "findings": []
}
```

---

## 11. finding structure

validator finding の最小構造：

```json
{
  "finding_id": "finding-0001",
  "severity": "warn",
  "reason": "skeleton_index_present",
  "source_domain": "validation",
  "target_id": "target-example",
  "message": "reference_index is skeleton and cannot be used as pass evidence"
}
```

severity：

```text
info
warn
fail
blocked
```

---

## 12. cleanup gate report structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "validator_report",
  "validator_module": "cleanup_gate_validator",
  "preview_only": true,
  "dry_run_only": true,
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_of_truth_refs": [],
  "status": "warn",
  "cleanup": {
    "cleanup_ready": [],
    "cleanup_blocked": [],
    "cleanup_pending": []
  },
  "findings": []
}
```

MVP では `cleanup_ready` を厳しめに扱う。

unknown / skeleton / stale は ready 根拠にしない。

---

## 13. dashboard_snapshot structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "dashboard_snapshot",
  "preview_only": true,
  "dry_run_only": true,
  "projection_only": true,
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "source_of_truth_refs": [],
  "overall_status": "warn",
  "validator_summary": [],
  "cleanup_summary": {
    "cleanup_ready": 0,
    "cleanup_blocked": 0,
    "cleanup_pending": 0
  },
  "blocking_reasons": [],
  "warnings": [],
  "external_artifacts": [],
  "reconstruction_deltas": []
}
```

Dashboard snapshot は source of truth ではない。

---

## 14. external_artifact_registry structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "external_artifact_registry",
  "preview_only": true,
  "dry_run_only": true,
  "artifacts": []
}
```

artifact entry：

```json
{
  "artifact_id": "artifact-example",
  "source_project": "SansaVRM-MuJoCo-Adapter",
  "artifact_kind": "draft_schema",
  "declared_stage": "draft",
  "freshness_status": "unknown",
  "validation_required": true,
  "cleanup_impact": "unknown"
}
```

---

## 15. reconstruction_delta_registry structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "reconstruction_delta_registry",
  "preview_only": true,
  "dry_run_only": true,
  "deltas": []
}
```

delta entry：

```json
{
  "reconstruction_delta_id": "delta-example",
  "delta_kind": "validation_delta",
  "source": "manual",
  "affected_scope": [],
  "status": "open",
  "rerun_required": true,
  "cleanup_ready_invalidated": true
}
```

---

## 16. status taxonomy

validator status：

```text
pass
warn
fail
blocked
not_applicable
```

cleanup state：

```text
cleanup_ready
cleanup_blocked
cleanup_pending
```

freshness status：

```text
fresh
stale
unknown
not_applicable
```

release stage：

```text
draft
experimental
preview
release_candidate
canonical_release
deprecated
obsolete
superseded
```

---

## 17. MVP strict mode

`--strict true` の場合：

```text
- unknown-as-pass を fail
- skeleton-as-pass を fail
- missing source_of_truth_refs を fail
- missing required flags を fail
- dashboard projection validation failure を fail
```

MVP CI では strict mode を有効にすることを推奨する。

---

## 18. generated artifact rule

生成物は以下に出力する。

```text
reports/federation/
```

生成物は source of truth ではない。

生成物を Git 管理対象にする場合は、明示理由と freshness policy が必要である。

---

## 19. 禁止事項

以下を禁止する。

```text
- preview_only=false を MVP artifact に設定すること
- dry_run_only=false を MVP artifact に設定すること
- dashboard_snapshot を source of truth として扱うこと
- skeleton index を cleanup_ready の根拠にすること
- stub validator report を production readiness に使うこと
```

---

## 20. HLDocS feedback

本 skeleton から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction MVP には implementation specification skeleton が必要
- preview_only / dry_run_only / projection_only を artifact metadata として持つべき
- source_of_truth_refs を必須化すべき
- skeleton index を pass 根拠にしない規則が必要
- strict mode による unknown-as-pass / skeleton-as-pass 検出が必要
```

---

## 21. 結論

federation MVP implementation specification skeleton は、SansaVRM federation MVP の初期実装に必要な CLI、JSON、artifact path、status taxonomy を揃えるための最小仕様である。

これにより、Preview Federation MVP を read-only / dry-run / validator-first のまま、実装可能な形へ落とし込める。
