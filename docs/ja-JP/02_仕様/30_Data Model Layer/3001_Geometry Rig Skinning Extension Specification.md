<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-3001-geometry-rig-skinning-extension-specification
lang: ja-JP
canonical_title: Geometry Rig Skinning Extension Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Data Model Layer > Geometry Rig Skinning Extension Specification

# Geometry Rig Skinning Extension Specification

## 1. 目的

本仕様は、SansaVRM における Geometry / Rig / Skinning の実体保持方法を定義する。

本仕様は以下の変換・保持の基盤となる。

```text
- VRM 0.x / 1.0
- FBX
- MMD
- glTF
- URDF visual / collision
- MuJoCo geom / mesh
```

---

## 2. 基本方針

SansaVRM は Geometry / Rig / Skinning を Core Semantic へ過剰に統合しない。

以下を原則とする。

```text
1. 共通化できる実体は Geometry / Rig / Skinning Extension として保持する
2. Format 固有情報は Compatibility Extension または Preservation Layer へ退避する
3. 元形式へ戻すための情報は破棄しない
4. JSON 完全一致や binary 完全一致は要求しない
5. skinning の意味保持を優先する
6. 不明情報は source_raw または raw_binary_ref で保持する
```

---

## 3. 対象範囲

本仕様の対象は以下である。

```text
- mesh
- submesh
- primitive
- vertex attribute
- index buffer
- material slot
- bone hierarchy
- bind pose
- inverse bind matrix
- skinning method
- skinning weight
- format-specific skinning preservation
```

---

## 4. 非対象範囲

以下は本仕様の直接対象外とする。

```text
- morph target 実体
- expression mapping
- animation clip
- physics runtime
- material shader 実装
- renderer-specific optimization
```

これらは別仕様で扱う。

---

## 5. Geometry Extension

## 5.1 Geometry Container

Geometry Extension は、モデルに含まれる形状情報を保持する。

最低限以下を持つ。

```text
geometry_id
name
source_format
source_ref
mesh_list
material_slot_list
preservation_ref
```

---

## 5.2 Mesh

Mesh は形状単位である。

最低限以下を持つ。

```text
mesh_id
name
primitive_list
vertex_attribute_set
index_buffer_ref
material_slot_ref
bounds
source_raw
```

---

## 5.3 Primitive / Submesh

Primitive は描画または collision 用の分割単位である。

最低限以下を持つ。

```text
primitive_id
primitive_type
vertex_range
index_range
material_slot_id
usage
```

`usage` は以下を許可する。

```text
visual
collision
physics
proxy
preserve_only
```

---

## 5.4 Vertex Attribute

Vertex Attribute は最低限以下を扱う。

```text
position
normal
tangent
uv
color
joint_indices
joint_weights
custom_attribute
```

Format 固有 attribute は `custom_attribute` または Preservation Layer へ退避する。

---

## 5.5 Material Slot

Material Slot は material 実体ではなく、Geometry と Material の接続点を示す。

最低限以下を持つ。

```text
material_slot_id
name
source_material_ref
binding_target
```

Material の shader / rendering 詳細は別仕様または Compatibility Extension 側で扱う。

---

## 6. Rig Extension

## 6.1 Bone Hierarchy

Rig Extension は bone / joint / node 階層を保持する。

最低限以下を持つ。

```text
rig_id
root_bone_id
bone_list
source_format
source_raw
```

---

## 6.2 Bone

Bone は以下を持つ。

```text
bone_id
name
parent_bone_id
local_transform
bind_transform
inverse_bind_matrix
semantic_role
source_bone_ref
```

`semantic_role` は humanoid 等の意味付けに使用できる。

---

## 6.3 Bind Pose

Bind Pose は skinning の基準姿勢である。

最低限以下を持つ。

```text
bind_pose_id
bone_bind_transforms
inverse_bind_matrices
coordinate_space
```

---

## 6.4 Coordinate Space

Rig は以下の座標情報を保持できなければならない。

```text
up_axis
forward_axis
handedness
unit_scale
rotation_order
pre_rotation
post_rotation
```

---

## 7. Skinning Extension

## 7.1 Skinning Definition

