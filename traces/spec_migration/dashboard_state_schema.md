# dashboard state schema

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における dashboard state/schema model を定義する。

dashboard は migration graph viewer として、relocation / canonicalization / rewrite / validation / cleanup の進行状態を可視化する。

本ドキュメントは、dashboard を state mutator ではなく observer として扱うための状態分離を定義する。

---

## 2. 基本方針

Dashboard は以下を行う。

```text
- migration graph の表示
- document fate decision の表示
- canonicalization 状態の表示
- rewrite transaction 状態の表示
- validator 結果の表示
- cleanup gate 状態の表示
- blocking reason の表示
```

Dashboard は以下を行わない。

```text
- doc_id の変更
- sec_id の変更
- traceability の変更
- rewrite の実行
- validator の代替
- cleanup gate の代替
```

Dashboard は observer であり、canonical state の source of truth ではない。

---

## 3. state domain separation

Dashboard は以下の state domain を分離して表示する。

```text
migration_state:
relocation / dry-doc / migration manifest 上の状態

fate_state:
document fate decision の状態

canonicalization_state:
canonical identity 確定状態

rewrite_state:
実ファイル・参照・traceability 反映状態

validation_state:
validator / CI / integrity 検証状態

alias_state:
legacy alias readiness / generated state

cleanup_state:
cleanup gate 判定状態
```

1つの文書または migration node は、複数 state domain を同時に持つ。

---

## 4. lifecycle overview

Dashboard 上の概念的 lifecycle は以下とする。

```text
planned
  ↓
relocating
  ↓
pending_decision
  ↓
fate_decided
  ↓
canonicalized
  ↓
rewrite_planned
  ↓
rewrite_validated
  ↓
legacy_alias_ready
  ↓
cleanup_ready
  ↓
cleaned
```

ただし、これは表示用の主要経路であり、内部 state domain を単一状態へ潰してはならない。

---

## 5. migration_state

migration_state の許容値：

```text
not_registered
planned
relocating
relocated
placeholder_only
migrated_partial
migration_verified
migration_blocked
```

### not_registered

migration manifest に未登録の状態。

### planned

relocation が計画された状態。

### relocating

relocation 実行中の状態。

### relocated

relocation が完了したが、semantic verification 前の状態。

### placeholder_only

placeholder relocation のみ存在する状態。

### migrated_partial

一部のみ migration された状態。

### migration_verified

migration manifest と実ファイル状態が一致している状態。

### migration_blocked

migration manifest または実ファイル状態に不整合がある状態。

---

## 6. fate_state

fate_state の許容値：

```text
not_decided
maintain_original_doc_id
issue_new_doc_id
merge_into_other_document
split_into_multiple_documents
obsolete
drop
pending
blocked
```

fate_state は canonicalization 前に確定する。

`pending` または `blocked` の文書は cleanup_ready へ進めない。

---

## 7. canonicalization_state

canonicalization_state の許容値：

```text
not_required
pending
completed
blocked
conflicted
```

### completed

canonicalization manifest 上で canonical_doc_id が確定している状態。

### conflicted

canonical_doc_id の重複、旧 canonical との競合、または temporary dual canonical state が未整理の状態。

重要：

```text
canonicalized
≠
rewrite_validated
```

canonicalization_state が completed であっても、実ファイル・参照・traceability への反映が完了しているとは限らない。

---

## 8. rewrite_state

rewrite_state の許容値：

```text
not_required
rewrite_planned
rewrite_ready
rewrite_executing
rewrite_executed
rewrite_validating
rewrite_validated
rewrite_blocked
rewrite_failed
rewrite_rolled_back
rewrite_superseded
```

rewrite_state は rewrite transaction model に従う。

`rewrite_executed` は validator 未確認状態であり、cleanup gate へ進めない。

cleanup gate へ進めるには `rewrite_validated` が必要である。

---

## 9. validation_state

validation_state の許容値：

```text
not_run
pass
warn
fail
blocked
not_applicable
```

Dashboard は少なくとも以下を分けて表示する。

```text
- semantic verification
- rewrite validator
- federation validator
- integrity validator
- CI validation
```

`pass` は対象 validator の pass であり、全体 pass を意味しない。

---

## 10. alias_state

alias_state の許容値：

```text
not_required
required
ready
generated
blocked
expired
```

legacy alias が必要な文書では、cleanup_ready 前に `generated` である必要がある。

---

## 11. cleanup_state

cleanup_state の許容値：

```text
not_evaluated
cleanup_pending
cleanup_ready
cleanup_blocked
cleaning
cleaned
cleanup_failed
```

cleanup_ready は dashboard が決める状態ではない。

cleanup gate の判定結果を dashboard が表示する。

---

## 12. dashboard node schema draft

