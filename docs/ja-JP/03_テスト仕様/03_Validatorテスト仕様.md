[目次](../目次.md) > テスト仕様 > Validatorテスト仕様

# Validatorテスト仕様

## 1. 目的

本仕様は、[Validator実装仕様](../02_仕様/05_Validator実装仕様.md) に基づく Validator の検証テスト仕様を定義する。

本仕様の目的は以下とする。

- SansaVRM の意味整合性検証が正しく機能することを確認する
- 参照整合性、接続整合性、状態整合性が保証されることを検証する
- エラーコードおよび severity が仕様通りであることを確認する
- diagnostics が正しく生成されることを保証する

---

## 2. テスト対象

以下を対象とする。

- ID一意性検証
- 参照整合性検証
- 接続整合性検証（connections / current_connections）
- StateAction 整合性検証
- owner / *_ref 整合性検証
- Compatibility 検証
- Rights / Revenue 検証
- diagnostics 生成
- diagnostics の順序安定性

---

## 3. テスト分類

---

### 3.1 ID一意性検証

対象：

- model_id
- module_id
- slot_id
- state_id
- property_id

---

### 3.2 参照整合性検証

対象：

- module_id / slot_id / state_id
- owner_id
- *_ref

---

### 3.3 接続整合性検証

対象：

- connections（主）
- current_connections（補助整合確認）

検証内容：

- 正本と局所参照の一致
- max_connections 制約
- exclusive 制約
- target_slot_types 適合

---

### 3.4 StateAction 検証

対象：

- module_enable / disable
- slot_bind / unbind
- property_override
- visibility_change

---

### 3.5 Compatibility 検証

対象：

- required / forbidden
- constraint_rules

---

### 3.6 Rights / Revenue 検証

対象：

- source_raw
- normalized
- policy / weight / rate

---

### 3.7 diagnostics 検証

対象：

- type
- severity
- message

---

### 3.8 Property分類整合性検証

対象：

- property_type
- context

検証内容：

- property_type と key の整合
- property_type と context の整合

---

### 3.9 MuJoCo変換前提整合性

対象：

- property_type
- context
- connection_type

検証内容：

- joint以外のConnectionが混在しないこと（MuJoCo対象）

---

## 4. テスト方針

---

### 4.1 正常系

- 正しいデータが Validator を通過すること

---

### 4.2 異常系

- 不正なデータが必ず検出されること
- 複数エラーが同時に検出されること

---

### 4.3 網羅性

以下を必ずカバーする。

- すべてのエラーコード
- すべての severity
- 複合エラー（複数不整合）

---

### 4.4 再現性

- 同一入力で同一結果が返ること

---

## 5. テストケース設計

各テストケースは以下を持つ。

- テストID
- テスト分類
- 入力データ
- 期待結果（pass / fail）
- エラーコード
- severity
- diagnostics内容

---

## 6. テストケース

---

### VALIDATOR-REF-001

- テストID：VALIDATOR-REF-001
- 分類：参照整合性
- 内容：存在しない module_id を参照
- 期待結果：fail
- エラーコード：REF_NOT_FOUND
- severity：error

---

### VALIDATOR-CONN-002

- テストID：VALIDATOR-CONN-002
- 分類：接続整合性
- 内容：max_connections 超過
- 期待結果：fail
- エラーコード：MAX_CONNECTION_EXCEEDED
- severity：error

---

### VALIDATOR-STATE-003

- テストID：VALIDATOR-STATE-003
- 分類：StateAction
- 内容：無効な property_override
- 期待結果：fail
- エラーコード：PROPERTY_OVERRIDE_TYPE_MISMATCH
- severity：error

---

## 7. 成功条件

以下を満たすこと。

- 正常系テストがすべて pass
- 異常系テストがすべて fail
- エラーコードが仕様と一致する
- severity が仕様と一致する
- diagnostics の内容が期待と一致する

---

## 8. 失敗条件

以下の場合は失敗とする。

- 不正データが通過する
- 正常データが失敗する
- エラーコードが不正
- severity が不正
- diagnostics が不正

---

## 9. テストデータ管理

- JSON データとして管理
- ケースごとにファイル分割

例：

```text
tests/validator/
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

- 全テストを自動実行
- CI に組み込む
- Validator変更時は必ず実行

---

## 12. 将来拡張

- DSL条件式テスト
- 大規模モデル検証
- パフォーマンステスト

---

## 13. 結論

本テスト仕様により、Validator の意味整合性検証の正当性を保証する。

---

[目次](../目次.md) > テスト仕様 > Validatorテスト仕様
