# dashboard snapshot generator

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における dashboard snapshot generator を定義する。

dashboard snapshot generator は、migration orchestration graph、各 validator report、cleanup gate report、external artifact state を統合し、人間が確認可能な dashboard snapshot を生成する。

Dashboard snapshot は表示用 projection であり、source of truth ではない。

---

## 2. 基本方針

Dashboard snapshot generator は以下を行う。

```text
- orchestration graph を表示用 graph へ投影する
- validator result を node / edge state へ投影する
- cleanup gate result を cleanup_state として表示する
- blocking reason / warning を集約する
- cross-project artifact state を表示する
- rollback impact / cleanup impact を表示する
```

Dashboard snapshot generator は以下を行わない。

```text
- cleanup_ready を独自判定しない
- validator result を変更しない
- orchestration graph を変更しない
- rewrite を実行しない
- cleanup を実行しない
- doc_id / sec_id を変更しない
```

---

## 3. 入力

Dashboard snapshot generator の入力は以下とする。

```text
- migration orchestration graph
- dashboard_state_schema
- manifest validator report
- canonicalization validator report
- rewrite validator report
- cleanup gate validator report
- integrity / tamper validator report
- validator orchestration report
- external_artifact_index
- alias_index
- hash_index
- CI result summary
```

---

## 4. 出力

Dashboard snapshot generator は以下を出力する。

```text
- dashboard snapshot JSON
- dashboard summary Markdown section
- node state summary
- edge state summary
- cleanup readiness view
- blocking reason view
- warning view
- cross-project artifact view
- rollback / cleanup impact view
```

---

## 5. source of truth

Dashboard snapshot の source of truth は以下とする。

```text
migration_state:
migration manifest / manifest validator report

fate_state:
canonicalization manifest / canonicalization validator report

canonicalization_state:
canonicalization validator report

rewrite_state:
rewrite transaction report / rewrite validator report

validation_state:
validator reports

alias_state:
alias index / alias validator result

cleanup_state:
cleanup gate validator report

integrity_state:
integrity / tamper validator report

cross_project_state:
external_artifact_index / cross-project validation result
```

Dashboard snapshot 自体を source of truth として再入力してはならない。

---

## 6. snapshot lifecycle

Dashboard snapshot の lifecycle は以下とする。

```text
generate_requested
  ↓
input_collected
  ↓
projection_built
  ↓
projection_validated
  ↓
snapshot_generated
  ↓
snapshot_published
```

snapshot_published は表示可能状態を意味する。

cleanup_ready や validation pass を意味しない。

---

## 7. node projection

node projection は orchestration graph node と validator result を統合する。

node に含める情報：

```text
- node_id
- node_kind
- dry_doc_id
- old_doc_id
- canonical_doc_id
- current_path
- previous_paths
- migration_state
- fate_state
- canonicalization_state
- rewrite_state
- validation_state
- alias_state
- cleanup_state
- integrity_state
- cross_project_state
- blocking_reasons
- warnings
- source_of_truth_refs
```

---

## 8. edge projection

edge projection は orchestration graph edge と validator result を統合する。

edge に含める情報：

```text
- edge_id
- edge_kind
- source_node_id
- target_node_id
- blocking
- validation_state
- cleanup_impact
- rollback_impact
- blocking_reasons
- warnings
```

---

## 9. cleanup readiness view

cleanup readiness view は cleanup gate validator report を投影する。

表示対象：

```text
- cleanup_ready nodes
- cleanup_blocked nodes
- cleanup_pending nodes
- cleanup blocking reasons
- cleanup impact summary
- external artifact freshness affecting cleanup
```

Dashboard snapshot generator は cleanup_ready を独自判定してはならない。

---

## 10. blocking reason view

blocking reason view は、各 validator report の blocking reason を source_domain 付きで集約する。

source_domain：

```text
migration
fate
canonicalization
rewrite
validation
alias
cleanup
integrity
cross_project
```

同一 blocking reason が複数 source から出る場合は、重複排除せず source ごとに保持する。

---

## 11. warning view

warning view は、継続可能だが注意が必要な状態を表示する。

例：

```text
- pending document_fate outside cleanup scope
- temporary dual canonical with declared exit condition
- representation hash mismatch but semantic hash matched
- legacy alias required outside cleanup scope
- external artifact stale outside cleanup scope
- active dry-run placeholder relocation
```

warning は cleanup_ready を意味しない。

---

## 12. cross-project artifact view

