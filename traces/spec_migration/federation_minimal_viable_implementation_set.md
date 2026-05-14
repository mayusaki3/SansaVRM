# federation minimal viable implementation set

## 1. 目的

本ドキュメントは、SansaVRM federation を Preview Federation として最小成立させるための minimal viable implementation set を定義する。

本セットは、production readiness を目的としない。

目的は、cross-project handoff、external artifact freshness、reconstruction delta、cleanup gate、dashboard projection を最小限の実装で検証可能にすることである。

---

## 2. 基本方針

Minimal viable implementation set は以下を満たす。

```text
- read-only を基本とする
- destructive cleanup を行わない
- apply execution を行わない
- validator / dashboard / CI artifact を優先する
- cross-project artifact を最小限扱う
- Preview Federation の known limitations を明示する
```

Minimal viable implementation set は以下を目的としない。

```text
- production cleanup
- automatic canonicalization apply
- downstream repository modification
- full governance automation
- complete long-term audit retention
```

---

## 3. MVP scope

MVP scope は以下とする。

```text
- SansaVRM repository internal migration state
- MuJoCo Adapter handoff response status
- Studio AI feedback response status
- external artifact placeholder metadata
- reconstruction delta tracking
- cleanup gate dry-run
- dashboard snapshot generation
- CI dry-run validation
```

---

## 4. MVP components

MVP components は以下とする。

```text
1. minimal index builder
2. minimal manifest validator
3. minimal canonicalization validator
4. minimal rewrite validator
5. minimal cleanup gate validator
6. minimal external artifact index
7. minimal federation validator
8. minimal dashboard snapshot generator
9. minimal CI dry-run workflow
10. minimal reconstruction delta registry
```

---

## 5. minimal index builder

最小 index builder は以下を生成する。

```text
- filesystem_index
- migration_index
- canonical_index
- rewrite_index
- external_artifact_index
- hash_index
```

初期段階では reference_index / traceability_index / sec_id_index は skeleton でもよい。

ただし、cleanup_ready 判定では skeleton index を pass 扱いしてはならない。

---

## 6. minimal manifest validator

最小 manifest validator は以下を検査する。

```text
- manifest loadability
- migration_entry_id duplicate
- dry_doc_id duplicate
- target_path existence
- placeholder relocation remaining
- migration_state unknown
```

出力：

```text
manifest_validator_report.json
manifest_validator_summary.md
```

---

## 7. minimal canonicalization validator

最小 canonicalization validator は以下を検査する。

```text
- document_fate unknown
- pending document_fate
- canonical_doc_id missing
- canonical_doc_id duplicate
- temporary dual canonical unresolved
- cleanup_allowed invalid
```

semantic_equivalent は unknown の場合 cleanup_ready 不可とする。

---

## 8. minimal rewrite validator

最小 rewrite validator は以下を検査する。

```text
- rewrite transaction exists where required
- transaction_status validity
- rewrite_state = validated / not_required
- operation_kind unknown
- affected_files mismatch
- rollback scope missing
```

reference / traceability / sec_id の詳細検査は skeleton でもよい。

ただし、未実装 index を理由に cleanup_ready を pass にしてはならない。

---

## 9. minimal cleanup gate validator

最小 cleanup gate validator は以下を判定する。

```text
cleanup_ready
cleanup_blocked
cleanup_pending
```

cleanup_ready 条件は厳しめにする。

```text
- manifest validator pass
- canonicalization validator pass
- rewrite validator pass または not_applicable
- no known unresolved references
- placeholder relocation なし
- temporary dual canonical なし
- required alias not pending
- no stale required external artifact
```

不明なものは cleanup_pending または cleanup_blocked とする。

---

## 10. minimal external artifact index

最小 external artifact index は以下を扱う。

```text
- artifact_id
- source_project
- artifact_kind
- path_or_url
- declared_stage
- freshness_status
- validation_required
- cleanup_impact
```

初期対象：

```text
- MuJoCo Adapter draft schema
- MuJoCo Adapter fixture placeholder
- Studio AI fixture placeholder
- cross-project handoff response documents
```

---

## 11. minimal federation validator

最小 federation validator は以下を検査する。

```text
- handoff response document exists
- external artifact stage is draft / experimental / preview / canonical のいずれか
- required external artifact freshness が unknown ではない
- draft artifact が canonical boundary に混入していない
- cross-project delta unresolved が cleanup scope にない
```

Production-level schema drift detection は MVP 範囲外でよい。

---

## 12. minimal dashboard snapshot generator

最小 dashboard snapshot は以下を表示する。

```text
- validator summary
- cleanup readiness summary
- blocking reason list
- reconstruction delta list
- external artifact freshness summary
- cross-project handoff status
```

Dashboard は source of truth ではない。

source_of_truth_refs は必須とする。

---

## 13. minimal CI dry-run workflow

CI では以下を実行する。

```text
1. index builder dry-run
2. manifest validator
3. canonicalization validator
4. rewrite validator
5. cleanup gate validator dry-run
6. federation validator minimal
7. dashboard snapshot artifact generation
```

CI では以下を禁止する。

```text
- apply execution
- cleanup execution
- downstream repository modification
- governance approval automation
```

---

## 14. minimal reconstruction delta registry

reconstruction delta registry は以下を記録する。

```text
- reconstruction_delta_id
- delta_kind
- source
- affected_scope
- status
- rerun_required
- cleanup_ready_invalidated
```

最小実装では Markdown / JSON のどちらでもよい。

ただし、CI / dashboard が読み取れる machine-readable 形式を優先する。

---

## 15. MVP non-goals

MVP の非対象：

```text
- full traceability validator
- full sec_id validator
- full distributed execution protocol
- automatic rollback execution
- production cleanup execution
- long-term audit retention implementation
- complete governance workflow automation
```

---

## 16. Preview Federation 完了条件

Preview Federation MVP の完了条件：

```text
- CI が validator summary を生成できる
- dashboard snapshot を artifact として生成できる
- cleanup_ready / blocked / pending を dry-run 判定できる
- cross-project handoff status を表示できる
- external artifact freshness を unknown / stale / fresh として分類できる
- reconstruction delta 発生時に rerun_required を表示できる
```

---

## 17. production 移行前に必要な追加

Production readiness 前に必要：

```text
- full reference validator
- full traceability validator
- full sec_id validator
- integrity / tamper validator implementation
- governance decision record implementation
- audit trail implementation
- recovery action request implementation
- federation execution protocol dry-run implementation
```

---

## 18. HLDocS feedback

本 MVP set から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction framework は最小 read-only MVP から始めるべき
- cleanup gate dry-run を early milestone にすべき
- dashboard projection は source of truth ではなく確認用 artifact とすべき
- unknown index / unknown freshness を pass として扱ってはならない
- production cleanup は MVP 範囲外にすべき
```

---

## 19. 結論

federation minimal viable implementation set は、SansaVRM federation を Preview Federation として最小成立させるための実装集合である。

これにより、destructive operation なしに、index、validator、cleanup gate、dashboard、cross-project handoff、external artifact freshness、reconstruction delta を検証できる。
