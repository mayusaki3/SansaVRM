# federation validator implementation model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation validator の実装モデルを定義する。

federation validator は、migration manifest federation、canonicalization manifest、rewrite transaction、validator report、cleanup gate を横断して検査する cross-layer observer である。

本モデルは、validator を rewrite executor や cleanup executor と混同しないための実装境界を定義する。

---

## 2. 基本方針

federation validator は以下を行う。

```text
- manifest federation の整合性検査
- dry-doc と canonical_doc_id の対応検査
- document fate decision の検査
- rewrite transaction の検査
- sec_id continuity の検査
- traceability reference の検査
- legacy alias readiness の検査
- cleanup gate prerequisites の検査
- dashboard snapshot 用 state の集約
- machine-readable report / human-readable report の出力
```

federation validator は以下を行わない。

```text
- rewrite の実行
- doc_id の発行
- sec_id の生成
- traceability の修正
- legacy alias の生成
- cleanup の実行
- dashboard state の直接変更
```

---

## 3. 実装上の位置づけ

```text
source manifests / indexes
  ↓
federation validator
  ↓
validator reports
  ↓
dashboard snapshot generator
  ↓
cleanup gate
```

validator は observer / gate であり、state mutator ではない。

---

## 4. 入力

federation validator の入力は以下とする。

```text
- migration manifest federation
- canonicalization manifest
- rewrite transaction plan/report
- current filesystem snapshot
- reference index
- traceability index
- sec_id index
- testspec/code reference index
- legacy alias manifest
- validator configuration
- CI result summary
```

入力は machine-readable を優先する。

Markdown 文書のみが存在する場合は、validator 入力用の抽出 index を事前生成する。

---

## 5. 入力 index

validator 実装では、少なくとも以下の index を構築する。

```text
migration_index:
dry_doc_id / migration_entry_id / source_path / target_path

canonical_index:
dry_doc_id / old_doc_id / canonical_doc_id / document_fate / canonicalization_status

rewrite_index:
rewrite_transaction_id / operation_kind / target_file / old_value / new_value / transaction_status

reference_index:
source_file / reference_target / resolved_target / reference_kind

traceability_index:
doc_id / sec_id / testspec_id / code_reference / relationship_kind

sec_id_index:
doc_id / sec_id / source_file / section_path

alias_index:
old_doc_id / old_path / canonical_doc_id / canonical_path / alias_state

validation_index:
validator_name / target / status / blocking_reasons
```

---

## 6. validator modules

federation validator は以下の module に分割する。

```text
manifest_validator
canonicalization_validator
rewrite_validator
reference_validator
traceability_validator
sec_id_validator
alias_validator
cleanup_gate_validator
dashboard_projection_validator
```

### manifest_validator

migration manifest federation の登録状態、重複、参照不能 entry を検査する。

### canonicalization_validator

canonicalization manifest の document_fate、canonical_doc_id、pending、collision を検査する。

### rewrite_validator

rewrite transaction の operation、old/new value、transaction status、rollback scope を検査する。

### reference_validator

文書間参照、旧path参照、canonical target 参照の解決状態を検査する。

### traceability_validator

仕様・テスト・コード間の紐づけが canonical_doc_id / sec_id に追従しているかを検査する。

### sec_id_validator

sec_id の preserve / map / split / merge / remove の整合性を検査する。

### alias_validator

legacy alias の必要性、生成状態、有効期限、canonical target を検査する。

### cleanup_gate_validator

cleanup_ready / cleanup_blocked の条件を検査する。

### dashboard_projection_validator

dashboard snapshot に投影する state domain が source of truth と矛盾していないかを検査する。

---

## 7. module dependency

module dependency は以下とする。

```text
manifest_validator
  ↓
canonicalization_validator
  ↓
rewrite_validator
  ↓
reference_validator
  ↓
traceability_validator
  ↓
sec_id_validator
  ↓
alias_validator
  ↓
cleanup_gate_validator
  ↓
dashboard_projection_validator
```

ただし、実装上は全 module が同一 index set を読む。

上記順序は判定上の依存順であり、filesystem ordering ではない。

---

## 8. report outputs

federation validator は以下を出力する。

```text
- machine-readable JSON report
- human-readable Markdown report
- dashboard snapshot input
- cleanup gate input
- blocking reason list
- warning list
- validator summary
```

Markdown report は integrity report として扱える。

JSON report は dashboard / cleanup gate / CI で使用する。

---

## 9. machine-readable report schema draft

