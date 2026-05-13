# canonicalization execution plan

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における canonicalization execution plan を定義する。

canonicalization execution plan は、document fate decision と canonicalization manifest に基づき、canonical identity を実運用上どの順序で切り替えるかを定義する。

本ドキュメントは、canonicalization manifest、rewrite transaction model、dashboard state schema、federation validator implementation model、cleanup gate dependency graph を接続する operational plan である。

---

## 2. 基本方針

canonicalization execution は以下を分離して扱う。

```text
identity decision:
canonicalization manifest 上の canonical_doc_id / document_fate 確定

rewrite execution:
実ファイル・参照・traceability・testspec/code reference への反映

alias compatibility:
旧 doc_id / 旧 path / 旧参照の互換維持

cleanup execution:
旧 path / placeholder / temporary state の削除
```

canonicalization execution は cleanup ではない。

cleanup は cleanup gate 通過後にのみ実施する。

---

## 3. execution lifecycle

canonicalization execution の全体 lifecycle は以下とする。

```text
1. migration freeze
2. canonicalization manifest review
3. execution scope selection
4. preflight validation
5. canonical switch preparation
6. rewrite transaction execution
7. rewrite validation
8. legacy alias generation
9. dashboard snapshot generation
10. cleanup gate evaluation
11. cleanup execution planning
```

---

## 4. migration freeze

canonicalization execution 前に、対象範囲を freeze する。

freeze 対象：

```text
- migration manifest federation
- canonicalization manifest
- affected document set
- affected reference set
- affected traceability set
- affected testspec/code reference set
```

freeze 中は、対象範囲の document fate decision を変更してはならない。

変更が必要な場合は、現在の execution scope を中断し、新しい canonicalization_id を発行する。

---

## 5. execution scope selection

execution scope は、以下のいずれかで選択する。

```text
single_document_scope
layer_scope
closed_dependency_scope
validation_layer_scope
cross_project_boundary_scope
```

初期実行では、closed_dependency_scope を優先する。

closed_dependency_scope は、未解決参照を外部へ広げずに rewrite / validation / alias generation まで実行できる範囲である。

---

## 6. preflight validation

preflight validation では以下を確認する。

```text
- migration manifest federation が読み込める
- canonicalization manifest が読み込める
- document_fate が pending ではない
- canonical_doc_id collision がない
- placeholder relocation が execution scope に残っていない、または明示的に blocked として扱われている
- rewrite transaction plan を生成できる
- rollback scope を生成できる
- required legacy alias を判定できる
- reference index / traceability index / sec_id index が生成できる
```

preflight validation が FAIL の場合、canonical switch を開始してはならない。

---

## 7. canonical switch preparation

canonical switch preparation では、以下を準備する。

```text
- canonical_doc_id mapping
- old_doc_id to canonical_doc_id mapping
- old_path to canonical_path mapping
- temporary dual canonical declaration
- rewrite transaction plan
- rollback plan
- alias generation plan
- validator run plan
```

canonical switch preparation は、実ファイルを変更しない。

---

## 8. temporary dual canonical state

temporary dual canonical state は、dry-run または migration 中に旧 path と新 path が一時的に canonical 候補として併存する状態である。

許容条件：

```text
- temporary 状態であることが manifest に明示されている
- canonical source が1つに決まっている
- legacy alias 化または canonical switch の終了条件が定義されている
- cleanup gate 前に解消される
```

禁止：

```text
- temporary dual canonical state を恒久状態にすること
- canonical source 未定義のまま rewrite を実行すること
- dual canonical のまま cleanup_ready とすること
```

---

## 9. rewrite execution ordering

rewrite execution は rewrite transaction model の ordering rules に従う。

基本順序：

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

path rewrite を先行させる必要がある場合は、temporary path mapping を rewrite transaction 内に明示する。

---

## 10. rollback strategy

rollback は transaction 単位で行う。

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

rollback 方針：

```text
- semantic transaction を優先して戻す
- representation transaction のみを戻して semantic transaction を残してはならない
- rollback 後は dashboard snapshot を更新し、rewrite_rolled_back として表示する
- rollback 後に cleanup_ready を維持してはならない
```

---

## 11. partial execution handling

partial execution は原則として cleanup_ready に進めない。

