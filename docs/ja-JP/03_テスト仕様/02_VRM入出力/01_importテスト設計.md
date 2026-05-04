<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260504-000501Z-SV0V
lang: ja-JP
canonical_title: importテスト設計
document_type: testspec
canonical_document: true
-->

[目次](../../目次.md) > テスト仕様 > VRM入出力 > importテスト設計

# importテスト設計

## 1. 目的

VRM 0.x / VRM 1.0 import 処理が SansaVRM Model へ正しく変換されることを確認するテスト設計を定義する。

---

## 2. テスト対象

対象crate：

- `crates/sansavrm-vrm`

対象機能：

- VRM 0.x import
- VRM 1.0 import
- humanoid Property化
- Module / Connection / Geometry 変換
- Material / Texture / Metadata 変換
- passthrough保持
- validator連携

---

## 3. テスト方針

### 3.1 基本方針

- VRM仕様構造を直接coreへ持ち込まないことを確認する
- import結果が SansaVRM Model として validator を通過することを確認する
- JSON完全一致ではなく、意味情報の保持を確認する
- 未対応情報は破棄せず passthrough に保持されることを確認する

---

## 4. VRM 0.x importテスト

### T-F5-001 vrm0_import_minimal

目的：

- 最小構成の VRM 0.x を import できることを確認する。

入力：

- `extensions.VRM`
- `extensions.VRM.humanoid`
- 最小node
- 最小mesh
- 最小meta

期待結果：

- import成功
- Module が生成される
- Rig Property が生成される
- Metadata Property が生成される
- validator PASS

---

### T-F5-002 vrm0_import_humanoid

目的：

- VRM 0.x humanoid が Rig Property + Binding に変換されることを確認する。

入力：

- `extensions.VRM.humanoid.humanBones`

期待結果：

- PropertyType::Rig が生成される
- PropertyContext::Simulation / Execution が設定される
- bone → Module の Binding が生成される
- node index が Module ID に変換される

---

### T-F5-003 vrm0_import_nodes_connections

目的：

- node と hierarchy が Module / Connection に変換されることを確認する。

入力：

- `nodes[]`
- `nodes[].children`

期待結果：

- nodeごとに Module が生成される
- 親子関係ごとに Connection が生成される
- root node は親なしとして扱われる

---

### T-F5-004 vrm0_import_mesh_geometry

目的：

- mesh が Geometry に変換されることを確認する。

入力：

- `meshes[]`
- `nodes[].mesh`

期待結果：

- Geometry が生成される
- mesh index と Geometry ID の対応が保持される
- Module から Geometry を参照できる

---

### T-F5-005 vrm0_import_blendshape_expression

目的：

- blendShapeMaster が Expression Property に変換されることを確認する。

入力：

- `extensions.VRM.blendShapeMaster.blendShapeGroups`

期待結果：

- PropertyType::Expression が生成される
- PropertyContext::Rendering / Execution が設定される
- presetName / name が保持される
- bind情報が Geometry ID へ変換される

---

### T-F5-006 vrm0_import_material_texture

目的：

- VRM 0.x materialProperties と glTF material / texture が Material / Texture Property に変換されることを確認する。

入力：

- `materials[]`
- `extensions.VRM.materialProperties[]`
- `textures[]`
- `images[]`
- `samplers[]`

期待結果：

- Material Property が生成される
- Texture Property が生成される
- texture参照が Texture Property ID に変換される
- MToon固有情報が保持される

---

### T-F5-007 vrm0_import_meta

目的：

- VRM 0.x meta が Metadata Property に変換されることを確認する。

入力：

- `extensions.VRM.meta`

期待結果：

- PropertyType::Metadata が生成される
- PropertyContext::Description が設定される
- title / version / author / license 情報が保持される

---

## 5. VRM 1.0 importテスト

### T-F5-008 vrm1_import_minimal

目的：

- 最小構成の VRM 1.0 を import できることを確認する。

入力：

- `extensions.VRMC_vrm`
- `extensions.VRMC_vrm.humanoid`
- 最小node
- 最小mesh
- 最小meta

期待結果：

