<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-legacy-path-cleanup-plan
lang: ja-JP
canonical_title: 旧path cleanup計画
document_type: spec
canonical_document: true
-->

[目次](../../../目次.md) > 実装計画 > 共通 > 旧path cleanup計画

# 旧path cleanup計画

## 1. 目的

本ドキュメントは、SansaVRM 仕様再配置 dry-run 後に発生している旧pathと新Layer pathの並存状態を整理するための計画を定義する。

---

## 2. 現在状態

現在、以下の状態が並存している。

```text
- 旧path: docs/ja-JP/02_仕様/01_共通/
- 新Layer path: docs/ja-JP/02_仕様/10_Core Semantic Layer/
- 新Layer path: docs/ja-JP/02_仕様/20_Preservation Compatibility Layer/
- 新Layer path: docs/ja-JP/02_仕様/30_Data Model Layer/
- 新Layer path: docs/ja-JP/02_仕様/40_Runtime Integration Layer/
- 新Layer path: docs/ja-JP/02_仕様/50_Validation Layer/
```

---

## 3. cleanup 前提

旧path削除は、以下が完了するまで実施しない。

```text
1. migration_manifest.dry-run.json に全Layer文書を登録
2. mapping_status を pending / partial / complete / verified に分類
3. placeholder relocation を明示
4. sec_id mapping 方針を決定
5. 目次リンクが新Layer pathを参照
6. CIで migration manifest validation が通過
```

---

## 4. cleanup 対象分類

旧path文書は以下へ分類する。

```text
- migrated_complete
- migrated_partial
- placeholder_only
- legacy_alias
- obsolete_candidate
- keep_in_common
```

---

## 5. migrated_complete

新Layer pathへ内容移行済みで、意味保持が確認できたもの。

cleanup時に旧pathを削除候補にできる。

---

## 6. migrated_partial

新Layer pathは存在するが、全文移行またはsec_id mappingが未完了のもの。

cleanup禁止。

---

## 7. placeholder_only

新Layer pathは仮配置のみで、旧仕様本文が未移行のもの。

cleanup禁止。

対象例：

```text
4002_MuJoCo連携仕様
5001_JSONスキーマ仕様
```

---

## 8. legacy_alias

旧pathを互換リンクとして一定期間残すもの。

必要に応じて旧文書を以下へ差し替える。

```text
この文書は新Layer pathへ移行済みです。
正式仕様は以下を参照してください。
```

---

## 9. obsolete_candidate

旧pathにのみ存在し、新Layer構造に入らない候補。

削除前に正式仕様セットへの影響を確認する。

---

## 10. keep_in_common

Layer分離後も共通仕様として残す文書。

例：

```text
仕様概要
メタモデル仕様
glTF拡張仕様
CoreAPI仕様
```

---

## 11. cleanup 手順

cleanup は以下の順序で行う。

```text
1. migration manifest 拡張
2. orphan detection
3. duplicate path / duplicate doc_id 検査
4. 旧path分類表作成
5. legacy alias 化
6. verified 後に削除判断
```

---

## 12. 禁止事項

以下は禁止する。

```text
- manifest未登録の旧path削除
- placeholder_only の旧path削除
- sec_id未確認の旧path削除
- semantic_equivalent未確認の旧path削除
```

---

## 13. HLDocS feedback

本 cleanup 計画で得られた知見：

```text
- 旧path削除は relocation とは別フェーズにすべき
- legacy alias 状態が必要
- placeholder relocation 中の削除は禁止すべき
- index / TOC は migration conflict hotspot になりやすい
- cleanup gate は migration manifest に依存すべき
```

---

## 14. 結論

旧path cleanup は、単なる削除作業ではなく、migration manifest / sec_id continuity / semantic equivalence / CI validation を前提とする後段フェーズである。

現時点では旧path削除を行わず、まず migration manifest の全Layer登録と分類を優先する。

---

[目次](../../../目次.md) > 実装計画 > 共通 > 旧path cleanup計画
