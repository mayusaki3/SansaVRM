# cleanup execution plan

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における cleanup execution plan を定義する。

cleanup execution は、cleanup gate validator により cleanup_ready と判定された対象について、旧 path、placeholder、temporary state、obsolete artifact、期限切れ legacy alias を安全に削除または無効化する実行フェーズである。

cleanup execution は cleanup gate validator の出力を前提とし、独自に cleanup_ready を判定しない。

---

## 2. 基本方針

cleanup execution は以下を行う。

```text
- cleanup_ready 対象のみを処理する
- cleanup execution batch を作成する
- cleanup dry-run を実行する
- cleanup impact を確認する
- cleanup を実行する
- cleanup 後 validator を実行する
- cleanup report を生成する
```

cleanup execution は以下を行わない。

```text
- cleanup_ready を独自判定しない
- canonical_doc_id を変更しない
- sec_id を生成しない
- rewrite transaction を代替しない
- unresolved reference を自動修正しない
- cleanup_blocked 対象を処理しない
```

---

## 3. 入力

cleanup execution の入力は以下とする。

```text
- cleanup gate validator report
- cleanup impact summary
- migration orchestration graph
- dashboard snapshot
- filesystem_index
- alias_index
- reference_index
- traceability_index
- external_artifact_index
- rollback plan
- CI context
```

cleanup gate validator report が存在しない場合、cleanup execution を開始してはならない。

---

## 4. 出力

cleanup execution は以下を出力する。

```text
- cleanup execution plan
- cleanup dry-run report
- cleanup execution report
- cleanup rollback package
- post-cleanup validator report
- post-cleanup dashboard snapshot input
```

---

## 5. cleanup target kinds

cleanup 対象は以下に分類する。

```text
old_path
placeholder_file
temporary_canonical_file
obsolete_document
expired_legacy_alias
stale_report
stale_dashboard_snapshot
stale_external_artifact
```

### old_path

canonical path へ移行済みの旧 path。

### placeholder_file

dry-run relocation 用の placeholder。

### temporary_canonical_file

temporary dual canonical state 解消後に不要となった一時 canonical 候補。

### obsolete_document

obsolete と判定され、legacy alias / historical note が整備済みの文書。

### expired_legacy_alias

expiration policy により削除可能となった legacy alias。

### stale_report / stale_dashboard_snapshot

現在の validator / dashboard state と対応しない古い report。

### stale_external_artifact

cleanup scope 内で不要となった cross-project artifact。

---

## 6. execution lifecycle

cleanup execution lifecycle は以下とする。

```text
1. cleanup gate report intake
2. cleanup target extraction
3. cleanup batch planning
4. cleanup dry-run
5. dry-run validation
6. approval boundary
7. cleanup execution
8. post-cleanup validation
9. cleanup report generation
10. dashboard snapshot regeneration
```

---

## 7. cleanup target extraction

cleanup target extraction は cleanup gate validator report の cleanup_ready list から行う。

抽出時に確認する情報：

```text
- target path
- target kind
- canonical replacement
- alias status
- reference status
- rollback requirement
- external artifact dependency
```

cleanup_blocked / cleanup_pending の対象は抽出してはならない。

---

## 8. cleanup batch planning

cleanup batch は、依存関係が閉じた subgraph として作成する。

batch 条件：

```text
- すべての target が cleanup_ready
- unresolved incoming blocking edge がない
- rollback package を生成可能
- post-cleanup validation scope が定義済み
- external artifact impact が既知
```

cleanup batch は小さく保つ。

初期運用では single layer または closed_dependency_scope を推奨する。

---

## 9. cleanup dry-run

cleanup dry-run は実ファイルを変更せず、以下を生成する。

```text
- delete candidate list
- alias expiration candidate list
- expected filesystem diff
- expected reference state
- expected dashboard state
- rollback package preview
```

dry-run で unexpected diff が出た場合、cleanup execution に進んではならない。

---

## 10. dry-run validation

dry-run validation では以下を確認する。

```text
- cleanup target が cleanup_ready に含まれている
- delete candidate が cleanup impact summary と一致する
- expected filesystem diff に未知の変更がない
- expected reference state に unresolved reference がない
- rollback package preview が生成できる
- post-cleanup validator scope が決定できる
```

dry-run validation が fail の場合、cleanup execution は blocked とする。

---

## 11. approval boundary

cleanup は破壊的操作を含むため、dry-run 後に approval boundary を置く。

approval boundary で確認する情報：

