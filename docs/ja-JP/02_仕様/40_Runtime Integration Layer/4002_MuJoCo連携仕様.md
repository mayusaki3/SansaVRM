<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4002-mujoco-integration-specification
lang: ja-JP
canonical_title: MuJoCo連携仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Runtime Integration Layer > MuJoCo連携仕様

# MuJoCo連携仕様

## 1. 移行状態

本ドキュメントは、以下の旧仕様を Runtime Integration Layer へ移行するための dry-run relocation placeholder である。

```text
旧path:
docs/ja-JP/02_仕様/01_共通/09_MuJoCo連携仕様.md

新path:
docs/ja-JP/02_仕様/40_Runtime Integration Layer/4002_MuJoCo連携仕様.md
```

---

## 2. 旧仕様の扱い

旧仕様は長文であり、現時点では全文移行を未実施とする。

本 dry-run では以下を目的とする。

```text
- 新Layer path の確保
- doc_id migration の仮確定
- hierarchy link の仮確定
- migration_manifest.dry-run.json での partial 管理
```

---

## 3. 移行方針

本仕様は、後続ステップで旧仕様全文を参照し、以下を満たす形で差し替える。

```text
- 内容削除禁止
- 意味変更禁止
- 章構造の保持または migration map への明示
- hierarchy link の Runtime Integration Layer 化
- 関連仕様リンクの新Layer path 対応
```

---

## 4. 旧仕様の要約ではないこと

本ドキュメントは旧仕様の要約ではない。

本ドキュメントは dry-run relocation 用の仮配置であり、本移行完了時には旧仕様全文に差し替える。

---

## 5. 関連仕様

本仕様は以下と連携する。

```text
物理・制御メタモデル仕様
Adapter Extension Property Specification
Format Compatibility Preservation Specification
Conversion Profile Specification
Physics Extension Specification
Migration Manifest Specification
```

---

[目次](../../目次.md) > 仕様 > Runtime Integration Layer > MuJoCo連携仕様
