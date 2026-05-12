# legacy alias template

## 1. 目的

本テンプレートは、SansaVRM 仕様再配置後に旧pathへ残す legacy alias 文書の標準形式を定義する。

legacy alias は旧path削除の代替または移行期間中の互換リンクとして使用する。

---

## 2. 使用条件

legacy alias は以下を満たす場合に使用できる。

```text
- 新Layer path へ全文移行済み
- semantic_equivalent = true
- mapping_status = verified
- sec_id continuity 確認済み
- 目次が新Layer pathを参照済み
```

---

## 3. 使用禁止条件

以下の場合は legacy alias 化してはならない。

```text
- placeholder_only
- migrated_partial
- semantic_equivalent = unknown
- sec_id continuity 未確認
- formal_doc_id 未発行
- doc_id_aliases 未登録
```

---

## 4. legacy alias の役割

legacy alias は以下を行う。

```text
- 旧path利用者へ移行先を案内する
- 旧URLの参照切れを防ぐ
- migration manifest の alias graph と対応する
- 旧仕様本文の二重保守を防ぐ
```

---

## 5. 標準テンプレート

以下を旧path文書へ差し替える。

```markdown
<!--
HLDocS:LLM-MANAGED
doc_id: __LEGACY_DOC_ID__
lang: ja-JP
canonical_title: __CANONICAL_TITLE__
document_type: spec
canonical_document: false
alias_document: true
alias_target_doc_id: __TARGET_DOC_ID__
alias_target_path: __TARGET_PATH__
-->

[目次](../../../目次.md) > 仕様 > 共通 > __CANONICAL_TITLE__

# __CANONICAL_TITLE__

## 1. 移行案内

本ドキュメントは旧path互換用の alias 文書である。

正式仕様は以下へ移行済みである。

```text
__TARGET_PATH__
```

---

## 2. 参照先

- [__CANONICAL_TITLE__](__TARGET_RELATIVE_LINK__)

---

## 3. 注意

本ドキュメント本文は保守対象外とする。

仕様内容は移行先文書を正とする。

---

[目次](../../../目次.md) > 仕様 > 共通 > __CANONICAL_TITLE__
```

---

## 6. alias metadata

legacy alias には以下の metadata を持たせる。

```text
alias_document: true
alias_target_doc_id
alias_target_path
```

---

## 7. manifest 連携

migration manifest には以下を登録する。

```json
{
  "alias": {
    "legacy_path": "__LEGACY_PATH__",
    "target_path": "__TARGET_PATH__",
    "legacy_doc_id": "__LEGACY_DOC_ID__",
    "target_doc_id": "__TARGET_DOC_ID__",
    "alias_status": "active"
  }
}
```

---

## 8. legacy alias と cleanup の関係

legacy alias は cleanup の前段または代替である。

```text
verified relocation
↓
legacy alias 化
↓
一定期間維持
↓
削除判断
```

---

## 9. CI Requirements

CI は legacy alias を検査対象に含める。

検査内容：

```text
- alias_target_path が存在する
- alias_target_doc_id が存在する
- canonical_document = false
- alias_document = true
- 旧本文が残っていない
```

---

## 10. 対象候補

以下は verified 後に legacy alias 化候補である。

```text
05_Validator実装仕様.md
07_変換仕様.md
08_物理・制御メタモデル仕様.md
10_Core Semantic Definition.md
11_Semantic Preservation Matrix.md
12_RoundTrip Semantic Criteria.md
13_Adapter Extension Property Specification.md
14_Format Compatibility Preservation Specification.md
15_Geometry Rig Skinning Extension Specification.md
16_Morph Extension Specification.md
17_Animation Extension Specification.md
18_Physics Extension Specification.md
19_Conversion Profile Specification.md
```

---

## 11. HLDocS feedback

本テンプレートで得られた知見：

```text
- relocation後に旧pathを即削除しない運用が必要
- alias_document metadata が必要
- alias_target_doc_id / alias_target_path が必要
- canonical_document=false の alias 文書を許容すべき
- CIでalias integrityを検査すべき
```

---

## 12. 結論

legacy alias は、仕様再配置後の旧path互換性を維持し、旧本文の二重保守を防ぐための移行支援文書である。

legacy alias 化は verified relocation 後にのみ実施し、placeholder_only や partial migration には適用しない。
