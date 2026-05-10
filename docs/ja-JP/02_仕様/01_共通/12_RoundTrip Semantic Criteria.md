<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000005Z-SV02
lang: ja-JP
canonical_title: RoundTrip Semantic Criteria
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > RoundTrip Semantic Criteria

# RoundTrip Semantic Criteria

## 1. 目的

本ドキュメントは、SansaVRM における RoundTrip 成功条件を定義する。

本仕様は以下の基準となる。

```text
- Import / Export
- RoundTrip validation
- diagnostics
- compatibility analysis
- conversion report
- regression test
```

---

## 2. 基本方針

SansaVRM の RoundTrip は、以下を原則とする。

```text
1. semantic preservation を優先する
2. JSON byte一致は要求しない
3. structural equivalence を許可する
4. approximation を許可する
5. unsupported は diagnostics 化する
6. passthrough を保持する
```

---

## 3. RoundTrip の定義

本仕様における RoundTrip とは以下を指す。

```text
Import
↓
SansaVRM Core
↓
Export
↓
ReImport
```

または：

```text
Format A
↓
SansaVRM
↓
Format B
↓
SansaVRM
```

---

## 4. RoundTrip Category

## 4.1 Same-Format RoundTrip

### 定義

同一フォーマット間の往復変換。

### 例

```text
VRM0
↓
SansaVRM
↓
VRM0
```

---

## 4.2 Cross-Format RoundTrip

### 定義

異なるフォーマット間の往復変換。

### 例

```text
VRM0
↓
SansaVRM
↓
VRM1
↓
SansaVRM
```

---

## 4.3 Partial RoundTrip

### 定義

一部 semantic が unsupported または approximation となる RoundTrip。

### 例

```text
MMD
↓
SansaVRM
↓
VRM1
```

---

## 5. RoundTrip Success Level

## 5.1 Full Semantic Success

### 条件

以下を満たす。

```text
- semantic contradiction がない
- semantic loss がない
- unsupported semantic がない
- approximation が許可範囲内
```

---

## 5.2 Structural Success

### 条件

以下を満たす。

```text
- structure semantic が維持される
- transform semantic が維持される
- runtime semantic loss を許可
```

---

## 5.3 Approximation Success

### 条件

以下を満たす。

```text
- approximation reason が diagnostics に記録される
- semantic contradiction がない
- Runtime behavior が近似範囲内
```

---

## 5.4 Preservation Only Success

### 条件

以下を満たす。

```text
- semantic を保持している
- Core Semantic としては理解しない
- passthrough で維持できる
```

---

## 5.5 Failure

### 条件

以下のいずれか。

```text
- semantic contradiction
- unsupported mandatory semantic
- transform corruption
- reference corruption
- namespace corruption
- unrecoverable import loss
```

---

## 6. Semantic Equivalence

## 6.1 定義

Semantic Equivalence とは以下を意味する。

```text
Runtime または変換後において、
意味的状態が等価である
```

---

## 6.2 非要求事項

以下は要求しない。

```text
- JSON field ordering
- whitespace
- float textual formatting
- exporter-specific metadata ordering
- extension ordering
```

---

## 7. Structure Semantic Criteria

以下を維持する。

```text
- hierarchy
- ownership
- connection
- attachment
- reference integrity
```

---

## 8. Transform Semantic Criteria

以下を維持する。

```text
- pose semantic
- coordinate semantic
- bind semantic
- orientation semantic
```

ただし座標系変換は許可する。

---

## 9. Humanoid Semantic Criteria

以下を維持する。

```text
- humanoid mapping
- body semantic
- retarget semantic
- IK semantic
```

bone 名完全一致は要求しない。

---

## 10. Expression Semantic Criteria

以下を維持する。

```text
- facial semantic
- expression semantic
- morph semantic
- material switching semantic
```

---

## 11. Physics Semantic Criteria

Physics semantic は以下を原則とする。

```text
- 数値完全一致は要求しない
- 意味的挙動一致を目標とする
- Runtime approximation を許可する
```

---

## 12. Runtime Semantic Criteria

Runtime semantic は static format へ完全変換できない場合がある。

その場合：

```text
- approximation
- preservation_only
- unsupported
```

を明示する。

---

## 13. Rights Semantic Criteria

Rights semantic は以下を満たす。

```text
- 破棄禁止
- passthrough 許可
- extension storage 許可
```

---

## 14. Unknown Extension Criteria

Unknown extension は以下を原則とする。

```text
- preservation_only 許可
- passthrough 許可
- validator warning 許可
- validator error 原則禁止
```

---

## 15. Cross-Format Semantic Rule

異なるフォーマット間では、以下を許可する。

```text
- semantic approximation
- structural preservation
- runtime semantic reduction
```

ただし、以下は禁止する。

```text
- silent semantic destruction
- silent transform corruption
- silent reference corruption
```

---

## 16. Import Loss と Export Loss

## 16.1 Import Loss

Import 時点で semantic を取得できなかった状態。

### 例

```text
URDF に expression semantic が存在しない
```

---

## 16.2 Export Loss

Export 先の表現能力不足による semantic loss。

### 例

```text
Runtime semantic
↓
VRM0
```

---

## 17. Diagnostics Requirements

RoundTrip 時は以下を diagnostics に含める。

```text
- preservation level
- semantic loss
- approximation reason
- import loss
- export loss
- unsupported semantic
- passthrough status
```

---

## 18. Validator Requirements

Validator は以下を検証する。

```text
- semantic contradiction
- impossible mapping
- transform corruption
- reference corruption
- namespace corruption
- preservation inconsistency
```

---

## 19. Regression Test Requirements

RoundTrip regression test は以下を検証する。

```text
- semantic preservation stability
- approximation stability
- namespace stability
- passthrough stability
- diagnostics stability
```

---

## 20. 代表ケース

## 20.1 VRM0 → SansaVRM → VRM0

期待：

```text
Full Semantic Success
```

---

## 20.2 VRM0 → SansaVRM → VRM1

期待：

```text
Cross-Format Semantic Success
```

---

## 20.3 URDF → SansaVRM → MuJoCo

期待：

```text
Structural Success
+
Physics Semantic Preservation
```

---

## 20.4 MMD → SansaVRM → VRM1

期待：

```text
Approximation Success
```

---

## 21. 今後追加される仕様

以下を追加予定。

```text
- Physics Approximation Policy
- Runtime Semantic Mapping
- Namespace Migration Policy
- Semantic Compatibility Matrix
- Semantic Diff Format
```

---

## 22. 結論

SansaVRM の RoundTrip は、JSON 完全一致を目的としない。

目的は：

```text
semantic preservation
```

である。

そのため、RoundTrip 判定では：

```text
- Full Semantic Success
- Structural Success
- Approximation Success
- Preservation Only Success
- Failure
```

を区別する。

これにより、異なる外部フォーマット間でも、意味的整合性を維持した変換を実現する。

---

[目次](../../目次.md) > 仕様 > 共通 > RoundTrip Semantic Criteria
