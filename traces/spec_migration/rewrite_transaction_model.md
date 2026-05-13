# rewrite transaction model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における rewrite transaction model を定義する。

rewrite transaction は、canonicalization manifest により確定した document fate decision を、実ファイル・参照・traceability・検証状態へ安全に反映するための実行単位である。

本モデルは、canonicalization manifest / canonicalization report / cleanup gate の間を接続する。

---

## 2. 基本方針

rewrite transaction は、以下を混同しない。

```text
semantic transaction:
canonical identity / document fate / traceability 意味の変更

representation transaction:
path / filename / alias / formatting / filesystem ordering の変更
```

semantic transaction と representation transaction は、同じ実行計画に含まれてよいが、validator と cleanup gate では別々に判定する。

---

## 3. transaction の位置づけ

rewrite transaction は、document fate decision 後、cleanup gate 前に実行する。

```text
planned_relocation
  ↓
dry-doc assigned
  ↓
relocation / split / merge
  ↓
semantic verification
  ↓
document fate decision
  ↓
canonicalization manifest
  ↓
rewrite planner
  ↓
rewrite transaction
  ↓
rewrite validator
  ↓
canonicalization report
  ↓
legacy alias phase
  ↓
cleanup gate
```

---

## 4. transaction boundary

rewrite transaction の境界は、以下の単位で定義する。

```text
- canonicalization_id
- source_migration_id
- rewrite_transaction_id
- target document set
- affected reference set
- affected traceability set
- affected testspec/code set
```

1つの rewrite transaction は、複数文書にまたがってよい。

ただし、cross-document consistency を保てない単位で分割してはならない。

---

## 5. transaction kinds

rewrite transaction は以下に分類する。

```text
identity_rewrite
reference_rewrite
sec_id_rewrite
traceability_rewrite
testspec_rewrite
code_reference_rewrite
path_rewrite
legacy_alias_rewrite
cleanup_preparation
```

### identity_rewrite

canonical_doc_id の確定または変更を反映する。

### reference_rewrite

文書間リンク、仕様参照、旧path参照を canonical target へ更新する。

### sec_id_rewrite

sec_id の preserve / map / split / merge / remove を反映する。

### traceability_rewrite

仕様・テスト・コードの紐づけ情報を canonical_doc_id / sec_id に合わせて更新する。

### testspec_rewrite

testspec 側の参照対象を更新する。

### code_reference_rewrite

コードまたは設定ファイル内の仕様参照を更新する。

### path_rewrite

filesystem ordering / migration grouping に基づく path を更新する。

### legacy_alias_rewrite

旧 doc_id / 旧 path / 旧参照を互換参照として残す。

### cleanup_preparation

cleanup gate に渡すための状態を記録する。

---

## 6. rewrite planner

rewrite planner は canonicalization manifest を入力とし、rewrite transaction plan を生成する。

入力：

```text
- canonicalization manifest
- migration manifest federation
- reference index
- traceability index
- sec_id index
- testspec/code reference index
- current filesystem snapshot
```

出力：

```text
- rewrite_transaction_id
- ordered rewrite operations
- affected files
- expected canonical state
- rollback scope
- validator requirements
- cleanup gate prerequisites
```

---

## 7. rewrite operation schema draft

```json
{
  "rewrite_transaction_id": "rewrite-YYYYMMDD-NNN",
  "canonicalization_id": "canonicalization-YYYYMMDD-NNN",
  "source_migration_id": "migration-YYYYMMDD-NNN",
  "transaction_status": "planned",
  "operations": [
    {
      "operation_id": "rewrite-op-0001",
      "operation_kind": "reference_rewrite",
      "target_file": "docs/ja-JP/example.md",
      "source_identity": "dry-doc-example",
      "old_value": "doc-old",
      "new_value": "doc-new",
      "rewrite_scope": "document_reference",
      "requires_validation": true,
      "rollback_action": "restore_old_value"
    }
  ]
}
```

---

## 8. transaction status

transaction_status の許容値：

```text
planned
ready
executing
executed
validating
validated
blocked
failed
rolled_back
superseded
```

### planned

rewrite plan が作成された状態。

### ready

必要な入力が揃い、実行可能な状態。

### executing

rewrite を実行中の状態。

### executed

rewrite は完了したが、validator 未確認の状態。

### validating

rewrite validator による確認中の状態。

