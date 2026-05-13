# SansaVRM-MuJoCo-Adapter 申し送りへの回答

## 1. 目的

本ドキュメントは、SansaVRM-MuJoCo-Adapter からの追加申し送りに対する SansaVRM 側の回答を記録する。

本回答は、SansaVRM 側で進行中の大規模仕様再構成において整理済みの Layer 責務、canonicalization / rewrite / integrity / validation 方針を前提とする。

---

## 2. 回答時点の前提

SansaVRM 側では、以下の整理を追加済みである。

```text
- canonicalization_manifest_schema
- canonicalization_report_format
- cleanup_gate_dependency_graph
- rewrite_transaction_model
- dashboard_state_schema
- federation_validator_implementation_model
- canonicalization_execution_plan
- integrity_tamper_model
```

このため、本回答では以下を前提とする。

```text
- filesystem ordering と semantic identity は分離する
- dry-doc は migration workspace identifier であり canonical doc_id ではない
- canonicalization と cleanup は別フェーズとする
- rewrite は transaction として扱う
- dashboard は observer として扱う
- validator は rewrite executor ではなく cross-layer observer とする
- identity / integrity proof / validation result は分離する
```

---

## 3. HLDocS 参照先

Adapter 側で更新された HLDocS 参照先は、SansaVRM 側でも受領済みとして扱う。

```text
https://github.com/mayusaki3/HLDocS/tree/develop/docs/ja-JP/仕様/00_共通
```

今後、Adapter 入力 JSON、Extension Property、updated_extension_properties、diagnostics、conversion_report、Rust CLI / export API を HLDocS 準拠で正本化する場合は、この develop 側共通仕様を参照する。

---

## 4. Adapter 入力 JSON 仕様の配置方針

Adapter 入力 JSON 仕様は、以下の境界仕様として扱う。

```text
Import Export Layer
↔
Runtime Integration Layer
```

理由：

```text
- Adapter 入力 JSON は Core semantic そのものではない
- Runtime / Adapter が解釈可能な export representation である
- MuJoCo Adapter 以外の Adapter へも展開される可能性がある
- 変換対象・実行対象・検証対象の境界を定義するため
```

したがって、Adapter 入力 JSON 仕様は Core Semantic Layer へ直接配置しない。

ただし、Adapter 入力 JSON が参照する canonical semantic identity は Core Semantic Layer に属する。

---

## 5. sansavrm_adapter_input.schema.json の扱い

`sanavrm_adapter_input.schema.json` または `sansavrm_adapter_input.schema.json` は、Adapter 入力 JSON 仕様に対する machine-readable schema として扱う。

配置責務：

```text
Import Export Layer:
JSON representation / schema / export profile

Runtime Integration Layer:
Adapter 実行境界 / runtime requirement / target adapter contract

Validation Layer:
schema validation result / diagnostics
```

schema 自体は Core semantic ではない。

Core semantic へ入るのは、schema が参照する canonical identity / semantic graph / semantic dependency のみである。

---

## 6. Extension Property 責務

MuJoCo / Meridian / sysid / HIL-SIL 固有情報は、Core 標準仕様へ直接入れない。

SansaVRM 側でも、Adapter 側の方針を維持する。

```text
Extension Property:
Preservation Compatibility Layer を主責務とする
```

ただし、Extension Property が runtime adapter contract として使われる場合は、Runtime Integration Layer から参照する。

整理：

```text
Core Semantic Layer:
標準化された semantic identity / semantic graph のみ

Preservation Compatibility Layer:
source_raw / preserve_only / adapter raw / extension metadata

Runtime Integration Layer:
Adapter が実行時に必要とする extension property の読み取り契約

Validation Layer:
Extension Property の分類結果・不整合・diagnostics
```

---

## 7. updated_extension_properties.json の責務

`updated_extension_properties.json` は、Adapter 出力から SansaVRM 側へ戻る更新候補として扱う。

ただし、Core semantic へ直接反映しない。

分類：

```text
Runtime / Import Export boundary concern
```

扱い：

```text
1. Adapter output として受領する
2. Validation Layer で diagnostics / classification を行う
3. Core semantic candidate が含まれる場合は review 対象にする
4. 必要に応じて rewrite transaction を通す
5. canonicalization / validation を通過したもののみ正本へ反映する
```

禁止：

```text
updated_extension_properties.json を review / rewrite / validation なしに Core へ反映すること
```

---

## 8. diagnostics.json / conversion_report.json の責務

`diagnostics.json` と `conversion_report.json` は Validation Layer の成果物として扱う。

```text
diagnostics.json:
Validation Layer concern

conversion_report.json:
Validation Layer concern
```

これらは Core semantic identity ではない。

ただし、canonicalization / rewrite / cleanup gate の判断材料として使用してよい。

特に、以下の情報は dashboard / validator に取り込む候補とする。

```text
- conversion status
- unsupported item
- preserve_only item
- source_raw item
- adapter warning
- adapter failure
- runtime requirement mismatch
```

---

## 9. model.xml / controller_config.json / runtime_requirements.json の責務

Adapter 側の標準出力候補は、以下のように扱う。

```text
model.xml:
Runtime / Import Export representation

controller_config.json:
Runtime Integration artifact

runtime_requirements.json:
Runtime Integration / Validation boundary artifact

updated_extension_properties.json:
Runtime / Import Export boundary update candidate

diagnostics.json:
Validation Layer artifact

conversion_report.json:
Validation Layer artifact
```

