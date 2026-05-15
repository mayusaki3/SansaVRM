# provenance graph validation model

## 1. 目的

本ドキュメントは、SansaVRM における provenance graph validation model を定義する。

本 model は、rights provenance governance model および provenance / restriction merge validator detail を受け、multi-parent derivation graph、component provenance、temporary bridge、restriction inheritance、distribution-ready 判定に必要な graph validation を整理する。

---

## 2. 基本方針

provenance graph validation は以下を扱う。

```text
- provenance node / edge
- multi-parent derivation graph
- component-level provenance graph
- unresolved source detection
- cycle detection
- restriction propagation
- temporary bridge detection
- distribution readiness impact
- cleanup readiness impact
```

provenance graph validation は以下を行わない。

```text
- legal rights decision を確定しない
- restriction conflict を自動解消しない
- temporary bridge を canonical provenance として扱わない
- provenance graph を Core semantic identity と混同しない
```

---

## 3. graph positioning

provenance graph は以下に属する。

```text
Preservation Compatibility Layer
Validation Layer
future Distribution Governance Layer
```

Core Semantic Layer へ直接入れすぎない。

理由：

```text
provenance graph は semantic identity そのものではなく、
rights / distribution / validation / preservation concern であるため。
```

---

## 4. node kinds

provenance node kind は以下とする。

```text
asset_node
component_node
source_package_node
acquisition_node
conversion_node
assembly_node
derivation_node
editor_node
tool_node
policy_node
review_node
temporary_bridge_node
```

---

## 5. edge kinds

provenance edge kind は以下とする。

```text
acquired_from
converted_from
assembled_from
derived_from
modified_from
component_from
edited_by
generated_by_tool
policy_changed_by
reviewed_by
bridged_by
supersedes
```

---

## 6. node requirements

各 node は最低限以下を持つ。

```text
node_id
node_kind
source_refs
stage
validation_status
```

optional：

```text
rights_holder
license_snapshot
restriction_state
timestamp
```

`source_refs` がない node は distribution_ready の根拠にできない。

---

## 7. edge requirements

各 edge は最低限以下を持つ。

```text
edge_id
edge_kind
from_node
to_node
source_refs
validation_status
```

edge が unresolved の場合、distribution_ready を block または review_required とする。

---

## 8. multi-parent derivation graph

Studio AI assembly では multi-parent derivation graph が基本になる。

例：

```text
A(body)
B(clothing)
C(hair)
↓ assembled_from edges
D(character)
```

検査対象：

```text
- all parent nodes resolved
- component scope defined
- edge kind appropriate
- restriction propagation captured
- editor / assembler node present where required
```

---

## 9. component-level provenance

component-level provenance は component_node を使う。

例：

```text
D.character.hair
  component_from → C.hair

D.character.clothing
  component_from → B.clothing
```

検査対象：

```text
- component boundary ambiguity
- duplicate component source
- missing derived component
- source component missing
- component merge rule missing
```

---

## 10. temporary bridge node

temporary_bridge_node は old/new または source/derived の一時接続を表す。

例：

```text
legacy alias
temporary redirect
old/new mapping table
compatibility bridge
```

重要：

```text
temporary_bridge_node
```

は source of truth ではない。

Distribution-ready / cleanup-ready の根拠にしてはならない。

---

## 11. cycle detection

provenance graph は原則として derivation / conversion / assembly の因果方向を持つ。

以下は検出対象：

```text
- derived_from cycle
- assembled_from cycle
- converted_from cycle
- component_from cycle
```

cycle が意図された reversible workflow を表す場合でも、明示的な review_required とする。

---

## 12. unresolved source detection

unresolved source は以下を含む。

```text
- missing parent asset
- missing source component
- missing acquisition node
- missing license snapshot where required
- missing tool node for AI-generated asset
- missing editor node for assembled asset
```

unresolved source が distribution scope にある場合、distribution_ready を block する。

---

## 13. restriction propagation

restriction propagation は parent node / component node から derived asset へ restriction を伝播する。

検査：

```text
- parent restriction present
- component restriction present
- merged restriction result present
- conflict recorded
- review_required recorded where conflict exists
```