- import成功
- Module が生成される
- Rig Property が生成される
- Metadata Property が生成される
- validator PASS

---

### T-F5-009 vrm1_import_humanoid

目的：

- VRM 1.0 humanoid が Rig Property + Binding に変換されることを確認する。

入力：

- `extensions.VRMC_vrm.humanoid.humanBones`

期待結果：

- PropertyType::Rig が生成される
- PropertyContext::Simulation / Execution が設定される
- bone → Module の Binding が生成される
- node index が Module ID に変換される

---

### T-F5-010 vrm1_import_nodes_connections

目的：

- VRM 1.0 glTF node と hierarchy が Module / Connection に変換されることを確認する。

入力：

- `nodes[]`
- `nodes[].children`

期待結果：

- nodeごとに Module が生成される
- 親子関係ごとに Connection が生成される
- 複数rootを許容する

---

### T-F5-011 vrm1_import_mesh_geometry

目的：

- VRM 1.0 mesh が Geometry に変換されることを確認する。

入力：

- `meshes[]`
- `nodes[].mesh`

期待結果：

- Geometry が生成される
- primitive / attributes / morph target が保持される
- Module から Geometry を参照できる

---

### T-F5-012 vrm1_import_expression

目的：

- VRM 1.0 expressions が Expression Property に変換されることを確認する。

入力：

- `extensions.VRMC_vrm.expressions.preset`
- `extensions.VRMC_vrm.expressions.custom`

期待結果：

- PropertyType::Expression が生成される
- preset / custom の区別が保持される
- morphTargetBinds が Geometry ID に変換される
- materialColorBinds が Material Property ID に変換される
- override系設定が保持される

---

### T-F5-013 vrm1_import_material_texture

目的：

- VRM 1.0 material と `VRMC_materials_mtoon` が Material / Texture Property に変換されることを確認する。

入力：

- `materials[]`
- `materials[].extensions.VRMC_materials_mtoon`
- `textures[]`
- `images[]`
- `samplers[]`

期待結果：

- Material Property が生成される
- Texture Property が生成される
- MToon固有情報が保持される
- textureInfo が Texture Property ID に変換される

---

### T-F5-014 vrm1_import_meta

目的：

- VRM 1.0 meta が Metadata Property に変換されることを確認する。

入力：

- `extensions.VRMC_vrm.meta`

期待結果：

- PropertyType::Metadata が生成される
- PropertyContext::Description が設定される
- name / version / authors / licenseUrl / avatarPermission 等が保持される

---

## 6. 異常系テスト

### T-F5-015 vrm0_missing_extension

目的：

- VRM 0.x extension が存在しない場合に import 失敗することを確認する。

期待結果：

- import失敗
- `extensions.VRM` 不在エラー

---

### T-F5-016 vrm1_missing_vrmc_vrm_extension

目的：

- VRM 1.0 extension が存在しない場合に import 失敗することを確認する。

期待結果：

- import失敗
- `VRMC_vrm` 不在エラー

---

### T-F5-017 missing_humanoid_node

目的：

- humanoid bone が参照する node index が存在しない場合に import 失敗することを確認する。

期待結果：

- import失敗
- node未解決エラー

---

### T-F5-018 duplicate_human_bone

目的：

- humanoid bone が重複する場合に import 失敗することを確認する。

期待結果：

- import失敗
- duplicate bone エラー

---

### T-F5-019 cycle_node_hierarchy

目的：

- node hierarchy に循環参照がある場合に import 失敗することを確認する。

期待結果：

- import失敗
- cycle hierarchy エラー

---

### T-F5-020 expression_unresolved_reference

目的：

- expression / blendshape が未解決の mesh / material を参照する場合の扱いを確認する。

期待結果：

- VRM 1.0 expression の未解決参照は import失敗
- VRM 0.x blendshape の未解決参照も import失敗
- passthrough可能な未知フィールドは破棄しない

---

## 7. 情報保持テスト

### T-F5-021 passthrough_unknown_extension

目的：

- 未対応 extension が passthrough に保持されることを確認する。

期待結果：

- import成功
- unknown extension が passthrough に保持される
- export時に復元可能な配置情報を持つ

