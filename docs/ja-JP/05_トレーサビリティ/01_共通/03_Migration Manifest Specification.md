<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000016Z-SV02
lang: ja-JP
canonical_title: Migration Manifest Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > トレーサビリティ > 共通 > Migration Manifest Specification

# Migration Manifest Specification

## 1. 目的

本仕様は、SansaVRM 仕様体系再構築時に使用する migration manifest の保存形式を定義する。

本仕様は以下を対象とする。

```text
- doc_id migration
- sec_id migration
- layer migration
- file path migration
- specification split
- specification merge
- semantic continuity tracking
- validator / CI migration check
```

---

## 2. 基本方針

migration manifest は、旧仕様体系と新仕様体系の対応関係を機械的に検査可能にするための正本である。

以下を原則とする。

```text
1. migration manifest は必ず機械可読形式で保持する
2. doc_id / sec_id 変更時は manifest 登録を必須とする
3. semantic continuity を明示する
4. split / merge を表現可能にする
5. validator / CI から検査可能にする
6. 人間向け説明は Markdown、機械向け manifest は JSON とする
```

---

## 3. 保存場所

migration manifest は以下に保存する。

```text
traces/spec_migration/migration_manifest.json
```

人間向け説明は以下に保存できる。

```text
docs/ja-JP/05_トレーサビリティ/01_共通/
```

---

## 4. manifest root structure

manifest root は最低限以下を持つ。

```json
{
  "manifest_version": "1.0",
  "migration_id": "migration-YYYYMMDD-001",
  "migration_type": "layer_refactor",
  "source_revision": "旧仕様体系の識別子",
  "target_revision": "新仕様体系の識別子",
  "entries": []
}
```

---

## 5. migration_type

migration_type は以下を許可する。

```text
layer_refactor
file_renumbering
doc_split
doc_merge
sec_split
sec_merge
path_relocation
validator_refactor
runtime_separation
compatibility_separation
```

---

## 6. entry structure

entry は最低限以下を持つ。

```json
{
  "entry_id": "entry-0001",
  "migration_type": "file_renumbering",
  "old": {},
  "new": {},
  "semantic_equivalent": true,
  "mapping_status": "complete",
  "reason": "layer_refactor",
  "sec_mappings": []
}
```

---

## 7. old / new structure

old / new は以下を持つ。

```json
{
  "doc_id": "doc-old",
  "path": "docs/.../old.md",
  "canonical_title": "旧タイトル",
  "layer_id": "old-layer",
  "document_type": "spec"
}
```

---

## 8. sec_mappings

sec_mappings は section 対応を示す。

最低限以下を持つ。

```json
{
  "old_sec_id": "sec-old",
  "new_sec_id": "sec-new",
  "mapping_type": "equivalent",
  "semantic_equivalent": true,
  "reason": "section_relocation"
}
```

---

## 9. mapping_type

mapping_type は以下を許可する。

```text
equivalent
split
merge
renamed
moved
removed
new
preserved
superseded
```

---

## 10. mapping_status

mapping_status は以下を許可する。

```text
complete
partial
pending
rejected
obsolete
```

---

## 11. semantic_equivalent

semantic_equivalent は以下を示す。

```text
旧仕様の意味が新仕様へ継承されているか
```

以下の場合は `false` とする。

```text
- semantic loss がある
- 旧仕様を廃止した
- 新仕様で意味を意図的に変更した
```

---

## 12. split / merge expression

## 12.1 split

split の場合、old は単一、new は複数を許可する。

```json
{
  "migration_type": "doc_split",
  "old": { "doc_id": "doc-old" },
  "new": [
    { "doc_id": "doc-new-a" },
    { "doc_id": "doc-new-b" }
  ]
}
```

---

## 12.2 merge

merge の場合、old は複数、new は単一を許可する。

```json
{
  "migration_type": "doc_merge",
  "old": [
    { "doc_id": "doc-old-a" },
    { "doc_id": "doc-old-b" }
  ],
  "new": { "doc_id": "doc-new" }
}
```

---

## 13. validator requirements

validator は migration manifest に対して以下を検査する。

```text
- manifest schema validity
- entry_id uniqueness
- old reference existence
- new reference existence
- orphan doc_id absence
- orphan sec_id absence
- invalid split mapping absence
- invalid merge mapping absence
- semantic_equivalent consistency
```

---

## 14. CI requirements

CI は以下を検査する。

```text
- migration_manifest.json が存在する
- schema validation が成功する
- mapping_status が pending のまま残っていない
- removed / superseded が loss_report または理由を持つ
- Traceability 参照が migration 後に解決可能
```

---

## 15. migration manifest schema

migration manifest は JSON Schema により検証する。

推奨保存先：

```text
schemas/traceability/migration_manifest.schema.json
```

---

## 16. relation to traceability manifest

migration manifest は traceability manifest を置き換えない。

役割は以下である。

```text
traceability manifest:
現在の仕様・テスト・実装対応を表す

migration manifest:
旧仕様体系から新仕様体系への対応を表す
```

---

## 17. 実施時の禁止事項

以下は禁止する。

```text
- manifest無しのdoc_id変更
- manifest無しのsec_id変更
- mapping_status pending のまま本移行完了扱いにすること
- semantic loss を理由なしに許可すること
- removed を loss_report 無しで確定すること
```

---

## 18. 関連仕様

本仕様は以下と連携する。

```text
Traceability Migration Specification
トレーサビリティ運用方針
正式仕様セット
仕様依存マップ
仕様再配置計画
```

---

## 19. 結論

migration manifest は、SansaVRM の仕様体系再構築において doc_id / sec_id 再生成を安全に扱うための機械可読な正本である。

これにより、Layer再編、番号再配置、split / merge を許可しつつ、semantic continuity と traceability continuity を検証可能にする。

---

[目次](../../目次.md) > トレーサビリティ > 共通 > Migration Manifest Specification
