# validator orchestration

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における validator orchestration を定義する。

validator orchestration は、migration orchestration graph を入力として、どの validator module を、どの順序・範囲・頻度で実行するかを決定する実行計画モデルである。

本モデルは、federation validator implementation model を実運用・CI・incremental validation に接続する。

---

## 2. 基本方針

validator orchestration は以下を扱う。

```text
- validator scheduling
- incremental validation
- selective validation
- parallel validation
- validator cache
- CI orchestration
- report aggregation
- dashboard projection input generation
```

validator orchestration は validator result を生成するが、rewrite / cleanup / dashboard state mutation は行わない。

---

## 3. 入力

validator orchestration の入力は以下とする。

```text
- migration orchestration graph
- migration manifest federation
- canonicalization manifest
- rewrite transaction plan/report
- current filesystem snapshot
- reference index
- traceability index
- sec_id index
- alias index
- external artifact index
- previous validator cache
- CI context
```

---

## 4. validator phases

validator orchestration は以下の phase を持つ。

```text
1. graph intake
2. impact analysis
3. validation scope selection
4. module scheduling
5. cache reuse decision
6. validation execution
7. report aggregation
8. dashboard projection input generation
9. CI status mapping
```

---

## 5. validation scope

validation scope は以下のいずれかとする。

```text
full_scope
changed_subgraph_scope
closed_dependency_scope
single_transaction_scope
cross_project_scope
ci_pull_request_scope
```

### full_scope

全 migration / canonicalization / rewrite / cleanup 対象を検査する。

### changed_subgraph_scope

変更された node と、その影響範囲のみを検査する。

### closed_dependency_scope

依存関係が閉じた subgraph を検査する。

### single_transaction_scope

1つの rewrite transaction を中心に検査する。

### cross_project_scope

MuJoCo Adapter / Studio AI などの cross-project handoff と external artifact を含めて検査する。

### ci_pull_request_scope

PR 差分から影響範囲を抽出して検査する。

---

## 6. impact analysis

impact analysis は migration orchestration graph を用いて、validation 対象を決定する。

影響伝播対象：

```text
- changed document node
- changed manifest node
- changed canonicalization node
- changed rewrite transaction node
- changed external artifact node
- downstream reference node
- traceability-linked node
- cleanup-blocked node
- rollback-dependent node
```

impact analysis は filesystem ordering を使用しない。

---

## 7. module scheduling

validator module の基本順序は以下とする。

```text
1. manifest_validator
2. canonicalization_validator
3. rewrite_validator
4. reference_validator
5. traceability_validator
6. sec_id_validator
7. alias_validator
8. integrity_tamper_validator
9. cleanup_gate_validator
10. dashboard_projection_validator
11. orchestration_graph_validator
```

ただし、selective validation により、影響がない module は省略可能とする。

省略した module は `not_run` ではなく、cache reuse できる場合のみ `cached_pass` または `cached_warn` として扱う。

---

## 8. incremental validation

incremental validation は、変更された subgraph と依存範囲のみを検査する。

実行条件：

```text
- previous validator cache が存在する
- orchestration graph id または graph hash が比較可能である
- changed nodes / changed edges が抽出可能である
- external artifact freshness が確認済みである
```

incremental validation が使用できない場合は full_scope に fallback する。

---

## 9. selective validation

selective validation は、impact analysis の結果に基づいて module を選択する。

例：

```text
canonicalization manifest のみ変更:
canonicalization_validator
reference_validator
cleanup_gate_validator
dashboard_projection_validator

rewrite transaction のみ変更:
rewrite_validator
reference_validator
traceability_validator
cleanup_gate_validator

external artifact のみ変更:
integrity_tamper_validator
cross_project validation
cleanup_gate_validator
```

selective validation では、未実行 module が cleanup_ready 判定に必要な場合、cache または full validation が必要である。

---

## 10. parallel validation

以下の module は、入力 index が揃っている場合に並列実行可能である。

```text
reference_validator
traceability_validator
sec_id_validator
alias_validator
integrity_tamper_validator
```

以下は順序依存を持つ。

```text
manifest_validator → canonicalization_validator
canonicalization_validator → rewrite_validator
rewrite_validator → cleanup_gate_validator
cleanup_gate_validator → dashboard_projection_validator
```

parallel validation の結果は report aggregation phase で統合する。

---

## 11. validator cache

validator cache は、過去の validator result を再利用するための machine-readable cache である。

cache key 候補：

```text
- validator module name
- module version
- input graph hash
- target node hash
- target edge hash
- source file hash
- manifest hash
- external artifact hash
- configuration hash
```

