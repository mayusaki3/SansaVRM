# canonicalization execution engine

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における canonicalization execution engine を定義する。

canonicalization execution engine は、canonicalization manifest と rewrite transaction plan に基づき、canonical identity の切替、参照更新、traceability 更新、legacy alias 準備を実行する executor である。

validator は検査を行う observer / gate であり、execution engine とは分離する。

---

## 2. 基本方針

canonicalization execution engine は以下を行う。

```text
- canonicalization execution scope を固定する
- execution batch を作成する
- rewrite transaction plan を実行する
- execution checkpoint を作成する
- rollback package を作成する
- execution artifact を生成する
- 実行後 validator を起動可能な状態を作る
```

canonicalization execution engine は以下を行わない。

```text
- document_fate を独自決定しない
- canonical_doc_id を独自発行しない
- validator 判定を捏造しない
- cleanup を実行しない
- cleanup_ready を判定しない
- dashboard state を直接変更しない
```

---

## 3. 入力

canonicalization execution engine の入力は以下とする。

```text
- canonicalization manifest
- rewrite transaction plan
- migration orchestration graph
- index builder outputs
- manifest validator report
- canonicalization validator report
- execution configuration
- current filesystem snapshot
```

必要に応じて以下も参照する。

```text
- integrity / tamper validator report
- external_artifact_index
- cross-project handoff response
```

---

## 4. 出力

canonicalization execution engine は以下を出力する。

```text
- canonicalization execution report
- executed rewrite transaction report
- execution checkpoint
- rollback package
- updated filesystem snapshot
- validator input package
- dashboard projection input
```

---

## 5. execution lifecycle

canonicalization execution engine の lifecycle は以下とする。

```text
1. execution request intake
2. scope freeze
3. preflight validation confirmation
4. execution batch planning
5. checkpoint creation
6. dry-run execution
7. dry-run validation
8. approval boundary
9. rewrite execution
10. execution artifact generation
11. post-execution validator handoff
12. execution report generation
```

---

## 6. execution request intake

execution request では、以下を明示する。

```text
- canonicalization_id
- source_migration_id
- requested_scope
- target document set
- target rewrite transaction set
- execution_mode
- approval_policy
```

execution_mode の許容値：

```text
dry_run
apply
rollback
```

---

## 7. scope freeze

scope freeze は、実行対象を固定する。

freeze 対象：

```text
- canonicalization manifest version
- rewrite transaction plan version
- target document set
- affected reference set
- affected traceability set
- affected sec_id set
- affected testspec/code reference set
- external artifact dependency set
```

scope freeze 後に入力が変わった場合は、現在の execution を superseded とし、新しい execution_id を発行する。

---

## 8. preflight validation confirmation

実行前に以下を確認する。

```text
- manifest validator が fail ではない
- canonicalization validator が fail ではない
- requested scope に pending document_fate が含まれない
- invalid_canonical_conflict がない
- rewrite transaction plan が存在する
- rollback package を作成可能
- execution batch が closed dependency scope である、または external dependency rule がある
```

preflight validation confirmation が fail の場合、execution を開始してはならない。

---

## 9. execution batch planning

execution batch は依存関係が閉じた subgraph として作成する。

batch 条件：

```text
- target rewrite transactions が同一 canonicalization scope に属する
- unresolved incoming blocking edge がない
- rollback scope が batch 内で閉じている
- post-execution validator scope が定義済み
- external artifact dependency が確認済み
```

初期運用では小さい closed_dependency_scope を batch とする。

---

## 10. checkpoint creation

実行前に checkpoint を作成する。

checkpoint に含める情報：

```text
- execution_id
- canonicalization_id
- rewrite_transaction_ids
- filesystem snapshot hash
- affected file contents
- affected index hashes
- validator report references
- dashboard snapshot reference
- external artifact references
```

checkpoint が作成できない場合、apply mode を実行してはならない。

---

## 11. dry-run execution

canonicalization execution engine は、apply 前に dry-run execution を実行する。

Dry-run では実ファイルを変更せず、以下を生成する。

```text
- expected file diff
- expected reference diff
- expected traceability diff
- expected sec_id diff
- expected alias preparation diff
- expected validator scope
- expected rollback package
```

Dry-run の結果に unexpected diff がある場合、apply mode へ進めない。

---

## 12. approval boundary

apply mode の前に approval boundary を置く。

approval boundary で提示する情報：

```text
- execution batch
- expected diff
- affected files
- affected references
- rollback package location
- post-execution validation plan
- known warnings
```

approval がない場合、apply mode を実行してはならない。

---

## 13. rewrite execution

rewrite execution は rewrite transaction plan に従って実行する。

