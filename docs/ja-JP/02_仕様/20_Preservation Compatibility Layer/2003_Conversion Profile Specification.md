<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-2003-conversion-profile-specification
lang: ja-JP
canonical_title: Conversion Profile Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Preservation Compatibility Layer > Conversion Profile Specification

# Conversion Profile Specification

## 1. 目的

本仕様は、SansaVRM における形式間変換ルールを定義する Conversion Profile の保持方法を定義する。

本仕様は以下の形式間変換の基盤となる。

```text
- VRM 0.x ↔ VRM 1.0
- MMD ↔ SansaVRM
- FBX ↔ SansaVRM
- glTF ↔ SansaVRM
- URDF ↔ SansaVRM
- MuJoCo ↔ SansaVRM
- SansaVRM → Runtime-specific format
```

---

## 2. 基本方針

SansaVRM は形式間変換を固定ロジックとして持たない。

変換ルールは Conversion Profile として定義する。

以下を原則とする。

```text
1. mapping rule を profile 化する
2. profile は Runtime 非依存とする
3. 非可逆変換を許容する
4. loss_report に変換損失を記録する
5. Format 固有 mapping は preservation 可能にする
6. profile 差し替えによる変換戦略変更を許可する
```

---

## 3. 対象範囲

本仕様の対象は以下である。

```text
- coordinate mapping
- unit mapping
- bone mapping
- morph mapping
- animation mapping
- material mapping
- physics mapping
- semantic mapping
- approximation policy
- loss policy
```

---

## 4. 非対象範囲

以下は本仕様の直接対象外とする。

```text
- runtime playback
- renderer optimization
- realtime simulation
- GUI workflow
- application-specific pipeline
```

これらは Adapter または Runtime 側で扱う。

---

## 5. Conversion Profile Container

Conversion Profile Container は変換ルール群の最上位単位である。

最低限以下を持つ。

```text
profile_id
name
source_format
target_format
profile_version
mapping_rule_list
approximation_policy
loss_policy
source_raw
```

---

## 6. Coordinate Mapping

Coordinate Mapping は座標系変換を定義する。

最低限以下を持つ。

```text
source_up_axis
target_up_axis
source_forward_axis
target_forward_axis
source_handedness
target_handedness
rotation_order_mapping
unit_scale_mapping
```

---

## 7. Bone Mapping

Bone Mapping は Rig Semantic の対応を定義する。

最低限以下を持つ。

```text
source_bone_id
target_bone_id
semantic_role
mapping_policy
fallback_rule
source_raw
```

`mapping_policy` の例：

```text
exact
approximate
alias
ignore
preserve_only
custom
```

---

## 8. Morph Mapping

Morph Mapping は Morph Semantic の対応を定義する。

最低限以下を持つ。

```text
source_morph_id
target_morph_id
morph_type_mapping
weight_mapping
blend_rule_mapping
source_raw
```

---

## 9. Animation Mapping

Animation Mapping は animation channel の対応を定義する。

最低限以下を持つ。

```text
source_channel
target_channel
interpolation_mapping
time_unit_mapping
frame_rate_mapping
layer_mapping
source_raw
```

---

## 10. Material Mapping

Material Mapping は material semantic の対応を定義する。

最低限以下を持つ。

```text
source_material_type
target_material_type
parameter_mapping
shader_mapping
fallback_material
source_raw
```

Shader 固有 parameter は Compatibility Extension 側へ退避できる。

---

## 11. Physics Mapping

Physics Mapping は physics semantic の対応を定義する。

最低限以下を持つ。

```text
source_physics_type
target_physics_type
joint_mapping
collider_mapping
constraint_mapping
runtime_parameter_policy
source_raw
```

---

## 12. Semantic Mapping

Semantic Mapping は Format 固有 semantic の対応を定義する。

最低限以下を持つ。

```text
semantic_category
source_semantic
target_semantic
mapping_rule
fallback_rule
```

