<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260504-000209Z-SV0J
lang: ja-JP
canonical_title: MuJoCo連携仕様
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > 共通 > MuJoCo連携仕様

# MuJoCo連携仕様

## 1. 目的

本仕様は、SansaVRM と MuJoCo（MJCF）との相互変換および連携方法を定義する。

本仕様の目的は以下とする。

- SansaVRM から MuJoCo モデルへの変換定義
- MuJoCo モデルから SansaVRM への変換定義
- 物理・制御モデルの対応関係の明確化
- シミュレーション連携の基盤確立

---

## 2. 基本方針

- SansaVRM を中間表現として使用する
- 接続関係は `Connection` により表現する
- 物理・制御情報は `Property` により表現する
- MuJoCo 固有要素は可能な限り抽象化する
- 非対応情報は `source_raw` に保持する
- 非可逆変換は `diagnostics` に記録する
- MuJoCo は標準ファイル変換対象ではなく、標準連携対象として扱う
- SansaVRM 本体は MuJoCo 実行系に依存しない
- SansaVRM 本体は MuJoCo 連携に必要な物理モデル情報、制御モデル情報、アクチュエータ情報、センサ情報を保持できる
- SansaVRM-MuJoCo-Adapter は SansaVRM 本体が提供する入出力 API を経由して情報を取得する
- MuJoCo 固有または将来追加されるパラメータは、namespace 付きカスタムパラメータとして保持できる
- カスタムパラメータは完全自由な key-value ではなく、登録済みスキーマに基づいて検証可能でなければならない
- カスタムパラメータ登録スキーマは、MJCF へ直接入出力できるパラメータ、Adapter 側補助成果物へ分離するパラメータ、保持のみのパラメータを識別できなければならない
- MJCF への直接入出力可否は、実装側の推測ではなく、登録スキーマにより判定する

---

## 3. 対象

本仕様では以下を対象とする。

- MJCF（MuJoCo XML）
- body / joint / geom / actuator / sensor

SansaVRM 側で保持する MuJoCo 連携情報は以下を含む。

- 物理モデル情報
- ジョイント情報
- コライダー情報
- アクチュエータ情報
- センサ情報
- Adapter が参照する変換補助情報
- namespace 付きカスタムパラメータ
- カスタムパラメータ登録スキーマ
- MJCF 入出力可否を判定するためのスキーマ情報
- Adapter 側補助成果物への分離先を判定するためのスキーマ情報

---

## 4. モデル対応関係

### 4.1 基本構造

| MuJoCo   | SansaVRM           |
| -------- | ------------------ |
| body     | Module             |
| joint    | Connection (joint) |
| geom     | Property           |
| site     | Slot               |
| actuator | Property           |
| sensor   | Property           |

---

### 4.2 階層構造

- MuJoCo の body 階層は Connection により表現する
- parent → child の方向で Connection を生成する

---

## 5. MuJoCo → SansaVRM

### 5.1 基本方針

- body を Module に変換する
- joint を Connection に変換する
- 物理情報は Property に変換する

---

### 5.2 マッピング

| MJCF      | SansaVRM         |
| --------- | ---------------- |
| body.name | Module.module_id |
| joint     | Connection       |
| geom      | Property         |
| mass      | Property         |
| inertia   | Property         |
| actuator  | Property         |
| sensor    | Property         |

---

### 5.3 Connection生成

- joint を Connection として生成する

保持情報：

- `connection_id`
- `from_id`（親 body）
- `to_id`（子 body）
- `connection_type = joint`
- `enabled = true`

---

### 5.4 Slot生成

Slot は以下の場合に生成する：

- 接続制約が必要な場合
- インターフェース分類が必要な場合

例：

- joint接続ポイント
- センサ接続ポイント

Slot は以下に使用される：

- 接続制約の補助
- センサ・アクチュエータのインターフェース定義

---

### 5.5 Property生成

以下を Property に変換する：

- 物理情報（mass / inertia / friction）
- 制御情報（actuator）
- センサ情報（sensor）

---

### 5.6 未対応情報

以下は `source_raw` に保存する：

- カスタムタグ
- 未解釈属性

---

## 6. SansaVRM → MuJoCo

### 6.1 基本方針

