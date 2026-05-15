# mixed reconstruction governance detail

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における mixed reconstruction governance detail を定義する。

mixed reconstruction は、旧構成と新構成を明確に分離できず、同一 namespace / path / document set 内で旧要素と新要素が混在する再構成である。

本 detail は、mixed reconstruction における boundary freeze、partial overwrite、temporary coexistence、contamination detection、cleanup hold の扱いを整理する。

---

## 2. 基本方針

mixed reconstruction governance は以下を扱う。

```text
- shared namespace の管理
- partial overwrite の管理
- old/new element tagging
- boundary freeze
- temporary dual state
- contamination detection
- rollback / restore planning
- comparison before cleanup
```

mixed reconstruction governance は以下を行わない。

```text
- mixed state を separated state として扱わない
- partial overwrite を completed rewrite として扱わない
- temporary dual state を canonical completed として扱わない
- comparison 前に old elements を削除しない
- cleanup_hold を無視しない
```

---

## 3. mixed reconstruction が発生する条件

mixed reconstruction は以下の場合に発生する。

```text
- old/new が同じ directory に存在する
- old document を直接編集して new structure に変換する
- old section と new section が同一 document 内に共存する
- staged replacement により一部だけ new semantics になる
- reference rewrite が一括完了せず、old/new references が混在する
- canonical_doc_id は新しいが path は旧のまま残る
```

---

## 4. mixed scope taxonomy

mixed scope は以下に分類する。

```text
path_mixed_scope
document_mixed_scope
section_mixed_scope
reference_mixed_scope
traceability_mixed_scope
artifact_mixed_scope
```

### path_mixed_scope

同一 directory / path tree に old/new artifact が混在する。

### document_mixed_scope

同一 document 内に old/new semantics が混在する。

### section_mixed_scope

同一 document の章単位で old/new が混在する。

### reference_mixed_scope

old reference と new reference が同時に存在する。

### traceability_mixed_scope

old doc_id / sec_id / ref_id と new id が混在する。

### artifact_mixed_scope

old artifact と new artifact が同じ registry / dashboard / report に混在する。

---

## 5. required tagging

mixed reconstruction では、対象要素に old/new/mixed の tagging が必要である。

最小 tag：

```text
old_element
new_element
mixed_element
temporary_bridge
legacy_alias
unknown_origin
```

`unknown_origin` は cleanup_ready の根拠にしてはならない。

---

## 6. boundary freeze

mixed reconstruction では、作業開始前に boundary freeze を行う。

freeze 対象：

```text
- mixed scope
- old element list
- new element list
- shared namespace
- temporary bridge elements
- allowed overwrite range
- forbidden overwrite range
- rollback restore point
```

boundary freeze なしに partial overwrite を開始してはならない。

---

## 7. partial overwrite governance

partial overwrite は、旧構成を直接書き換えながら新構成へ移行する操作である。

partial overwrite では以下を必須とする。

```text
- overwrite target を明示する
- overwrite before hash を記録する
- overwrite after expected hash を記録する
- rollback restore point を記録する
- affected references を記録する
- comparison_required を true にする
```

partial overwrite 完了だけでは reconstruction completed ではない。

---

## 8. temporary bridge

temporary bridge は old/new を一時的に接続する要素である。

例：

```text
- legacy alias
- temporary redirect
- old/new mapping table
- compatibility note
- bridge index entry
```

temporary bridge は source of truth ではない。

cleanup 前に bridge の扱いを決める必要がある。

---

## 9. temporary dual state

temporary dual state は、old と new が同時に active に見える状態である。

例：

```text
- old_doc_id と canonical_doc_id が同一意味で並存する
- old path と new path が同一 document を指す
- old section と new section が同一 semantics を持つ
```

temporary dual state は comparison / validator scope では許容される場合がある。

ただし cleanup / canonical release scope では解消が必要である。

---

## 10. contamination guard

