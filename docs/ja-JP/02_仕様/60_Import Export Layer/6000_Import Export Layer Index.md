<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-6000-import-export-layer-index
lang: ja-JP
canonical_title: Import Export Layer Index
document_type: index
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Import Export Layer > Import Export Layer Index

# Import Export Layer Index

## 1. 目的

本ドキュメントは、SansaVRM における Import Export Layer の責務、構成、依存関係を定義する。

Import Export Layer は、外部フォーマットと SansaVRM の間で行う import / export flow を扱う層である。

---

## 2. Import Export Layer の責務

Import Export Layer は以下を扱う。

```text
- external format import
- external format export
- format-specific mapping flow
- adapter invocation flow
- VRM import/export
- FBX import/export
- MMD import/export
- URDF import/export
- MuJoCo adapter import/export
```

---

## 3. 非責務

Import Export Layer は以下を直接扱わない。

```text
- Core Semantic の定義
- preservation policy の定義
- data model primitive の定義
- runtime execution
- validation observability
```

これらは上流 Layer を参照する。

---

## 4. 上流依存

Import Export Layer は以下に依存する。

```text
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
40_Runtime Integration Layer
50_Validation Layer
```

---

## 5. Compatibility Layer との違い

Compatibility Layer は semantic preservation policy を定義する。

Import Export Layer は、その policy を利用して実際の import / export flow を定義する。

---

## 6. Validation Layer との関係

Import Export Layer は Validation Layer を利用する。

例：

```text
- import 後 validation
- export 前 validation
- loss_report 出力
- diagnostics 出力
- roundtrip verification
```

---

## 7. 現在の移行対象

現時点では、既存 `02_VRM入出力` の文書群を Import Export Layer へ dry-run relocation する。

```text
6001_VRM 0.x 1.0 差分整理
6002_humanoid Property設計
6003_VRM 0.x import詳細設計
6004_VRM 1.0 import詳細設計
```

---

## 8. 将来追加候補

将来的に以下を追加する。

```text
FBX Import Export Specification
MMD Import Export Specification
URDF Import Export Specification
MuJoCo Adapter Import Export Specification
VRM Export Profile Specification
```

---

## 9. dry-run relocation 方針

本 Layer への移行は dry-run relocation として行う。

```text
- 旧pathは削除しない
- 新pathを先に作成する
- migration manifest に登録する
- semantic_equivalent を確認する
- cleanup は verified 後に別フェーズで判断する
```

---

## 10. 結論

Import Export Layer は、Compatibility / Data Model / Validation を利用して、外部フォーマットとの import / export flow を定義する責務層である。

これにより、VRM / FBX / MMD / URDF / MuJoCo などの外部形式連携を、Core Semantic や Validation から分離して管理できる。

---

[目次](../../目次.md) > 仕様 > Import Export Layer > Import Export Layer Index
