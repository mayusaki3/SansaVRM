# federation MVP artifact lifecycle specification

## 1. 目的

本ドキュメントは、SansaVRM federation MVP における artifact lifecycle を定義する。

本 lifecycle は Preview Federation MVP の範囲を対象とし、artifact の生成、更新、freshness 判定、stale 化、superseded 化、obsolete 化、cleanup 対象化を扱う。

本仕様は Production Federation の完全 lifecycle ではない。

---

## 2. 基本方針

artifact lifecycle は以下を扱う。

```text
- artifact の生成元を記録する
- artifact freshness を明示する
- artifact stage を明示する
- stale / superseded / obsolete を分離する
- cleanup dependency と artifact freshness を接続する
- dashboard / CI / validator が同じ artifact state を参照できるようにする
```

artifact lifecycle は以下を行わない。

```text
- artifact を自動修正しない
- artifact producer authority を代替しない
- stale artifact を fresh として扱わない
- draft artifact を canonical dependency として扱わない
- cleanup execution を実行しない
```

---

## 3. artifact kinds

MVP で扱う artifact_kind は以下とする。

```text
index_bundle
validator_report
dashboard_snapshot
dashboard_summary
external_artifact_registry
reconstruction_delta_registry
handoff_response_document
draft_schema
fixture
conversion_report
diagnostics_report
execution_report
cleanup_report
```

---

## 4. artifact stage

artifact stage は release lifecycle と接続する。

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

MVP では原則として以下を使用する。

```text
draft
experimental
preview
superseded
obsolete
```

`canonical_release` は MVP では使用しない。

---

## 5. artifact freshness status

freshness_status は以下とする。

```text
fresh
stale
unknown
not_applicable
```

### fresh

参照元 source / schema / registry / execution と整合している。

### stale

参照元が更新され、artifact の再生成または再検証が必要。

### unknown

fresh / stale を判定できない。

### not_applicable

freshness 判定の対象外。

---

## 6. artifact lifecycle state

artifact_lifecycle_state は以下とする。

```text
planned
generated
validated
published_preview
stale
superseded
obsolete
cleanup_pending
cleanup_blocked
cleanup_ready
archived
```

MVP では destructive cleanup を行わないため、`cleanup_ready` は dry-run 判定のみとする。

---

## 7. lifecycle transitions

主な許可遷移：

```text
planned → generated
generated → validated
validated → published_preview
published_preview → stale
published_preview → superseded
stale → generated
superseded → obsolete
obsolete → cleanup_pending
cleanup_pending → cleanup_ready
cleanup_pending → cleanup_blocked
```

MVP では以下を禁止する。

```text
cleanup_ready → deleted
obsolete → deleted
```

削除は Production cleanup phase の対象であり、MVP では dry-run のみとする。

---

## 8. stale condition

artifact は以下の場合に stale となる。

```text
- source registry が更新された
- schema_version が更新された
- source_project revision が更新された
- reconstruction delta が artifact scope に影響した
- validator module version が更新された
- handoff contract が更新された
- artifact dependency hash が変化した
```

stale artifact は cleanup scope に入る場合、cleanup_blocked の候補となる。

---

## 9. superseded condition

artifact は以下の場合に superseded となる。

```text
- replacement artifact が生成された
- reconstruction delta により旧 artifact が現行判断に使えなくなった
- schema / lifecycle / contract の新 version が導入された
- dashboard snapshot が新 snapshot に置き換えられた
```

superseded artifact は active 判定に使ってはならない。

ただし audit trail として保持してよい。

---

## 10. obsolete condition

artifact は以下の場合に obsolete となる。

```text
- downstream reference がない
- replacement artifact が active である
- cleanup gate が obsolete 扱いを許可している
- required retention policy を満たしている
```

obsolete artifact は cleanup candidate になり得る。

MVP では cleanup candidate 表示までとし、削除は行わない。

---

## 11. cleanup impact

cleanup_impact は以下とする。

```text
none
optional
required
unknown
```

### none

