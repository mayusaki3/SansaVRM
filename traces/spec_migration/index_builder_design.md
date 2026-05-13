# index builder design

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における index builder design を定義する。

index builder は、migration manifest、canonicalization manifest、rewrite transaction、filesystem snapshot、reference、traceability、external artifact を machine-readable index に正規化し、validator / dashboard / cleanup gate / CI が共通して参照できるようにする。

---

## 2. 基本方針

index builder は以下を行う。

```text
- source manifest / report / filesystem snapshot を読み込む
- validator が利用する正規化 index を生成する
- dashboard projection input の元情報を提供する
- cleanup gate 判定に必要な dependency 情報を提供する
- CI / incremental validation の差分検出に必要な hash を生成する
```

index builder は以下を行わない。

```text
- doc_id を発行しない
- sec_id を生成しない
- rewrite を実行しない
- cleanup を実行しない
- validation 判定を確定しない
```

index builder は observer / normalizer であり、state mutator ではない。

---

## 3. 入力

index builder の入力は以下とする。

```text
- migration manifest federation
- canonicalization manifest
- rewrite transaction plan/report
- legacy alias manifest
- current filesystem snapshot
- previous filesystem snapshot
- validator configuration
- external artifact metadata
- CI context
```

将来的には以下も入力対象とする。

```text
- HLDocS LLM-MANAGED block
- sec_id marker
- traceability marker
- testspec reference
- code reference
- import/export schema
- adapter diagnostics / conversion_report
```

---

## 4. 出力 index

index builder は少なくとも以下を生成する。

```text
migration_index
canonical_index
rewrite_index
filesystem_index
reference_index
traceability_index
sec_id_index
alias_index
external_artifact_index
validation_input_index
hash_index
```

初期実装では、以下を優先する。

```text
1. migration_index
2. canonical_index
3. rewrite_index
4. filesystem_index
5. reference_index
6. hash_index
```

---

## 5. migration_index

migration_index は migration manifest federation を正規化する。

主な項目：

```text
- migration_entry_id
- dry_doc_id
- source_path
- target_path
- migration_state
- layer_group
- workspace_identifier
- placeholder_state
- mapping_status
```

schema draft：

```json
{
  "migration_index": [
    {
      "migration_entry_id": "migration-entry-0001",
      "dry_doc_id": "dry-doc-example",
      "source_path": "docs/old.md",
      "target_path": "docs/new.md",
      "migration_state": "migration_verified",
      "placeholder_state": "none",
      "mapping_status": "verified"
    }
  ]
}
```

---

## 6. canonical_index

canonical_index は canonicalization manifest を正規化する。

主な項目：

```text
- canonicalization_entry_id
- dry_doc_id
- migration_entry_id
- document_fate
- old_doc_id
- canonical_doc_id
- canonicalization_status
- traceability_action
- sec_id_action
- semantic_equivalent
- cleanup_allowed
```

schema draft：

```json
{
  "canonical_index": [
    {
      "canonicalization_entry_id": "canonical-entry-0001",
      "dry_doc_id": "dry-doc-example",
      "migration_entry_id": "migration-entry-0001",
      "document_fate": "issue_new_doc_id",
      "old_doc_id": "doc-old",
      "canonical_doc_id": "doc-new",
      "canonicalization_status": "completed",
      "semantic_equivalent": true
    }
  ]
}
```

---

## 7. rewrite_index

rewrite_index は rewrite transaction plan/report を正規化する。

主な項目：

```text
- rewrite_transaction_id
- canonicalization_id
- source_migration_id
- transaction_status
- operation_id
- operation_kind
- target_file
- old_value
- new_value
- rewrite_scope
- rollback_action
- affected_files
```

schema draft：

```json
{
  "rewrite_index": [
    {
      "rewrite_transaction_id": "rewrite-YYYYMMDD-NNN",
      "transaction_status": "validated",
      "operations": [
        {
          "operation_id": "rewrite-op-0001",
          "operation_kind": "reference_rewrite",
          "target_file": "docs/example.md",
          "old_value": "doc-old",
          "new_value": "doc-new"
        }
      ]
    }
  ]
}
```

---

## 8. filesystem_index

filesystem_index は現在の repository snapshot を正規化する。

主な項目：

```text
- path
- file_kind
- exists
- size
- content_hash
- last_seen_ref
- detected_document_type
- detected_doc_id
- detected_canonical_title
```

filesystem_index は filesystem ordering の記録であり、semantic dependency を意味しない。