```json
{
  "schema_version": "1.0",
  "dashboard_snapshot_id": "dashboard-YYYYMMDD-NNN",
  "source_migration_id": "migration-YYYYMMDD-NNN",
  "canonicalization_id": "canonicalization-YYYYMMDD-NNN",
  "generated_at": "YYYY-MM-DDTHH:MM:SSZ",
  "nodes": [
    {
      "node_id": "dashboard-node-0001",
      "dry_doc_id": "dry-doc-example",
      "migration_entry_id": "migration-entry-0001",
      "old_doc_id": "doc-old",
      "canonical_doc_id": "doc-new",
      "current_path": "docs/ja-JP/example.md",
      "previous_paths": [
        "docs/ja-JP/old-example.md"
      ],
      "migration_state": "migration_verified",
      "fate_state": "issue_new_doc_id",
      "canonicalization_state": "completed",
      "rewrite_state": "rewrite_validated",
      "validation_state": {
        "semantic_verification": "pass",
        "rewrite_validator": "pass",
        "federation_validator": "pass",
        "integrity_validator": "pass",
        "ci_validation": "pass"
      },
      "alias_state": "generated",
      "cleanup_state": "cleanup_ready",
      "blocking_reasons": [],
      "warnings": []
    }
  ]
}
```

---

## 13. dashboard edge schema draft

Dashboard は node 間の関係を edge として表示する。

```json
{
  "edges": [
    {
      "edge_id": "dashboard-edge-0001",
      "edge_kind": "split_from",
      "source_node_id": "dashboard-node-0001",
      "target_node_id": "dashboard-node-0002",
      "relationship_source": "canonicalization_manifest",
      "validation_state": "pass",
      "blocking_reasons": []
    }
  ]
}
```

edge_kind の許容値：

```text
relocated_to
merged_into
split_from
supersedes
aliases
references
blocks_cleanup_of
depends_on
```

---

## 14. blocking reason taxonomy

Dashboard は blocking reason を分類して表示する。

```text
manifest_missing
placeholder_relocation_remaining
migrated_partial
semantic_equivalent_unknown
fate_not_decided
canonical_doc_id_missing
canonical_doc_id_collision
rewrite_not_validated
rewrite_failed
sec_id_collision
traceability_unresolved
testspec_reference_unresolved
code_reference_unresolved
legacy_alias_not_generated
federation_validator_failed
integrity_validator_failed
ci_validation_failed
cleanup_gate_failed
```

blocking reason は source domain を持つ。

```text
source_domain:
migration | fate | canonicalization | rewrite | validation | alias | cleanup
```

---

## 15. aggregate summary schema draft

Dashboard は全体集計を表示する。

```json
{
  "summary": {
    "total_nodes": 0,
    "migration_blocked": 0,
    "fate_pending": 0,
    "canonicalization_completed": 0,
    "rewrite_validated": 0,
    "cleanup_ready": 0,
    "cleanup_blocked": 0,
    "cleaned": 0,
    "warn_count": 0,
    "fail_count": 0
  }
}
```

---

## 16. graph view requirements

Dashboard graph view は以下を表示できる必要がある。

```text
- dry-doc から canonical_doc_id への対応
- old path から current path への移動
- merge / split の関係
- obsolete / drop の候補
- unresolved reference の方向
- cleanup を block している node
- legacy alias が必要な node
- validator FAIL の発生箇所
```

---

## 17. state transition restrictions

以下の遷移を禁止する。

```text
pending_decision → cleanup_ready
canonicalization_state=pending → rewrite_validated
rewrite_state=rewrite_executed → cleanup_ready
validation_state=fail → cleanup_ready
alias_state=required → cleanup_ready
cleanup_state=cleanup_ready → cleaned without cleanup execution
```

Dashboard はこれらを検出した場合、表示上 `state_transition_violation` として扱う。

---

## 18. source of truth

Dashboard の source of truth は以下とする。

```text
migration_state:
migration manifest federation

fate_state:
canonicalization manifest document_fate

canonicalization_state:
canonicalization manifest / canonicalization validator

rewrite_state:
rewrite transaction report

validation_state:
validator reports / CI result

alias_state:
legacy alias manifest / rewrite transaction report

cleanup_state:
cleanup gate report
```

Dashboard snapshot 自体を source of truth として扱ってはならない。

---

## 19. HLDocS feedback

本 schema から、HLDocS 側へ以下をフィードバックする。

```text
- dashboard は state mutator ではなく observer として扱うべき
- migration / fate / canonicalization / rewrite / validation / alias / cleanup の state domain を分離すべき
- canonicalized と rewrite_validated を別状態として扱うべき
- cleanup_ready は dashboard ではなく cleanup gate の判定結果として扱うべき
- blocking reason は source domain 付きで分類すべき
- graph view では merge / split / alias / unresolved reference を edge として扱うべき
```

---

## 20. 結論

Dashboard は、大規模仕様再構成の状態を統合表示する migration graph viewer である。

Dashboard は canonical state を変更せず、各 manifest / validator / cleanup gate の結果を state domain ごとに分離して表示する。

これにより、planned relocation、document fate decision、canonicalization、rewrite transaction、legacy alias、cleanup gate を混同せずに追跡できる。