restriction conflict を自動解消してはならない。

---

## 14. editor / tool graph validation

editor / tool は graph node として扱う。

検査：

```text
- assembled asset has editor / assembler node where required
- AI generated asset has tool_node
- converted asset has conversion_node and tool_node where required
- editor rights are not used as original author replacement
```

---

## 15. graph validation status

Graph validation status は以下とする。

```text
graph_valid
graph_warn
graph_blocked
graph_review_required
graph_incomplete
graph_superseded
```

`graph_valid` は legal clearance を意味しない。

---

## 16. distribution readiness relation

Distribution-ready 条件：

```text
- graph_valid or reviewed graph_warn
- no unresolved source in distribution scope
- no unresolved restriction conflict in distribution scope
- no temporary_bridge_node used as canonical evidence
- required editor / tool nodes present
```

Internal use は distribution-ready と分離する。

---

## 17. cleanup readiness relation

Cleanup-ready に影響する条件：

```text
- old provenance edge removal safe
- temporary bridge no longer required
- provenance rollback scope exists
- no unknown_origin used as cleanup evidence
- provenance comparison completed
```

provenance graph blocker unresolved のまま cleanup execution へ進めてはならない。

---

## 18. reconstruction relation

Mixed reconstruction では provenance graph contamination が発生しうる。

検査：

```text
- old provenance edge inside new canonical boundary
- temporary bridge used as canonical provenance
- partial overwrite provenance used as active distribution evidence
- superseded provenance node used as active
```

---

## 19. graph report structure

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "provenance_graph_report",
  "graph_validation_status": "graph_review_required",
  "nodes": [],
  "edges": [],
  "findings": [],
  "distribution_readiness_impact": "review_required",
  "cleanup_impact": "cleanup_hold"
}
```

---

## 20. reason codes

候補：

```text
provenance_node_missing_source_refs
provenance_edge_unresolved
provenance_cycle_detected
multi_parent_source_unresolved
component_source_missing
component_boundary_ambiguous
temporary_bridge_used_as_canonical_provenance
restriction_propagation_missing
restriction_conflict_unresolved
editor_node_missing
tool_node_missing
superseded_provenance_used_as_active
provenance_graph_incomplete
```

---

## 21. CI mapping

CI fail 条件：

```text
- temporary_bridge_used_as_canonical_provenance
- unresolved source in distribution scope
- restriction_conflict_unresolved in distribution scope
- superseded provenance used as active
- unknown_origin used as cleanup evidence
```

CI warn 条件：

```text
- graph_incomplete outside distribution scope
- review_required outside cleanup scope
- cycle detected in non-distribution experimental scope
```

---

## 22. dashboard display

Dashboard 表示候補：

```text
- graph validation status
- node / edge count
- unresolved source count
- restriction conflict count
- temporary bridge count
- distribution readiness impact
- cleanup impact
```

Dashboard は graph validation result を変更しない。

---

## 23. 禁止事項

以下を禁止する。

```text
- provenance graph valid を legal clearance と扱うこと
- temporary bridge node を canonical provenance と扱うこと
- graph cycle を無視して distribution-ready とすること
- restriction conflict を自動 override すること
- internal usable を distribution-ready と扱うこと
```

---

## 24. Studio AI feedback

Studio AI 側へのフィードバック：

```text
- assembly workflow は graph-first で扱うべき
- component-level provenance を必須候補にすべき
- temporary bridge は source of truth ではない
- distribution-ready には graph validation が必要
- tool / editor node を graph に含めるべき
```

---

## 25. HLDocS feedback

HLDocS 側へのフィードバック候補：

```text
- provenance graph validation は traceability model と接続可能
- multi-parent graph を扱える validation vocabulary が必要
- temporary bridge と canonical evidence を分離すべき
- distribution readiness と cleanup readiness を分離すべき
```

---

## 26. 結論

provenance graph validation model は、SansaVRM の rights provenance / component assembly / derivation / conversion / editor / tool provenance を graph として検証する model である。

これにより、Studio AI の multi-parent assembly workflow を、distribution governance、cleanup readiness、reconstruction comparison に安全に接続できる。