- Module を body に変換する
- Connection を joint に変換する
- Property を geom / actuator / sensor に変換する

SansaVRM から MuJoCo への変換は、SansaVRM-MuJoCo-Adapter が担当する。

SansaVRM 本体は、Adapter が参照するための構造化された入出力 API を提供する。

SansaVRM-MuJoCo-Adapter は、SansaVRM 本体の内部データ構造へ直接依存してはならない。
Adapter は SansaVRM 本体が提供する API を経由して、Module / Connection / Property / Slot / physics / actuator / sensor / custom parameter を取得する。

MJCF に直接出力できる情報は、登録スキーマの `io_scope` および `mjcf_mapping` に基づいて判定する。

MJCF に直接出力できない情報は、登録スキーマの `io_scope` および `adapter_artifact` に基づいて、Adapter 側で controller_config 等の補助成果物へ分離してよい。

分離内容、出力先、非可逆性、fallback の有無は diagnostics または conversion report に記録する。

### Connectionタイプ制約

MuJoCo変換では以下の Connection のみを対象とする：

- joint

それ以外の Connection は：

- 無視
または
- diagnostics に記録

MuJoCo への Property 出力分類は以下とする。

- Physics / Geometry / Material / Texture → geom
- Actuator → actuator
- Sensor → sensor
- Actuator + Execution / Simulation → actuator
- Sensor + IO / Execution → sensor
- その他 → MJCF へ直接出力しない

※ property_type を主判定とし、context は補助判定とする。
※ Control は概念上の分類であり、PropertyContext の値としては使用しない。

---

### 6.2 body生成

- Module ごとに body を生成する
- 親子関係は Connection に基づく

ルート Module（親 Connection を持たない Module）を root body とする。

---

### 6.3 joint生成

- `connection_type = joint` の Connection を対象とする

変換：

- `from_id` → parent
- `to_id` → child

---

### 6.4 Property分類ルール

Property は以下の規則に基づき分類する：

- physics系（mass, inertia 等） → geom
- Geometry / Material / Texture は geom生成または描画変換に使用される構造情報とする
- actuator系（torque, motor 等） → actuator
- sensor系（position, velocity, force 等） → sensor

判定優先順位：

1. property_type（必須）
2. context（補助）
3. key（フォールバック）

context は分類を補強するが、
property_type と矛盾してはならない。

---

### 6.5 geom生成

Property から生成する：

- collision
- visual
- shape情報

shape情報は以下を含む：

- type（box / sphere / capsule / mesh 等）
- size
- position
- rotation

geom は Module 単位で生成する。

関連する Property（collision / visual 等）をまとめて1つの geom に統合する。

---

### 6.6 actuator生成

Property から生成する：

- motor
- position actuator
- velocity actuator

actuator_type は property_type を優先し、context を補助判定として決定する。

actuator 判定ルール：

- property_type = Actuator → actuator
- context = Control / Execution → actuator（補助）

---

### 6.7 sensor生成

Property から生成する：

- joint position
- joint velocity
- force
- contact

sensor_type は property_type を優先し、context を補助判定として決定する。

sensor 判定ルール：

- property_type = Sensor → sensor
- context = IO / Execution → sensor（補助）

property_type を優先し、
context は補助判定としてのみ使用する。

---

## 7. 非可逆変換

### 7.1 原則

完全な再現ができない場合：

- `source_raw` に保存
- `diagnostics` に記録

---

### 7.2 例

- MuJoCo固有パラメータ
- 制御構造の差異

---

## 8. Validator連携

- 変換後に必ず validate を実行する
- エラーがある場合は出力を禁止（strictモード）

---

## 9. 制約

- MuJoCo は body がツリー構造であるため、Connection はツリー構造に制限される
- ループ構造は extensions に退避する
- 非対応の接続タイプは無視または警告

---

## 10. オプション

- `strict`
- `preserve_raw`
- `lossy_allowed`

---

## 11. Adapter入出力API

### 11.1 基本方針

SansaVRM 本体は、SansaVRM-MuJoCo-Adapter が参照するための入出力 API を提供する。

Adapter は SansaVRM の内部表現へ直接依存してはならない。
Adapter は本章で定義する API または同等の安定した公開 API を経由して情報を取得する。

