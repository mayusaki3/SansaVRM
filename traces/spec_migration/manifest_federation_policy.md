# manifest federation policy

## 1. 目的

本ポリシーは、SansaVRM 仕様再配置 dry-run において分離された複数の migration manifest を、federated migration graph として扱うための方針を定義する。

本ポリシーは manifest 統合実行を指示するものではない。

---

## 2. 背景

仕様Layer再配置では、以下の manifest が分離して存在している。

```text
traces/spec_migration/migration_manifest.dry-run.json
traces/spec_migration/migration_manifest.validation-layer.dry-run.json
```

単一 manifest に全entryを集約すると、巨大化と競合が発生しやすい。

---

## 3. 基本方針

巨大 relocation では、単一 manifest ではなく federated manifest を許可する。

以下を原則とする。

```text
1. Layer別 sub-manifest を許可する
2. root manifest は sub-manifest を参照できる
3. federation validator により全体整合性を検査する
4. entry_id は federation 全体で一意とする
5. doc_id / path / sec_id の重複は federation 全体で検査する
```

---

## 4. manifest 種別

manifest は以下に分類する。

| 種別 | 役割 |
|---|---|
| root_manifest | 全体の入口 |
| layer_manifest | Layer単位の migration graph |
| topic_manifest | 特定関心事の migration graph |
| validation_manifest | validation / observability 系 migration graph |

---

## 5. root_manifest

root_manifest は sub-manifest を参照する。

最低限以下を持つ。

```text
manifest_version
migration_id
sub_manifest_refs
federation_policy
```

---

## 6. sub_manifest_refs

sub_manifest_refs は以下を持つ。

```text
manifest_id
path
scope
required
```

例：

```json
{
  "manifest_id": "validation-layer",
  "path": "traces/spec_migration/migration_manifest.validation-layer.dry-run.json",
  "scope": "50-validation-layer",
  "required": true
}
```

---

## 7. federation validator

federation validator は複数 manifest を読み込み、全体整合性を検査する。

検査対象：

```text
- duplicate entry_id
- duplicate new.path
- duplicate formal_doc_id
- duplicate sec_id mapping conflict
- orphan sub_manifest
- missing required sub_manifest
- invalid migration_status
```

---

## 8. entry_id 一意性

entry_id は federation 全体で一意でなければならない。

Layer別 prefix を推奨する。

例：

```text
core-entry-0001
validation-entry-0001
runtime-entry-0001
```

---

## 9. path 一意性

new.path は federation 全体で一意でなければならない。

old.path は複数 entry で参照される可能性があるが、split relocation の場合は明示する。

---

## 10. doc_id alias graph

doc_id alias は federation 全体で解決する。

以下を検査する。

```text
- alias loop
- unresolved alias
- duplicate formal_doc_id
- dry_doc_id without formal_doc_id
```

---

## 11. sec_id mapping graph

sec_id mapping は federation 全体で検査する。

以下を検査する。

```text
- duplicate old_sec_id mapping conflict
- duplicate new_sec_id conflict
- orphan sec_id
- missing sec_id mapping
```

---

## 12. manifest merge policy

sub-manifest は必要に応じて root manifest へ統合できる。

ただし、統合は必須ではない。

推奨は以下とする。

```text
- 開発中: federated manifest
- release前: federation validator による全体検査
- archive時: root manifest + sub-manifest のまま保存
```

---

## 13. CI Requirements

CI は root manifest から sub-manifest を解決し、federation validation を実行できる。

fail 条件例：

```text
- required sub-manifest missing
- duplicate entry_id
- duplicate new.path
- unresolved doc_id alias
- pending entry remains for release
```

---

## 14. cleanup との関係

旧path cleanup は federation validator の成功後にのみ検討する。

cleanup gate：

```text
- all required manifests loaded
- no duplicate path
- no orphan sec_id
- all cleanup target entries verified
```

---

## 15. HLDocS feedback

本ポリシーで得られた知見：

```text
- 巨大 relocation では single manifest が競合源になる
- federated migration graph が必要
- root manifest / sub-manifest 構造が有効
- federation validator が必要
- doc_id alias graph と sec_id mapping graph は federation 全体で解決すべき
```

---

## 16. 結論

SansaVRM の仕様再配置では、単一 manifest ではなく federated manifest を許可する。

Layer別 sub-manifest を root manifest から参照し、federation validator により全体整合性を検査する。
