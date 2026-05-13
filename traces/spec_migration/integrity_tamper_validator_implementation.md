# integrity tamper validator implementation

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成および asset lifecycle における integrity / tamper validator implementation を定義する。

integrity / tamper validator は、semantic identity、integrity proof、validation result を分離した上で、semantic-preserving rewrite、representation rewrite、signature、provenance、cross-project artifact の整合性を検査する。

---

## 2. 基本方針

integrity / tamper validator は以下を行う。

```text
- semantic_integrity を検査する
- representation_integrity を検査する
- signature_integrity を検査する
- provenance_integrity を検査する
- normalized semantic hash を検査する
- representation hash を検査する
- integrity proof と Core semantic identity の対応を検査する
- cleanup gate / dashboard / federation validator へ結果を提供する
```

integrity / tamper validator は以下を行わない。

```text
- Core semantic identity を変更しない
- integrity proof を生成しない
- signature を生成しない
- provenance chain を修正しない
- representation hash mismatch のみで semantic rewrite を実行しない
- cleanup を実行しない
```

---

## 3. 入力

integrity / tamper validator の入力は以下とする。

```text
- integrity descriptors
- normalized semantic projection
- Core semantic identity index
- representation files / filesystem_index
- provenance chain
- signature metadata
- hash_index
- rewrite transaction report
- external_artifact_index
- validator configuration
```

必要に応じて以下も参照する。

```text
- canonicalization validator report
- rewrite validator report
- cleanup gate validator report
- cross-project handoff response
```

---

## 4. 出力

integrity / tamper validator は以下を出力する。

```text
- integrity / tamper validator JSON report
- integrity / tamper validator Markdown report section
- semantic_integrity result
- representation_integrity result
- signature_integrity result
- provenance_integrity result
- tamper_state
- blocking reason list
- warning list
- dashboard projection input
- validator cache entry
```

---

## 5. integrity result model

各 integrity result の許容値：

```text
pass
warn
fail
blocked
not_applicable
```

### pass

検査対象が一致し、改ざんまたは不整合が検出されない。

### warn

不一致はあるが、semantic equivalence または cleanup continuation を直ちに否定しない。

### fail

semantic mismatch、署名不正、provenance 破損など、対象 scope の継続を禁止する不整合がある。

### blocked

必要な入力が不足し、判定できない。

### not_applicable

対象外。

---

## 6. tamper_state

tamper_state の許容値：

```text
not_checked
no_tamper_detected
semantic_mismatch
representation_mismatch
signature_invalid
provenance_broken
integrity_input_missing
projection_error
```

`representation_mismatch` は単独では semantic tamper を意味しない。

---

## 7. semantic_integrity check

semantic_integrity は normalized semantic projection に基づいて検査する。

検査：

```text
- target_identity が Core semantic identity に解決できる
- normalized semantic projection が再生成可能である
- normalized semantic hash が一致する
- canonical semantic graph が期待値と一致する
- semantic dependency が欠落していない
```

判定：

```text
normalized semantic hash match:
pass

normalized semantic hash mismatch:
fail または review_required

projection regeneration failed:
blocked または projection_error
```

---

## 8. representation_integrity check

representation_integrity は具体 representation の hash を検査する。

対象例：

```text
- JSON representation
- VRM representation
- glTF representation
- MJCF representation
- archive representation
```

判定：

```text
representation hash match:
pass

representation hash mismatch + semantic_integrity pass:
warn

representation hash mismatch + semantic_integrity fail:
fail

representation target missing:
blocked または fail
```

representation hash mismatch のみで semantic_equivalent=false としてはならない。

---

## 9. signature_integrity check

signature_integrity は signature metadata と signed target を検査する。

検査：

```text
- signature_id が存在する
- signed_target_kind が存在する
- signed_target_id が解決できる
- signature_algorithm が許容値である
- signature verification result が取得できる
- signed target hash と現在 hash が一致する
```

判定：

```text
signature valid:
pass

signature invalid:
fail または policy dependent warn

signature missing but optional:
not_applicable または warn

signature required but missing:
fail
```

---

## 10. provenance_integrity check

provenance_integrity は provenance chain の連続性と矛盾を検査する。

検査：

```text
- source_format が存在する
- source_tool / source_adapter が記録されている
- transformation_steps が順序付けられている
- rewrite transaction と transformation history が矛盾しない
- export/import chain が循環していない
- external artifact reference が stale ではない
```

判定：

```text
chain complete:
pass

optional field missing:
warn

required chain broken:
fail

freshness unknown:
blocked または warn
```

---

## 11. cross-project artifact check

MuJoCo Adapter / Studio AI 由来の external artifact は以下を検査する。