### 11.2 読み取りAPI

SansaVRM 本体は、少なくとも以下の情報を取得できる API を提供する。

- モデルメタデータ
- Module 一覧
- Connection 一覧
- Slot 一覧
- Property 一覧
- 物理モデル情報
- ジョイント情報
- コライダー情報
- アクチュエータ情報
- センサ情報
- Adapter 固有パラメータ
- カスタムパラメータ
- カスタムパラメータ登録スキーマ
- MJCF 入出力対象パラメータ
- Adapter 側補助成果物対象パラメータ
- 保持のみのパラメータ
- 未対応パラメータ

### 11.3 書き込みAPI

SansaVRM 本体は、Adapter または変換処理が以下の情報を記録できる API を提供する。

- Adapter 固有パラメータ
- カスタムパラメータ
- 変換レポート
- diagnostics
- 非可逆変換情報
- 生成成果物メタデータ

### 11.4 検証API

SansaVRM 本体は、以下の検証を行える API を提供する。

- 共通物理情報の検証
- Adapter 固有パラメータの検証
- カスタムパラメータのスキーマ検証
- 必須パラメータの存在検証
- 未対応パラメータの検出
- fallback 可能性の判定
- MJCF 入出力可否の検証
- Adapter 側補助成果物への分離可否の検証
- 対象 MuJoCo バージョンにおける対応可否の検証

### 11.5 Adapterの責務

SansaVRM-MuJoCo-Adapter は以下を担当する。

- SansaVRM API からの情報取得
- MJCF の生成
- controller_config の生成
- MuJoCo 固有パラメータの解釈
- MuJoCo 固有の近似変換
- 変換レポートの生成
- MuJoCo 実行環境に依存する検証

### 11.6 SansaVRM本体の責務

SansaVRM 本体は以下を担当する。

- MuJoCo 連携情報の保持
- 共通物理情報の保持
- Adapter が参照する API の提供
- カスタムパラメータ登録機構の提供
- 登録済みカスタムパラメータの検証
- MJCF 入出力可否の判定に必要なスキーマ情報の保持
- Adapter 側補助成果物への分離に必要なスキーマ情報の保持
- Adapter から返却された diagnostics の保持

---

## 12. カスタムパラメータ登録

### 12.1 基本方針

SansaVRM は、MuJoCo 固有または将来追加されるパラメータを保持するため、namespace 付きカスタムパラメータ登録機構を提供する。

カスタムパラメータは、完全自由な key-value として扱ってはならない。
カスタムパラメータは登録済みスキーマに基づき、対象、型、単位、既定値、制約、fallback 方針を検証可能でなければならない。

また、カスタムパラメータ登録スキーマは、そのパラメータが以下のいずれに該当するかを識別できなければならない。

- MJCF へ直接入出力できるパラメータ
- MJCF へ直接入出力せず、Adapter 側補助成果物へ分離するパラメータ
- MJCF と Adapter 側補助成果物の両方へ出力するパラメータ
- SansaVRM 内に保持するだけのパラメータ
- 現在は未対応のパラメータ
- 解釈せず source_raw として保持するパラメータ

### 12.2 namespace

カスタムパラメータは namespace を持つ。

例：

- `mujoco`
- `urdf`
- `vrm`
- `unity`
- `o3de`
- `vendor`
- `experimental`

MuJoCo 固有パラメータは `mujoco` namespace を使用する。

### 12.3 適用対象

カスタムパラメータは、少なくとも以下の対象に紐づけられる。

- model
- module
- connection
- slot
- property
- joint
- collider
- actuator
- sensor

### 12.4 登録スキーマ

カスタムパラメータ登録スキーマは、少なくとも以下を持つ。

- namespace
- name
- target_type
- value_type
- unit
- required
- default
- min
- max
- enum
- description
- adapter_support
- fallback
- io_scope
- mjcf_mapping
- adapter_artifact
- mujoco_version
- supported_since
- deprecated_since

### 12.5 io_scope

`io_scope` は、そのカスタムパラメータの入出力範囲を定義する。

`io_scope` は以下のいずれかとする。

- `mjcf`
- `adapter_artifact`
- `both`
- `preserve_only`
- `unsupported`
- `source_raw`

