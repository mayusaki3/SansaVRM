<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-2002-format-compatibility-preservation-specification
lang: ja-JP
canonical_title: Format Compatibility Preservation Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Preservation Compatibility Layer > Format Compatibility Preservation Specification

# Format Compatibility Preservation Specification

## 1. 目的

本仕様は、FBX / MMD などの外部フォーマットを SansaVRM 経由で扱う際に、元形式へ戻すための情報保持と、別形式へ意味変換する際の損失可視化を定義する。

本仕様は以下を対象とする。

```text
- MMD → SansaVRM → MMD
- FBX → SansaVRM → FBX
- MMD → SansaVRM → VRM
- FBX → SansaVRM → VRM
- VRM → SansaVRM → 他形式
```

---

## 2. 基本方針

SansaVRM は全フォーマット完全統一仕様を目指さない。

SansaVRM は以下を原則とする。

```text
1. 共通化できる部分だけ Core Semantic として扱う
2. Format 固有情報は Compatibility Extension へ隔離する
3. 非対応情報は破棄しない
4. 非可逆変換は loss_report に記録する
5. 元形式へ戻すための preservation layer を持つ
6. Core 肥大化を禁止する
```

---

## 3. 変換目的の分離

## 3.1 情報維持優先

### 対象

```text
MMD → SansaVRM → MMD
FBX → SansaVRM → FBX
```

### 目的

```text
- 元形式へ戻せること
- SansaVRM が理解できない情報も保持すること
- 非対応情報の破棄を防ぐこと
```

---

## 3.2 相互変換

### 対象

```text
VRM1 → SansaVRM → VRM0
FBX → SansaVRM → VRM1
MMD → SansaVRM → VRM1
```

### 目的

```text
- 可能な範囲で意味変換する
- 非可逆を許容する
- loss_report に明示記録する
```

---

## 4. Format Preservation Layer

Format Preservation Layer は、SansaVRM 標準仕様で扱えない元形式固有情報を保持するための層である。

### 最低保持項目

```text
source_format
source_version
raw_properties
raw_binary_ref
unsupported_features
roundtrip_required
preservation_level
```

### 方針

```text
- raw JSON 化できる情報は raw_properties に保持する
- バイナリ保持が必要な情報は raw_binary_ref に外部 Blob 参照として保持する
- 本体肥大化を避けるため、大きな raw binary の直接埋め込みは原則避ける
```

---

## 5. Loss Report

Loss Report は、変換時の欠落、近似、非対応、警告を記録する。

### 最低保持項目

```text
losses
approximations
unsupported
warnings
source_path
target_path
severity
reason
```

### 用途

```text
- 非可逆変換可視化
- Validator 連携
- CI 検証
- Adapter 診断
- silent loss 防止
```

---

## 6. Conversion Profile

Conversion Profile は、形式間変換時のマッピングルールを定義する。

### 対象

```text
bone mapping
expression mapping
material mapping
physics mapping
coordinate mapping
unit scale mapping
morph mapping
animation mapping
```

### 用途

```text
- VRM 化
- MMD → VRM
- FBX → VRM
- 将来形式追加
```

---

## 7. Coordinate / Unit Definition

SansaVRM は座標・単位系を明示的に保持できなければならない。

### 最低保持項目

```text
up_axis
forward_axis
handedness
unit_scale
rotation_order
pre_rotation
post_rotation
```

### 理由

```text
- FBX 互換
- MMD 互換
- VRM 互換
- MuJoCo 互換
- Unity / O3DE 互換
```

---

## 8. Geometry / Rig / Skinning Extension

Geometry / Rig / Skinning は Core へ直接肥大化させず、Geometry Extension として扱う。

### 最低保持項目

```text
mesh
bone hierarchy
bind pose
inverse bind matrix
skinning method
```

### MMD 対応項目

```text
BDEF1
BDEF2
BDEF4
SDEF
QDEF
```

---

## 9. Morph Extension

Expression だけでは MMD / FBX の morph 実体を保持するには不足する。

Morph Extension は以下を保持できなければならない。

```text
morph target
morph type
weight range
material morph
bone morph
UV morph
group morph
```

---

## 10. Animation Extension

Animation Extension は animation 実体を保持する。

