# provenance restriction merge validator detail

## 1. 目的

本ドキュメントは、SansaVRM における provenance / restriction merge validator detail を定義する。

本 detail は、rights provenance governance model を validator 実装観点へ落とし込み、provenance chain、multi-parent derivation、restriction merge、editor rights、tool provenance、distribution readiness の検査方針を整理する。

---

## 2. 基本方針

provenance / restriction merge validator は以下を扱う。

```text
- provenance chain validation
- multi-parent derivation graph validation
- component provenance validation
- restriction merge validation
- editor rights validation
- tool provenance validation
- distribution readiness validation
- reconstruction provenance contamination detection
```

provenance / restriction merge validator は以下を行わない。

```text
- 法律上の権利判定を確定しない
- restriction conflict を自動解消しない
- editor rights を original author replacement と扱わない
- incomplete provenance を distribution-ready と扱わない
- temporary bridge provenance を canonical provenance と扱わない
```

---

## 3. validator modules

provenance 系 validator は以下に分ける。

```text
provenance_chain_validator
component_provenance_validator
restriction_merge_validator
editor_rights_validator
tool_provenance_validator
distribution_readiness_validator
provenance_contamination_validator
```

MVP では単一 validator 内の submodule として扱ってよい。

---

## 4. input artifacts

入力候補：

```text
rights_provenance block
provenance graph registry
component provenance registry
restriction registry
editor rights registry
tool provenance registry
review result registry
reconstruction comparison report
```

初期段階では registry / dashboard / reason code レベルで扱ってよい。

---

## 5. common output

各 validator は以下を出力する。

```text
provenance_validator_report.json
restriction_merge_validator_report.json
editor_rights_validator_report.json
tool_provenance_validator_report.json
```

Report には以下を含める。

```text
- status
- findings
- source_of_truth_refs
- distribution_readiness_impact
- cleanup_impact
- review_required
```

---

## 6. provenance chain validation

検査対象：

```text
- unresolved source asset
- missing acquisition provenance
- missing rights holder
- missing license snapshot where required
- invalid provenance edge
- provenance graph cycle
- unknown provenance origin
```

unresolved source は distribution-ready を block する。

---

## 7. multi-parent derivation validation

Studio AI workflow では以下が発生する。

```text
SansaVRM A: body / face / rig
SansaVRM B: clothing / material
SansaVRM C: hair / accessory
↓
SansaVRM D: assembled character
```

検査：

```text
- source parent がすべて解決可能か
- component scope が明示されているか
- same component に複数 source が競合していないか
- parent restriction が merge 対象に含まれているか
- editor / assembler が記録されているか
```

multi-parent graph が不完全な場合、distribution readiness を block または review_required とする。

---

## 8. component provenance validation

検査対象：

```text
- source_asset
- source_component
- derived_component
- merge_rule
- component boundary
```

Fail / blocked 条件：

```text
- component source missing
- derived component unknown
- merge_rule missing in assembly scope
- component boundary ambiguous in cleanup / distribution scope
```

---

## 9. restriction merge validation

restriction merge は自動 override しない。

検査対象：

```text
- commercial_rule
- redistribution_rule
- modification_rule
- attribution_rule
- platform_rule
- tool_usage_rule
```

Conflict 例：

```text
commercial_allowed + commercial_prohibited
redistribution_allowed + redistribution_prohibited
attribution_required + attribution_missing
```

Conflict は原則 review_required とする。

重大 conflict は distribution_ready を block する。

---

## 10. editor rights validation

検査対象：

```text
- editor_id
- modification_scope
- assembly_scope
- conversion_scope
- editor_attribution_rule
- editor_restriction
```

重要：

```text
editor rights
```

は：

```text
original author replacement
```

ではない。

Fail / blocked 条件：

```text
- editor attribution required but missing
- modification scope unknown
- editor restriction conflicts with source restriction
- assembler missing in assembly scope
```

---

## 11. tool provenance validation

検査対象：

```text
- tool_name
- tool_version
- workflow_id
- execution_mode
- generated_timestamp
```

AI workflow では tool provenance が rights / distribution に影響する可能性がある。

Fail / warn 条件：

```text
- generated_by missing in AI generated asset
- tool_version missing in distribution scope
- execution_mode unknown where policy depends on mode
```

