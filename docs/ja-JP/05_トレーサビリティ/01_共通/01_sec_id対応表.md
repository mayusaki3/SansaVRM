<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260505-000702Z-SV12
lang: ja-JP
canonical_title: sec_id対応表
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > トレーサビリティ > 共通 > sec_id対応表

# sec_id対応表

## 1. 目的

本ドキュメントは、SansaVRM の仕様、テスト仕様、実装コード、テストコードの対応関係を `sec_id` 単位で管理するための対応表である。

本対応表は、以下の用途で使用する。

- 仕様から実装コードへの紐づけ確認
- テスト仕様からテストコードへの紐づけ確認
- `@hldocs.ref` の付与対象確認
- 未実装、未テスト、孤立参照の検査

---

## 2. 状態定義

| 状態 | 意味 |
|---|---|
| 未実装 | 実装コードまたはテストコードへの正式紐づけが未完了 |
| 実装済み | 実装コードに `@hldocs.ref` が存在する |
| テスト済み | テストコードに `@hldocs.ref` が存在する |
| 完了 | 実装コードとテストコードの双方に `@hldocs.ref` が存在する |
| 対応不要 | 現段階ではコードまたはテストとの直接対応を要求しない |

---

## 3. 対応表

| 分類 | spec_doc_id | testspec_doc_id | sec_id | 仕様ファイル | 仕様章 | テスト仕様ファイル | テストケースID | 実装予定ファイル | テスト予定ファイル | 状態 |
|---|---|---|---|---|---|---|---|---|---|---|
| JSON Schema | doc-20260504-000204Z-SV0E | doc-20260504-000402Z-SV0P | sec_a8k3m2q1 | 04_JSONスキーマ仕様.md | 6.1 構造検証 | 02_JSONスキーマテスト仕様.md | SCHEMA-STRUCT-001 | schemas/root.schema.json | crates/sansavrm-core/tests/schema_validation.rs | 未実装 |
| JSON Schema | doc-20260504-000204Z-SV0E | doc-20260504-000402Z-SV0P | sec_b7n4p9r2 | 04_JSONスキーマ仕様.md | 6.3 型検証 | 02_JSONスキーマテスト仕様.md | SCHEMA-TYPE-002 | schemas/defs.schema.json | crates/sansavrm-core/tests/schema_validation.rs | 未実装 |
| JSON Schema | doc-20260504-000204Z-SV0E | doc-20260504-000402Z-SV0P | sec_c6t5v8s3 | 04_JSONスキーマ仕様.md | 6.2 制約検証 | 02_JSONスキーマテスト仕様.md | SCHEMA-CONSTRAINT-003 | schemas/defs.schema.json | crates/sansavrm-core/tests/schema_validation.rs | 未実装 |
| JSON Schema | doc-20260504-000204Z-SV0E | doc-20260504-000402Z-SV0P | sec_d5w6x7u4 | 04_JSONスキーマ仕様.md | 6.4 参照構造検証 | 02_JSONスキーマテスト仕様.md | SCHEMA-REF-004 | schemas/root.schema.json | crates/sansavrm-core/tests/schema_validation.rs | 未実装 |
| JSON Schema | doc-20260504-000204Z-SV0E | doc-20260504-000402Z-SV0P | sec_e4y7z6v5 | 04_JSONスキーマ仕様.md | 7.6.1 Property 型検証 | 02_JSONスキーマテスト仕様.md | SCHEMA-PROPERTY-005 | schemas/defs.schema.json | crates/sansavrm-core/tests/schema_validation.rs | 未実装 |
| JSON Schema | doc-20260504-000204Z-SV0E | doc-20260504-000402Z-SV0P | sec_f3a8b5w6 | 04_JSONスキーマ仕様.md | 7.6.2 Property 制約検証 | 02_JSONスキーマテスト仕様.md | SCHEMA-PROPERTY-006 | schemas/defs.schema.json | crates/sansavrm-core/tests/schema_validation.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_f7a2d9m4 | 05_Validator実装仕様.md | 8.1 ID一意性検証 | 03_Validatorテスト仕様.md | 追加予定 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_unique_ids.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_g2c9d4x7 | 05_Validator実装仕様.md | 8.2 参照実在性検証 | 03_Validatorテスト仕様.md | VALIDATOR-REF-001 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_refs.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_h1e0f3y8 | 05_Validator実装仕様.md | 8.3 接続整合性検証 | 03_Validatorテスト仕様.md | VALIDATOR-CONN-002 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_connections.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_j9g1h2z9 | 05_Validator実装仕様.md | 8.4 StateAction 整合性検証 | 03_Validatorテスト仕様.md | VALIDATOR-STATE-003 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_state_actions.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_k8m4q2r7 | 05_Validator実装仕様.md | 8.5 owner_id / *_ref 整合性検証 | 03_Validatorテスト仕様.md | 追加予定 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_owner_refs.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_l6n3p8s2 | 05_Validator実装仕様.md | 8.6 Compatibility 整合性検証 | 03_Validatorテスト仕様.md | 追加予定 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_compatibility.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_m5q2r7t1 | 05_Validator実装仕様.md | 8.7 Rights / Revenue 整合性検証 | 03_Validatorテスト仕様.md | 追加予定 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_rights_revenue.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_n4s1u6v0 | 05_Validator実装仕様.md | 8.8 Property 分類整合性検証 | 03_Validatorテスト仕様.md | 追加予定 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_properties.rs | 未実装 |
| Validator | doc-20260504-000205Z-SV0F | doc-20260504-000403Z-SV0Q | sec_p3t0w5x9 | 05_Validator実装仕様.md | 8.9 glTF 補助整合性検証 | 03_Validatorテスト仕様.md | 追加予定 | crates/sansavrm-validator/src/validate.rs | crates/sansavrm-validator/tests/validator_gltf_indices.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_k8j2m1a0 | 06_CoreAPI仕様.md | 6.1 create_model | 04_CoreAPIテスト仕様.md | CORE-MODEL-001 | crates/sansavrm-core/src/model_api.rs | crates/sansavrm-core/tests/model_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_l7k3n0b1 | 06_CoreAPI仕様.md | 7.1 add_module | 04_CoreAPIテスト仕様.md | CORE-MODULE-002 | crates/sansavrm-core/src/module_api.rs | crates/sansavrm-core/tests/module_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_m6l4p9c2 | 06_CoreAPI仕様.md | 9.1 connect | 04_CoreAPIテスト仕様.md | CORE-CONN-003 | crates/sansavrm-core/src/connection_api.rs | crates/sansavrm-core/tests/connection_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_n5m5q8d3 | 06_CoreAPI仕様.md | 10.4 apply_state | 04_CoreAPIテスト仕様.md | CORE-STATE-004 | crates/sansavrm-core/src/state_api.rs | crates/sansavrm-core/tests/state_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_p4n6r7e4 | 06_CoreAPI仕様.md | 15.3 rollback | 04_CoreAPIテスト仕様.md | CORE-TX-005 | crates/sansavrm-core/src/transaction_api.rs | crates/sansavrm-core/tests/transaction_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_q3p7s6f5 | 06_CoreAPI仕様.md | 11.1 add_property | 04_CoreAPIテスト仕様.md | CORE-PROPERTY-006 | crates/sansavrm-core/src/property_api.rs | crates/sansavrm-core/tests/property_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_r2q8t5g6 | 06_CoreAPI仕様.md | 14.4 export_vrm | 04_CoreAPIテスト仕様.md | CORE-IO-007 | crates/sansavrm-core/src/io_api.rs | crates/sansavrm-core/tests/io_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_s1r9u4h7 | 06_CoreAPI仕様.md | 9.5 disable_connection | 04_CoreAPIテスト仕様.md | CORE-CONN-008 | crates/sansavrm-core/src/connection_api.rs | crates/sansavrm-core/tests/connection_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_t0s8v3j8 | 06_CoreAPI仕様.md | 12.1 evaluate 条件一致 | 04_CoreAPIテスト仕様.md | CORE-EVAL-009 | crates/sansavrm-core/src/state_api.rs | crates/sansavrm-core/tests/evaluate_api.rs | 未実装 |
| CoreAPI | doc-20260504-000206Z-SV0G | doc-20260504-000404Z-SV0R | sec_u9t7w2k9 | 06_CoreAPI仕様.md | 12.2 evaluate 条件不一致 | 04_CoreAPIテスト仕様.md | CORE-EVAL-010 | crates/sansavrm-core/src/state_api.rs | crates/sansavrm-core/tests/evaluate_api.rs | 未実装 |
| Convert | doc-20260504-000207Z-SV0H | doc-20260504-000405Z-SV0S | sec_v8u6x1l0 | 07_変換仕様.md | 6.1 glTF基本方針 | 05_変換テスト仕様.md | CONVERT-GLTF-001 | crates/sansavrm-gltf/src/import.rs | crates/sansavrm-gltf/tests/import_gltf.rs | 未実装 |
| Convert | doc-20260504-000207Z-SV0H | doc-20260504-000405Z-SV0S | sec_w7v5y0m1 | 07_変換仕様.md | 7.1 VRM基本方針 | 05_変換テスト仕様.md | CONVERT-VRM-002 | crates/sansavrm-vrm/src/import.rs | crates/sansavrm-vrm/tests/import_vrm.rs | 未実装 |
| Convert | doc-20260504-000207Z-SV0H | doc-20260504-000405Z-SV0S | sec_x6w4z9n2 | 07_変換仕様.md | 8.1 URDF基本方針 | 05_変換テスト仕様.md | CONVERT-URDF-003 | crates/sansavrm-urdf/src/import.rs | crates/sansavrm-urdf/tests/import_urdf.rs | 未実装 |
| Convert | doc-20260504-000207Z-SV0H | doc-20260504-000405Z-SV0S | sec_y5x3a8p3 | 07_変換仕様.md | 9.1 SansaVRM to glTF基本方針 | 05_変換テスト仕様.md | CONVERT-GLTF-004 | crates/sansavrm-gltf/src/export.rs | crates/sansavrm-gltf/tests/export_gltf.rs | 未実装 |
| Convert | doc-20260504-000207Z-SV0H | doc-20260504-000405Z-SV0S | sec_z4y2b7q4 | 07_変換仕様.md | 10.1 SansaVRM to VRM方針 | 05_変換テスト仕様.md | CONVERT-VRM0-005 | crates/sansavrm-vrm/src/export.rs | crates/sansavrm-vrm/tests/export_vrm.rs | 未実装 |
| Convert | doc-20260504-000207Z-SV0H | doc-20260504-000405Z-SV0S | sec_a3z1c6r5 | 07_変換仕様.md | 11.1 SansaVRM to URDF方針 | 05_変換テスト仕様.md | CONVERT-VRM1-006 | crates/sansavrm-vrm/src/export.rs | crates/sansavrm-vrm/tests/export_vrm.rs | 未実装 |
| RoundTrip | doc-20260504-000207Z-SV0H | doc-20260504-000406Z-SV0T | sec_b2a0d5s6 | 07_変換仕様.md | 12.1 glTF ラウンドトリップ | 06_ラウンドトリップテスト仕様.md | ROUNDTRIP-GLTF-001 | crates/sansavrm-gltf/src/export.rs | crates/sansavrm-gltf/tests/roundtrip_gltf.rs | 未実装 |
| RoundTrip | doc-20260504-000207Z-SV0H | doc-20260504-000406Z-SV0T | sec_c1b9e4t7 | 07_変換仕様.md | 12.2 VRM 0.x ラウンドトリップ | 06_ラウンドトリップテスト仕様.md | ROUNDTRIP-VRM0-002 | crates/sansavrm-vrm/src/export.rs | crates/sansavrm-vrm/tests/roundtrip_vrm.rs | 未実装 |
| RoundTrip | doc-20260504-000207Z-SV0H | doc-20260504-000406Z-SV0T | sec_d0c8f3u8 | 07_変換仕様.md | 12.3 VRM 1.0 ラウンドトリップ | 06_ラウンドトリップテスト仕様.md | ROUNDTRIP-VRM1-003 | crates/sansavrm-vrm/src/export.rs | crates/sansavrm-vrm/tests/roundtrip_vrm.rs | 未実装 |
| RoundTrip | doc-20260504-000207Z-SV0H | doc-20260504-000406Z-SV0T | sec_e9d7g2v9 | 07_変換仕様.md | 12.4 異フォーマット経由ラウンドトリップ | 06_ラウンドトリップテスト仕様.md | ROUNDTRIP-CROSS-004 | crates/sansavrm-gltf/src/export.rs | tests/roundtrip/cross_format.rs | 未実装 |

---

## 4. 運用メモ

- JSON Schema は JSON仕様上コメントを持てないため、schema本体への `@hldocs.ref` 直接埋め込みは行わない。
- JSON Schema の仕様対応は本対応表およびテストコード側 `@hldocs.ref` で管理する。
- Rustコードでは対応処理単位の直前に `@hldocs.ref` を付与する。
- 既存の `TODO(trace)` は正式 `@hldocs.ref` への置換対象とする。

---

[目次](../../目次.md) > トレーサビリティ > 共通 > sec_id対応表
