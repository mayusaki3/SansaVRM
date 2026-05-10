<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000004Z-SV02
lang: ja-JP
canonical_title: Semantic Preservation Matrix
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Semantic Preservation Matrix

# Semantic Preservation Matrix

## 1. 目的

本ドキュメントは、SansaVRM と外部フォーマット間の変換において、どの Semantic をどのレベルで保持できるかを定義する。

本仕様は以下の基準となる。

```text
- Import / Export
- RoundTrip
- diagnostics
- conversion report
- validator
- compatibility analysis
```

---

## 2. 基本方針

SansaVRM は以下を原則とする。

```text
1. Semantic Preservation を優先する
2. JSON byte一致は要求しない
3. Runtime semantic を重視する
4. Unknown extension を破壊しない
5. unsupported と preservation_only を分離する
6. approximation を明示する
```

---

## 3. Preservation Level 定義

## 3.1 Full Semantic Preservation

### 定義

意味的動作および意味的状態を維持できる。

### 例

```text
VRM humanoid
VRM expression
VRM lookAt
```

---

## 3.2 Structural Preservation

### 定義

構造は維持できるが、Runtime semantic は一部失われる。

### 例

```text
URDF → SansaVRM
```

における renderer semantic。

---

## 3.3 Approximation

### 定義

近似変換。

完全な semantic 一致ではない。

### 例

```text
MMD spring
↓
VRM spring
```

---

## 3.4 Preservation Only

### 定義

SansaVRM 内部では保持可能だが、Core Semantic として理解しない。

### 例

```text
vendor extension
unknown metadata
```

---

## 3.5 Unsupported

### 定義

Import または Export が未対応。

### 要求

unsupported は diagnostics に記録しなければならない。

---

## 4. Semantic Category

本仕様では、以下を semantic category とする。

| category | 内容 |
|---|---|
| structure | node / hierarchy / ownership |
| transform | transform / coordinate |
| humanoid | humanoid semantic |
| expression | expression semantic |
| physics | spring / rigidbody / constraint |
| lookAt | eye tracking / target tracking |
| runtime | runtime switching / dynamic connection |
| rights | ownership / license / revenue |
| extension | unknown extension / vendor extension |

---

## 5. VRM 0.x ↔ SansaVRM

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | |
| transform | Full Semantic Preservation | |
| humanoid | Full Semantic Preservation | |
| expression | Full Semantic Preservation | blendshape 含む |
| physics | Approximation | spring runtime 差異あり |
| lookAt | Full Semantic Preservation | |
| runtime | Preservation Only | Runtime state は passthrough 使用可 |
| rights | Preservation Only | VRM 未対応部分あり |
| extension | Preservation Only | unknown extension passthrough |

---

## 6. VRM 1.0 ↔ SansaVRM

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | |
| transform | Full Semantic Preservation | |
| humanoid | Full Semantic Preservation | |
| expression | Full Semantic Preservation | expression system |
| physics | Approximation | runtime 差異あり |
| lookAt | Full Semantic Preservation | |
| runtime | Preservation Only | Runtime semantic 拡張使用 |
| rights | Preservation Only | Core 側保持 |
| extension | Preservation Only | extension passthrough |

---

## 7. SansaVRM → VRM 0.x

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | |
| transform | Full Semantic Preservation | |
| humanoid | Full Semantic Preservation | |
| expression | Full Semantic Preservation | |
| physics | Approximation | runtime physics 差異 |
| lookAt | Full Semantic Preservation | |
| runtime | Unsupported | static format 制限 |
| rights | Preservation Only | extension 使用 |
| extension | Preservation Only | passthrough |

---

## 8. SansaVRM → VRM 1.0

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | |
| transform | Full Semantic Preservation | |
| humanoid | Full Semantic Preservation | |
| expression | Full Semantic Preservation | |
| physics | Approximation | |
| lookAt | Full Semantic Preservation | |
| runtime | Unsupported | Runtime semantic 制限 |
| rights | Preservation Only | extension 使用 |
| extension | Preservation Only | passthrough |

---

## 9. URDF ↔ SansaVRM

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | link/joint |
| transform | Full Semantic Preservation | |
| humanoid | Structural Preservation | humanoid semantic 不足 |
| expression | Unsupported | |
| physics | Full Semantic Preservation | inertial/constraint |
| lookAt | Unsupported | |
| runtime | Structural Preservation | runtime state 不足 |
| rights | Preservation Only | Core 側保持 |
| extension | Preservation Only | vendor extension |

---

## 10. MuJoCo ↔ SansaVRM

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | body hierarchy |
| transform | Full Semantic Preservation | |
| humanoid | Structural Preservation | humanoid semantic 不足 |
| expression | Unsupported | |
| physics | Full Semantic Preservation | actuator/sensor 含む |
| lookAt | Unsupported | |
| runtime | Full Semantic Preservation | runtime physics |
| rights | Preservation Only | Core 側保持 |
| extension | Preservation Only | adapter passthrough |

---

## 11. FBX ↔ SansaVRM

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | |
| transform | Full Semantic Preservation | |
| humanoid | Structural Preservation | DCC semantic 差異 |
| expression | Approximation | morph semantic 差異 |
| physics | Approximation | DCC physics 差異 |
| lookAt | Approximation | |
| runtime | Unsupported | |
| rights | Preservation Only | |
| extension | Preservation Only | proprietary data |

---

## 12. MMD ↔ SansaVRM

| semantic | preservation | 備考 |
|---|---|---|
| structure | Full Semantic Preservation | |
| transform | Full Semantic Preservation | |
| humanoid | Approximation | humanoid semantic 差異 |
| expression | Approximation | morph semantic 差異 |
| physics | Approximation | runtime 差異 |
| lookAt | Approximation | |
| runtime | Approximation | behavior semantic |
| rights | Preservation Only | |
| extension | Preservation Only | custom metadata |

---

## 13. Import Loss と Export Loss

## 13.1 Import Loss

### 定義

Import 時点で外部フォーマットから semantic を取得できなかった状態。

### 例

```text
URDF に expression semantic が存在しない
```

---

## 13.2 Export Loss

### 定義

Export 先フォーマットの表現能力不足による semantic loss。

### 例

```text
SansaVRM runtime semantic
↓
VRM 0.x
```

---

## 14. Diagnostics 要求

変換時は以下を diagnostics に含める。

```text
- preservation level
- approximation reason
- import loss
- export loss
- passthrough count
- unsupported semantic
```

---

## 15. Validator 要求

Validator は以下を検証する。

```text
- semantic contradiction
- impossible mapping
- namespace conflict
- unsupported export
- preservation inconsistency
```

---

## 16. 今後追加される仕様

以下を追加予定。

```text
- RoundTrip Semantic Criteria
- Physics Approximation Policy
- Runtime Semantic Mapping
- Namespace Migration Policy
- Semantic Compatibility Matrix
```

---

## 17. 結論

SansaVRM は、全フォーマットを完全一致変換することを目的としない。

目的は：

```text
semantic preservation
```

である。

そのため、変換では：

```text
- Full Semantic Preservation
- Structural Preservation
- Approximation
- Preservation Only
- Unsupported
```

を明示的に区別する。

これにより、VRM、URDF、MuJoCo、FBX、MMD 等の異なる意味体系を、破壊せず統合できる。

---

[目次](../../目次.md) > 仕様 > 共通 > Semantic Preservation Matrix
