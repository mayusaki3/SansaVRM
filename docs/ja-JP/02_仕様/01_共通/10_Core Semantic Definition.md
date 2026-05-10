<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000003Z-SV02
lang: ja-JP
canonical_title: Core Semantic Definition
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Core Semantic Definition

# Core Semantic Definition

## 1. 目的

本ドキュメントは、SansaVRM Core が保持すべき意味論（Semantic）を定義する。

本仕様は、以下の基準となる。

```text
- VRM Import / Export
- RoundTrip
- Validator
- URDF Mapping
- MuJoCo Mapping
- Traceability
```

SansaVRM は validator platform ではなく、外部フォーマット間の意味保存を行う統合中間表現である。

---

## 2. 基本方針

SansaVRM は以下を原則とする。

```text
1. 意味保持を優先する
2. JSON byte一致は要求しない
3. semantic preservation を保証対象とする
4. 不明情報は破壊しない
5. namespaced extension を許可する
6. passthrough を許可する
7. 外部フォーマット固有情報の強制正規化を行わない
```

---

## 3. Semantic の定義

本仕様における Semantic とは、以下を意味する。

```text
「Runtime または変換後において、
元データと同等の意味的動作または意味的状態を維持する情報」
```

これは以下を含む。

```text
- 構造
- 接続
- 状態
- 動作
- 制約
- 表現
- 物理意味
- 権利意味
- Runtime 意味
```

---

## 4. SansaVRM Core が保持すべき Semantic

## 4.1 Structure Semantic

### 定義

モデル構造および階層関係を表す意味論。

### 対象

```text
- node hierarchy
- module hierarchy
- ownership
- slot relationship
- attachment relationship
- connection relationship
```

### 要求

構造意味は、Import / Export / RoundTrip で維持されなければならない。

---

## 4.2 Transform Semantic

### 定義

座標系、変換、姿勢を表す意味論。

### 対象

```text
- translation
- rotation
- scale
- coordinate system
- local transform
- global transform
- bind pose
```

### 要求

Transform Semantic は、座標系変換後も意味的姿勢を維持しなければならない。

---

## 4.3 Humanoid Semantic

### 定義

Humanoid としての意味論。

### 対象

```text
- humanoid bone mapping
- humanoid constraints
- body semantic
- IK semantic
- retarget semantic
```

### 要求

Humanoid Semantic は bone 名ではなく、身体意味を保持しなければならない。

---

## 4.4 Expression Semantic

### 定義

表情および状態変化を表す意味論。

### 対象

```text
- expression
- blendshape
- morph target
- state expression
- material switching
- texture switching
```

### 要求

Expression Semantic は Runtime 上の視覚的意味を維持しなければならない。

---

## 4.5 Physics Semantic

### 定義

物理挙動を表す意味論。

### 対象

```text
- spring
- collider
- rigidbody
- constraint
- actuator
- inertia
- damping
```

### 要求

Physics Semantic は完全数値一致ではなく、意味的挙動一致を目標とする。

---

## 4.6 LookAt Semantic

### 定義

視線制御を表す意味論。

### 対象

```text
- eye tracking
- target tracking
- lookAt constraints
- angle limitation
```

### 要求

LookAt Semantic は Runtime 上の視線挙動を維持しなければならない。

---

## 4.7 Runtime Semantic

### 定義

Runtime 上の状態変化や接続を表す意味論。

### 対象

```text
- state transition
- module switching
- attachment switching
- runtime ownership
- dynamic connection
```

### 要求

Runtime Semantic は static file format に完全変換できない場合がある。

その場合は namespaced extension または passthrough を使用する。

---

## 4.8 Rights Semantic

### 定義

権利・許諾・収益分配を表す意味論。

### 対象

```text
- ownership
- license
- redistribution
- modification permission
- revenue share
```

### 要求

Rights Semantic は外部フォーマットが未対応でも破棄してはならない。

---

## 5. Semantic Preservation の原則

SansaVRM は以下を区別する。

| 種類 | 意味 |
|---|---|
| Semantic Preservation | 意味を保持できている |
| Structural Preservation | 構造のみ保持 |
| Approximation | 近似変換 |
| Unsupported | 非対応 |
| Preservation Only | 保持のみ可能 |

---

## 6. Import Loss と Export Loss

SansaVRM は以下を分離する。

| 種類 | 意味 |
|---|---|
| Import Loss | Import 時点で失われた意味 |
| Export Loss | Export 先表現能力不足による損失 |

この区別は diagnostics および conversion report に反映しなければならない。

---

## 7. Typed Property と Passthrough

SansaVRM は以下の二層構造を採用する。

| 層 | 用途 |
|---|---|
| Typed Property | Core Semantic として理解済みの情報 |
| Passthrough | 未理解または将来拡張情報 |

---

## 8. Unknown Extension Handling

未知 extension は以下を原則とする。

```text
- 破棄禁止
- passthrough 許可
- namespaced storage 許可
- validator warning は許可
- validator error は原則禁止
```

---

## 9. Namespace Stability

Core Semantic に属する namespace は安定性を要求する。

ただし experimental namespace は例外とする。

### Namespace 分類

| namespace | 安定性 |
|---|---|
| sansavrm.core | 安定 |
| sansavrm.runtime | 安定 |
| sansavrm.physics | 安定 |
| sansavrm.experimental | 非安定 |
| vendor.* | 外部依存 |

---

## 10. Validator の責務

Validator は以下を検証する。

```text
- Semantic contradiction
- Structural contradiction
- reference integrity
- namespace validity
- preservation state
```

Validator は以下を保証しない。

```text
- Runtime 完全一致
- Physics 数値完全一致
- Renderer 完全一致
```

---

## 11. Core Semantic の最小化

Core Semantic は最小限に保つ。

理由：

```text
Property formalization を過剰に進めると、
全外部フォーマット差異を Core が吸収する必要が生じる
```

そのため、Core Semantic に属さない情報は：

```text
- namespaced extension
- passthrough
- preservation_only
```

へ逃がす。

---

## 12. 外部フォーマットとの関係

### VRM

VRM は humanoid / expression / lookAt を重視する。

### URDF

URDF は structure / physics を重視する。

### MuJoCo

MuJoCo は runtime physics / actuator / sensor を重視する。

### FBX

FBX は DCC semantic を含む。

### MMD

MMD は motion / behavior semantic を含む。

---

## 13. 今後追加される関連仕様

以下を今後追加する。

```text
- Semantic Preservation Matrix
- RoundTrip Semantic Criteria
- Runtime Semantic Mapping
- Physics Approximation Policy
- Import Loss / Export Loss Policy
```

---

## 14. 結論

SansaVRM Core は、validator platform のための形式定義ではなく、外部フォーマット間の意味保存を行う統合中間表現である。

そのため、Core Semantic は：

```text
- 最小限
- 安定
- semantic 중심
- passthrough 許可
```

を原則とする。

これにより、VRM、URDF、MuJoCo、FBX、MMD 等の異なる意味体系を、破壊せず統合できる基盤を構築する。

---

[目次](../../目次.md) > 仕様 > 共通 > Core Semantic Definition