実行順序：

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
```

execution engine は operation order を変更してはならない。

必要な例外は rewrite transaction plan に明示されている必要がある。

---

## 14. partial execution handling

partial execution が発生した場合、execution status は `partial_failed` とする。

partial execution 時の処理：

```text
- 直ちに後続 operation を停止する
- checkpoint と実 filesystem state を比較する
- rollback 可能性を判定する
- validator handoff を blocked とする
- dashboard projection input に partial_failed を出す
```

partial execution のまま validated / cleanup_ready に進めてはならない。

---

## 15. rollback execution

rollback mode は checkpoint と rollback package に基づいて実行する。

rollback 対象：

```text
- changed files
- changed references
- changed traceability entries
- changed sec_id mappings
- generated aliases
- generated reports
- updated indexes
```

rollback 後は post-rollback validation を実行する。

rollback が失敗した場合は `rollback_failed` とし、manual recovery required とする。

---

## 16. execution artifact

canonicalization execution engine は以下の artifact を生成する。

```text
canonicalization_execution_report.json
canonicalization_execution_report.md
rewrite_execution_report.json
rollback_package.json
dry_run_diff.patch
validator_input_package.json
dashboard_projection_input.json
```

これらは validator / dashboard / cleanup gate の入力として使用できる。

---

## 17. execution status

execution status の許容値：

```text
requested
scope_frozen
preflight_passed
preflight_failed
dry_run_completed
dry_run_failed
approval_required
approved
executing
executed
partial_failed
validator_handoff_ready
completed
failed
rollback_requested
rolled_back
rollback_failed
superseded
```

`completed` は post-execution validator に渡せる状態を意味する。

`completed` は validator pass や cleanup_ready を意味しない。

---

## 18. post-execution validator handoff

execution engine は実行後、validator orchestration に渡す input package を生成する。

含める情報：

```text
- changed files
- changed indexes
- executed rewrite transaction ids
- expected validator scope
- execution report reference
- rollback package reference
```

validator pass は validator 側で判定する。

execution engine は validator result を直接確定してはならない。

---

## 19. report schema draft

```json
{
  "schema_version": "1.0",
  "canonicalization_execution_id": "canonicalization-exec-YYYYMMDD-NNN",
  "canonicalization_id": "canonicalization-YYYYMMDD-NNN",
  "execution_mode": "dry_run",
  "execution_status": "dry_run_completed",
  "rewrite_transaction_ids": [
    "rewrite-YYYYMMDD-NNN"
  ],
  "affected_files": [],
  "artifacts": {
    "dry_run_diff": "dry_run_diff.patch",
    "rollback_package": "rollback_package.json",
    "validator_input_package": "validator_input_package.json"
  }
}
```

---

## 20. CI integration

CI では canonicalization execution engine を原則 dry-run mode で実行する。

CI で実行する内容：

```text
- scope freeze simulation
- preflight validation confirmation
- dry-run execution
- expected diff generation
- validator input package generation
```

CI で apply mode を実行してはならない。

apply mode は明示 approval 後の dedicated execution step でのみ行う。

---

## 21. cross-project artifact との関係

execution scope が MuJoCo Adapter / Studio AI の artifact に影響する場合、以下を確認する。

```text
- handoff response が完了している
- external artifact freshness が確認済み
- downstream validation scope が定義されている
- artifact を stale にする場合の通知または revalidation plan がある
```

cross-project dependency が未解決の場合、execution は blocked とする。

---

## 22. dashboard との関係

execution engine は dashboard projection input を生成する。

Dashboard に表示する情報：

```text
- execution status
- execution mode
- affected files
- dry-run diff summary
- rollback availability
- validator handoff status
- partial failure state
```

Dashboard は execution を実行しない。

---

## 23. 禁止事項

以下を禁止する。

```text
- validator fail を無視して apply mode を実行すること
- checkpoint なしで apply mode を実行すること
- dry-run なしで apply mode を実行すること
- approval なしで apply mode を実行すること
- partial_failed を completed として扱うこと
- execution engine が cleanup_ready を判定すること
- execution engine が cleanup を実行すること
- CI で apply mode を実行すること
```

---

## 24. HLDocS feedback

本 execution engine model から、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization executor と validator は分離すべき
- canonicalization apply には checkpoint / dry-run / approval / rollback package が必要
- completed は validator pass ではなく validator handoff ready として扱うべき
- CI では canonicalization dry-run のみを行うべき
- cross-project artifact dependency は execution block 条件に含めるべき
```

---

## 25. 結論

canonicalization execution engine は、canonicalization manifest と rewrite transaction plan に基づき、canonical identity の切替と関連 rewrite を実行する executor である。

Execution engine は validator と cleanup gate から分離され、checkpoint、dry-run、approval、rollback package、post-execution validator handoff を通して安全に canonicalization を実行する。