---

## 12. policy provenance validation

検査対象：

```text
- restriction merge history
- review result history
- distribution state change
- commercial rule change
- attribution rule change
```

policy provenance が欠落している場合、review_required とする。

---

## 13. distribution readiness validation

distribution_ready 条件：

```text
- provenance chain resolved
- restriction conflicts resolved or reviewed
- editor attribution satisfied
- tool provenance required fields present
- no unresolved review_required in distribution scope
- no invalid provenance graph cycle
```

重要：

```text
asset usable internally
```

と：

```text
distribution-ready
```

は異なる。

---

## 14. reconstruction provenance contamination validation

mixed reconstruction / partial overwrite では provenance contamination が発生しうる。

検査：

```text
- old provenance edge が new canonical boundary に残っていないか
- temporary bridge provenance が canonical provenance として扱われていないか
- partial overwrite provenance が active distribution evidence になっていないか
- unknown_origin が cleanup evidence に使われていないか
```

contamination unresolved は cleanup_hold または distribution block とする。

---

## 15. reason codes

候補 reason code：

```text
provenance_source_missing
provenance_edge_invalid
provenance_cycle_detected
acquisition_provenance_missing
license_snapshot_missing
component_source_missing
component_boundary_ambiguous
restriction_conflict_unresolved
restriction_merge_review_required
editor_attribution_missing
editor_scope_unknown
tool_provenance_missing
tool_version_missing
temporary_provenance_bridge_used_as_source
unknown_origin_used_as_cleanup_evidence
distribution_ready_overissued
```

---

## 16. severity mapping

Blocked 条件：

```text
- distribution_ready_overissued
- unresolved restriction conflict in distribution scope
- unresolved provenance chain in distribution scope
- temporary bridge used as canonical provenance
- unknown_origin used as cleanup evidence
```

Warn 条件：

```text
- incomplete provenance outside distribution scope
- review_required outside distribution scope
- tool_version missing outside policy-dependent scope
```

---

## 17. cleanup readiness との関係

cleanup scope に provenance が関係する場合、以下を確認する。

```text
- old provenance edge の削除可否
- provenance rollback scope
- provenance comparison evidence
- editor attribution continuity
- restriction merge continuity
```

provenance blocker unresolved のまま cleanup execution へ進めてはならない。

---

## 18. dashboard display

Dashboard 表示候補：

```text
- provenance graph status
- unresolved provenance count
- restriction conflict count
- review_required count
- editor attribution status
- tool provenance status
- distribution readiness status
```

Dashboard は rights / legal decision を確定しない。

---

## 19. CI mapping

CI fail 条件：

```text
- distribution_ready_overissued
- unresolved restriction conflict in distribution scope
- unresolved provenance chain in distribution scope
- temporary provenance bridge used as source of truth
- unknown_origin used as cleanup evidence
```

CI warn 条件：

```text
- provenance incomplete outside distribution scope
- review_required pending outside distribution scope
- tool provenance incomplete outside distribution scope
```

---

## 20. 禁止事項

以下を禁止する。

```text
- restriction conflict を自動 override すること
- incomplete provenance を distribution-ready と扱うこと
- editor rights を original author replacement と扱うこと
- temporary bridge provenance を canonical provenance と扱うこと
- dashboard 表示を legal decision と扱うこと
```

---

## 21. Studio AI feedback

Studio AI 側へのフィードバック：

```text
- assembly workflow では multi-parent derivation graph を前提にする
- restriction merge は自動解決ではなく review_required を基本にする
- editor / assembler / converter を original author と分離する
- tool provenance は AI workflow では必須候補にする
- distribution-ready は provenance validation 後に判定する
```

---

## 22. HLDocS feedback

HLDocS 側へのフィードバック候補：

```text
- provenance validator と restriction merge validator を分けるべき
- distribution readiness は internal usability と分離すべき
- multi-parent provenance graph を扱える traceability model が必要
- temporary bridge provenance を source of truth として扱わない規則が必要
```

---

## 23. 結論

provenance / restriction merge validator detail は、SansaVRM における rights provenance / restriction merge / editor rights / tool provenance を検証する validator detail である。

これにより、Studio AI の assembly / conversion / derivation workflow を、distribution governance と cleanup governance に安全に接続できる。
