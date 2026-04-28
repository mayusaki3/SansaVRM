[目次](../目次.md) > 仕様 > CoreAPI仕様

# CoreAPI仕様

## 1. 目的

本仕様は、SansaVRM Core における API の設計および動作契約を定義する。

Core API は以下の責務を持つ。

- [メタモデル仕様](./02_メタモデル仕様.md) に基づくデータ操作
- [glTF拡張仕様](./03_glTF拡張仕様.md) との入出力連携
- [JSONスキーマ仕様](./04_JSONスキーマ仕様.md) 準拠データの生成・更新
- [Validator実装仕様](./05_Validator実装仕様.md) との統合検証
- Runtime に対する状態操作インターフェースの提供

本仕様は、実装言語に依存しない抽象 API を定義し、
Rust を含む各言語実装の基準とする。

---

## 2. 基本方針

- Core は「状態を持つモデル操作エンジン」とする
- API は副作用を明確に定義する
- Validator は API 内部または外部で必ず呼び出される
- すべての API はエラーを構造化して返す
- データは immutable を基本とし、変更は新しい状態を返す方式を推奨する
- ID は外部から指定可能とし、自動生成も許容する
- glTF / VRM / URDF 依存処理は Adapter 層に分離する

---

## 3. 用語

- Model：SansaVRM の全体構造
- Module：構成要素
- Slot：接続点
- State：状態定義
- Connection：Model に属する接続実体。from_id / to_id により Module または Slot を接続する。
- Context：実行時補助情報

---

## 4. APIカテゴリ

Core API は以下のカテゴリで構成する。

- Model管理
- Module操作
- Slot操作
- Connection操作
- State操作
- 評価（Evaluate）
- Validator統合
- I/O変換

---

## 5. データ型（抽象）

### 5.1 Transaction型

```text
Transaction {
  original_model: Model
  working_model: Model
  changes: Change[]
}
```

### 5.2 Result型

```text
Result<T> {
  success: boolean
  data: T | null
  errors: Error[]
  warnings: Error[]
  infos: Error[]
}
```

### 5.3 EvaluationResult型

```text
EvaluationResult {
  active_states: State[]
  applied_actions: Action[]
  connection_status: ConnectionStatus[]
  compatibility_results: CompatibilityResult[]
}
```

### 5.4 Error型

```text
Error {
  code: string
  message: string
  path: string
  severity: "error" | "warning" | "info"
}
```

---

## 6. Model管理API

### 6.1 create_model

```text
create_model(input) -> Result<Model>
```

#### 入力

- model_id（任意）
- 初期構造（任意）

#### 処理

- Modelを生成
- ID未指定時は生成
- 空の各コレクションを初期化

#### 出力

- Model

---

### 6.2 load_model

```text
load_model(document) -> Result<Model>
```

#### 処理

- JSONを読み込み
- JSON Schema 検証
- Validator 実行

---

### 6.3 export_model

```text
export_model(model) -> Result<JSON>
```

#### 処理

- 内部モデルをJSONへ変換
- Schema準拠を保証

---

## 7. Module操作API

### 7.1 add_module

```text
add_module(model, module_def) -> Result<Model>
```

#### 検証

- module_id 一意
- type 有効

---

### 7.2 remove_module

```text
remove_module(model, module_id) -> Result<Model>
```

#### 検証

- 参照されていないこと
- 参照がある場合は error または cascading delete のどちらかを明記

---

### 7.3 update_module

```text
update_module(model, module_id, patch) -> Result<Model>
```

---

## 8. Slot操作API

### 8.1 add_slot

```text
add_slot(model, slot_def) -> Result<Model>
```

### 8.2 remove_slot

```text
remove_slot(model, slot_id) -> Result<Model>
```

#### 検証

- 参照されていないこと
- 参照がある場合は error または cascading delete のどちらかを明記

### 8.3 update_slot

```text
update_slot(model, slot_id, patch) -> Result<Model>
```

---

## 9. Connection操作API

### 9.1 connect

```text
connect(model, from_id, to_id, connection_type, options) -> Result<Model>
```

#### 処理