---

### T-F5-022 passthrough_extras

目的：

- extras が passthrough に保持されることを確認する。

期待結果：

- node / material / modelルートの extras が保持される

---

### T-F5-023 passthrough_springbone

目的：

- Phase 1で直接変換しない springBone 情報が保持されることを確認する。

期待結果：

- VRM 0.x secondaryAnimation が保持される
- VRM 1.0 VRMC_springBone が保持される
- import成功

---

### T-F5-024 passthrough_node_constraint

目的：

- VRM 1.0 node constraint が passthrough に保持されることを確認する。

期待結果：

- VRMC_node_constraint が保持される
- import成功

---

## 8. roundtrip情報保持テスト

### T-F5-025 roundtrip_preserve_humanoid

目的：

- VRM → SansaVRM → VRM で humanoid bone対応が保持されることを確認する。

期待結果：

- JSON完全一致は不要
- bone名が保持される
- bone → node の意味的対応が保持される

---

### T-F5-026 roundtrip_preserve_meta

目的：

- VRM → SansaVRM → VRM で meta 情報が保持されることを確認する。

期待結果：

- JSON完全一致は不要
- title / name / author / authors / license 系情報が保持される

---

### T-F5-027 roundtrip_preserve_material_texture

目的：

- VRM → SansaVRM → VRM で material / texture 情報が保持されることを確認する。

期待結果：

- Material Property が再出力可能である
- Texture Property が再出力可能である
- MToon固有情報が失われない

---

### T-F5-028 roundtrip_preserve_unknown_fields

目的：

- 未対応 extension / extras / unknown fields が roundtrip で失われないことを確認する。

期待結果：

- passthrough情報が再出力可能である
- 元の配置階層へ復元できる

---

## 9. validator連携テスト

### T-F5-029 import_validator_pass

目的：

- import結果が SansaVRM validator を通過することを確認する。

期待結果：

- validator PASS
- PropertyType / PropertyContext の分類が仕様と一致する

---

### T-F5-030 import_validator_reject_invalid_context

目的：

- import処理が不正な PropertyContext を生成しないことを確認する。

期待結果：

- PropertyContext拡張なし
- validatorルール変更なし
- 不正contextが生成された場合はテスト失敗

---

## 10. テスト番号一覧

| テスト番号 | テスト名 |
|---|---|
| T-F5-001 | vrm0_import_minimal |
| T-F5-002 | vrm0_import_humanoid |
| T-F5-003 | vrm0_import_nodes_connections |
| T-F5-004 | vrm0_import_mesh_geometry |
| T-F5-005 | vrm0_import_blendshape_expression |
| T-F5-006 | vrm0_import_material_texture |
| T-F5-007 | vrm0_import_meta |
| T-F5-008 | vrm1_import_minimal |
| T-F5-009 | vrm1_import_humanoid |
| T-F5-010 | vrm1_import_nodes_connections |
| T-F5-011 | vrm1_import_mesh_geometry |
| T-F5-012 | vrm1_import_expression |
| T-F5-013 | vrm1_import_material_texture |
| T-F5-014 | vrm1_import_meta |
| T-F5-015 | vrm0_missing_extension |
| T-F5-016 | vrm1_missing_vrmc_vrm_extension |
| T-F5-017 | missing_humanoid_node |
| T-F5-018 | duplicate_human_bone |
| T-F5-019 | cycle_node_hierarchy |
| T-F5-020 | expression_unresolved_reference |
| T-F5-021 | passthrough_unknown_extension |
| T-F5-022 | passthrough_extras |
| T-F5-023 | passthrough_springbone |
| T-F5-024 | passthrough_node_constraint |
| T-F5-025 | roundtrip_preserve_humanoid |
| T-F5-026 | roundtrip_preserve_meta |
| T-F5-027 | roundtrip_preserve_material_texture |
| T-F5-028 | roundtrip_preserve_unknown_fields |
| T-F5-029 | import_validator_pass |
| T-F5-030 | import_validator_reject_invalid_context |

---

[目次](../../目次.md) > テスト仕様 > VRM入出力 > importテスト設計