Skinning Extension は Mesh と Rig の関係を保持する。

最低限以下を持つ。

```text
skinning_id
mesh_id
rig_id
bind_pose_id
skinning_method
influence_list
source_raw
```

---

## 7.2 Skinning Method

標準 skinning method は以下を許可する。

```text
linear_blend_skinning
dual_quaternion_skinning
preserve_only
custom
```

MMD 固有の BDEF / SDEF / QDEF は、標準化対象として扱うか preservation 対象として扱うかを明示する。

---

## 7.3 Influence

Influence は vertex と bone の重み関係である。

最低限以下を持つ。

```text
vertex_id
bone_id
weight
raw_weight
source_influence_type
```

---

## 8. MMD Skinning Preservation

MMD 対応では以下を保持できなければならない。

```text
BDEF1
BDEF2
BDEF4
SDEF
QDEF
```

### 方針

```text
- BDEF 系は標準 skinning へ変換可能
- SDEF / QDEF は approximation または preservation_only を許可
- 元形式へ戻す場合は source_raw を保持する
```

---

## 9. FBX Skinning Preservation

FBX 対応では以下を保持できなければならない。

```text
cluster
link
bind matrix
geometry transform
pre rotation
post rotation
rotation order
```

FBX 固有情報は Compatibility Extension または Preservation Layer に保存する。

---

## 10. VRM / glTF Skinning Mapping

VRM / glTF では以下へ対応する。

```text
skin
joints
inverseBindMatrices
mesh primitives
accessor
bufferView
```

SansaVRM は glTF / VRM に再出力可能な情報を保持する。

---

## 11. URDF / MuJoCo 連携

URDF / MuJoCo では Geometry を以下に利用する。

```text
visual
collision
mesh asset
geom generation
```

Skinning は通常 Runtime Mesh deformation 用であり、URDF / MuJoCo の静的 collision には直接使用しない。

ただし visual asset として保持可能とする。

---

## 12. Preservation Requirements

Geometry / Rig / Skinning は以下を満たす。

```text
- 元形式へ戻すための source_raw を保持できる
- 不明 attribute を破棄しない
- raw_binary_ref を参照できる
- loss_report と接続できる
- Conversion Profile と接続できる
```

---

## 13. Loss Report Requirements

以下の損失は loss_report に記録する。

```text
- vertex attribute loss
- skinning method approximation
- bone hierarchy change
- bind pose approximation
- coordinate conversion loss
- unsupported format-specific data
```

---

## 14. Validator Requirements

Validator は以下を検査する。

```text
- mesh reference integrity
- material slot reference integrity
- bone hierarchy integrity
- inverse bind matrix count
- skinning weight normalization
- invalid bone reference
- unsupported skinning method
- preservation_ref integrity
```

---

## 15. RoundTrip Requirements

RoundTrip では以下を検査する。

```text
- geometry semantic preservation
- rig hierarchy preservation
- skinning semantic preservation
- source_raw preservation
- raw_binary_ref preservation
- loss_report consistency
```

---

## 16. Adapter Boundary

### SansaVRM 本体

```text
- Geometry / Rig / Skinning の共通構造保持
- source_raw / preservation_ref 保持
- validator
- loss_report
```

### Adapter

```text
- format-specific parse
- format-specific export
- proprietary skinning interpretation
- renderer-specific optimization
```

---

## 17. 関連仕様

本仕様は以下と連携する。

```text
Core Semantic Definition
Semantic Preservation Matrix
RoundTrip Semantic Criteria
Adapter Extension Property Specification
Format Compatibility Preservation Specification
```

---

## 18. 結論

Geometry / Rig / Skinning Extension は、VRM / FBX / MMD / glTF / URDF / MuJoCo を接続するための実データモデル基盤である。

SansaVRM は Geometry / Rig / Skinning を保持するが、Format 固有情報は Compatibility Extension または Preservation Layer へ隔離する。

これにより、共通化できる情報を利用しつつ、元形式への RoundTrip に必要な情報を破棄しない。

---

[目次](../../目次.md) > 仕様 > Data Model Layer > Geometry Rig Skinning Extension Specification