mixed reconstruction では contamination guard を必須とする。

検査：

```text
- old reference が new canonical boundary に残っていないか
- new draft reference が old stable boundary に入り込んでいないか
- unknown_origin が cleanup target に含まれていないか
- partial overwrite artifact が active canonical evidence になっていないか
- temporary bridge が source of truth として使われていないか
```

---

## 11. cleanup hold conditions

mixed reconstruction では以下の場合 cleanup_hold とする。

```text
- boundary freeze がない
- partial overwrite rollback point がない
- unknown_origin が残っている
- temporary dual state が unresolved
- contamination finding が unresolved
- comparison_required が true で未完了
- temporary bridge policy が未定
```

---

## 12. comparison requirements

mixed reconstruction の comparison では、以下を比較する。

```text
- old element と new element の semantic equivalence
- partial overwrite 前後の差分
- section-level semantics
- reference rewrite completeness
- traceability mapping completeness
- temporary bridge の必要性
- old element 削除可否
```

比較調査なしに old element を削除してはならない。

---

## 13. rollback requirements

mixed reconstruction の rollback は separated reconstruction より厳格に扱う。

必要：

```text
- overwrite before hash
- restore point
- affected references
- temporary bridge rollback
- mixed registry rollback
- dashboard / report stale handling
```

rollback scope が不明な partial overwrite は apply してはならない。

---

## 14. mixed reconstruction registry

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "mixed_reconstruction_registry",
  "mixed_scopes": [
    {
      "mixed_scope_id": "mixed-scope-example",
      "scope_kind": "document_mixed_scope",
      "old_elements": [],
      "new_elements": [],
      "temporary_bridges": [],
      "boundary_frozen": false,
      "comparison_required": true,
      "cleanup_hold": true
    }
  ]
}
```

---

## 15. validator requirements

mixed reconstruction では以下の validator が必要である。

```text
- boundary validator
- contamination validator
- partial overwrite validator
- temporary dual state validator
- bridge policy validator
- comparison readiness validator
```

MVP ではこれらは reason code / registry / dashboard 表示まででもよい。

---

## 16. CI mapping

CI fail 条件：

```text
- partial overwrite without boundary freeze
- partial overwrite without rollback point
- unknown_origin used as cleanup evidence
- temporary bridge used as source of truth
- mixed reconstruction treated as separated
- cleanup_ready while cleanup_hold=true
```

CI warn 条件：

```text
- temporary dual state in comparison scope
- comparison_required=true
- temporary bridge exists outside cleanup scope
```

---

## 17. dashboard display

Dashboard は mixed reconstruction を表示する。

表示対象：

```text
- mixed scope kind
- old/new/mixed element counts
- boundary freeze status
- partial overwrite status
- temporary bridge status
- contamination findings
- cleanup hold reason
- comparison status
```

Dashboard は mixed scope を separated scope に変換してはならない。

---

## 18. 禁止事項

以下を禁止する。

```text
- mixed reconstruction を separated reconstruction として扱うこと
- boundary freeze なしに partial overwrite を行うこと
- rollback point なしに partial overwrite を行うこと
- temporary bridge を source of truth として扱うこと
- comparison 前に old element を削除すること
- cleanup_hold 中に cleanup execution へ進むこと
```

---

## 19. HLDocS feedback

本 detail から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction には mixed reconstruction governance が必要
- old/new を分離できない場合の boundary freeze が必要
- partial overwrite は rollback point と comparison_required を必須にすべき
- temporary bridge は source of truth ではないと明記すべき
- mixed reconstruction は cleanup_hold を強く要求すべき
```

---

## 20. 結論

mixed reconstruction governance detail は、old/new が同一 namespace / path / document set 内に混在する再構成を安全に扱うための governance detail である。

これにより、partial overwrite、temporary dual state、temporary bridge、contamination risk を明示し、比較調査と cleanup readiness を経るまで旧要素を削除しない運用を可能にする。
