[目次](../目次.md) > テスト仕様 > CoreAPIテスト仕様

# CoreAPIテスト仕様

## 1. 目的

本仕様は、[CoreAPI仕様](../02_仕様/06_CoreAPI仕様.md) に基づく Core API の検証テスト仕様を定義する。

本仕様の目的は以下とする。

- Core API が仕様通りに動作することを確認する
- モデル操作（生成・更新・削除・接続・状態適用）が正しく行われることを検証する
- Validator と連携した整合性維持が保証されることを確認する
- 副作用およびトランザクション処理が正しく機能することを検証する

---

## 2. テスト対象

以下の API を対象とする。

- Model 管理 API
- Module 操作 API
- Slot 操作 API
- Connection 操作 API
- State 操作 API
- Evaluate API
- Validator 統合 API
- Transaction API
- Property 操作 API
- I/O 変換 API

---

## 3. テスト分類

---

### 3.1 Model 管理

対象：

- create_model
- load_model
- export_model

検証内容：

- 正常生成
- ID生成
- 初期状態

---

### 3.2 Module 操作

対象：

- add_module
- update_module
- remove_module

検証内容：

- 追加成功
- 一意性制約
- 削除制約で、参照が存在する場合はエラーになること
- 削除制約で、cascading delete が定義されている場合は適用されること

---

### 3.3 Slot 操作

対象：

- add_slot
- update_slot
- remove_slot

検証内容：

- Slot 作成
- Module との関連
- 削除制約で、参照が存在する場合はエラーになること
- 削除制約で、cascading delete が定義されている場合は適用されること

---

### 3.4 Connection 操作

対象：

- connect
- disconnect
- list_connections
- enable_connection
- disable_connection

検証内容：

- 接続生成
- 制約適用（max / exclusive / type）
- current_connections 同期
- changes が正しく記録されること
- connection_id による切断
- Connection の有効化 / 無効化
- from_id / to_id / connection_type が正しく保存されること

---

### 3.5 State 操作

対象：

- add_state
- remove_state
- evaluate_state
- apply_state

検証内容：

- State 登録
- 条件評価
- Action 実行
- 副作用反映
- 削除制約で、参照が存在する場合はエラーになること
- 削除制約で、cascading delete が定義されている場合は適用されること

---

### 3.6 Evaluate API

対象：

- evaluate

検証内容：

- active_states が条件式に基づき正しく選択されること
- applied_actions が state 定義に一致すること
- connection_status が接続状態を正しく反映すること
- compatibility_results が制約評価と一致すること

---

### 3.7 Validator 統合

対象：

- validate

検証内容：

- API操作後に整合性が維持されること
- Validator結果の反映

---

### 3.8 Property 操作

対象：

- add_property
- update_property
- remove_property
- list_properties

検証内容：

- property_id が一意であること
- owner_id が存在すること
- value_type と value が整合すること
- property_type / role が用途と整合すること

---

### 3.9 Transaction

対象：

- begin
- commit
- rollback

検証内容：

- 状態保持
- rollback の復元
- commit の確定

---

### 3.10 I/O 変換 API

対象：

- import_gltf
- export_gltf
- import_vrm
- export_vrm
- import_urdf
- export_urdf
- import_mujoco
- export_mujoco

検証内容：

- 各 import が Model を生成すること
- 各 export が対象フォーマットを生成すること
- export_vrm は version 指定（0.x / 1.0）に従うこと
- export_vrm の既定 version は 1.0 であること
- 指定 version で表現できない情報は diagnostics warning になること

---

## 4. テスト方針

---

### 4.1 正常系

- API が仕様通りに成功すること
- 期待されたモデル状態が得られること

---

### 4.2 異常系

- 不正入力でエラーが返ること
- Validator による検出が行われること

---

### 4.3 副作用検証

以下を必ず確認する。

- connections の変化
- current_connections の同期
- property の変更
- visibility の変更
- connection enabled 状態の変更
- property_type / role の保持

---

### 4.4 トランザクション

以下を確認する。

- rollback で完全復元されること
- commit 後に変更が反映されること

---

### 4.5 再現性

- 同一操作で同一結果となること

---

## 5. テストケース設計

各テストケースは以下を持つ。

- テストID
- テスト分類
- API呼び出し
- 入力データ
- 前提状態
- 期待結果
- 副作用
- Validator結果
- diagnostics

---

## 6. テストケース

---

### CORE-MODEL-001

- テストID：CORE-MODEL-001
- 分類：Model管理
- 内容：create_model 実行
- 期待結果：Model が生成される

---

### CORE-MODULE-002

- テストID：CORE-MODULE-002
- 分類：Module操作
- 内容：module追加
- 期待結果：moduleが追加される

---

### CORE-CONN-003

- テストID：CORE-CONN-003
- 分類：Connection操作
- 内容：接続生成
- 期待結果：
  - connections 更新
  - current_connections 同期
  - from_id / to_id / connection_type が正しく設定される

---

### CORE-STATE-004

- テストID：CORE-STATE-004
- 分類：State操作
- 内容：state適用
- 期待結果：actionsが適用される

---

### CORE-TX-005

- テストID：CORE-TX-005
- 分類：Transaction
- 内容：rollback
- 期待結果：元状態に戻る

---

### CORE-PROPERTY-006

- テストID：CORE-PROPERTY-006
- 分類：Property操作
- 内容：property_type / role 付き Property を追加
- 期待結果：Property が owner_id に紐付いて追加される

---

### CORE-IO-007

- テストID：CORE-IO-007
- 分類：I/O変換
- 内容：export_vrm(model, "1.0", options)
- 期待結果：VRM 1.0 仕様に準拠した出力が生成される

---

### CORE-CONN-008

- テストID：CORE-CONN-008
- 分類：Connection操作
- 内容：disable_connection 実行
- 期待結果：対象 connection の enabled が false になる

---

### CORE-EVAL-009

- テストID：CORE-EVAL-009
- 分類：Evaluate API
- 内容：evaluate 実行（条件一致する state が存在するケース）
- 前提状態：
  - 有効な state が1つ以上存在
  - 条件式が true となる
- 期待結果：
  - active_states に対象 state が含まれる
  - applied_actions が state 定義と一致する
  - connection_status が現在の接続状態を反映する

---

### CORE-EVAL-010

- テストID：CORE-EVAL-010
- 分類：Evaluate API
- 内容：evaluate 実行（条件一致しないケース）
- 前提状態：
  - state は存在するが条件が false
- 期待結果：
  - active_states が空
  - applied_actions が空
  - 状態変更が発生しない

---

## 7. 成功条件

以下を満たすこと。

- 正常系テストがすべて pass
- 異常系テストがすべて fail
- モデル状態が期待と一致する
- 副作用が正しく反映される
- Validator結果が期待と一致する

---

## 8. 失敗条件

以下の場合は失敗とする。

- APIが誤動作する
- モデル状態が不正
- 副作用が不正
- Validator結果が不正

---

## 9. テストデータ管理

- JSONデータで管理
- ケースごとに分離

例：

```text
tests/core/
  valid/
  invalid/
```

---

## 10. 実行環境

- Core API 実装（Rust）
- Validator
- テストランナー
- CI

---

## 11. 自動化

- 全テスト自動実行
- CIに組み込む
- API変更時は必ず実行

---

## 12. 将来拡張

- 並列実行テスト
- パフォーマンステスト
- 大規模モデル検証

---

## 13. 結論

本テスト仕様により、Core API の正当性と整合性を保証する。

---

[目次](../目次.md) > テスト仕様 > CoreAPIテスト仕様