```text
- cleanup target list
- deleted file list
- alias expiration list
- rollback package location
- post-cleanup validation plan
- known risks
```

CI 自動 cleanup を行う場合でも、approval policy を明示する。

---

## 12. cleanup execution

cleanup execution は approved cleanup batch に対してのみ実行する。

実行対象：

```text
- old_path deletion
- placeholder_file deletion
- temporary_canonical_file deletion
- obsolete_document deletion or archival
- expired_legacy_alias deletion
- stale report archival or deletion
```

cleanup execution 中に想定外の差分が発生した場合、処理を停止し cleanup_failed とする。

---

## 13. alias expiration

legacy alias は expiration policy に従って削除または無効化する。

削除条件：

```text
- alias_state = expired
- canonical target が存在する
- old reference が残っていない
- cleanup gate validator が cleanup_ready と判定している
```

alias expiration は cleanup execution の一部である。

ただし、alias generation は cleanup execution では行わない。

---

## 14. rollback package

cleanup execution 前に rollback package を生成する。

rollback package には以下を含める。

```text
- deleted file content
- deleted path list
- alias state before cleanup
- filesystem_index before cleanup
- reference_index before cleanup
- validator reports before cleanup
- dashboard snapshot before cleanup
```

rollback package が生成できない cleanup batch は実行してはならない。

---

## 15. post-cleanup validation

cleanup 後、以下の validator を実行する。

```text
- manifest_validator
- reference_validator
- traceability_validator
- alias_validator
- cleanup_gate_validator
- dashboard_projection_validator
```

必要に応じて以下も実行する。

```text
- integrity_tamper_validator
- external artifact validation
- CI validation
```

post-cleanup validation が fail の場合、cleanup_failed として扱う。

---

## 16. cleanup status

cleanup execution status の許容値：

```text
planned
dry_run_ready
dry_run_passed
dry_run_failed
approval_required
approved
executing
completed
failed
rolled_back
superseded
```

`completed` は post-cleanup validation が pass した状態を意味する。

---

## 17. cleanup report schema draft

```json
{
  "schema_version": "1.0",
  "cleanup_execution_id": "cleanup-YYYYMMDD-NNN",
  "cleanup_gate_validator_run_id": "validator-YYYYMMDD-NNN",
  "status": "completed",
  "targets": [
    {
      "target_id": "cleanup-target-0001",
      "target_kind": "old_path",
      "path": "docs/old.md",
      "canonical_replacement": "docs/new.md",
      "action": "delete",
      "rollback_available": true
    }
  ],
  "post_cleanup_validation": {
    "status": "pass"
  }
}
```

---

## 18. CI 連携

CI では cleanup execution を通常は dry-run までに制限する。

推奨：

```text
pull_request:
cleanup dry-run のみ

protected branch:
明示 approval 後に cleanup execution 可

release preparation:
cleanup execution 可
```

CI fail 条件：

```text
- cleanup dry-run unexpected diff
- cleanup target が cleanup_ready に含まれていない
- rollback package preview 生成不可
- post-cleanup validation fail
```

---

## 19. dashboard との関係

cleanup execution は dashboard snapshot generator に以下を渡す。

```text
- cleanup execution status
- deleted target list
- alias expiration status
- rollback availability
- post-cleanup validation result
```

Dashboard は cleanup execution 結果を表示するが、cleanup execution を実行してはならない。

---

## 20. 禁止事項

以下を禁止する。

```text
- cleanup_blocked / cleanup_pending 対象を削除すること
- cleanup gate validator report なしに cleanup を実行すること
- dry-run validation fail のまま cleanup を実行すること
- rollback package なしに破壊的 cleanup を実行すること
- alias generation を cleanup execution に混ぜること
- cleanup execution が unresolved reference を自動修正すること
- post-cleanup validation fail を completed とすること
```

---

## 21. HLDocS feedback

本 execution plan から、HLDocS 側へ以下をフィードバックする。

```text
- cleanup execution は cleanup gate validator の後続フェーズとして分離すべき
- cleanup には dry-run / approval / rollback package / post-validation が必要
- cleanup_ready 以外の対象は処理してはならない
- CI では cleanup dry-run と実 cleanup を分けるべき
- alias generation と alias expiration を分離すべき
```

---

## 22. 結論

cleanup execution plan は、cleanup gate validator により cleanup_ready と判定された対象を、安全に削除・無効化・アーカイブするための実行計画である。

これにより、dry-run、approval boundary、rollback package、post-cleanup validation を通して、破壊的操作を検証可能な形で実行できる。