各値の意味は以下とする。

- `mjcf`
  - MJCF に直接入出力できる
- `adapter_artifact`
  - MJCF には直接入出力せず、Adapter 側補助成果物へ出力する
- `both`
  - MJCF と Adapter 側補助成果物の両方へ出力する
- `preserve_only`
  - SansaVRM 内に保持するが、MJCF または Adapter 側補助成果物へは出力しない
- `unsupported`
  - 登録はされているが、現在の Adapter または対象 MuJoCo バージョンでは未対応とする
- `source_raw`
  - 解釈せず、元情報として保持する

### 12.6 mjcf_mapping

`mjcf_mapping` は、MJCF へ直接入出力できるパラメータについて、対応する MJCF 要素、属性、入出力方向を定義する。

`io_scope = mjcf` または `io_scope = both` の場合、原則として `mjcf_mapping` を定義する。

`mjcf_mapping` は、少なくとも以下を持つ。

- element
- attribute
- path
- direction
- value_conversion
- required_mujoco_version

`direction` は以下のいずれかとする。

- `import`
- `export`
- `import_export`

例：

```json
{
  "element": "joint",
  "attribute": "armature",
  "path": "joint.@armature",
  "direction": "import_export",
  "value_conversion": null,
  "required_mujoco_version": {
    "min": "2.3.0",
    "max": null
  }
}
```

### 12.7 adapter_artifact

`adapter_artifact` は、MJCF に直接入出力しないパラメータについて、Adapter 側補助成果物の出力先を定義する。

`io_scope = adapter_artifact` または `io_scope = both` の場合、原則として `adapter_artifact` を定義する。

`adapter_artifact` は、少なくとも以下を持つ。

- artifact_type
- path
- direction
- value_conversion
- required_adapter_version

`artifact_type` の例：

- `controller_config`
- `runtime_config`
- `conversion_report`
- `diagnostics`
- `external_metadata`

例：

```json
{
  "artifact_type": "controller_config",
  "path": "actuators[].command_delay_ms",
  "direction": "export",
  "value_conversion": null,
  "required_adapter_version": {
    "min": "0.1.0",
    "max": null
  }
}
```

### 12.8 MuJoCoバージョン情報

カスタムパラメータ登録スキーマは、対象 MuJoCo バージョン範囲を定義できる。

MuJoCo バージョンにより MJCF の要素、属性、意味、推奨値、非推奨項目が変化する可能性があるため、以下を保持できるものとする。

- mujoco_version
- supported_since
- deprecated_since

対象 MuJoCo バージョンで未対応または非推奨の場合は、fallback 方針に従う。

### 12.9 値の保持

カスタムパラメータ値は、以下を持つ。

- namespace
- name
- target_type
- target_id
- value
- source
- diagnostics

### 12.10 fallback

Adapter がカスタムパラメータを解釈できない場合、または対象 MuJoCo バージョンで利用できない場合、登録スキーマに定義された fallback 方針に従う。

fallback 方針は以下のいずれかとする。

- `use_default`
- `preserve_only`
- `warn`
- `error`
- `ignore`

### 12.11 MuJoCo固有パラメータの扱い

MuJoCo 固有パラメータは、登録スキーマに定義された `io_scope`、`mjcf_mapping`、`adapter_artifact` に基づいて出力先を判定する。

`io_scope = mjcf` または `io_scope = both` のパラメータは、`mjcf_mapping` に従って MJCF へ入出力できる。

`io_scope = adapter_artifact` または `io_scope = both` のパラメータは、`adapter_artifact` に従って SansaVRM-MuJoCo-Adapter 側の補助成果物へ出力できる。

`io_scope = preserve_only` のパラメータは、SansaVRM 内に保持するが、MJCF または Adapter 側補助成果物へは出力しない。

`io_scope = unsupported` のパラメータは、検証時に diagnostics へ記録する。

`io_scope = source_raw` のパラメータは、解釈せず source_raw として保持する。

MJCF への直接入出力可否は、実装側の推測ではなく登録スキーマにより判定する。

### 12.12 登録スキーマ例

MJCF へ直接入出力できる例：