- 接続可能性検証
- connection_rules 適用
- connections 更新
- current_connections 同期

#### 出力

- 更新後 Model
- 適用された connection 情報（任意）

---

### 9.2 disconnect

```text
disconnect(model, connection_id) -> Result<Model>
```

---

### 9.3 list_connections

```text
list_connections(model) -> Connection[]
```

---

### 9.4 enable_connection

```text
enable_connection(model, connection_id) -> Result<Model>
```

---

### 9.5 disable_connection

```text
disable_connection(model, connection_id) -> Result<Model>
```

---

## 10. State操作API

### 10.1 add_state

```text
add_state(model, state_def) -> Result<Model>
```

---

### 10.2 remove_state

```text
remove_state(model, state_id) -> Result<Model>
```

#### 検証

- 参照されていないこと
- 参照がある場合は error または cascading delete のどちらかを明記

---

### 10.3 evaluate_state

```text
evaluate_state(model, context) -> Result<State[]>
```

#### 処理

- conditions 評価
- 有効な State を抽出

---

### 10.4 apply_state

```text
apply_state(model, state_id) -> Result<Model>
```

#### 処理

- actions 実行
- Model 更新
- actions に応じて connections / properties / visibility が変更される
- Connection の有効 / 無効切替
- Control / Actuator / Sensor 状態への反映

---

## 11. Property操作API

### 11.1 add_property

```text
add_property(model, owner_id, property_def) -> Result<Model>
```

### 11.2 update_property

```text
update_property(model, property_id, patch) -> Result<Model>
```

### 11.3 remove_property

```text
remove_property(model, property_id) -> Result<Model>
```

### 11.4 list_properties

```text
list_properties(model, owner_id) -> Property[]
```

---

## 12. 評価API

### 12.1 evaluate

```text
evaluate(model, context) -> Result<EvaluationResult>
```

#### 処理

- State評価
- Compatibility評価
- 接続検証
- 結果生成

---

## 13. Validator統合

### 13.1 validate

```text
validate(model, options) -> Result<ValidationResult>
```

#### 処理

- JSON Schema（必要に応じて）
- Validator

---

## 14. I/O API

### 14.1 import_gltf

```text
import_gltf(document) -> Result<Model>
```

### 14.2 export_gltf

```text
export_gltf(model) -> Result<GLTF>
```

---

### 14.3 import_vrm

```text
import_vrm(document) -> Result<Model>
```

### 14.4 export_vrm

- export_vrm は VRM仕様に準拠した glTF を生成する

```text
export_vrm(model, version, options) -> Result<VRM>
```

#### 入力

- model
- version
  - "0.x"
  - "1.0"
- options

#### 処理

- version に応じた VRM 仕様へ変換する
- "1.0" を既定値とする
- "0.x" は互換出力として扱う
- 指定 version で表現できない情報は diagnostics に warning として記録する

---

### 14.5 import_urdf

```text
import_urdf(document) -> Result<Model>
```

---

### 14.6 export_urdf

- export_urdf は URDF XML を生成する

```text
export_urdf(model) -> Result<URDF>
```

---

### 14.7 import_mujoco

```text
import_mujoco(document) -> Result<Model>
```

### 14.8 export_mujoco

```text
export_mujoco(model) -> Result<MJCF>
```

---

## 15. トランザクション

### 15.1 begin

```text
begin(model) -> Transaction
```

### 15.2 commit

```text
commit(transaction) -> Result<Model>
```

### 15.3 rollback

```text
rollback(transaction) -> Model
```

---

## 16. エラーハンドリング

- すべてのAPIは Result を返す
- error は処理停止
- warning は処理継続
- info は参考情報

---

## 17. スレッド安全性

- 読み取りは並列可能
- 書き込みは排他制御
- immutable モデルを推奨

---

## 18. 非スコープ

- Renderer
- UI
- ネットワーク同期
- ストレージ

---

## 19. 将来拡張

- 非同期API
- 分散モデル同期
- プラグイン機構
- スクリプト統合

---

[目次](../目次.md) > 仕様 > CoreAPI仕様
