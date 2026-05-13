# migration orchestration graph

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における migration orchestration graph を定義する。

migration orchestration graph は、migration / canonicalization / rewrite / validation / alias / cleanup の依存関係を実行制御用の graph として扱うためのモデルである。

Dashboard graph は表示用であるのに対し、本 graph は execution planning / propagation / rollback / validation scheduling のために使用する。

---

## 2. 基本方針

migration orchestration graph は以下を扱う。

```text
- document dependency
- migration dependency
- canonicalization dependency
- rewrite dependency
- validation dependency
- alias dependency
- cleanup dependency
- rollback dependency
- cross-project dependency
```

filesystem ordering は orchestration dependency ではない。

Layer 番号、dry-doc 番号、ファイル番号は、実行順序の根拠として扱わない。

---

## 3. dashboard graph との違い

```text
dashboard graph:
状態表示 / review / human-readable visualization

migration orchestration graph:
実行計画 / dependency propagation / scheduling / rollback planning
```

Dashboard は orchestration graph を表示してよい。

ただし、Dashboard は orchestration graph を変更してはならない。

---

## 4. node kinds

orchestration graph の node_kind は以下とする。

```text
document_node
manifest_node
canonicalization_node
rewrite_transaction_node
validation_node
alias_node
cleanup_node
cross_project_handoff_node
external_artifact_node
```

### document_node

仕様文書、testspec、prompt、schema、report などの文書単位。

### manifest_node

migration manifest、canonicalization manifest、legacy alias manifest などの manifest 単位。

### canonicalization_node

document fate decision / canonical_doc_id decision を表す。

### rewrite_transaction_node

rewrite transaction model に基づく実行単位。

### validation_node

validator module または validator run を表す。

### alias_node

legacy alias 生成・維持・失効を表す。

### cleanup_node

cleanup gate および cleanup execution の単位。

### cross_project_handoff_node

MuJoCo Adapter / Studio AI などの cross-project handoff response / dependency を表す。

### external_artifact_node

Adapter output、Studio AI fixture、CI artifact などの外部成果物を表す。

---

## 5. edge kinds

edge_kind は以下とする。

```text
semantic_depends_on
references
relocates_to
canonicalizes_to
rewrites
validates
requires_alias
blocks_cleanup_of
requires_validation_of
requires_rewrite_of
requires_handoff_response
produces_artifact
consumes_artifact
rollback_depends_on
supersedes
```

### semantic_depends_on

semantic dependency を表す。

### references

文書・schema・code が他対象を参照していることを表す。

### relocates_to

旧 path から新 path への migration を表す。

### canonicalizes_to

dry-doc / old_doc_id から canonical_doc_id への収束を表す。

### rewrites

rewrite transaction が対象を変更することを表す。

### validates

validator が対象を検査することを表す。

### requires_alias

cleanup 前に alias が必要であることを表す。

### blocks_cleanup_of

ある node が別 node の cleanup を block していることを表す。

### requires_validation_of

実行前に validation が必要であることを表す。

### requires_rewrite_of

実行前に rewrite が必要であることを表す。

### requires_handoff_response

cross-project handoff response が必要であることを表す。

### produces_artifact / consumes_artifact

外部 artifact の入出力関係を表す。

### rollback_depends_on

rollback 時に連動して戻す必要があることを表す。

### supersedes

後続 node が前の node を置き換えることを表す。

---

## 6. graph schema draft

```json
{
  "schema_version": "1.0",
  "orchestration_graph_id": "orchestration-YYYYMMDD-NNN",
  "source_migration_id": "migration-YYYYMMDD-NNN",
  "canonicalization_id": "canonicalization-YYYYMMDD-NNN",
  "nodes": [
    {
      "node_id": "node-0001",
      "node_kind": "document_node",
      "identity": {
        "dry_doc_id": "dry-doc-example",
        "old_doc_id": "doc-old",
        "canonical_doc_id": "doc-new",
        "path": "docs/ja-JP/example.md"
      },
      "state": {
        "migration_state": "migration_verified",
        "canonicalization_state": "completed",
        "rewrite_state": "rewrite_validated",
        "cleanup_state": "cleanup_pending"
      }
    }
  ],
  "edges": [
    {
      "edge_id": "edge-0001",
      "edge_kind": "requires_validation_of",
      "source_node_id": "node-validator-0001",
      "target_node_id": "node-0001",
      "blocking": true
    }
  ]
}
```

---

## 7. dependency propagation

orchestration graph は dependency propagation を行う。

伝播対象：

```text
- canonicalization impact
- rewrite impact
- validation impact
- alias impact
- cleanup impact
- rollback impact
- cross-project impact
```

伝播は source node から edge に沿って行う。

ただし、semantic_depends_on と filesystem ordering を混同してはならない。

