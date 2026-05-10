<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000009Z-SV02
lang: ja-JP
canonical_title: Morph Extension Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Morph Extension Specification

# Morph Extension Specification

## 1. 目的

本仕様は、SansaVRM における Morph 実体の保持方法を定義する。

本仕様は以下の表現を統合するための基盤となる。

```text
- VRM expression
- VRM blendshape
- FBX BlendShape
- MMD morph
- glTF morph target
```

---

## 2. 基本方針

SansaVRM は Morph を Expression Semantic だけで扱わない。

Expression は意味論であり、Morph は変形・変更の実体である。

以下を原則とする。

```text
1. Expression Semantic と Morph 実体を分離する
2. 共通化できる morph は Morph Extension として保持する
3. Format 固有 morph は Compatibility Extension または Preservation Layer へ退避する
4. 未対応 morph は破棄せず source_raw または raw_binary_ref で保持する
5. 非可逆変換は loss_report に記録する
```

---

## 3. 対象範囲

本仕様の対象は以下である。

```text
- morph target
- morph set
- morph channel
- morph binding
- vertex morph
- material morph
- bone morph
- UV morph
- group morph
- weight range
- morph category
```

---

## 4. 非対象範囲

以下は本仕様の直接対象外とする。

```text
- animation clip
- expression preset policy
- material shader 実装
- runtime facial tracking
- application UI mapping
```

これらは別仕様で扱う。

---

## 5. Morph Extension

## 5.1 Morph Container

Morph Extension は、モデルに含まれる morph 実体を保持する。

最低限以下を持つ。

```text
morph_container_id
name
source_format
morph_set_list
expression_binding_list
preservation_ref
```

---

## 5.2 Morph Set

Morph Set は morph の集合である。

最低限以下を持つ。

```text
morph_set_id
name
morph_list
source_raw
```

---

## 5.3 Morph

Morph は単一の変更単位である。

最低限以下を持つ。

```text
morph_id
name
morph_type
category
weight_range
default_weight
binding_list
source_morph_ref
source_raw
```

---

## 5.4 Morph Type

morph_type は以下を許可する。

```text
vertex
normal
tangent
uv
material
bone
group
visibility
custom
preserve_only
```

---

## 5.5 Morph Category

category は morph の用途分類である。

```text
facial
phoneme
emotion
blink
look
body
cloth
material
system
custom
```

---

## 6. Morph Binding

Morph Binding は morph が変更する対象を示す。

最低限以下を持つ。

```text
binding_id
target_type
target_id
channel
value_delta
source_raw
```

---

## 7. Vertex Morph

Vertex Morph は頂点位置差分を保持する。

最低限以下を持つ。

```text
mesh_id
vertex_id
position_delta
normal_delta
tangent_delta
```

`normal_delta` と `tangent_delta` は任意とする。

---

## 8. Material Morph

Material Morph は material parameter の変更を保持する。

最低限以下を持つ。

```text
material_slot_id
parameter_name
operation
value
```

`operation` は以下を許可する。

```text
replace
add
multiply
custom
```

Shader 固有 parameter は Compatibility Extension へ退避してよい。

---

## 9. Bone Morph

Bone Morph は bone transform の変更を保持する。

最低限以下を持つ。

```text
bone_id
translation_delta
rotation_delta
scale_delta
coordinate_space
```

---

## 10. UV Morph

UV Morph は UV 差分を保持する。

最低限以下を持つ。

```text
mesh_id
vertex_id
uv_channel
uv_delta
```

---

## 11. Group Morph

Group Morph は複数 morph の合成である。

最低限以下を持つ。

```text
group_morph_id
member_morph_id
member_weight
```

---

## 12. Weight Range

Morph は weight range を持つ。

最低限以下を持つ。

```text
min
max
default
normalized
```

`normalized = true` の場合、通常 `0.0` から `1.0` を使用する。

---

## 13. Expression Binding

Expression Binding は Expression Semantic と Morph 実体の対応を示す。

最低限以下を持つ。

```text
expression_id
morph_id
weight
blend_rule
```

`blend_rule` は以下を許可する。

```text
override
additive
multiply
max
custom
```

---

## 14. VRM Mapping

VRM では Morph Extension を以下へ対応させる。

```text
VRM 0.x BlendShape
VRM 1.0 Expression
```

VRM preset は Expression Binding として保持する。

---

## 15. MMD Morph Preservation

MMD 対応では以下を保持できなければならない。

```text
vertex morph
bone morph
UV morph
material morph
group morph
```

MMD 固有情報は source_raw または Preservation Layer に保持する。

---

## 16. FBX BlendShape Preservation

FBX 対応では以下を保持できなければならない。

```text
blend shape channel
target shape
full weight
inbetween target
```

FBX 固有情報は Compatibility Extension または Preservation Layer に保持する。

---

## 17. glTF Morph Target Mapping

SansaVRM は glTF morph target へ再出力可能な情報を保持する。

対応対象：

```text
POSITION
NORMAL
TANGENT
weights
```

---

## 18. Preservation Requirements

Morph Extension は以下を満たす。

```text
- 元形式へ戻すための source_raw を保持できる
- Format 固有 morph を破棄しない
- raw_binary_ref を参照できる
- loss_report と接続できる
- Conversion Profile と接続できる
```

---

## 19. Loss Report Requirements

以下の損失は loss_report に記録する。

```text
- unsupported morph type
- material morph approximation
- bone morph approximation
- group morph flattening
- weight range normalization
- shader parameter loss
- inbetween target loss
```

---

## 20. Validator Requirements

Validator は以下を検査する。

```text
- morph target reference integrity
- morph weight range validity
- material slot reference integrity
- bone reference integrity
- mesh reference integrity
- group morph cycle
- unsupported morph type
- expression binding integrity
```

---

## 21. RoundTrip Requirements

RoundTrip では以下を検査する。

```text
- morph semantic preservation
- source_raw preservation
- expression binding preservation
- group morph preservation
- loss_report consistency
```

---

## 22. Adapter Boundary

### SansaVRM 本体

```text
- Morph 共通構造保持
- Expression Binding 保持
- source_raw / preservation_ref 保持
- validator
- loss_report
```

### Adapter

```text
- format-specific morph parse
- format-specific morph export
- shader-specific material morph interpretation
- proprietary morph preservation
```

---

## 23. 関連仕様

本仕様は以下と連携する。

```text
Core Semantic Definition
Geometry Rig Skinning Extension Specification
Semantic Preservation Matrix
RoundTrip Semantic Criteria
Format Compatibility Preservation Specification
Conversion Profile Specification
```

---

## 24. 結論

Morph Extension は、VRM expression、MMD morph、FBX BlendShape、glTF morph target を接続するための実データモデル基盤である。

SansaVRM は Expression Semantic と Morph 実体を分離し、共通化できる morph を Morph Extension として保持する。

Format 固有情報は Compatibility Extension または Preservation Layer に隔離し、元形式への RoundTrip に必要な情報を破棄しない。

---

[目次](../../目次.md) > 仕様 > 共通 > Morph Extension Specification