partial execution が許容されるのは、以下の場合のみである。

```text
- execution scope が明示的に分割されている
- dependent transaction が定義されている
- unresolved references が既知の blocked reason として記録されている
- dashboard に partial state が表示される
- cleanup gate が cleanup_blocked と判定する
```

---

## 12. legacy alias generation timing

legacy alias は rewrite validation 後、cleanup gate 前に生成する。

```text
rewrite transaction executed
  ↓
rewrite validator PASS
  ↓
legacy alias generation
  ↓
alias validator PASS
  ↓
cleanup gate evaluation
```

legacy alias が必要な文書では、alias_state=generated になるまで cleanup_ready にしてはならない。

---

## 13. cleanup execution boundary

cleanup execution は canonicalization execution に含めない。

canonicalization execution の終了条件は以下である。

```text
- rewrite_state = rewrite_validated
- alias_state = generated または not_required
- federation validator = pass
- dashboard snapshot が生成可能
- cleanup gate evaluation が実行済み
```

cleanup 実行は、cleanup gate が cleanup_ready を返した後の別フェーズとする。

---

## 14. dashboard projection

canonicalization execution は dashboard に以下を提供する。

```text
- execution scope
- temporary dual canonical state
- rewrite transaction status
- rollback status
- alias generation status
- cleanup gate status
- blocking reasons
```

Dashboard はこれらを表示するが、execution state を変更してはならない。

---

## 15. execution report

canonicalization execution は、以下の report を生成する。

```text
- canonicalization execution report
- rewrite transaction report
- legacy alias report
- cleanup gate input report
- dashboard snapshot input
```

execution report には以下を含める。

```text
execution_id
canonicalization_id
source_migration_id
execution_scope
preflight_status
rewrite_status
alias_status
validator_status
cleanup_gate_status
rollback_status
blocking_reasons
next_required_actions
```

---

## 16. execution status

execution status の許容値：

```text
planned
frozen
preflight_passed
preflight_failed
prepared
executing
rewrite_validated
alias_generated
cleanup_gate_evaluated
completed
blocked
failed
rolled_back
superseded
```

`completed` は cleanup completed を意味しない。

`completed` は canonicalization execution が cleanup gate へ状態を渡せる状態を意味する。

---

## 17. 初期適用方針

初期 canonicalization execution は小さい closed_dependency_scope で行う。

推奨対象：

```text
Validation Layer 周辺の migration / report / validator 文書群
```

理由：

```text
- dependency が比較的閉じている
- validator / dashboard / cleanup gate の検証対象として適切
- cross-project handoff response の前提を確認しやすい
```

---

## 18. 禁止事項

以下を禁止する。

```text
- migration freeze なしで canonical switch を実行すること
- pending document fate の文書を canonical switch 対象にすること
- preflight validation FAIL のまま rewrite を実行すること
- rewrite validation 前に legacy alias を正式化すること
- legacy alias required のまま cleanup_ready にすること
- temporary dual canonical state のまま cleanup_ready にすること
- cleanup execution を canonicalization execution と同時に実行すること
```

---

## 19. cross-project handoff との関係

MuJoCo Adapter / Studio AI への handoff response は、canonicalization execution plan を前提に回答する。

特に以下を回答可能にする。

```text
- canonical switch がどの時点で安定するか
- Adapter JSON仕様や Studio AI feedback がどの Layer に配置されるか
- updated_extension_properties / export profiles / diagnostics が Core semantic を汚染しないこと
- rewrite / validation / cleanup の境界
- legacy alias 期間中の互換方針
```

---

## 20. HLDocS feedback

本 execution plan から、HLDocS 側へ以下をフィードバックする。

```text
- canonicalization は identity decision と rewrite execution に分けるべき
- cleanup は canonicalization execution に含めず別フェーズにすべき
- temporary dual canonical state には終了条件が必要
- migration freeze boundary を定義すべき
- rollback は transaction 単位で定義すべき
- dashboard は execution state の observer として扱うべき
```

---

## 21. 結論

canonicalization execution plan は、canonicalization manifest 上の identity decision を、安全に rewrite transaction、legacy alias、dashboard、cleanup gate へ接続するための運用計画である。

これにより、canonical switch、rewrite validation、legacy alias、cleanup gate を混同せずに段階実行できる。