```json
{
  "namespace": "mujoco",
  "name": "armature",
  "target_type": "joint",
  "value_type": "number",
  "unit": "kg*m^2",
  "required": false,
  "default": 0.0,
  "min": 0.0,
  "max": null,
  "enum": null,
  "description": "MuJoCo joint-side reflected inertia.",
  "adapter_support": {
    "sansa-vrm-mujoco-adapter": "supported"
  },
  "io_scope": "mjcf",
  "mjcf_mapping": {
    "element": "joint",
    "attribute": "armature",
    "path": "joint.@armature",
    "direction": "import_export",
    "value_conversion": null,
    "required_mujoco_version": {
      "min": "2.3.0",
      "max": null
    }
  },
  "adapter_artifact": null,
  "mujoco_version": {
    "min": "2.3.0",
    "max": null
  },
  "supported_since": "2.3.0",
  "deprecated_since": null,
  "fallback": {
    "behavior": "use_default",
    "value": 0.0
  }
}
```

Adapter 側補助成果物へ分離する例：

```json
{
  "namespace": "mujoco",
  "name": "command_delay_ms",
  "target_type": "actuator",
  "value_type": "number",
  "unit": "ms",
  "required": false,
  "default": 0,
  "min": 0,
  "max": null,
  "enum": null,
  "description": "Command delay applied by the MuJoCo adapter runtime.",
  "adapter_support": {
    "sansa-vrm-mujoco-adapter": "supported"
  },
  "io_scope": "adapter_artifact",
  "mjcf_mapping": null,
  "adapter_artifact": {
    "artifact_type": "controller_config",
    "path": "actuators[].command_delay_ms",
    "direction": "export",
    "value_conversion": null,
    "required_adapter_version": {
      "min": "0.1.0",
      "max": null
    }
  },
  "mujoco_version": {
    "min": null,
    "max": null
  },
  "supported_since": null,
  "deprecated_since": null,
  "fallback": {
    "behavior": "warn",
    "value": 0
  }
}
```

MJCF と Adapter 側補助成果物の両方へ出力する例：

```json
{
  "namespace": "mujoco",
  "name": "torque_limit_nm",
  "target_type": "actuator",
  "value_type": "number",
  "unit": "N*m",
  "required": false,
  "default": null,
  "min": 0,
  "max": null,
  "enum": null,
  "description": "Torque limit for actuator output.",
  "adapter_support": {
    "sansa-vrm-mujoco-adapter": "supported"
  },
  "io_scope": "both",
  "mjcf_mapping": {
    "element": "actuator",
    "attribute": "forcerange",
    "path": "actuator.*.@forcerange",
    "direction": "export",
    "value_conversion": {
      "type": "symmetric_range",
      "source_unit": "N*m",
      "target_format": "-value value"
    },
    "required_mujoco_version": {
      "min": "2.3.0",
      "max": null
    }
  },
  "adapter_artifact": {
    "artifact_type": "controller_config",
    "path": "actuators[].torque_limit_nm",
    "direction": "export",
    "value_conversion": null,
    "required_adapter_version": {
      "min": "0.1.0",
      "max": null
    }
  },
  "mujoco_version": {
    "min": "2.3.0",
    "max": null
  },
  "supported_since": "2.3.0",
  "deprecated_since": null,
  "fallback": {
    "behavior": "error"
  }
}
```

---

## 13. 非スコープ

- シミュレーション実行
- リアルタイム制御
- UI / 可視化
- ネットワーク同期
- MuJoCo 実行ランタイムの仕様
- MuJoCo controller runtime の仕様
- MJCF 生成アルゴリズムの詳細
- controller_config 生成アルゴリズムの詳細
- MuJoCo シミュレーション検証手順
- SansaVRM-MuJoCo-Adapter 固有の実装仕様

---

## 14. 将来拡張

- soft body対応
- constraint拡張
- RL（強化学習）連携
- 分散シミュレーション
- Adapter capability 宣言
- Adapter 別 custom parameter schema の配布
- MuJoCo controller runtime との連携
- 複数 physics backend への対応
- MuJoCo バージョン別 custom parameter schema の管理
- MJCF mapping schema の更新管理

---

[目次](../../目次.md) > 仕様 > 共通 > MuJoCo連携仕様