cache を再利用できる条件：

```text
- validator module version が一致する
- target input hash が一致する
- upstream dependency hash が一致する
- external artifact freshness が確認済み
- validation configuration が一致する
```

cache を再利用してはならない条件：

```text
- validator module version が変わった
- blocking edge が追加・削除された
- canonical_doc_id mapping が変わった
- rewrite transaction が変更された
- external artifact freshness が不明
- cleanup gate condition が変更された
```

---

## 12. cache status

validator result の cache status は以下とする。

```text
not_cached
cached_pass
cached_warn
cache_invalid
cache_miss
cache_blocked
```

`cached_pass` は、その module の入力が完全に一致している場合のみ使用する。

cleanup_ready 判定に使用する cache は、cache provenance を report に含める。

---

## 13. CI orchestration

CI では validation scope を PR 差分から決定する。

CI の基本手順：

```text
1. changed files extraction
2. graph impact analysis
3. validation scope selection
4. cache reuse decision
5. validator execution
6. report aggregation
7. CI status mapping
8. artifact upload
```

CI fail 条件：

```text
- invalid manifest federation
- canonical_doc_id collision
- unresolved references on drop target
- sec_id collision
- traceability unresolved
- rewrite_failed
- cleanup_ready node with blocking reason
- integrity_tamper_validator fail on semantic_integrity
- orchestration graph invalid
```

CI warn 条件：

```text
- pending document fate
- legacy alias required but not generated
- representation hash mismatch with semantic hash match
- external artifact stale but not required for cleanup
- partial migration in active dry-run scope
```

---

## 14. report aggregation

report aggregation は module result を統合して、以下を生成する。

```text
- overall_status
- module_status_summary
- node_status_summary
- edge_status_summary
- cleanup_readiness_summary
- blocking_reason_summary
- warning_summary
- cache_summary
- CI status mapping
```

集約時は、fail が warn より優先される。

ただし、module が not_applicable の場合は overall_status を悪化させない。

---

## 15. dashboard projection input

validator orchestration は dashboard projection input を生成する。

含める情報：

```text
- node validation state
- edge validation state
- blocking reasons
- warning list
- cache status
- stale artifact status
- cleanup impact
- rollback impact
```

dashboard は projection input を表示するだけであり、validation result を変更しない。

---

## 16. cross-project validation

MuJoCo Adapter / Studio AI の external artifact は、cross_project_scope で検査する。

検査対象：

```text
- handoff response status
- draft schema freshness
- fixture compatibility
- diagnostics / conversion_report consistency
- updated_extension_properties classification
- export profile / bake pipeline fixture consistency
```

cross-project artifact が cleanup に影響する場合、cleanup_gate_validator は blocking reason を付与する。

---

## 17. validator orchestration report schema draft

```json
{
  "schema_version": "1.0",
  "validator_orchestration_id": "validator-orchestration-YYYYMMDD-NNN",
  "orchestration_graph_id": "orchestration-YYYYMMDD-NNN",
  "validation_scope": "changed_subgraph_scope",
  "overall_status": "warn",
  "modules": [
    {
      "module_name": "rewrite_validator",
      "status": "pass",
      "cache_status": "not_cached",
      "targets": ["rewrite-YYYYMMDD-NNN"]
    }
  ],
  "ci_status": "warning",
  "blocking_reasons": [],
  "warnings": []
}
```

---

## 18. 禁止事項

以下を禁止する。

```text
- cache が不明な module を pass 扱いすること
- external artifact freshness 未確認で cached_pass とすること
- cleanup_ready 判定に必要な validator を selective validation で省略すること
- dashboard projection result を validator source of truth として使うこと
- CI PR 差分のみを根拠に downstream dependency を無視すること
- validator orchestration が rewrite / cleanup を実行すること
```

---

## 19. HLDocS feedback

本 orchestration model から、HLDocS 側へ以下をフィードバックする。

```text
- validator は full validation だけでなく incremental / selective validation を扱うべき
- validator cache には input hash / module version / dependency hash が必要
- CI は changed files ではなく orchestration graph impact で validation scope を決めるべき
- cleanup_ready 判定に必要な module は省略してはならない
- cross-project artifact freshness を validation 条件に含めるべき
```

---

## 20. 結論

validator orchestration は、migration orchestration graph をもとに validator module の実行範囲・順序・cache・CI status を決定するモデルである。

これにより、full validation、incremental validation、selective validation、parallel validation、cross-project validation、CI validation を一貫して扱える。