```json
{
  "schema_version": "1.0",
  "validator_run_id": "validator-YYYYMMDD-NNN",
  "source_migration_id": "migration-YYYYMMDD-NNN",
  "canonicalization_id": "canonicalization-YYYYMMDD-NNN",
  "overall_status": "warn",
  "modules": [
    {
      "module_name": "canonicalization_validator",
      "status": "pass",
      "checked_items": 10,
      "warnings": [],
      "failures": []
    }
  ],
  "nodes": [
    {
      "dry_doc_id": "dry-doc-example",
      "canonical_doc_id": "doc-example",
      "status": "warn",
      "blocking_reasons": [],
      "warnings": [
        "legacy_alias_required"
      ]
    }
  ]
}
```

---

## 10. status model

各 module と overall_status の許容値：

```text
pass
warn
fail
blocked
not_applicable
```

### pass

検査対象がすべて条件を満たす。

### warn

問題はあるが、dry-run または次フェーズ継続は可能。

### fail

不整合があり、cleanup_ready や canonical switch を禁止する。

### blocked

必要な入力が不足しており、判定不能。

### not_applicable

対象外。

---

## 11. blocking reason taxonomy

blocking reason は dashboard_state_schema と同じ taxonomy を使用する。

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

validator は blocking reason に source_domain を付与する。

```text
source_domain:
migration | fate | canonicalization | rewrite | validation | alias | cleanup
```

---

## 12. cleanup gate 判定

cleanup_gate_validator は以下を満たす場合のみ cleanup_ready とする。

```text
- migration_state = migration_verified
- fate_state が pending / blocked ではない
- canonicalization_state = completed または not_required
- rewrite_state = rewrite_validated または not_required
- semantic verification = pass
- rewrite validator = pass または not_applicable
- federation validator = pass
- CI validation = pass
- unresolved references = 0
- required legacy alias = generated または not_required
- placeholder relocation が残っていない
```

いずれかを満たさない場合は cleanup_blocked とする。

---

## 13. dashboard 連携

federation validator は dashboard snapshot generator へ以下を渡す。

```text
- node status
- edge status
- state domain values
- blocking reasons
- warnings
- source of truth references
```

Dashboard は validator report を表示するが、validator report を変更してはならない。

---

## 14. CI 連携

CI では以下の用途で federation validator を実行する。

```text
- manifest 構造検査
- canonical_doc_id collision 検査
- unresolved reference 検査
- cleanup_ready 禁止状態の検出
- dashboard snapshot 生成可能性検査
```

CI で fail とする条件：

```text
- invalid_canonical_conflict
- unresolved references on drop target
- sec_id collision
- traceability reference unresolved
- rewrite_state = rewrite_failed
- cleanup_ready と判定された node に blocking reason が存在する
```

warn とする条件：

```text
- pending document fate
- legacy alias required but not yet generated
- placeholder relocation exists in dry-run scope
- migration partial during active dry-run
```

---

## 15. human-readable report structure

Markdown report は以下の構造とする。

```text
1. summary
2. module status summary
3. cleanup readiness summary
4. blocking reason list
5. warning list
6. canonicalization issues
7. rewrite issues
8. reference / traceability issues
9. legacy alias issues
10. dashboard projection issues
11. next required decisions
```

---

## 16. 実装単位

初期実装は以下の順で進める。

```text
1. index builder
2. manifest_validator
3. canonicalization_validator
4. rewrite_validator
5. reference_validator
6. cleanup_gate_validator
7. JSON report generator
8. Markdown report generator
9. dashboard snapshot generator
```

traceability_validator / sec_id_validator / alias_validator は、index が揃い次第追加する。

---

## 17. 禁止事項

以下を禁止する。

```text
- validator が rewrite を実行すること
- validator が doc_id を採番すること
- validator が sec_id を生成すること
- validator が cleanup を実行すること
- dashboard snapshot を source of truth として再入力すること
- Markdown report だけを唯一の machine-readable source として扱うこと
```

---

## 18. HLDocS feedback

本実装モデルから、HLDocS 側へ以下をフィードバックする。

```text
- federation validator は cross-layer observer として定義すべき
- validator 入力には manifest / index / report を分離して持つべき
- validator は machine-readable JSON と human-readable Markdown を両方出力すべき
- dashboard は validator report の projection として扱うべき
- cleanup gate は validator module として独立定義すべき
- validator は rewrite / cleanup を実行しないと明記すべき
```

---

## 19. 結論

federation validator は、大規模仕様再構成における migration / canonicalization / rewrite / traceability / cleanup を横断検査する observer である。

validator は source of truth を変更せず、manifest と index を検査し、machine-readable report と human-readable report を出力する。

これにより、dashboard 表示、cleanup gate 判定、CI 検査を同一の検査結果から一貫して扱える。