これらはいずれも Core semantic へ直接戻さない。

Core へ戻す可能性があるのは、review / rewrite / validation 後に semantic candidate として採用された情報のみである。

---

## 10. 仮 schema 作成可否

Adapter 側で仮の `sansavrm_adapter_input.schema.json` を作成することは許可する。

ただし条件を付ける。

```text
- draft / provisional / experimental のいずれかを明示する
- SansaVRM 正本仕様ではないことを明示する
- 後で SansaVRM 側の HLDocS 準拠正本に合わせて破棄または修正する
- Core semantic を確定したものとして扱わない
- Extension Property / updated_extension_properties の分類は仮分類として扱う
```

推奨名：

```text
sansavrm_adapter_input.schema.draft.json
```

または：

```text
sansavrm_adapter_input.schema.experimental.json
```

---

## 11. Validation Layer との境界

Adapter 関連の validation は、以下に分離する。

```text
schema validation:
Adapter 入力 JSON が schema に適合するか

conversion validation:
Adapter 変換が成立したか

runtime requirement validation:
runtime_requirements が対象 runtime と整合するか

extension property validation:
Extension Property が分類可能か、Core へ混入していないか

integrity validation:
representation hash / normalized semantic hash / provenance / signature の検証
```

Adapter は diagnostics / conversion_report を出力してよい。

SansaVRM 側の federation validator は、それらを cross-layer observer として集約する。

---

## 12. canonicalization / rewrite との関係

Adapter 入力 JSON 仕様、Extension Property、updated_extension_properties は、canonicalization / rewrite と以下の関係を持つ。

```text
Adapter 入力 JSON:
canonical semantic identity を参照する export representation

Extension Property:
Core に入れない preservation / compatibility 情報

updated_extension_properties:
review / rewrite / validation を通す更新候補

diagnostics / conversion_report:
rewrite / cleanup gate の判断材料
```

canonicalization 中に Adapter 関連の representation が変化しても、normalized semantic hash が一致する場合は semantic-preserving rewrite として扱える。

representation hash mismatch のみで semantic_equivalent=false と判定してはならない。

---

## 13. SansaVRM Rust CLI / export API の扱い

SansaVRM Rust CLI / export API は、Import Export Layer を主責務とする。

Runtime Adapter 実行に関係する引数・出力契約は Runtime Integration Layer と接続する。

Validation 出力は Validation Layer に接続する。

整理：

```text
CLI / export API input:
Import Export Layer

Adapter runtime options:
Runtime Integration Layer

Validation / diagnostics output:
Validation Layer

Preservation metadata:
Preservation Compatibility Layer
```

---

## 14. Adapter 側で再開可能な作業

以下は、仮 schema 前提で Adapter 側が再開可能とする。

```text
1. sansavrm_adapter_input.schema.draft.json の作成
2. sansavrm_adapter_input fixture の作成
3. Extension Property classifier の draft 実装
4. runtime_requirements writer の draft 実装
5. updated_extension_properties writer の draft 実装
6. MJCF writer の段階実装
```

ただし、以下は SansaVRM 正本仕様確定後に再検証する。

```text
- schema の正式名
- required / optional field
- Core semantic candidate の扱い
- Extension Property 分類
- updated_extension_properties 取り込み方針
- diagnostics / conversion_report の正式 schema
```

---

## 15. SansaVRM 側の次タスク

SansaVRM 側では、以下を後続タスクとして扱う。

```text
1. Adapter 入力 JSON 仕様の HLDocS 準拠正本化
2. sansavrm_adapter_input schema の正本化
3. Extension Property / extension_property_schemas 仕様の整理
4. updated_extension_properties 取り込み仕様の整理
5. diagnostics / conversion_report の Validation Layer schema 整理
6. Rust CLI / export API 仕様の Import Export Layer への配置
7. Adapter 側 draft schema / fixture との再突合
```

---

## 16. 回答まとめ

```text
HLDocS参照先:
受領。develop/docs/ja-JP/仕様/00_共通 を正として扱う。

Adapter入力JSON仕様:
Import Export Layer ↔ Runtime Integration Layer の境界仕様として扱う。

Extension Property:
Core へ直接入れず、Preservation Compatibility Layer を主責務とする。

updated_extension_properties.json:
Runtime / Import Export boundary の更新候補。Core へ直接反映しない。

diagnostics.json / conversion_report.json:
Validation Layer artifact として扱う。

仮schema:
Adapter側で draft / experimental として作成可。ただし正本ではない。

Validation境界:
Adapter diagnostics は SansaVRM federation validator が集約する。

canonicalization/rewrite影響:
Adapter由来情報は review / rewrite / validation を通す。
```

---

## 17. 結論

SansaVRM 側では、MuJoCo Adapter からの申し送りを受領し、Adapter 入力 JSON、Extension Property、updated_extension_properties、diagnostics、conversion_report の責務境界を Layer 単位で整理した。

Adapter 側は仮 schema / fixture / classifier / writer の draft 実装を進めてよい。

ただし、SansaVRM 正本仕様確定後に、schema、Extension Property 分類、updated_extension_properties 取り込み、diagnostics / conversion_report を再検証する。
