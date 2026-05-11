<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-3004-physics-extension-specification
lang: ja-JP
canonical_title: Physics Extension Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Data Model Layer > Physics Extension Specification

# Physics Extension Specification

## 1. 目的

本仕様は、SansaVRM における Physics 実体および Physics abstraction の保持方法を定義する。

本仕様は以下を統合するための基盤となる。

```text
- VRM spring bone
- MMD physics
- Bullet physics metadata
- MuJoCo physics
- URDF physical model
- Unity physics metadata
- O3DE physics metadata
```

---

## 2. 基本方針

SansaVRM 本体は Physics Runtime 非依存とする。

Physics Runtime 固有情報は Compatibility Extension または Adapter Extension Property へ隔離する。

以下を原則とする。

```text
1. Physics 実体は Physics Extension として保持する
2. Runtime 実装を Core Semantic に持ち込まない
3. Backend 固有 parameter は preservation 可能にする
4. 非可逆変換は loss_report に記録する
5. Constraint / Joint / Collider の意味保持を優先する
6. Runtime-specific optimization は Adapter 側責務とする
```

---

## 3. 対象範囲

本仕様の対象は以下である。

```text
- rigid body
- collider
- collision shape
- collision group
- joint
- constraint
- spring
- damping
- mass
- inertia
- runtime physics binding
```

---

## 4. 非対象範囲

以下は本仕様の直接対象外とする。

```text
- physics solver implementation
- realtime simulation engine
- GPU acceleration
- deterministic networking
- runtime threading model
```

これらは Runtime / Adapter 側で扱う。

---

## 5. Physics Container

Physics Container は Physics 情報の最上位保持単位である。

最低限以下を持つ。

```text
physics_container_id
name
source_format
rigid_body_list
collider_list
joint_list
constraint_list
preservation_ref
source_raw
```

---

## 6. Rigid Body

Rigid Body は物理挙動単位である。

最低限以下を持つ。

```text
rigid_body_id
name
body_type
mass
inertia
center_of_mass
collider_ref_list
joint_ref_list
runtime_binding_ref
source_raw
```

`body_type` は以下を許可する。

```text
static
dynamic
kinematic
trigger
custom
preserve_only
```

---

## 7. Collider

Collider は collision 判定形状である。

最低限以下を持つ。

```text
collider_id
shape_type
transform
material_ref
collision_group
source_raw
```

---

## 8. Collision Shape

shape_type は以下を許可する。

```text
sphere
capsule
box
cylinder
mesh
convex_mesh
heightfield
custom
preserve_only
```

Format 固有 shape は `custom` または `preserve_only` とし、元情報を保持する。

---

## 9. Collision Group

Collision Group は衝突制御を示す。

最低限以下を持つ。

```text
collision_group_id
name
mask
filter_rule
source_raw
```

---

## 10. Joint

Joint は Rigid Body 間の接続である。

最低限以下を持つ。

```text
joint_id
joint_type
parent_body_id
child_body_id
anchor
axis
limit
spring
motor
source_raw
```

`joint_type` は以下を許可する。

```text
fixed
hinge
slider
ball
cone_twist
six_dof
custom
preserve_only
```

---

## 11. Constraint

Constraint は joint 以外の拘束条件を示す。

最低限以下を持つ。

```text
constraint_id
constraint_type
target_ref_list
parameter_set
source_raw
```

---

## 12. Spring

Spring は spring 系物理を抽象化する。

最低限以下を持つ。

```text
spring_id
stiffness
damping
gravity_scale
collision_group_ref
runtime_parameter_ref
source_raw
```

---

## 13. Runtime Physics Binding

Runtime Physics Binding は Runtime 固有 parameter を参照する。

最低限以下を持つ。

```text
runtime_binding_id
runtime_type
adapter_scope
property_ref_list
runtime_raw_ref
```

`runtime_type` の例：

```text
mmd
bullet
mujoco
unity
o3de
custom
```

---

## 14. VRM Spring Bone Mapping

VRM 対応では以下を保持できなければならない。

```text
spring bone
collider group
joint chain
stiffness
drag force
gravity
hit radius
```

VRM 固有 parameter は Preservation Layer または Compatibility Extension に保持する。

---

## 15. MMD Physics Preservation

MMD 対応では以下を保持できなければならない。

```text
rigid body
joint
collision group
bone binding
physics mode
shape parameter
```

MMD 固有情報は source_raw または Preservation Layer に保持する。

---

## 16. MuJoCo Mapping

MuJoCo 対応では以下へ接続する。

```text
body
geom
joint
actuator
sensor
constraint
```

MuJoCo 固有 parameter は Adapter Extension Property または Compatibility Extension に保持する。

---

## 17. URDF Mapping

URDF 対応では以下へ接続する。

```text
link
joint
visual
collision
inertial
```

URDF 固有 parameter は Compatibility Extension に保持する。

---

## 18. Preservation Requirements

Physics Extension は以下を満たす。

```text
- 元形式へ戻すための source_raw を保持できる
- Runtime 固有 physics parameter を破棄しない
- raw_binary_ref を参照できる
- loss_report と接続できる
- Conversion Profile と接続できる
```

---

## 19. Loss Report Requirements

以下の損失は loss_report に記録する。

```text
- unsupported joint type
- unsupported collider type
- runtime parameter loss
- solver parameter loss
- constraint approximation
- collision filter approximation
- spring approximation
```

---

## 20. Validator Requirements

Validator は以下を検査する。

```text
- rigid body reference integrity
- collider reference integrity
- joint body reference integrity
- collision group validity
- invalid constraint reference
- unsupported shape type
- unsupported joint type
- preservation_ref integrity
```

---

## 21. RoundTrip Requirements

RoundTrip では以下を検査する。

```text
- physics semantic preservation
- source_raw preservation
- runtime binding preservation
- collision group preservation
- loss_report consistency
```

---

## 22. Adapter Boundary

### SansaVRM 本体

```text
- Physics 共通構造保持
- source_raw / preservation_ref 保持
- validator
- loss_report
```

### Adapter

```text
- format-specific physics parse
- format-specific physics export
- runtime-specific parameter interpretation
- runtime-specific optimization
- solver binding
```

---

## 23. 関連仕様

本仕様は以下と連携する。

```text
Core Semantic Definition
Geometry Rig Skinning Extension Specification
Animation Extension Specification
Adapter Extension Property Specification
Format Compatibility Preservation Specification
MuJoCo連携仕様
RoundTrip Semantic Criteria
```

---

## 24. 結論

Physics Extension は、VRM spring bone、MMD physics、Bullet、MuJoCo、URDF を接続するための実データモデル基盤である。

SansaVRM は Physics 実体を保持するが、Runtime 固有情報は Compatibility Extension または Adapter Extension Property に隔離する。

これにより、共通化できる physics 情報を利用しつつ、元形式への RoundTrip に必要な情報を破棄しない。

---

[目次](../../目次.md) > 仕様 > Data Model Layer > Physics Extension Specification