```text
- artifact freshness
- artifact content hash
- handoff response status
- diagnostics / conversion_report consistency
- updated_extension_properties classification
- export profile fixture consistency
```

cleanup 判定に必要な artifact が stale の場合は fail または blocked とする。

cleanup 判定に不要な artifact が stale の場合は warn とする。

---

## 12. rewrite / canonicalization との関係

canonicalization / rewrite 中は representation hash が変化する可能性がある。

判定方針：

```text
semantic_integrity=pass + representation_integrity=warn:
semantic-preserving rewrite として継続可能

semantic_integrity=fail:
semantic tamper / semantic rewrite / projection error として block

representation_integrity=pass + semantic_integrity=fail:
projection error または semantic index error として fail
```

---

## 13. cleanup gate との関係

cleanup gate validator は integrity / tamper validator result を参照する。

cleanup_ready 可能：

```text
semantic_integrity = pass または not_applicable
representation_integrity = pass / warn / not_applicable
signature_integrity = pass / warn / not_applicable
provenance_integrity = pass / warn / not_applicable
```

cleanup_blocked：

```text
semantic_integrity = fail
signature_integrity = fail where signature required
provenance_integrity = fail in cleanup scope
integrity input missing for required cleanup target
```

---

## 14. report schema draft

```json
{
  "schema_version": "1.0",
  "validator_module": "integrity_tamper_validator",
  "validator_run_id": "validator-YYYYMMDD-NNN",
  "status": "warn",
  "targets": [
    {
      "target_id": "semantic-target-example",
      "canonical_doc_id": "doc-example",
      "semantic_integrity": "pass",
      "representation_integrity": "warn",
      "signature_integrity": "not_applicable",
      "provenance_integrity": "pass",
      "tamper_state": "representation_mismatch",
      "warnings": [
        "representation hash changed but normalized semantic hash matched"
      ]
    }
  ],
  "cache_status": "not_cached"
}
```

---

## 15. blocking reasons

integrity / tamper validator が出力する blocking reason 候補：

```text
semantic_integrity_failed
normalized_semantic_hash_mismatch
semantic_projection_failed
representation_target_missing
signature_required_but_missing
signature_invalid
provenance_chain_broken
integrity_input_missing
external_artifact_stale_for_cleanup
cross_project_handoff_missing
```

---

## 16. CI mapping

CI fail 条件：

```text
- semantic_integrity fail
- normalized semantic hash mismatch in cleanup scope
- signature invalid where signature required
- provenance chain broken in cleanup scope
- required external artifact stale
- integrity input missing for cleanup target
```

CI warn 条件：

```text
- representation hash mismatch but semantic hash matched
- optional signature missing
- optional provenance field missing
- external artifact stale outside cleanup scope
```

---

## 17. cache 条件

integrity / tamper validator の cache reuse 条件：

```text
- normalized semantic projection hash が一致する
- representation hash が一致する
- signature metadata hash が一致する
- provenance chain hash が一致する
- external_artifact_index hash が一致する
- validator module version が一致する
- configuration hash が一致する
```

cache reuse 禁止条件：

```text
- Core semantic identity changed
- normalized projection rule changed
- representation file changed
- signature metadata changed
- provenance chain changed
- external artifact freshness changed
- validator module version changed
```

---

## 18. dashboard projection

integrity / tamper validator は dashboard projection input として以下を渡す。

```text
- integrity_state
- tamper_state
- semantic_integrity
- representation_integrity
- signature_integrity
- provenance_integrity
- blocking reasons
- warnings
- external artifact freshness
```

Dashboard はこれを表示するが、integrity result を変更してはならない。

---

## 19. 禁止事項

以下を禁止する。

```text
- representation hash mismatch のみで semantic_equivalent=false と断定すること
- integrity validator が integrity proof を生成すること
- integrity validator が Core semantic を変更すること
- integrity validator が provenance chain を自動修正すること
- optional signature missing を無条件 fail とすること
- stale external artifact を freshness 確認なしに pass とすること
```

---

## 20. HLDocS feedback

本 implementation model から、HLDocS 側へ以下をフィードバックする。

```text
- semantic integrity と representation integrity を分離すべき
- normalized semantic hash を validation 対象として扱うべき
- representation hash mismatch は warn と semantic fail を分離すべき
- signature / provenance は Core semantic ではなく Preservation / Validation concern として扱うべき
- cross-project artifact freshness を integrity validation に含めるべき
```

---

## 21. 結論

integrity / tamper validator は、semantic identity、integrity proof、validation result を分離し、semantic-preserving rewrite と実際の改ざん・不整合を区別するための validator module である。

これにより、canonicalization、rewrite、cleanup gate、cross-project artifact validation を integrity 観点で安全に接続できる。
