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

検証内容：

- 接続生成
- 制約適用（max / exclusive / type）
- current_connections 同期
- changes が正しく記録されること

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

### 3.8 Transaction

対象：

- begin
- commit
- rollback

検証内容：

- 状態保持
- rollback の復元
- commit の確定

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

---

## 6. テストケース例

---

### CORE-MODEL-001

```md id="core_case_001"
テストID：CORE-MODEL-001
分類：Model管理
内容：create_model 実行
期待結果：Model が生成される
```

---

### CORE-MODULE-002

```md id="core_case_002"
テストID：CORE-MODULE-002
分類：Module操作
内容：module追加
期待結果：moduleが追加される
```

---

### CORE-CONN-003

```md id="core_case_003"
テストID：CORE-CONN-003
分類：Connection操作
内容：接続生成
期待結果：connections更新、current_connections同期
```

---

### CORE-STATE-004

```md id="core_case_004"
テストID：CORE-STATE-004
分類：State操作
内容：state適用
期待結果：actionsが適用される
```

---

### CORE-TX-005

```md id="core_case_005"
テストID：CORE-TX-005
分類：Transaction
内容：rollback
期待結果：元状態に戻る
```

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
