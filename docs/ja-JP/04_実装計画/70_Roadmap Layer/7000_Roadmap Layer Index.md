<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-7000-roadmap-layer-index
lang: ja-JP
canonical_title: Roadmap Layer Index
document_type: index
canonical_document: true
-->

[目次](../../../目次.md) > 実装計画 > Roadmap Layer > Roadmap Layer Index

# Roadmap Layer Index

## 1. 目的

本ドキュメントは、SansaVRM における Roadmap Layer の責務、構成、依存関係を定義する。

Roadmap Layer は、SansaVRM の将来構造、実装順序、移行計画、cleanup 計画、release 計画を扱う層である。

---

## 2. Roadmap Layer の責務

Roadmap Layer は以下を扱う。

```text
- implementation roadmap
- architecture evolution plan
- dependency roadmap
- migration plan
- cleanup plan
- release planning
- feedback planning
```

---

## 3. 非責務

Roadmap Layer は以下を直接扱わない。

```text
- Core Semantic の定義
- Data Model の実体定義
- Runtime 実行仕様
- Validation rule 本体
- Import / Export flow 本体
```

これらは各 Layer で扱う。

---

## 4. 上流依存

Roadmap Layer は全 Layer を参照できる。

```text
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
40_Runtime Integration Layer
50_Validation Layer
60_Import Export Layer
```

---

## 5. Roadmap Layer の位置づけ

Roadmap は単なる TODO ではない。

SansaVRM では Roadmap を future architecture dependency specification として扱う。

---

## 6. 現在の移行対象

既存 `04_実装計画/01_共通` の以下を Roadmap Layer へ dry-run relocation する候補とする。

```text
7001_初版実装ロードマップ
7002_ロードマップ再整理
7003_仕様依存マップ
7004_仕様再配置計画
7005_仕様再配置dry-run計画
7006_旧path_cleanup計画
7007_Layer_reorder最終計画
```

---

## 7. 追加候補

今後、以下を追加する。

```text
HLDocS feedback issue draft
Preview release roadmap
VRM I/O completion roadmap
Import Export Layer migration roadmap
Manifest federation validator roadmap
```

---

## 8. Roadmap と Validation の関係

Roadmap Layer は Validation Layer の結果を参照する。

例：

```text
- coverage analysis
- migration integrity check
- loss report
- compatibility analysis
- roundtrip verification
```

---

## 9. Roadmap と migration の関係

Roadmap Layer は migration plan を保持するが、migration manifest そのものではない。

```text
Roadmap:
いつ、どの順序で移行するか

Migration Manifest:
何がどこへ移行したか
```

---

## 10. dry-run relocation 方針

Roadmap Layer への移行は dry-run relocation として行う。

```text
- 旧pathは削除しない
- 新pathを先に作成する
- migration manifest に登録する
- cleanup は verified 後に別フェーズで判断する
```

---

## 11. 結論

Roadmap Layer は、SansaVRM の future architecture dependency を管理する Layer である。

これにより、仕様・実装・移行・cleanup・release の順序を、他 Layer と分離して管理できる。

---

[目次](../../../目次.md) > 実装計画 > Roadmap Layer > Roadmap Layer Index
