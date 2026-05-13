# cleanup gate validator implementation

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における cleanup gate validator implementation を定義する。

cleanup gate validator は、migration / canonicalization / rewrite / alias / validation / integrity の全条件を確認し、旧 path・placeholder・temporary state・obsolete artifact を cleanup してよいかを判定する。

cleanup_ready の source of truth は cleanup gate validator の出力であり、dashboard ではない。

---

## 2. 基本方針

cleanup gate validator は以下を行う。

```text
- cleanup_ready / cleanup_blocked を判定する
- cleanup を block する理由を分類する
- unresolved references を検査する
- placeholder relocation の残存を検査する
- temporary dual canonical state の解消を検査する
- rewrite_validated を検査する
- required legacy alias の生成状態を検査する
- integrity / CI / federation validator の結果を集約する
```

cleanup gate validator は以下を行わない。

```text
- cleanup を実行しない
- file を削除しない
- alias を生成しない
- rewrite を実行しない
- dashboard state を直接変更しない
```

---

## 3. 入力

cleanup gate validator の入力は以下とする。

```text
- migration_index
- canonical_index
- rewrite_index
- reference_index
- traceability_index
- sec_id_index
- alias_index
- external_artifact_index
- validation_input_index
- manifest validator result
- canonicalization validator result
- rewrite validator result
- integrity / tamper validator result
- CI result summary
- orchestration graph
```

---

## 4. 出力

cleanup gate validator は以下を出力する。

```text
- cleanup gate JSON report
- cleanup gate Markdown report section
- cleanup_ready list
- cleanup_blocked list
- cleanup_pending list
- blocking reason list
- cleanup impact summary
- dashboard projection input
- CI status mapping
```

---

## 5. cleanup_state

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

cleanup gate validator は主に以下を出力する。

```text
cleanup_ready
cleanup_blocked
cleanup_pending
```

`cleaning`、`cleaned`、`cleanup_failed` は cleanup execution 側の状態であり、cleanup gate validator が直接設定しない。

---

## 6. cleanup_ready 条件

cleanup_ready は以下をすべて満たす場合のみ成立する。

```text
- migration_state = migration_verified
- document_fate != pending
- canonicalization_status = completed または not_required
- canonical_doc_id collision がない
- temporary dual canonical state が解消済み
- semantic_equivalent = true または document_fate が obsolete / drop
- rewrite_state = rewrite_validated または not_required
- unresolved references = 0
- traceability reference が解決済み
- sec_id collision がない
- required legacy alias = generated または not_required
- placeholder relocation が残っていない
- manifest validator = pass または許容 warn
- canonicalization validator = pass または許容 warn
- rewrite validator = pass または not_applicable
- integrity validator が fail ではない
- federation validator = pass
- CI validation = pass
```

いずれかを満たさない場合は cleanup_ready にしてはならない。

---

## 7. cleanup_blocked 条件

以下のいずれかに該当する場合は cleanup_blocked とする。

```text
- manifest_missing
- placeholder_relocation_remaining
- migrated_partial
- fate_not_decided
- canonical_doc_id_missing
- canonical_doc_id_collision
- temporary_dual_canonical_unresolved
- semantic_equivalent_unknown
- rewrite_not_validated
- rewrite_failed
- unresolved_references
- traceability_unresolved
- testspec_reference_unresolved
- code_reference_unresolved
- sec_id_collision
- legacy_alias_not_generated
- integrity_validator_failed
- federation_validator_failed
- ci_validation_failed
- external_artifact_stale_for_cleanup
```

---

## 8. cleanup_pending 条件

cleanup_pending は、cleanup_ready ではないが、active dry-run または判断待ちとして継続可能な状態である。

例：

```text
- pending document_fate outside cleanup scope
- legacy alias required but cleanup scope 外
- placeholder relocation exists in active dry-run scope
- representation hash mismatch but semantic hash matched
- external artifact stale but cleanup target ではない
```

cleanup_pending は cleanup_ready ではない。

---

## 9. unresolved references check

cleanup gate validator は reference_index / traceability_index を用いて unresolved references を検査する。

検査対象：

```text
- old_doc_id reference
- old_path reference
- drop target reference
- obsolete target reference
- sec_id reference
- testspec reference
- code reference
- external artifact reference
```

cleanup 対象に unresolved references が残る場合は `cleanup_blocked` とする。

---

## 10. placeholder relocation check

placeholder relocation が残る場合、cleanup_ready にしてはならない。

判定：

```text
active dry-run scope:
cleanup_pending または cleanup_blocked

cleanup target scope:
cleanup_blocked
```

placeholder relocation は migration complete ではない。

---

## 11. temporary dual canonical check

temporary dual canonical state は cleanup gate 前に解消されていなければならない。

判定：

```text
canonical_resolved:
pass

temporary_dual_canonical with declared exit condition:
cleanup_blocked

invalid_canonical_conflict:
cleanup_blocked
```

一時状態として許容されていても、cleanup_ready にはできない。

---

## 12. rewrite validation check

cleanup_ready には rewrite_state が `rewrite_validated` である必要がある。

以下は cleanup_blocked とする。

```text
rewrite_planned
rewrite_ready
rewrite_executing
rewrite_executed
rewrite_validating
rewrite_blocked
rewrite_failed
rewrite_rolled_back
```

