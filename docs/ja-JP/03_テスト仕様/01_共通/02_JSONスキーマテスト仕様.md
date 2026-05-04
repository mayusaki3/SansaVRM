<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260504-000402Z-SV0P
lang: ja-JP
canonical_title: JSONスキーマテスト仕様
document_type: testspec
canonical_document: true
-->

[目次](../../目次.md) > テスト仕様 > 共通 > JSONスキーマテスト仕様

# JSONスキーマテスト仕様

## 1. 目的

本仕様は、[JSONスキーマ仕様](../../02_仕様/01_共通/04_JSONスキーマ仕様.md) に基づく JSON Schema の検証テスト仕様を定義する。

本仕様の目的は以下とする。

- SansaVRM Format の構造が正しく定義されていることを検証する
- JSON Schema による型・必須項目・制約が正しく機能することを確認する
- 不正データが適切に検出されることを保証する

---

## 2. テスト対象

以下の JSON Schema を対象とする。

- ルートスキーマ
- 各拡張スキーマ
- 共通定義（defs）

---

## 3. テスト分類

テストは以下の分類で実施する。

### 3.1 構造検証

対象：

- 必須プロパティ
- ネスト構造
- 配列構造

---

### 3.2 型検証

対象：

- string / number / boolean / object / array
- enum
- null許容
- property_type の enum 検証
- context の enum 検証

---

### 3.3 制約検証

対象：

- required
- additionalProperties
- oneOf / anyOf / allOf
- if / then / else
- Property の required（property_type / context）検証

---

### 3.4 参照検証

対象：

- $ref の解決
- defs.schema.json との整合
- 循環参照が発生しないこと
- 存在しない参照が検出されること

---

### 3.5 境界値検証

対象：

- 空配列
- 空オブジェクト
- null
- 最小 / 最大値
- 0 / 負数

---

## 4. テスト方針

### 4.1 正常系

- 正しい構造の JSON が検証を通過すること

---

### 4.2 異常系

- 不正な JSON が必ず検出されること
- エラー位置が正しく特定されること

---

### 4.3 網羅性

以下を必ずカバーする。

- 全必須フィールド
- 全型パターン
- 全制約条件

---

## 5. テストケース設計

各テストケースは以下を持つ。

- テストID
- テスト分類
- 対象スキーマ
- 入力JSON
- 期待結果（pass / fail）
- エラー内容（fail時）

---

## 6. テストケース

---

### SCHEMA-STRUCT-001

- テストID：SCHEMA-STRUCT-001
- 分類：構造検証
- 内容：実際のJSONを1つ載せること
- 期待結果：pass

---

### SCHEMA-TYPE-002

- テストID：SCHEMA-TYPE-002
- 分類：型検証
- 内容：string に number を指定
- 期待結果：fail

---

### SCHEMA-CONSTRAINT-003

- テストID：SCHEMA-CONSTRAINT-003
- 分類：制約検証
- 内容：additionalProperties 禁止違反
- 期待結果：fail

---

### SCHEMA-REF-004

- テストID：SCHEMA-REF-004
- 分類：参照検証
- 内容：無効な $ref
- 期待結果：fail

---

### SCHEMA-PROPERTY-005

- テストID：SCHEMA-PROPERTY-005
- 分類：型検証
- 内容：property_type に無効値を指定
- 期待結果：fail

---

### SCHEMA-PROPERTY-006

- テストID：SCHEMA-PROPERTY-006
- 分類：制約検証
- 内容：context が欠落
- 期待結果：fail

---

## 7. 成功条件

以下を満たすこと。

- 正常系テストがすべて pass
- 異常系テストがすべて fail
- エラー内容が期待と一致する
- エラー位置（JSONパス）が正しく特定されること
- エラー種別（type / keyword）が期待と一致すること

---

## 8. 失敗条件

以下の場合は失敗とする。

- 不正データが通過する
- 正常データが失敗する
- エラー内容が不正確

---

## 9. テストデータ管理

- JSON ファイルとして管理
- 正常 / 異常でディレクトリ分離

例：

```text
tests/schema/valid/
tests/schema/invalid/
```

---

## 10. 実行環境

- JSON Schema Validator（AJV等）
- テストランナー
- CI 環境

---

## 11. 自動化

- すべてのテストは自動実行
- CI で毎回実行
- スキーマ変更時は必ず再実行

---

## 12. 将来拡張

- スキーマ差分テスト
- 自動ケース生成
- fuzzテスト

---

## 13. 結論

本テスト仕様により、JSON Schema の正当性と完全性を保証する。

---

[目次](../../目次.md) > テスト仕様 > 共通 > JSONスキーマテスト仕様