cleanup 判定に影響しない。

### optional

cleanup 判定の補助情報。

### required

cleanup 判定に必須。

### unknown

cleanup 影響が不明。

MVP では `required` または `unknown` の artifact freshness が unknown/stale の場合、cleanup_ready にしてはならない。

---

## 12. artifact registry entry

最小構造：

```json
{
  "artifact_id": "artifact-example",
  "artifact_kind": "draft_schema",
  "source_project": "SansaVRM-MuJoCo-Adapter",
  "declared_stage": "draft",
  "lifecycle_state": "generated",
  "freshness_status": "unknown",
  "cleanup_impact": "unknown",
  "source_refs": [],
  "dependency_hash": null,
  "replacement_artifact_id": null
}
```

---

## 13. artifact source refs

source_refs は artifact の根拠を示す。

候補：

```text
schema file
registry file
validator report
handoff response document
execution report
commit revision
external project reference
```

source_refs がない artifact は source_of_truth_refs missing として warn または fail とする。

---

## 14. validator report lifecycle

validator_report は以下の lifecycle を持つ。

```text
generated
validated
stale
superseded
archived
```

validator_report は source input hash と validator module version に依存する。

以下の場合 stale：

```text
- input index changed
- validator module version changed
- reason taxonomy changed
- strict mode changed
```

---

## 15. dashboard artifact lifecycle

Dashboard artifact は projection artifact である。

```text
generated
published_preview
stale
superseded
archived
```

Dashboard artifact が stale でも source of truth は変わらない。

ただし stale dashboard を active dashboard として表示してはならない。

---

## 16. external artifact lifecycle

external artifact は producer project に authority がある。

SansaVRM MVP は以下を記録する。

```text
- source_project
- artifact_kind
- declared_stage
- freshness_status
- cleanup_impact
- handoff contract relation
```

SansaVRM は external artifact を直接修正しない。

---

## 17. reconstruction delta impact

reconstruction delta は artifact lifecycle を変化させる。

例：

```text
validation_delta:
validator_report → stale

schema_delta:
draft_schema / fixture → stale

cleanup_delta:
cleanup_report / dashboard_snapshot → stale

cross_project_delta:
external artifact → stale / superseded candidate
```

---

## 18. CI mapping

CI fail 条件：

```text
- stale required artifact in cleanup scope
- unknown required artifact freshness in cleanup scope
- superseded artifact used as active
- dashboard artifact missing projection_only
- validator report stale but used as active
```

CI warn 条件：

```text
- stale optional artifact
- unknown optional artifact freshness
- obsolete artifact outside cleanup scope
- dashboard snapshot stale but regenerated in same run
```

---

## 19. dashboard display

Dashboard は artifact lifecycle を表示する。

表示対象：

```text
- artifact_id
- artifact_kind
- declared_stage
- lifecycle_state
- freshness_status
- cleanup_impact
- source_refs
- replacement_artifact_id
```

Dashboard は artifact lifecycle を変更しない。

---

## 20. 禁止事項

以下を禁止する。

```text
- stale artifact を active fresh として扱うこと
- superseded artifact を active 判定に使うこと
- draft artifact を canonical dependency として扱うこと
- required artifact freshness unknown のまま cleanup_ready にすること
- dashboard artifact を source of truth として扱うこと
- MVP で artifact deletion を実行すること
```

---

## 21. HLDocS feedback

本 lifecycle から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction artifact には lifecycle state が必要
- stale / superseded / obsolete を分離すべき
- artifact freshness と cleanup impact を接続すべき
- dashboard artifact は projection artifact として扱うべき
- reconstruction delta は artifact lifecycle を変化させるものとして扱うべき
```

---

## 22. 結論

federation MVP artifact lifecycle specification は、Preview Federation MVP における artifact の生成・検証・stale 化・superseded 化・obsolete 化・cleanup 候補化を定義する仕様である。

これにより、validator report、dashboard snapshot、external artifact、handoff response を同じ lifecycle vocabulary で扱える。
