<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-1003-roundtrip-semantic-criteria
lang: ja-JP
canonical_title: RoundTrip Semantic Criteria
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Core Semantic Layer > RoundTrip Semantic Criteria

# RoundTrip Semantic Criteria

## 1. 目的

本ドキュメントは、SansaVRM における RoundTrip の意味論的成立条件を定義する。

本仕様は以下に適用される。

```text
- VRM 0.x
- VRM 1.0
- URDF
- MuJoCo
- FBX
- MMD
- 将来Adapter
```

---

## 2. 基本方針

SansaVRM は以下を原則とする。

```text
1. byte一致を要求しない
2. semantic preservation を優先する
3. Runtime semantic を重視する
4. unsupported は明示する
5. approximation は明示する
6. unknown extension は破棄しない
```

---

## 3. RoundTrip の定義

本仕様における RoundTrip とは以下を意味する。

```text
Source Format
↓ Import
SansaVRM
↓ Export
Target Format
↓ ReImport
SansaVRM
```

この過程において、Semantic が維持されることを目的とする。

---

## 4. RoundTrip 成立条件

## 4.1 Full Semantic RoundTrip

### 定義

意味的動作および意味的状態が維持される。

### 要求

以下を満たす。

```text
- semantic contradiction が存在しない
- runtime semantic が維持される
- humanoid semantic が維持される
- expression semantic が維持される
- transform semantic が維持される
```

---

## 4.2 Structural RoundTrip

### 定義

構造は維持されるが、一部 Runtime semantic が失われる。

### 要求

以下を満たす。

```text
- hierarchy が維持される
- transform relation が維持される
- topology が維持される
```

---

## 4.3 Preservation RoundTrip

### 定義

Core Semantic として理解しない情報を passthrough により保持する。

### 要求

以下を満たす。

```text
- unknown extension が破棄されない
- vendor metadata が保持される
- adapter proprietary information が保持される
```

---

## 4.4 Approximation RoundTrip

### 定義

近似変換を含む RoundTrip。

### 要求

以下を満たす。

```text
- approximation reason を diagnostics に記録する
- semantic degradation を conversion report に記録する
```

---

## 4.5 Unsupported RoundTrip

### 定義

RoundTrip 不成立。

### 要求

以下を満たす。

```text
- unsupported reason を diagnostics に記録する
- silent loss を禁止する
```

---

## 5. Semantic Category Requirements

## 5.1 Structure Semantic

### 要求

```text
- hierarchy preservation
- ownership preservation
- attachment preservation
```

---

## 5.2 Transform Semantic

### 要求

```text
- local transform preservation
- coordinate semantic preservation
- bind pose preservation
```

---

## 5.3 Humanoid Semantic

### 要求

```text
- humanoid role preservation
- retarget semantic preservation
- IK semantic preservation
```

---

## 5.4 Expression Semantic

### 要求

```text
- expression semantic preservation
- morph semantic preservation
- state switching semantic preservation
```

---

## 5.5 Physics Semantic

### 要求

Physics semantic は以下を区別する。

| 状態 | 意味 |
|---|---|
| Full | Runtime 挙動が近似維持される |
| Approximation | 数値差異が存在する |
| Unsupported | Runtime semantic を維持できない |

---

## 5.6 Runtime Semantic

### 要求

Runtime semantic は static format に完全変換できない場合がある。

その場合は以下を許可する。

```text
- passthrough
- preservation_only
- runtime extension
```

---

## 6. Format別 RoundTrip 方針

## 6.1 VRM

### 方針

```text
semantic preservation 優先
```

### 重点

```text
- humanoid
- expression
- lookAt
- spring
```

---

## 6.2 URDF

### 方針

```text
structure / physics semantic 優先
```

### 制限

```text
- expression semantic 不足
- renderer semantic 不足
```

---

## 6.3 MuJoCo

### 方針

```text
runtime physics semantic 優先
```

### 重点

```text
- actuator
- sensor
- sysid
- runtime mapping
```

---

## 6.4 FBX

### 方針

```text
DCC semantic preservation 優先
```

### 制限

```text
- proprietary semantic
- coordinate semantic 差異
```

---

## 6.5 MMD

### 方針

```text
motion / behavior semantic preservation 優先
```

### 制限

```text
- humanoid semantic 差異
- physics semantic 差異
```

---

## 7. Diagnostics Requirements

RoundTrip 時は以下を diagnostics に含める。

```text
- preservation level
- approximation reason
- semantic degradation
- unsupported semantic
- passthrough count
- runtime loss
```

---

## 8. Conversion Report Requirements

conversion report は以下を含める。

```text
- import loss
- export loss
- semantic degradation
- approximation
- unsupported
- preservation_only
```

---

## 9. Validator Requirements

Validator は以下を検証する。

```text
- semantic contradiction
- impossible roundtrip
- namespace conflict
- unsupported export
- preservation inconsistency
```

---

## 10. Silent Loss 禁止

SansaVRM は silent loss を禁止する。

### silent loss 定義

```text
semantic loss が diagnostics / conversion report に記録されない状態
```

### 要求

semantic loss が発生した場合は以下のいずれかへ分類する。

```text
- approximation
- unsupported
- preservation_only
```

---

## 11. 今後追加される仕様

以下を追加予定。

```text
- Runtime Semantic Mapping
- Physics Approximation Policy
- Namespace Migration Policy
- Adapter Compatibility Policy
```

---

## 12. 結論

SansaVRM の RoundTrip は byte一致ではなく：

```text
semantic preservation
```

を目的とする。

そのため、RoundTrip では：

```text
- Full Semantic RoundTrip
- Structural RoundTrip
- Preservation RoundTrip
- Approximation RoundTrip
- Unsupported RoundTrip
```

を区別する。

これにより、VRM、URDF、MuJoCo、FBX、MMD 等の異なる意味体系を、破壊せず統合できる。

---

[目次](../../目次.md) > 仕様 > Core Semantic Layer > RoundTrip Semantic Criteria