---

## 9. reference_index

reference_index は文書間参照、path 参照、doc_id 参照を正規化する。

主な項目：

```text
- source_path
- source_doc_id
- reference_kind
- raw_reference
- resolved_target_path
- resolved_target_doc_id
- resolution_status
```

reference_kind 候補：

```text
markdown_link
doc_id_reference
sec_id_reference
schema_reference
code_reference
external_artifact_reference
```

---

## 10. traceability_index

traceability_index は仕様・テスト・コードの紐づけを正規化する。

主な項目：

```text
- doc_id
- sec_id
- testspec_id
- code_reference
- relationship_kind
- resolution_status
```

初期実装では skeleton のみとし、traceability marker の正式抽出は後続実装とする。

---

## 11. sec_id_index

sec_id_index は sec_id の存在と衝突を検査可能にする。

主な項目：

```text
- doc_id
- sec_id
- path
- section_heading
- occurrence_count
- collision_status
```

初期実装では、sec_id が存在しない文書は `not_applicable` として扱う。

---

## 12. alias_index

alias_index は legacy alias を正規化する。

主な項目：

```text
- old_doc_id
- old_path
- canonical_doc_id
- canonical_path
- alias_state
- alias_required
- alias_generated
- expiration_policy
```

---

## 13. external_artifact_index

external_artifact_index は cross-project artifact を正規化する。

対象例：

```text
- MuJoCo Adapter draft schema
- Adapter fixture
- diagnostics.json
- conversion_report.json
- Studio AI fixture
- export profile sample
```

主な項目：

```text
- artifact_id
- artifact_kind
- source_project
- path_or_url
- freshness_status
- content_hash
- validation_required
- validation_status
```

---

## 14. hash_index

hash_index は incremental validation と validator cache のために生成する。

主な項目：

```text
- source_kind
- source_id
- hash_algorithm
- content_hash
- dependency_hash
- graph_hash
- generated_at
```

hash 対象：

```text
- manifest content
- canonicalization content
- rewrite transaction content
- filesystem content
- graph node
- graph edge
- external artifact
```

---

## 15. validation_input_index

validation_input_index は各 validator module へ渡す対象をまとめる。

主な項目：

```text
- validator_module
- target_scope
- required_indexes
- cache_key
- fallback_scope
```

これにより、validator orchestration は module ごとの入力を安定して決定できる。

---

## 16. normalization rules

index builder は以下の normalization を行う。

```text
- path separator を `/` に統一する
- duplicate path を検出する
- dry_doc_id と canonical_doc_id を混同しない
- extensionless 表示名と .md 実ファイル名を区別する
- filesystem ordering を dependency ordering として扱わない
- missing field は null または explicit unknown とする
```

---

## 17. error handling

index builder は、入力不足や不整合を即座に破壊的に修正しない。

代わりに以下を記録する。

```text
- index_build_warning
- index_build_error
- unresolved_source
- malformed_entry
- duplicate_entry
- missing_required_field
```

重大な入力不足がある場合、validator orchestration は full_scope または blocked に fallback する。

---

## 18. incremental support

index builder は前回 index と今回 index を比較し、差分を生成する。

差分対象：

```text
- added_node
- removed_node
- changed_node
- added_edge
- removed_edge
- changed_edge
- changed_file
- changed_manifest_entry
- changed_external_artifact
```

この差分は validator orchestration の impact analysis に渡す。

---

## 19. 禁止事項

以下を禁止する。

```text
- index builder が source file を書き換えること
- index builder が doc_id / sec_id を採番すること
- index builder が validation pass/fail を確定すること
- index builder が cleanup_ready を確定すること
- dashboard snapshot を source of truth として index を生成すること
- filesystem ordering を semantic dependency として index 化すること
```

---

## 20. HLDocS feedback

本 design から、HLDocS 側へ以下をフィードバックする。

```text
- validator 実装には source 文書から machine-readable index を生成する段階が必要
- index builder は state mutator ではなく normalizer として扱うべき
- migration / canonicalization / rewrite / reference / traceability index を分離すべき
- incremental validation には graph hash / dependency hash が必要
- external artifact freshness を index 化すべき
```

---

## 21. 結論

index builder は、SansaVRM の大規模仕様再構成において validator / dashboard / cleanup gate / CI が共通して参照する machine-readable index を生成する基盤である。

これにより、manifest、canonicalization、rewrite、reference、traceability、external artifact を一貫した validation input として扱える。
