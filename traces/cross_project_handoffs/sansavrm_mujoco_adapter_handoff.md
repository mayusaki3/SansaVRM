# SansaVRM-MuJoCo-Adapter からの追加申し送り

## 1. 目的

本ドキュメントは、SansaVRM-MuJoCo-Adapter 側から SansaVRM 側へ申し送られた追加事項を記録する。

SansaVRM 側の仕様再構成、Adapter入力JSON仕様、Extension Property、Rust CLI / export API の検討に反映する。

---

## 2. HLDocS仕様参照先の更新

SansaVRM-MuJoCo-Adapter 側では、以後 HLDocS 仕様参照先を以下に変更する。

```text
https://github.com/mayusaki3/HLDocS/tree/develop/docs/ja-JP/仕様/00_共通
```

SansaVRM 側でも、MuJoCo Adapter / Extension Property / Adapter入力JSON まわりのドキュメントを作成・修正する場合は、同じ HLDocS develop 側仕様を正として扱う。

---

## 3. HLDocS準拠で整理する対象

SansaVRM 側で今後定義する以下の仕様は、HLDocS準拠ドキュメントとして整理する。

```text
- Adapter入力用JSON仕様
- sansavrm_adapter_input.json schema
- extension_properties仕様
- extension_property_schemas仕様
- updated_extension_properties.json 取り込み仕様
- SansaVRM Rust CLI / export API 仕様
```

---

## 4. SansaVRM-MuJoCo-Adapter側の現在の待ち状態

SansaVRM-MuJoCo-Adapter側では、SansaVRM Rustモジュールとの本格連携実装を以下が確定するまで待つ。

```text
- Adapter入力用JSON仕様
- sansavrm_adapter_input.json schema
- extension_properties構造
- extension_property_schemas構造
- custom_parameters と Extension Property の関係
- updated_extension_properties.json の取り込み方針
- SansaVRM Rust CLI または export API の形式
```

---

## 5. Adapter側で既に仮定している標準出力候補

SansaVRM-MuJoCo-Adapter側では、以下の成果物を標準出力候補として整理済みである。

```text
model.xml
controller_config.json
runtime_requirements.json
updated_extension_properties.json
diagnostics.json
conversion_report.json
```

SansaVRM側では、このうち特に以下の取り扱いを決める必要がある。

```text
updated_extension_properties.json
diagnostics.json
conversion_report.json
```

---

## 6. Extension Propertyの扱い

SansaVRM-MuJoCo-Adapter側では、以下の方針で整理済みである。

```text
MuJoCo / Meridian / sysid / HIL-SIL 固有情報は、
SansaVRM Core 標準仕様へ直接入れず、
SansaVRM Extension Property として扱う。
```

SansaVRM側でも、この方針を維持する。

---

## 7. 仕様確定後にAdapter側で再開する作業

SansaVRM側で Adapter入力JSON仕様または暫定schema/fixtureが提示されたら、Adapter側では以下を再開する。

```text
1. sansavrm_adapter_input.schema.json の作成
2. sansavrm_adapter_input fixture の作成
3. Extension Property classifier 実装
4. runtime_requirements writer 実装
5. updated_extension_properties writer 実装
6. MJCF writer の段階実装
```

---

## 8. 仮schema作成可否の確認事項

SansaVRM側で仕様確定に時間がかかる場合、Adapter側で仮の `sansavrm_adapter_input.schema.json` を作成してよいか確認が必要である。

ただし、仮schemaを作る場合でも、後でSansaVRM側の正本仕様に合わせて破棄・修正する前提とする。

---

## 9. SansaVRM側への反映方針

本申し送りは、以下へ反映する。

```text
- Runtime Integration Layer
- Import Export Layer
- Preservation Compatibility Layer
- Validation Layer
- Roadmap Layer
- migration / canonicalization / cleanup roadmap
```

特に、Adapter入力JSON仕様は `Import Export Layer` と `Runtime Integration Layer` の境界仕様として扱う。

Extension Property は `Preservation Compatibility Layer` の方針を維持し、Core標準仕様へ直接混入しない。

Diagnostics / conversion_report は `Validation Layer` の出力として扱う。

updated_extension_properties.json は、Adapter出力からSansaVRM側へ戻す Extension Property 更新入力として扱う。

---

## 10. 直近の追加タスク

```text
1. Adapter入力JSON仕様の正本化方針をRoadmapへ追加
2. sansavrm_adapter_input.json schema の仕様化をRoadmapへ追加
3. updated_extension_properties.json 取り込み仕様をRoadmapへ追加
4. diagnostics.json / conversion_report.json のValidation Layer上の扱いを整理
5. SansaVRM Rust CLI / export API 仕様をImport Export Layerへ追加候補として登録
6. 仮schemaをAdapter側で作成してよいかの判断を保留タスクとして登録
```

---

## 11. 現時点の判断

```text
HLDocS参照先更新:
受領済み。SansaVRM側でも同じdevelop共通仕様を正として扱う。

Adapter入力JSON仕様:
SansaVRM側でHLDocS準拠の正本化が必要。

Extension Property方針:
Core標準仕様へ直接入れず、Extension Propertyとして扱う方針を維持。

Adapter側の本格連携:
SansaVRM側のAdapter入力JSON仕様、schema、CLI/export API確定まで待ち。

仮schema作成可否:
未判断。Roadmap上の保留タスクとする。
```

---

## 12. 結論

SansaVRM-MuJoCo-Adapter 側の申し送りにより、SansaVRM側では Adapter入力JSON仕様、Extension Property構造、updated_extension_properties.json 取り込み仕様、Rust CLI / export API 仕様を優先的に整理する必要がある。

これらは現在進行中の大規模再構成ロードマップに追加し、canonicalization / cleanup より前に仕様正本化候補として扱う。
