[目次](../目次.md) > 仕様 > MuJoCo連携仕様

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

---

## 3. 対象

本仕様では以下を対象とする。

- MJCF（MuJoCo XML）
- body / joint / geom / actuator / sensor

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

### Connectionタイプ制約

MuJoCo変換では以下の Connection のみを対象とする：

- joint

それ以外の Connection は：

- 無視
または
- diagnostics に記録

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

## 11. 非スコープ

- シミュレーション実行
- リアルタイム制御
- UI / 可視化
- ネットワーク同期

---

## 12. 将来拡張

- soft body対応
- constraint拡張
- RL（強化学習）連携
- 分散シミュレーション

---

[目次](../目次.md) > 仕様 > MuJoCo連携仕様
