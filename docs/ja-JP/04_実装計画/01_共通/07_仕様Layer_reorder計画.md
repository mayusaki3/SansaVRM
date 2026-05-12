<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-spec-layer-reorder-plan
lang: ja-JP
canonical_title: 仕様Layer reorder計画
document_type: spec
canonical_document: true
-->

[目次](../../../目次.md) > 実装計画 > 共通 > 仕様Layer reorder計画

# 仕様Layer reorder計画

## 1. 目的

本ドキュメントは、SansaVRM 仕様再配置 dry-run 後の Layer 並び順、番号規則、依存順序、reorder 実施条件を定義する。

本計画は cleanup や旧path削除を指示するものではない。

---

## 2. reorder と cleanup の分離

reorder と cleanup は別フェーズとして扱う。

```text
reorder:
新Layer構造の番号・配置・依存順序を正規化する

cleanup:
旧path削除、legacy alias化、obsolete判定を行う
```

現時点では reorder は条件付きで進行可能だが、cleanup は不可とする。

---

## 3. Layer ordering

仕様Layerは以下の順序とする。

```text
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
40_Runtime Integration Layer
50_Validation Layer
60_Import Export Layer
```

---

## 4. Layer dependency

Layer dependency は以下とする。

```text
Core Semantic Layer
↓
Preservation Compatibility Layer
↓
Data Model Layer
↓
Runtime Integration Layer
↓
Validation Layer
↓
Import Export Layer
```

ただし Validation Layer は Runtime execution には依存しない。

Validation Layer は Runtime Integration Layer の semantic を検査対象として参照する。

---

## 5. 10_Core Semantic Layer

Core Semantic Layer は意味定義を扱う。

現在の順序：

```text
1001_Core Semantic Definition
1002_Semantic Preservation Matrix
1003_RoundTrip Semantic Criteria
```

依存順序：

```text
意味定義
↓
保持方針
↓
RoundTrip成立条件
```

---

## 6. 20_Preservation Compatibility Layer

Preservation Compatibility Layer は preservation / compatibility / mapping policy を扱う。

現在の順序：

```text
2001_Adapter Extension Property Specification
2002_Format Compatibility Preservation Specification
2003_Conversion Profile Specification
```

依存順序：

```text
extension boundary
↓
preservation policy
↓
conversion mapping policy
```

---

## 7. 30_Data Model Layer

Data Model Layer は実データ実体を扱う。

現在の順序：

```text
3001_Geometry Rig Skinning Extension Specification
3002_Morph Extension Specification
3003_Animation Extension Specification
3004_Physics Extension Specification
```

依存順序：

```text
形状・骨格・スキニング
↓
変形
↓
時間変化
↓
物理実体
```

---

## 8. 40_Runtime Integration Layer

Runtime Integration Layer は runtime binding / runtime semantic を扱う。

現在の順序：

```text
4001_物理・制御メタモデル仕様
4002_MuJoCo連携仕様
```

依存順序：

```text
runtime-independent physical/control metamodel
↓
MuJoCo-specific runtime integration
```

---

## 9. 50_Validation Layer

Validation Layer は validation core と validation observability を扱う。

現在の順序：

```text
5000_Validation Layer Index
5001_JSONスキーマ仕様
5002_Validator実装仕様
5003_変換仕様
5004_Diagnostics仕様
5005_Traceability Migration仕様
5006_Coverage Analysis仕様
5007_Loss Report仕様
5008_Compatibility Analysis仕様
5009_RoundTrip Verification仕様
```

依存順序：

```text
index
↓
schema
↓
validator
↓
conversion validity
↓
diagnostics
↓
migration traceability
↓
coverage
↓
loss
↓
compatibility
↓
roundtrip verification
```

---

## 10. 60_Import Export Layer

Import Export Layer は、format-specific import/export を扱う予定の Layer である。

現時点では skeleton のみ存在する。

想定対象：

```text
VRM Import / Export
FBX Import / Export
MMD Import / Export
glTF Import / Export
URDF Import / Export
```

MuJoCo は format import/export ではなく Runtime Integration として扱う。

---

## 11. numbering policy

番号規則は以下とする。

```text
1000番台: Core Semantic Layer
2000番台: Preservation Compatibility Layer
3000番台: Data Model Layer
4000番台: Runtime Integration Layer
5000番台: Validation Layer
6000番台: Import Export Layer
```

各 Layer の index は `x000` を使用できる。

例：

```text
5000_Validation Layer Index.md
```

---

## 12. reorder gate

reorder 実施前に以下を確認する。

```text
- 目次に新Layerが登録済み
- layer index が存在する
- migration manifest または sub-manifest に登録済み
- placeholder_only 文書を削除しない
- old path cleanup を同時に行わない
```

---

## 13. cleanup gate

cleanup は reorder とは別に、以下を満たした後に行う。

```text
- semantic_equivalent = true
- mapping_status = verified
- sec_id continuity 確認済み
- legacy alias 方針確定済み
- CI validation 成功
```

---

## 14. 未完了事項

現時点で以下は未完了である。

```text
- dry-doc formalization policy
- manifest federation policy
- sec_id policy
- legacy alias template
- placeholder全文移行
- 60_Import Export Layer の実体仕様
```

---

## 15. reorder 現時点判定

```text
Layer順序: 確定可能
番号規則: 確定可能
旧path削除: 不可
legacy alias適用: 保留
Import Export Layer実体化: 後続
```

---

## 16. HLDocS feedback

本計画で得られた知見：

```text
- reorder と cleanup は分離すべき
- dependency ordering に基づく番号規則が有効
- Layer index は x000 として扱うとよい
- cleanup gate は migration manifest に依存すべき
- Import/Export と Runtime Integration は分けるべき
```

---

## 17. 結論

SansaVRM の仕様Layerは、Core Semantic、Preservation Compatibility、Data Model、Runtime Integration、Validation、Import Export の順に整理する。

reorder は dependency normalization として先行可能だが、旧path cleanup は migration verified 後に実施する。

---

[目次](../../../目次.md) > 実装計画 > 共通 > 仕様Layer reorder計画