`rewrite_executed` は実行済みだが validator 未確認であるため cleanup_ready 不可とする。

---

## 13. legacy alias check

legacy alias が required の場合、alias_state は `generated` でなければならない。

```text
alias_state=generated:
pass

alias_state=required / ready / blocked:
cleanup_blocked

alias_state=not_required:
pass
```

alias expiration は cleanup execution 側で扱うが、expiration policy が必要な場合は cleanup gate で存在確認する。

---

## 14. integrity / tamper check

integrity / tamper は以下のように扱う。

```text
semantic_integrity=fail:
cleanup_blocked

semantic_integrity=pass:
pass

representation_integrity=warn かつ semantic_integrity=pass:
cleanup_pending または pass

signature_integrity=warn:
policy に応じて cleanup_pending または pass

provenance_integrity=fail:
対象 scope に応じて cleanup_blocked または cleanup_pending
```

representation hash mismatch のみで cleanup_blocked としてはならない。

---

## 15. external artifact freshness check

cleanup 対象が cross-project artifact に依存する場合、external artifact freshness を確認する。

対象例：

```text
- MuJoCo Adapter draft schema
- Adapter fixture
- diagnostics.json
- conversion_report.json
- Studio AI fixture
- export profile sample
```

cleanup 判定に必要な artifact が stale または missing の場合は cleanup_blocked とする。

cleanup 判定に不要な artifact が stale の場合は cleanup_pending または warn とする。

---

## 16. cleanup impact summary

cleanup gate validator は cleanup impact summary を生成する。

含める情報：

```text
- cleanup target files
- cleanup target aliases
- affected references
- affected traceability links
- affected dashboard nodes
- affected external artifacts
- rollback limitations
```

cleanup impact summary は cleanup execution plan の入力となる。

---

## 17. report schema draft

```json
{
  "schema_version": "1.0",
  "validator_module": "cleanup_gate_validator",
  "validator_run_id": "validator-YYYYMMDD-NNN",
  "status": "warn",
  "cleanup_ready": [],
  "cleanup_blocked": [
    {
      "target_id": "dry-doc-example",
      "blocking_reasons": [
        {
          "reason": "legacy_alias_not_generated",
          "source_domain": "alias"
        }
      ]
    }
  ],
  "cleanup_pending": [],
  "cache_status": "not_cached"
}
```

---

## 18. CI mapping

CI fail 条件：

```text
- cleanup_ready node に blocking reason が存在する
- semantic_integrity=fail の cleanup target
- unresolved references が残る cleanup target
- rewrite_state != rewrite_validated の cleanup target
- temporary dual canonical unresolved の cleanup target
- legacy alias required だが generated ではない cleanup target
- external artifact stale が cleanup 判定に必要
```

CI warn 条件：

```text
- cleanup_pending が存在する
- active dry-run scope に placeholder relocation が存在する
- representation hash mismatch but semantic hash matched
- non-cleanup scope の external artifact stale
```

---

## 19. cache 条件

cleanup gate validator の cache reuse 条件：

```text
- migration_index hash が一致する
- canonical_index hash が一致する
- rewrite_index hash が一致する
- reference_index hash が一致する
- traceability_index hash が一致する
- sec_id_index hash が一致する
- alias_index hash が一致する
- integrity validator result hash が一致する
- CI context hash が一致する
- validator module version が一致する
```

cache reuse 禁止条件：

```text
- cleanup gate rule changed
- blocking reason taxonomy changed
- external artifact freshness changed
- rewrite validator result changed
- alias index changed
- reference index changed
- CI result changed
```

---

## 20. dashboard projection

cleanup gate validator は dashboard projection input として以下を渡す。

```text
- cleanup_state
- cleanup_ready / blocked / pending list
- blocking reasons
- cleanup impact summary
- stale artifact status
- rollback limitations
```

Dashboard はこれを表示するが、cleanup_ready を独自に判定してはならない。

---

## 21. 禁止事項

以下を禁止する。

```text
- cleanup gate validator が file を削除すること
- cleanup gate validator が alias を生成すること
- dashboard が cleanup_ready を独自判定すること
- rewrite_executed を rewrite_validated とみなすこと
- temporary dual canonical state のまま cleanup_ready にすること
- placeholder relocation を migration complete とみなすこと
- representation hash mismatch のみで semantic cleanup を block すること
```

---

## 22. HLDocS feedback

本 implementation model から、HLDocS 側へ以下をフィードバックする。

```text
- cleanup_ready は dashboard ではなく cleanup gate validator の出力とすべき
- cleanup は verification-driven であり filesystem ordering に依存しない
- cleanup gate は unresolved references / alias / rewrite / integrity / CI を統合判定すべき
- temporary dual canonical state は cleanup_ready に進めてはならない
- representation integrity と semantic integrity を分けて cleanup 判定すべき
```

---

## 23. 結論

cleanup gate validator は、SansaVRM の大規模仕様再構成において cleanup_ready / cleanup_blocked / cleanup_pending を判定する source of truth である。

これにより、旧 path、placeholder、temporary state、obsolete artifact の削除を、検証済みの graph dependency と validator result に基づいて安全に判断できる。
