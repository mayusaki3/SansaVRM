<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-layer-reorder-final-plan
lang: ja-JP
canonical_title: Layer reorder最終計画
document_type: spec
canonical_document: true
-->

[目次](../../../目次.md) > 実装計画 > 共通 > Layer reorder最終計画

# Layer reorder最終計画

## 1. 目的

本ドキュメントは、SansaVRM 仕様再配置 dry-run で得られた Layer dependency をもとに、最終的な Layer 並び順と移行方針を定義する。

本計画は実ファイル移動の即時実行指示ではない。

---

## 2. 基本方針

Layer reorder は単なるファイル名変更ではなく、semantic dependency ordering として扱う。

以下を原則とする。

```text
1. Core semantic を最上流に置く
2. Preservation / Compatibility を Core の後段に置く
3. 実データモデルを Compatibility の後段に置く
4. Runtime integration を Data Model の後段に置く
5. Validation を cross-layer observer として後段に置く
6. Import / Export を Validation 利用側として後段に置く
7. Roadmap を future architecture layer として最後段に置く
```

---

## 3. 最終Layer番号案

```text
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
40_Runtime Integration Layer
50_Validation Layer
60_Import Export Layer
70_Roadmap Layer
```

10刻みを採用し、将来的な中間 Layer 挿入余地を残す。

---

## 4. Layer dependency diagram

```text
10_Core Semantic Layer
  ↓
20_Preservation Compatibility Layer
  ↓
30_Data Model Layer
  ↓
40_Runtime Integration Layer
  ↓
50_Validation Layer
  ↓
60_Import Export Layer
  ↓
70_Roadmap Layer
```

---

## 5. 10_Core Semantic Layer

### 責務

```text
- semantic definition
- semantic preservation principle
- roundtrip semantic criteria
```

### 現在の主文書

```text
1001_Core Semantic Definition
1002_Semantic Preservation Matrix
1003_RoundTrip Semantic Criteria
```

---

## 6. 20_Preservation Compatibility Layer

### 責務

```text
- adapter extension boundary
- format preservation
- conversion profile
- preservation_only / passthrough / approximation policy
```

### 現在の主文書

```text
2001_Adapter Extension Property Specification
2002_Format Compatibility Preservation Specification
2003_Conversion Profile Specification
```

---

## 7. 30_Data Model Layer

### 責務

```text
- geometry
- rig
- skinning
- morph
- animation
- physics abstraction
```

### 現在の主文書

```text
3001_Geometry Rig Skinning Extension Specification
3002_Morph Extension Specification
3003_Animation Extension Specification
3004_Physics Extension Specification
```

---

## 8. 40_Runtime Integration Layer

### 責務

```text
- runtime binding
- physical / control metamodel
- MuJoCo integration
- actuator / sensor / sysid / HIL integration
```

### 現在の主文書

```text
4001_物理・制御メタモデル仕様
4002_MuJoCo連携仕様
```

### 注意

`4002_MuJoCo連携仕様` は placeholder relocation 状態であり、全文移行完了まで旧path削除禁止とする。

---

## 9. 50_Validation Layer

### 責務

```text
- schema validation
- validator
- conversion validity
- diagnostics
- migration traceability
- coverage
- loss report
- compatibility analysis
- roundtrip verification
```

### 現在の主文書

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

### 注意

`5001_JSONスキーマ仕様` は placeholder relocation 状態であり、全文移行完了まで旧path削除禁止とする。

---

## 10. 60_Import Export Layer

### 責務

```text
- external format import
- external format export
- VRM import/export
- FBX import/export
- MMD import/export
- URDF import/export
- adapter flow
```

### 移行候補

```text
docs/ja-JP/02_仕様/02_VRM入出力/01_VRM 0.x 1.0 差分整理.md
docs/ja-JP/02_仕様/02_VRM入出力/02_humanoid Property設計.md
docs/ja-JP/02_仕様/02_VRM入出力/03_VRM 0.x import詳細設計.md
docs/ja-JP/02_仕様/02_VRM入出力/04_VRM 1.0 import詳細設計.md
```

### 方針

既存 `02_VRM入出力` を即削除せず、`60_Import Export Layer` へ dry-run relocation する。

---

## 11. 70_Roadmap Layer

### 責務

```text
- implementation roadmap
- architecture evolution plan
- migration plan
- cleanup plan
- release planning
```

### 移行候補

```text
docs/ja-JP/04_実装計画/01_共通/01_初版実装ロードマップ.md
docs/ja-JP/04_実装計画/01_共通/02_ロードマップ再整理.md
docs/ja-JP/04_実装計画/01_共通/03_仕様依存マップ.md
docs/ja-JP/04_実装計画/01_共通/04_仕様再配置計画.md
docs/ja-JP/04_実装計画/01_共通/05_仕様再配置dry-run計画.md
docs/ja-JP/04_実装計画/01_共通/06_旧path_cleanup計画.md
```

### 方針

Roadmap は単なる TODO ではなく future architecture dependency specification として扱う。

---

## 12. reorder と cleanup の分離

reorder と cleanup は別フェーズとする。

```text
reorder:
dependency ordering の正規化

cleanup:
migration verification 後の旧path整理
```

---

## 13. reorder 実施条件

reorder は以下を満たす場合に実施できる。

```text
- 新Layer path 作成済み
- 目次登録済み
- migration manifest 登録済み
- cleanup分類表作成済み
- placeholder relocation が明示済み
```

---

## 14. cleanup 実施条件

cleanup は以下を満たすまで実施しない。

```text
- placeholder relocation 解消済み
- semantic_equivalent verified
- sec_id continuity 確認済み
- legacy alias policy 適用済み
- manifest federation validator 整備済み
- CI validation 通過済み
```

---

## 15. 現時点判定

```text
Layer reorder計画: 完了
60_Import Export Layer移行: 次フェーズ
70_Roadmap Layer移行: 次フェーズ
cleanup: 未実施
旧path削除: 禁止
```

---

## 16. HLDocS feedback

本計画で得られた知見：

```text
- reorder は dependency ordering として扱うべき
- cleanup は reorder とは別フェーズにすべき
- Roadmap は future architecture layer として扱える
- Import Export は Compatibility とは別Layerにすべき
- Validation は cross-layer observer として扱える
```

---

## 17. 結論

SansaVRM の Layer reorder は、仕様ファイルの並び替えではなく、semantic dependency graph の正規化である。

本計画では、10〜70 の Layer 構造により、Core semantic から Roadmap までの依存方向を明示する。

---

[目次](../../../目次.md) > 実装計画 > 共通 > Layer reorder最終計画
