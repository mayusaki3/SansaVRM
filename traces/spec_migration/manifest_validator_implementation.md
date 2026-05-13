# manifest validator implementation

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における manifest validator implementation を定義する。

manifest validator は、migration manifest federation と filesystem snapshot の整合性を検査し、後続の canonicalization validator / rewrite validator / cleanup gate が安全に実行できる前提を確認する。

---

## 2. 基本方針

manifest validator は以下を行う。

```text
- migration manifest federation を検査する
- migration_entry_id / dry_doc_id の一意性を検査する
- source_path / target_path の重複を検査する
- placeholder relocation の残存を検査する
- mapping_status を検査する
- filesystem snapshot との整合を検査する
- orchestration graph へ blocking reason を提供する
```

manifest validator は以下を行わない。

```text
- manifest を修正しない
- file を移動しない
- doc_id を採番しない
- canonicalization を実行しない
- cleanup を実行しない
```

---

## 3. 入力

manifest validator の入力は index builder が生成した以下の index とする。

```text
- migration_index
- filesystem_index
- hash_index
- validation_input_index
```

必要に応じて以下も参照する。

```text
- external_artifact_index
- orchestration graph
- previous validator cache
```

---

## 4. 出力

manifest validator は以下を出力する。

```text
- manifest validator JSON report
- manifest validator Markdown report section
- blocking reason list
- warning list
- dashboard projection input
- validator cache entry
```

---

## 5. 検査対象

manifest validator は以下を検査する。

```text
1. manifest loadability
2. entry identity uniqueness
3. dry_doc_id uniqueness
4. migration_entry_id uniqueness
5. path normalization
6. source_path existence
7. target_path existence
8. duplicate path
9. placeholder relocation
10. mapping_status
11. migration_state consistency
12. filesystem snapshot consistency
13. hash consistency
14. federation consistency
```

---

## 6. manifest loadability

migration manifest federation が読み込めない場合、manifest validator は `fail` とする。

判定：

```text
pass:
すべての対象 manifest が読み込める

warn:
任意 manifest が欠落しているが、対象 scope の検査には不要

fail:
必須 manifest が読み込めない

blocked:
manifest の所在が未定義
```

---

## 7. entry identity uniqueness

以下は federation 全体で一意でなければならない。

```text
- migration_entry_id
- dry_doc_id
```

同一 `dry_doc_id` が複数 entry に現れる場合は `fail` とする。

ただし、split / merge 予定が manifest 上で明示されている場合は、canonicalization validator 側で扱うため、manifest validator では `warn` とする。

---

## 8. path normalization

manifest validator は path を以下のように正規化して検査する。

```text
- path separator は `/` に統一する
- trailing slash を除去する
- duplicate whitespace を検出する
- extensionless 表示名と .md 実ファイル名を混同しない
```

正規化後に path collision が発生する場合は `fail` とする。

---

## 9. source_path / target_path existence

filesystem_index を使用して、source_path / target_path の存在を確認する。

判定：

```text
source_path missing:
既に relocation 済みで target_path が存在する場合は warn
source_path も target_path も存在しない場合は fail

target_path missing:
planned state なら warn
migration_verified state なら fail
```

---

## 10. duplicate path detection

以下の重複を検出する。

```text
- same source_path in multiple entries
- same target_path in multiple entries
- source_path and target_path collision
- normalized path collision
```

duplicate path は基本的に `fail` とする。

ただし、temporary dual canonical state が別 manifest で明示されている場合は、manifest validator では `warn` とし、canonicalization validator に委譲する。

---

## 11. placeholder relocation check

placeholder relocation が残っている場合、manifest validator は `warn` または `fail` を返す。

判定：

```text
warn:
active dry-run scope で placeholder が明示されている

fail:
migration_verified / cleanup_ready 対象に placeholder が残っている

blocked:
placeholder_state が不明
```

placeholder relocation が残る node は cleanup_ready にしてはならない。

---

## 12. mapping_status check

mapping_status の許容値：

```text
pending
partial
verified
blocked
not_applicable
```

判定：

```text
verified:
pass

pending / partial:
warn または cleanup_blocking warning

blocked:
fail

unknown:
blocked
```

---

## 13. migration_state consistency

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

不明な migration_state は `fail` とする。

migration_state と filesystem state が矛盾する場合は `fail` とする。

例：