cross-project artifact view は external_artifact_index と validator result を表示する。

表示対象：

```text
- source_project
- artifact_kind
- path_or_url
- freshness_status
- validation_required
- validation_status
- cleanup_impact
- downstream dependency
```

対象例：

```text
- MuJoCo Adapter draft schema
- Adapter fixture
- diagnostics.json
- conversion_report.json
- Studio AI fixture
- export profile sample
```

---

## 13. rollback / cleanup impact view

Dashboard snapshot は rollback impact と cleanup impact を表示する。

表示対象：

```text
- affected rewrite transactions
- generated aliases
- affected validator reports
- affected dashboard nodes
- affected external artifacts
- rollback limitations
- cleanup target files
- cleanup blocked dependencies
```

Dashboard は rollback を実行しない。

---

## 14. snapshot schema draft

```json
{
  "schema_version": "1.0",
  "dashboard_snapshot_id": "dashboard-YYYYMMDD-NNN",
  "orchestration_graph_id": "orchestration-YYYYMMDD-NNN",
  "validator_orchestration_id": "validator-orchestration-YYYYMMDD-NNN",
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "overall_status": "warn",
  "summary": {
    "total_nodes": 0,
    "cleanup_ready": 0,
    "cleanup_blocked": 0,
    "cleanup_pending": 0,
    "fail_count": 0,
    "warn_count": 0
  },
  "nodes": [],
  "edges": [],
  "blocking_reasons": [],
  "warnings": [],
  "source_of_truth_refs": []
}
```

---

## 15. overall_status

overall_status の許容値：

```text
pass
warn
fail
blocked
```

判定方針：

```text
fail:
cleanup_ready node に blocking reason がある、または validator fail がある

blocked:
必要な source of truth が不足して snapshot が不完全

warn:
cleanup_pending / warning / active dry-run issue が存在する

pass:
表示対象に fail / blocked / warn がない
```

---

## 16. projection validation

snapshot 生成前に projection validation を行う。

検査：

```text
- source node が存在する
- source edge が存在する
- cleanup_state が cleanup gate report と一致する
- validator state が validator report と一致する
- blocking reason に source_domain がある
- cleanup_ready node に blocking reason がない
- stale artifact status が external_artifact_index と一致する
```

projection validation が fail の場合、snapshot を `blocked` として出力する。

---

## 17. Markdown summary structure

Markdown summary は以下の構成とする。

```text
1. Dashboard summary
2. Cleanup readiness summary
3. Blocking reason summary
4. Warning summary
5. Validator module summary
6. Cross-project artifact summary
7. Rollback / cleanup impact summary
8. Next required actions
```

---

## 18. CI artifact output

CI では dashboard snapshot を artifact として出力できる。

出力候補：

```text
dashboard_snapshot.json
dashboard_summary.md
blocking_reasons.json
cleanup_readiness.json
cross_project_artifacts.json
```

CI status は validator orchestration report を正とし、dashboard snapshot の overall_status は補助情報とする。

---

## 19. cache 条件

Dashboard snapshot generator の cache reuse 条件：

```text
- orchestration graph hash が一致する
- validator orchestration report hash が一致する
- cleanup gate report hash が一致する
- external_artifact_index hash が一致する
- dashboard generator version が一致する
- projection configuration hash が一致する
```

cache reuse 禁止条件：

```text
- cleanup gate result changed
- validator report changed
- external artifact freshness changed
- blocking reason taxonomy changed
- dashboard_state_schema changed
```

---

## 20. 禁止事項

以下を禁止する。

```text
- dashboard snapshot を source of truth として扱うこと
- dashboard snapshot generator が cleanup_ready を独自判定すること
- dashboard snapshot generator が validator result を変更すること
- dashboard snapshot generator が rewrite / cleanup / alias generation を実行すること
- projection validation fail の snapshot を pass として公開すること
```

---

## 21. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- dashboard は source of truth ではなく projection として扱うべき
- cleanup_ready は cleanup gate validator の出力を表示すべき
- dashboard snapshot には source_of_truth_refs が必要
- projection validation が必要
- CI artifact として dashboard snapshot を出力できるとよい
```

---

## 22. 結論

Dashboard snapshot generator は、orchestration graph、validator reports、cleanup gate report、external artifact state を統合し、人間が確認可能な dashboard snapshot を生成する projection component である。

Dashboard snapshot は表示・レビュー・CI artifact には使用できるが、canonical state、validation result、cleanup_ready の source of truth として扱ってはならない。
