[目次](../目次.md) > 仕様 > CoreAPI仕様

# CoreAPI仕様

## 1. 目的

本仕様は、SansaVRM Core における API の設計および動作契約を定義する。

Core API は以下の責務を持つ。

- メタモデル（02）に基づくデータ操作
- glTF拡張（03）との入出力連携
- JSONスキーマ（04）準拠データの生成・更新
- Validator（05）との統合検証
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
- Connection：Slot間接続
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
connect(model, from_slot_id, to_slot_id, options) -> Result<Model>
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
disconnect(model, from_slot_id, to_slot_id) -> Result<Model>
```

---

### 9.3 list_connections

```text
list_connections(model) -> Connection[]
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

---

## 11. 評価API

### 11.1 evaluate

```text
evaluate(model, context) -> Result<EvaluationResult>
```

#### 処理

- State評価
- Compatibility評価
- 接続検証
- 結果生成

---

## 12. Validator統合

### 12.1 validate

```text
validate(model, options) -> Result<ValidationResult>
```

#### 処理

- JSON Schema（必要に応じて）
- Validator（05）

---

## 13. I/O API

### 13.1 import_gltf

```text
import_gltf(document) -> Result<Model>
```

### 13.2 export_gltf

```text
export_gltf(model) -> Result<GLTF>
```

---

### 13.3 import_vrm

```text
import_vrm(document) -> Result<Model>
```

---

### 13.4 import_urdf

```text
import_urdf(document) -> Result<Model>
```

---

## 14. トランザクション

### 14.1 begin

```text
begin(model) -> Transaction
```

### 14.2 commit

```text
commit(transaction) -> Result<Model>
```

### 14.3 rollback

```text
rollback(transaction) -> Model
```

---

## 15. エラーハンドリング

- すべてのAPIは Result を返す
- error は処理停止
- warning は処理継続
- info は参考情報

---

## 16. スレッド安全性

- 読み取りは並列可能
- 書き込みは排他制御
- immutable モデルを推奨

---

## 17. 非スコープ

- Renderer
- UI
- ネットワーク同期
- ストレージ

---

## 18. 将来拡張

- 非同期API
- 分散モデル同期
- プラグイン機構
- スクリプト統合

---

[目次](../目次.md) > 仕様 > CoreAPI仕様
