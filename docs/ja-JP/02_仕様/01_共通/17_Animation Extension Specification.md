<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000010Z-SV02
lang: ja-JP
canonical_title: Animation Extension Specification
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > Animation Extension Specification

# Animation Extension Specification

## 1. 目的

本仕様は、SansaVRM における Animation 実体の保持方法を定義する。

本仕様は以下の animation / motion 情報を統合するための基盤となる。

```text
- FBX animation
- MMD motion
- VRM / glTF animation
- runtime animation metadata
- format-specific animation preservation
```

---

## 2. 基本方針

SansaVRM は Animation を Core Semantic へ直接肥大化させない。

Animation は Animation Extension として保持し、Format 固有情報は Compatibility Extension または Preservation Layer へ退避する。

以下を原則とする。

```text
1. Animation clip / timeline / curve を共通実体として保持する
2. Format 固有情報は破棄しない
3. 非可逆変換は loss_report に記録する
4. Runtime 再生仕様は本体仕様へ固定しない
5. time unit / frame rate / interpolation を明示する
6. FBX stack / layer / tangent 等は preservation 可能にする
```

---

## 3. 対象範囲

本仕様の対象は以下である。

```text
- animation clip
- animation track
- animation channel
- animation curve
- keyframe
- interpolation
- timeline
- frame rate
- time unit
- animation layer
- animation stack
- motion binding
```

---

## 4. 非対象範囲

以下は本仕様の直接対象外とする。

```text
- runtime playback engine
- animation state machine
- application UI timeline
- network synchronization
- realtime motion capture protocol
```

これらは Runtime Extension または Adapter Extension Property 側で扱う。

---

## 5. Animation Container

Animation Container は animation 情報の最上位保持単位である。

最低限以下を持つ。

```text
animation_container_id
name
source_format
clip_list
stack_list
preservation_ref
source_raw
```

---

## 6. Animation Clip

Animation Clip は再生可能な animation 単位である。

最低限以下を持つ。

```text
clip_id
name
start_time
end_time
duration
time_unit
frame_rate
track_list
source_clip_ref
source_raw
```

---

## 7. Animation Track

Animation Track は対象ごとの animation グループである。

最低限以下を持つ。

```text
track_id
target_type
target_id
channel_list
source_track_ref
```

`target_type` の例：

```text
bone
node
mesh
morph
material
camera
light
custom
```

---

## 8. Animation Channel

Animation Channel は対象 property の時間変化を表す。

最低限以下を持つ。

```text
channel_id
property_path
curve_id
value_type
coordinate_space
```

`property_path` の例：

```text
translation
rotation
scale
morph_weight
material.parameter
visibility
```

---

## 9. Animation Curve

Animation Curve は keyframe 列を保持する。

最低限以下を持つ。

```text
curve_id
keyframe_list
interpolation
pre_behavior
post_behavior
source_raw
```

---

## 10. Keyframe

Keyframe は時間と値の対応を示す。

最低限以下を持つ。

```text
time
value
in_tangent
out_tangent
interpolation
source_raw
```

`tangent` は任意とする。

---

## 11. Interpolation

Interpolation は以下を許可する。

```text
step
linear
cubic
bezier
hermite
custom
preserve_only
```

Format 固有 interpolation は `custom` または `preserve_only` とし、元情報を source_raw へ保持する。

---

## 12. Time Unit / Frame Rate

Animation は以下を明示する。

```text
time_unit
frame_rate
time_origin
frame_index_origin
```

`time_unit` の例：

```text
second
frame
tick
custom
```

---

## 13. FBX Animation Preservation

FBX 対応では以下を保持できなければならない。

```text
animation stack
animation layer
curve node
curve
tangent
pre/post behavior
time mode
```

FBX 固有情報は Compatibility Extension または Preservation Layer に保存する。

---

## 14. MMD Motion Preservation

MMD 対応では以下を保持できなければならない。

```text
bone motion
morph motion
camera motion
light motion
self shadow motion
IK on/off
interpolation curve
frame number
```

MMD 固有情報は source_raw または Preservation Layer に保持する。

---

## 15. VRM / glTF Animation Mapping

VRM / glTF では以下へ対応する。

```text
animation
sampler
channel
input accessor
output accessor
interpolation
```

SansaVRM は glTF / VRM へ再出力可能な animation 情報を保持する。

---

## 16. Morph Animation

Morph animation は Morph Extension の morph_id と接続する。

最低限以下を保持する。

```text
morph_id
weight_curve
blend_rule
```

---

## 17. Bone Animation

Bone animation は Rig Extension の bone_id と接続する。

最低限以下を保持する。

```text
bone_id
translation_curve
rotation_curve
scale_curve
coordinate_space
```

---

## 18. Material Animation

Material animation は Material Slot または Material Extension と接続する。

最低限以下を保持する。

```text
material_slot_id
parameter_path
value_curve
operation
```

Shader 固有 parameter は Compatibility Extension に保持してよい。

---

## 19. Preservation Requirements

Animation Extension は以下を満たす。

```text
- 元形式へ戻すための source_raw を保持できる
- Format 固有 animation 情報を破棄しない
- raw_binary_ref を参照できる
- loss_report と接続できる
- Conversion Profile と接続できる
```

---

## 20. Loss Report Requirements

以下の損失は loss_report に記録する。

```text
- interpolation loss
- tangent loss
- layer flattening
- stack flattening
- frame rate conversion
- time unit conversion
- unsupported target channel
- coordinate conversion approximation
```

---

## 21. Validator Requirements

Validator は以下を検査する。

```text
- clip time range validity
- track target reference integrity
- channel curve reference integrity
- keyframe time order
- interpolation validity
- morph reference integrity
- bone reference integrity
- material slot reference integrity
- preservation_ref integrity
```

---

## 22. RoundTrip Requirements

RoundTrip では以下を検査する。

```text
- animation semantic preservation
- keyframe preservation
- interpolation preservation
- source_raw preservation
- loss_report consistency
```

---

## 23. Adapter Boundary

### SansaVRM 本体

```text
- Animation 共通構造保持
- source_raw / preservation_ref 保持
- validator
- loss_report
```

### Adapter

```text
- format-specific animation parse
- format-specific animation export
- proprietary curve / tangent interpretation
- runtime playback conversion
```

---

## 24. 関連仕様

本仕様は以下と連携する。

```text
Core Semantic Definition
Geometry Rig Skinning Extension Specification
Morph Extension Specification
RoundTrip Semantic Criteria
Format Compatibility Preservation Specification
Conversion Profile Specification
```

---

## 25. 結論

Animation Extension は、FBX animation、MMD motion、VRM / glTF animation を接続するための実データモデル基盤である。

SansaVRM は animation 実体を保持するが、Format 固有情報は Compatibility Extension または Preservation Layer に隔離する。

これにより、共通化できる animation 情報を利用しつつ、元形式への RoundTrip に必要な情報を破棄しない。

---

[目次](../../目次.md) > 仕様 > 共通 > Animation Extension Specification