```text
migration_verified だが target_path が存在しない
→ fail

planned だが target_path が既に存在する
→ warn

placeholder_only だが cleanup_ready 候補
→ fail
```

---

## 14. filesystem snapshot consistency

filesystem_index と migration_index を突合する。

確認項目：

```text
- target_path exists
- target_path content_hash exists
- detected_doc_id が取得可能か
- file_kind が expected kind と一致するか
- duplicate detected_doc_id がないか
```

`detected_doc_id` の重複は canonicalization validator でも検査するが、manifest validator では早期 warning として出力する。

---

## 15. hash consistency

hash_index を用いて、manifest 入力の差分を検出する。

検査対象：

```text
- manifest hash
- migration_index hash
- filesystem_index hash
- graph input hash
```

hash が取得できない場合は `warn` とする。

incremental validation で hash が必要な場合に欠落している場合は `blocked` とする。

---

## 16. federation consistency

複数 manifest を federation として扱う場合、以下を検査する。

```text
- federation id が一致する
- source_migration_id が一致する
- duplicate entry がない
- parent / child manifest の参照が解決できる
- layer group が filesystem ordering として扱われている
```

layer group を semantic dependency として扱っている manifest entry が見つかった場合は `warn` とする。

---

## 17. report schema draft

```json
{
  "schema_version": "1.0",
  "validator_module": "manifest_validator",
  "validator_run_id": "validator-YYYYMMDD-NNN",
  "status": "warn",
  "checked_entries": 10,
  "failures": [],
  "warnings": [
    {
      "reason": "placeholder_relocation_remaining",
      "source_domain": "migration",
      "migration_entry_id": "migration-entry-0001",
      "dry_doc_id": "dry-doc-example"
    }
  ],
  "cache_status": "not_cached"
}
```

---

## 18. blocking reasons

manifest validator が出力する blocking reason 候補：

```text
manifest_missing
manifest_malformed
migration_entry_id_duplicate
dry_doc_id_duplicate
source_path_missing
target_path_missing
duplicate_path
placeholder_relocation_remaining
migrated_partial
mapping_status_blocked
migration_state_invalid
filesystem_snapshot_missing
hash_required_but_missing
federation_reference_unresolved
```

---

## 19. CI mapping

CI fail 条件：

```text
- 必須 manifest が読み込めない
- migration_entry_id duplicate
- dry_doc_id duplicate
- migration_verified target_path missing
- duplicate path collision
- mapping_status blocked
- invalid migration_state
- federation reference unresolved
```

CI warn 条件：

```text
- planned target_path already exists
- source_path missing but target_path exists
- placeholder relocation in active dry-run scope
- mapping_status pending / partial
- hash missing for non-incremental validation
```

---

## 20. cache 条件

manifest validator の cache reuse 条件：

```text
- manifest hash が一致する
- migration_index hash が一致する
- filesystem_index hash が一致する
- validator module version が一致する
- configuration hash が一致する
```

cache reuse 禁止条件：

```text
- manifest file changed
- filesystem snapshot changed
- path normalization rule changed
- validator module version changed
- federation target changed
```

---

## 21. dashboard projection

manifest validator は dashboard projection input として以下を渡す。

```text
- migration_state
- placeholder_state
- mapping_status
- manifest validation status
- manifest blocking reasons
- manifest warnings
```

Dashboard はこれを表示するが、manifest validation result を変更してはならない。

---

## 22. 禁止事項

以下を禁止する。

```text
- duplicate path を自動修正すること
- missing target_path を自動生成すること
- dry_doc_id duplicate を自動解決すること
- manifest validator が canonical_doc_id を発行すること
- placeholder relocation を cleanup_ready として扱うこと
- filesystem ordering を semantic dependency として扱うこと
```

---

## 23. HLDocS feedback

本 implementation model から、HLDocS 側へ以下をフィードバックする。

```text
- migration manifest federation の検査は canonicalization / cleanup 前に必要
- placeholder relocation は cleanup_blocking state として扱うべき
- path normalization と semantic identity は分離すべき
- manifest validator は修正器ではなく observer / gate として扱うべき
- filesystem ordering を semantic dependency と誤用しない検査が必要
```

---

## 24. 結論

manifest validator は、migration manifest federation と filesystem snapshot の整合性を検査する最初の validator module である。

これにより、後続の canonicalization validator、rewrite validator、cleanup gate が安全に実行できる前提を確保する。