### validated

rewrite validator が PASS した状態。

### blocked

必要な入力、参照、判断が不足している状態。

### failed

rewrite または検証に失敗した状態。

### rolled_back

rollback が実行された状態。

### superseded

後続 transaction により置き換えられた状態。

---

## 9. atomicity

rewrite transaction は、以下の atomicity を満たす必要がある。

```text
- canonical_doc_id rewrite と traceability rewrite は不整合な中間状態で確定してはならない
- sec_id rewrite と testspec rewrite は同一 transaction または明示された dependent transaction とする
- path rewrite と legacy alias rewrite は cleanup gate 前に整合していなければならない
- drop / obsolete に関する rewrite は unresolved references が残る場合 validated にしてはならない
```

---

## 10. rollback scope

rollback scope は operation ごとではなく transaction ごとに定義する。

rollback scope には以下を含める。

```text
- changed files
- old values
- new values
- generated aliases
- generated reports
- updated indexes
- validator outputs
```

rollback は semantic transaction を優先して戻す。

representation transaction のみを戻して semantic transaction を残してはならない。

---

## 11. ordering rules

rewrite operation の基本順序は以下とする。

```text
1. identity_rewrite
2. sec_id_rewrite
3. traceability_rewrite
4. reference_rewrite
5. testspec_rewrite
6. code_reference_rewrite
7. path_rewrite
8. legacy_alias_rewrite
9. cleanup_preparation
10. rewrite validation
```

ただし、path rewrite を先に行わないと参照更新できない場合は、temporary path mapping を transaction 内に明示する。

---

## 12. validator requirements

rewrite validator は以下を検査する。

```text
- rewrite_transaction_id が一意である
- canonicalization_id が存在する
- source_migration_id が存在する
- operation_kind が許容値である
- target_file が存在する、または create/delete として明示されている
- old_value が rewrite 前 snapshot と一致する
- new_value が rewrite 後 snapshot に存在する
- canonical_doc_id の重複がない
- sec_id collision がない
- traceability reference が解決できる
- testspec/code reference が解決できる
- drop / obsolete 対象に未解決参照が残っていない
- legacy alias が必要な場合に生成済みである
- cleanup_preparation が cleanup gate 条件と矛盾しない
```

---

## 13. dashboard state との関係

rewrite transaction は dashboard に以下の状態を提供する。

```text
rewrite_planned
rewrite_ready
rewrite_executing
rewrite_executed
rewrite_validating
rewrite_validated
rewrite_blocked
rewrite_failed
rewrite_rolled_back
```

既存の migration lifecycle state とは分離する。

```text
canonicalized:
canonicalization manifest 上の identity decision が完了した状態

rewrite_validated:
実ファイル・参照・traceability への反映が validator PASS した状態
```

`canonicalized` は `rewrite_validated` を意味しない。

---

## 14. cleanup gate との関係

cleanup gate へ進める条件は、canonicalization 完了だけでは不十分である。

cleanup gate へ進めるには、少なくとも以下が必要である。

```text
- canonicalization_status = completed または not_required
- rewrite transaction status = validated
- unresolved references = 0
- required legacy alias = generated または not_required
- rewrite validator = PASS
- federation validator = PASS
- CI validation = PASS
```

---

## 15. 禁止事項

以下を禁止する。

```text
- validator が rewrite を実行すること
- cleanup gate が rewrite を実行すること
- dashboard が canonical state を変更すること
- canonicalization manifest 未確定のまま doc_id を書き換えること
- pending document fate の文書を cleanup_ready にすること
- representation rewrite の成功だけで semantic rewrite 完了とみなすこと
```

---

## 16. HLDocS feedback

本モデルから、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization manifest と cleanup gate の間に rewrite transaction model が必要
- semantic transaction と representation transaction を分離すべき
- doc_id / sec_id / traceability / testspec / code reference の更新は transaction として扱うべき
- validator は rewrite executor ではなく observer / gate として扱うべき
- canonicalized と rewrite_validated は別状態として扱うべき
- rollback scope を transaction 単位で定義すべき
```

---

## 17. 結論

rewrite transaction model は、document fate decision を実ファイル・参照・traceability へ安全に反映するための実行モデルである。

これにより、canonical identity の確定、参照更新、sec_id 更新、testspec/code 更新、legacy alias、cleanup gate を不整合なく接続できる。
