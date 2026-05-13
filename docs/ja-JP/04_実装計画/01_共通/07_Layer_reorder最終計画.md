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

ただし、Layer番号自体は意味論を持たず、ファイルシステム上の並び順制御として扱う。

Layer の責務と dependency は、Layer Index、dependency diagram、migration manifest、および本文定義によって定義する。

以下を原則とする。

```text
1. Core semantic を上流に置く
2. Preservation / Compatibility を Core の依存先として配置する
3. 実データモデルを Compatibility の依存先として配置する
4. Runtime integration を Data Model の依存先として配置する
5. Validation を cross-layer observer として配置する
6. Import / Export を Validation 利用側として配置する
7. Roadmap を future architecture layer として最後段へ配置する
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

なお、上記番号は dependency 意味論ではなく、並び順制御目的である。

---

## 4. Layer dependency diagram

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
  ↓
Roadmap Layer
```

dependency は上記 diagram により定義する。

---

## 5. Core Semantic Layer

### 責務

```text
- semantic definition
- semantic preservation principle
- roundtrip semantic criteria
```

### 現在の主文書

```text
Core Semantic Definition
Semantic Preservation Matrix
RoundTrip Semantic Criteria
```

---

## 6. Preservation Compatibility Layer

### 責務

```text
- adapter extension boundary
- format preservation
- conversion profile
- preservation_only / passthrough / approximation policy
```

### 現在の主文書

```text
Adapter Extension Property Specification
Format Compatibility Preservation Specification
Conversion Profile Specification
```

---

## 7. Data Model Layer

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
Geometry Rig Skinning Extension Specification
Morph Extension Specification
Animation Extension Specification
Physics Extension Specification
```

---

## 8. Runtime Integration Layer

### 責務

```text
- runtime binding
- physical / control metamodel
- MuJoCo integration
- actuator / sensor / sysid / HIL integration
```

### 現在の主文書

```text
物理・制御メタモデル仕様
MuJoCo連携仕様
```

### 注意

`MuJoCo連携仕様` は placeholder relocation 状態であり、全文移行完了まで旧path削除禁止とする。

---

## 9. Validation Layer

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
Validation Layer Index
JSONスキーマ仕様
Validator実装仕様
変換仕様
Diagnostics仕様
Traceability Migration仕様
Coverage Analysis仕様
Loss Report仕様
Compatibility Analysis仕様
RoundTrip Verification仕様
```

### 注意

`JSONスキーマ仕様` は placeholder relocation 状態であり、全文移行完了まで旧path削除禁止とする。

---

## 10. Import Export Layer

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
VRM 0.x 1.0 差分整理
humanoid Property設計
VRM 0.x import詳細設計
VRM 1.0 import詳細設計
```

### 方針

既存 `02_VRM入出力` を即削除せず、`60_Import Export Layer` へ dry-run relocation する。

---

## 11. Roadmap Layer

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
初版実装ロードマップ
ロードマップ再整理
仕様依存マップ
仕様再配置計画
仕様再配置dry-run計画
旧path_cleanup計画
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
Import Export Layer移行: 次フェーズ
Roadmap Layer移行: 次フェーズ
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
- Layer番号は dependency 意味論ではなく並び順制御として扱うべき
```

---

## 17. 結論

SansaVRM の Layer reorder は、仕様ファイルの並び替えではなく、semantic dependency graph の正規化である。

Layer dependency は Layer Index、dependency diagram、および migration manifest によって定義する。

Layer番号は並び順制御用であり、dependency 意味論そのものではない。

---

[目次](../../../目次.md) > 実装計画 > 共通 > Layer reorder最終計画
