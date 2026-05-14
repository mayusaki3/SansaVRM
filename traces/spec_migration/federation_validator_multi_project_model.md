# federation validator multi-project model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における multi-project federation validator model を定義する。

本モデルは、SansaVRM 単体の validator ではなく、MuJoCo Adapter、Studio AI、将来の Adapter / Runtime / Export pipeline を含む cross-project dependency を検査する federation-level validator を扱う。

---

## 2. 基本方針

federation validator は以下を行う。

```text
- cross-project handoff contract を検査する
- external artifact freshness を検査する
- schema drift を検出する
- reconstruction delta propagation を検査する
- federation cleanup dependency を検査する
- federation rollback boundary を検査する
- project 間 validator rerun policy を決定する
- federation dashboard projection input を生成する
```

federation validator は以下を行わない。

```text
- 他 project の artifact を直接修正しない
- downstream project の schema を自動更新しない
- cleanup / rewrite / canonicalization を実行しない
- handoff response を捏造しない
- stale artifact を fresh として扱わない
```

---

## 3. federation graph

federation graph は、project 間 dependency を表す graph である。

対象 project 例：

```text
- SansaVRM
- SansaVRM-MuJoCo-Adapter
- SansaVRM Studio AI
- future runtime adapter
- future export pipeline
```

federation graph は migration orchestration graph と接続するが、同一ではない。

```text
migration orchestration graph:
SansaVRM 内部の execution / validation / cleanup dependency

federation graph:
project 間 artifact / schema / handoff / reconstruction delta dependency
```

---

## 4. federation node kinds

federation node の node_kind は以下とする。

```text
project_node
handoff_contract_node
schema_node
artifact_node
validator_report_node
reconstruction_delta_node
cleanup_dependency_node
rollback_boundary_node
```

### project_node

SansaVRM、MuJoCo Adapter、Studio AI などの project 単位。

### handoff_contract_node

cross-project handoff response / acceptance / pending decision を表す。

### schema_node

Adapter JSON schema、Extension Property schema、diagnostics schema など。

### artifact_node

fixture、conversion_report、diagnostics、updated_extension_properties、export sample など。

### validator_report_node

project 別 validator result。

### reconstruction_delta_node

project 間に伝播する reconstruction delta。

### cleanup_dependency_node

cleanup_ready 判定に影響する cross-project dependency。

### rollback_boundary_node

project 間 rollback の境界。

---

## 5. federation edge kinds

edge_kind は以下とする。

```text
requires_contract
produces_schema
consumes_schema
produces_artifact
consumes_artifact
validates_artifact
blocks_cleanup_of
requires_rerun_of
propagates_delta_to
stales_artifact
requires_handoff_response
requires_rollback_coordination
```

---

## 6. cross-project handoff contract

handoff contract は project 間の責務境界を表す。

最低限含める情報：

```text
- source_project
- target_project
- contract_id
- accepted_assumptions
- pending_decisions
- schema_dependencies
- artifact_dependencies
- validation_dependencies
- cleanup_dependencies
- reconstruction_delta_policy
```

handoff contract が未回答または不整合の場合、downstream project はその前提を stable として扱ってはならない。

---

## 7. schema drift detection

schema drift は、producer project と consumer project の schema 前提がずれた状態である。

検査対象：

```text
- schema version mismatch
- required field mismatch
- enum mismatch
- semantic meaning mismatch
- deprecated field still consumed
- draft schema used as canonical
```

schema drift が cleanup scope に影響する場合は cleanup_blocked とする。

PoC / draft 範囲のみなら warn として扱える。

---

## 8. stale artifact detection

stale artifact は、source schema / source manifest / source feedback に対して古くなった artifact である。

対象例：

```text
- Adapter draft schema
- Adapter fixture
- diagnostics.json
- conversion_report.json
- updated_extension_properties.json
- Studio AI fixture
- export profile sample
```

検査：

```text
- artifact source hash
- schema version
- generation timestamp
- source_project revision
- accepted handoff contract id
- validator report id
```

cleanup 判定に必要な stale artifact は fail または blocked とする。

cleanup 判定に不要な stale artifact は warn とする。

---

## 9. reconstruction delta propagation