---

## 8. cleanup propagation

cleanup_ready は node 単体ではなく、依存 graph 上で判定する。

cleanup が block される例：

```text
- unresolved reference が downstream node に残る
- alias が required だが generated ではない
- rewrite transaction が validated ではない
- validator node が fail または blocked
- cross-project handoff response が未完了
- external artifact の再検証が未完了
```

cleanup propagation では、`blocks_cleanup_of` edge を使用する。

---

## 9. rewrite propagation

rewrite transaction が発生した場合、以下へ影響を伝播する。

```text
- direct target document
- reference source documents
- traceability-linked testspec
- traceability-linked code
- dashboard snapshot input
- validator scheduling
- legacy alias requirement
- cleanup gate input
```

rewrite propagation の結果、追加 rewrite transaction が必要になる場合は、dependent rewrite transaction として graph に追加する。

---

## 10. rollback propagation

rollback は transaction 単体ではなく、graph 上の影響範囲で評価する。

rollback propagation の対象：

```text
- dependent rewrite transaction
- generated alias
- validator report
- dashboard snapshot
- cleanup gate result
- external artifact
```

rollback_depends_on edge が存在する場合、source transaction の rollback は dependent node の rollback または invalidation を要求する。

---

## 11. validation scheduling

validator scheduling は orchestration graph から生成する。

基本順序：

```text
1. manifest validation
2. canonicalization validation
3. rewrite validation
4. reference validation
5. traceability validation
6. sec_id validation
7. alias validation
8. cleanup gate validation
9. dashboard projection validation
```

ただし、graph 上で影響がない module は selective validation により省略可能とする。

---

## 12. execution batching

execution batch は、依存関係が閉じた subgraph として定義する。

batch 条件：

```text
- unresolved incoming blocking edge がない
- required validation が preflight pass している
- rollback scope が subgraph 内で閉じている、または external rollback rule が定義されている
- cleanup impact が既知である
```

初期実行では、小さい closed_dependency_scope を batch とする。

---

## 13. retry boundary

retry boundary は execution batch 単位で定義する。

retry 可能条件：

```text
- failed node が representation transaction のみである
- semantic transaction が未確定または rollback 済みである
- validator report が failed state を明示している
- dependent cleanup が未実行である
```

retry 禁止条件：

```text
- cleanup execution 済み
- semantic transaction が partial committed
- rollback scope が失われている
- external artifact が上書きされて追跡不能
```

---

## 14. cross-project dependency

MuJoCo Adapter / Studio AI などの cross-project handoff は graph node として扱う。

例：

```text
SansaVRM Adapter JSON spec
  ↓ requires_handoff_response
MuJoCo Adapter draft schema
  ↓ consumes_artifact
Adapter fixture validation
```

Studio AI では以下を graph 化できる。

```text
layered asset structure decision
  ↓
Studio AI fixture / workflow
  ↓
export profile validation
```

cross-project dependency は cleanup を block する場合がある。

---

## 15. graph validation

orchestration graph validator は以下を検査する。

```text
- node_id が一意である
- edge_id が一意である
- source_node_id / target_node_id が存在する
- edge_kind が許容値である
- blocking edge に解決条件がある
- rollback_depends_on edge に rollback rule がある
- cleanup_ready node に unresolved blocking edge がない
- cross_project_handoff_node に response status がある
- external_artifact_node に source / freshness / validation state がある
```

---

## 16. dashboard への projection

orchestration graph から dashboard graph へ projection する。

projection 対象：

```text
- node state
- edge state
- blocking reason
- validation state
- cleanup impact
- rollback impact
```

ただし、dashboard graph は orchestration graph の source of truth ではない。

---

## 17. 禁止事項

以下を禁止する。

```text
- filesystem ordering を orchestration dependency として扱うこと
- dashboard graph を execution source of truth として扱うこと
- cleanup_ready を node 単体で判定すること
- rollback scope が閉じていない batch を実行すること
- cross-project handoff 未回答のまま downstream contract を確定すること
- external artifact の freshness 未確認のまま validation pass とすること
```

---

## 18. HLDocS feedback

本 graph model から、HLDocS 側へ以下をフィードバックする。

```text
- migration orchestration graph と dashboard graph を分離すべき
- cleanup_ready は graph dependency を含めて判定すべき
- rewrite / validation / rollback / cleanup の propagation を明示すべき
- cross-project handoff を dependency node として扱えるべき
- filesystem ordering を execution ordering と誤用しない仕組みが必要
```

---

## 19. 結論

migration orchestration graph は、SansaVRM の大規模仕様再構成における実行制御用 graph である。

これにより、canonicalization、rewrite、validation、legacy alias、cleanup、rollback、cross-project handoff を、単一の依存関係モデルとして扱える。