対象例：

```text
humanoid
expression
physics
material
runtime
metadata
```

---

## 13. Approximation Policy

Approximation Policy は非完全変換時の近似方針を定義する。

最低限以下を持つ。

```text
policy_id
policy_type
approximation_level
fallback_behavior
loss_record_required
```

`approximation_level` の例：

```text
none
minimal
semantic_preserving
visual_preserving
runtime_preserving
custom
```

---

## 14. Loss Policy

Loss Policy は loss_report 記録ルールを定義する。

最低限以下を持つ。

```text
loss_category
severity
record_required
abort_conversion
warning_message
```

---

## 15. VRM Mapping

VRM 対応では以下を扱う。

```text
VRM0 humanoid ↔ VRM1 humanoid
VRM expression mapping
VRM spring mapping
meta mapping
lookAt mapping
```

VRM0 ↔ VRM1 の semantic 差異は Conversion Profile 側で扱う。

---

## 16. MMD Mapping

MMD 対応では以下を扱う。

```text
bone alias
morph mapping
physics mapping
coordinate conversion
frame-based animation conversion
```

MMD 固有 semantic は Preservation Layer または Compatibility Extension に保持する。

---

## 17. FBX Mapping

FBX 対応では以下を扱う。

```text
coordinate conversion
pre/post rotation conversion
material conversion
animation stack flattening
blendshape mapping
```

FBX 固有 semantic は Preservation Layer に保持する。

---

## 18. URDF / MuJoCo Mapping

URDF / MuJoCo 対応では以下を扱う。

```text
link ↔ body
joint ↔ joint
visual ↔ geom
collision ↔ collider
runtime actuator binding
runtime sensor binding
```

Runtime-specific parameter は Adapter Extension Property へ退避する。

---

## 19. Preservation Requirements

Conversion Profile は以下を満たす。

```text
- Format 固有 mapping を保持できる
- source_raw を保持できる
- raw_binary_ref を参照できる
- loss_report と接続できる
- validator と接続できる
```

---

## 20. Loss Report Requirements

以下の損失は loss_report に記録する。

```text
- unsupported semantic mapping
- coordinate approximation
- material approximation
- morph approximation
- animation flattening
- physics approximation
- runtime parameter loss
```

---

## 21. Validator Requirements

Validator は以下を検査する。

```text
- profile reference integrity
- unsupported mapping rule
- invalid coordinate mapping
- invalid unit mapping
- bone mapping consistency
- morph mapping consistency
- animation mapping consistency
- physics mapping consistency
- approximation policy validity
```

---

## 22. RoundTrip Requirements

RoundTrip では以下を検査する。

```text
- semantic mapping preservation
- coordinate conversion reversibility
- profile consistency
- source_raw preservation
- loss_report consistency
```

---

## 23. Adapter Boundary

### SansaVRM 本体

```text
- Conversion Profile 構造保持
- mapping semantic 保持
- validator
- loss_report
```

### Adapter

```text
- format-specific conversion execution
- runtime-specific mapping execution
- proprietary parameter interpretation
- optimization policy
```

---

## 24. 関連仕様

本仕様は以下と連携する。

```text
Core Semantic Definition
Geometry Rig Skinning Extension Specification
Morph Extension Specification
Animation Extension Specification
Physics Extension Specification
Adapter Extension Property Specification
Format Compatibility Preservation Specification
RoundTrip Semantic Criteria
```

---

## 25. 結論

Conversion Profile は、VRM、FBX、MMD、URDF、MuJoCo 等の形式間変換ルールを保持するための基盤である。

SansaVRM は変換ロジックを固定実装として持たず、Conversion Profile によって変換 semantic を定義する。

これにより、Format 固有 mapping や approximation policy を profile として差し替え可能にし、semantic integration platform としての柔軟性を維持する。

---

[目次](../../目次.md) > 仕様 > Preservation Compatibility Layer > Conversion Profile Specification