cross-project reconstruction delta は federation graph に沿って伝播する。

例：

```text
MuJoCo Adapter schema change
  ↓
SansaVRM Adapter JSON boundary update
  ↓
SansaVRM rewrite / validation rerun
  ↓
Studio AI export fixture rerun required
```

reconstruction delta propagation では以下を記録する。

```text
- source delta
- affected project nodes
- affected schemas
- affected artifacts
- validator rerun scope
- cleanup_ready invalidation
- replacement execution requirement
```

---

## 10. federation cleanup dependency

cleanup_ready は project 内だけでなく、cross-project dependency により block される場合がある。

block 条件：

```text
- downstream project が old schema をまだ消費している
- handoff response が pending
- required artifact が stale
- external fixture validation が未実行
- reconstruction delta が未反映
- rollback coordination が未定義
```

federation validator は cleanup gate validator へ blocking reason を渡す。

---

## 11. federation rollback boundary

project 間 rollback は、単一 project rollback と分離して扱う。

rollback boundary に含める情報：

```text
- rollback owner project
- affected external artifacts
- dependent project rerun requirements
- incompatible rollback cases
- manual recovery requirement
```

cross-project artifact を stale にする rollback は、downstream validator rerun を要求する。

---

## 12. federation validator rerun policy

federation validator は delta kind に応じて rerun scope を決定する。

```text
schema_delta:
schema producer / consumer validation rerun

artifact_delta:
artifact freshness / compatibility validation rerun

handoff_delta:
contract validation rerun

cleanup_delta:
cleanup gate rerun across affected projects

semantic_delta:
full federation impact validation
```

---

## 13. report schema draft

```json
{
  "schema_version": "1.0",
  "validator_module": "federation_validator",
  "validator_run_id": "federation-validator-YYYYMMDD-NNN",
  "status": "warn",
  "projects": [
    {
      "project_id": "SansaVRM",
      "status": "pass"
    }
  ],
  "schema_drifts": [],
  "stale_artifacts": [],
  "reconstruction_delta_propagation": [],
  "cleanup_blocks": [],
  "warnings": []
}
```

---

## 14. blocking reasons

federation validator が出力する blocking reason 候補：

```text
handoff_contract_missing
handoff_contract_pending
schema_drift_detected
schema_version_mismatch
artifact_stale
artifact_missing
external_validator_not_run
cross_project_delta_unresolved
federation_cleanup_dependency_unresolved
rollback_boundary_missing
```

---

## 15. CI mapping

CI fail 条件：

```text
- cleanup scope に影響する schema drift
- cleanup scope に必要な stale artifact
- required handoff contract missing
- cross_project_delta unresolved in affected scope
- rollback boundary missing for destructive change
```

CI warn 条件：

```text
- draft schema drift outside cleanup scope
- stale artifact outside cleanup scope
- optional handoff contract pending
- downstream rerun recommended
```

---

## 16. dashboard projection

federation validator は dashboard projection input として以下を渡す。

```text
- project status
- handoff contract status
- schema drift status
- stale artifact status
- reconstruction delta propagation state
- cleanup dependency state
- rollback boundary state
```

Dashboard は federation result を表示するが、cross-project state を変更してはならない。

---

## 17. 禁止事項

以下を禁止する。

```text
- draft schema を canonical schema として扱うこと
- stale artifact を freshness 確認なしに pass とすること
- handoff pending のまま downstream cleanup を実行すること
- cross-project delta を localized patch として扱うこと
- federation validator が他 project artifact を直接修正すること
```

---

## 18. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- cross-project handoff は dependency node として扱うべき
- multi-project reconstruction では schema drift / stale artifact / handoff pending を validator 対象にすべき
- reconstruction delta は federation graph 上で伝播すべき
- cleanup_ready は cross-project dependency により block され得る
- federation validator は修正器ではなく observer / gate として扱うべき
```

---

## 19. 結論

federation validator multi-project model は、SansaVRM と周辺 project 間の schema、artifact、handoff、reconstruction delta、cleanup dependency を検査する federation-level validator model である。

これにより、SansaVRM 単体では検出できない cross-project schema drift、stale artifact、handoff mismatch、cleanup dependency を検出できる。