### 最低保持項目

```text
animation clip
channels
interpolation
frame rate
time unit
```

### FBX 対応項目

```text
stack
layer
curve
tangent
```

---

## 11. Physics Extension

SansaVRM 本体は Physics Runtime 非依存とする。

Physics Extension は以下を保持できる。

```text
rigid body
collider
joint
constraint
collision group
```

### Backend 固有情報

Backend 固有情報は Compatibility Extension 側へ隔離する。

```text
MMD physics
Bullet
MuJoCo
Unity
O3DE
```

---

## 12. RoundTrip Guarantee Level

RoundTrip 保証レベルを以下に分ける。

| Level | 内容 |
|---|---|
| Level 0 | ファイル再出力のみ可能 |
| Level 1 | 見た目維持 |
| Level 2 | 挙動維持 |
| Level 3 | バイナリ近似維持 |
| Level 4 | 完全一致 |

### 推奨

```text
MMD / FBX は Level 1〜2 を基本目標とする
完全一致は要求しない
```

---

## 13. Raw Binary Handling

未知データの保持方法は以下を許可する。

```text
- JSON保持
- Base64 Binary保持
- 外部Blob参照
```

### 推奨

```text
外部Blob参照
```

理由は、SansaVRM 本体の肥大化を防ぐためである。

---

## 14. Standardization Scope

SansaVRM の標準化範囲を以下に分ける。

| scope | 内容 |
|---|---|
| Core | model / module / slot / state |
| Geometry Extension | mesh / rig / skinning / morph |
| Animation Extension | clip / curve / timeline |
| Physics Extension | rigid body / joint / collider |
| Format Compatibility Extension | FBX / MMD / VRM / MuJoCo / URDF |

Core 肥大化は禁止する。

---

## 15. Adapter Boundary

## 15.1 SansaVRM 本体の責務

```text
- 共通構造保持
- preservation layer
- validator
- diagnostics
- loss_report
- conversion_report
```

---

## 15.2 Adapter の責務

```text
- 実フォーマット解析
- 実フォーマット出力
- runtime依存処理
- proprietary format対応
- format固有の近似変換
```

---

## 16. Validator Requirements

Validator は以下を検査対象に含める。

```text
- loss_report の存在
- unsupported の存在
- silent loss の有無
- preservation_level の整合性
- format compatibility extension の参照整合性
```

---

## 17. 推奨アーキテクチャ

```text
FBX
 └ Adapter
     └ SansaVRM Core
         ├ Geometry Extension
         ├ Animation Extension
         ├ Physics Extension
         ├ Preservation Layer
         └ Loss Report

MMD
 └ Adapter
     └ SansaVRM Core
         ├ Geometry Extension
         ├ Animation Extension
         ├ Physics Extension
         ├ Preservation Layer
         └ Loss Report

VRM
 └ Adapter
     └ SansaVRM Core
```

---

## 18. 優先実装順

## Phase 1

```text
preservation layer
loss report
conversion report
```

---

## Phase 2

```text
coordinate system
rig
skinning
morph
```

---

## Phase 3

```text
animation
physics abstraction
```

---

## Phase 4

```text
FBX adapter
MMD adapter
```

---

## Phase 5

```text
VRM export profile
VRM compatibility
```

---

## 19. 既存仕様との関係

本仕様は以下と連携する。

```text
Core Semantic Definition
Semantic Preservation Matrix
RoundTrip Semantic Criteria
Adapter Extension Property Specification
MuJoCo連携仕様
変換仕様
```

---

## 20. 結論

FBX / MMD 対応では、MuJoCo 連携と同様に Adapter / Format 固有情報を Core へ直接取り込まない。

ただし、MuJoCo と異なり、FBX / MMD では runtime 用メタデータよりも以下を重視する。

```text
- 元形式へ戻すための情報保持
- Geometry / Rig / Skinning / Morph / Animation の実体保持
- 非可逆変換の可視化
- loss_report による silent loss 防止
```

SansaVRM は共通化できる部分のみを共通化し、Format 固有情報は Compatibility Extension と Preservation Layer に隔離する。

---

[目次](../../目次.md) > 仕様 > Preservation Compatibility Layer > Format Compatibility Preservation Specification
