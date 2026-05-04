<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260504-000302Z-SV0L
lang: ja-JP
canonical_title: humanoid Property設計
document_type: spec
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > VRM入出力 > humanoid Property設計

# humanoid Property設計

## 1. 目的

VRM humanoid を SansaVRM の Property / Binding として表現する設計を確定する。

---

## 2. VRM humanoid構造

### VRM 0.x / 1.0 共通

- bone名（Hips, Spine, Head など）
- node index（glTF node参照）
- optional bone（存在しない場合あり）

---

## 3. SansaVRM表現

### 3.1 構造

humanoid は以下で構成する。

- Property（Rig）
- Binding（bone → Module）

---

## 4. Property定義

PropertyType:
- Rig

PropertyContext:
- Simulation
- Execution

---

## 5. Propertyデータ構造

```json
{
  "type": "Rig",
  "bones": [
    {
      "name": "hips",
      "module_id": "module_001"
    },
    {
      "name": "head",
      "module_id": "module_010"
    }
  ]
}
```

---

## 6. Binding設計

### 6.1 役割

- bone → Module の対応関係を保持

### 6.2 要件

- 1 bone → 1 Module
- Module は glTF node 由来
- 未定義boneは保持しない
- 重複boneは禁止

---

## 7. 正規化ルール

### 7.1 bone名

- VRM標準bone名を使用
- 内部では lowercase に正規化

例：

- Hips → hips
- LeftUpperArm → leftupperarm

---

## 8. import処理仕様

### 8.1 入力

- VRM humanoid.bones

### 8.2 処理フロー

1. node index → Module ID 変換
2. bone名を正規化
3. bone → module 対応生成
4. Property生成
5. Binding生成

---

## 9. export処理仕様

### 9.1 処理フロー

1. Propertyからbone一覧取得
2. Module ID → node index 逆引き
3. humanoid.bones再構築
4. VRM形式へ出力

---

## 10. エラーハンドリング

### 10.1 import時

- node index 未解決 → エラー
- bone重複 → エラー
- 不正bone名 → 警告（保持は可能）

### 10.2 export時

- module未解決 → エラー
- 必須bone欠落 → 警告

---

## 11. テスト要件

### 11.1 正常系

- humanoid_bone_mapping
- full_humanoid_import
- minimal_humanoid_import

### 11.2 異常系

- missing_node
- duplicate_bone
- invalid_bone_name

---

## 12. 制約

- PropertyContextの拡張は禁止
- humanoid専用structをcoreに追加しない
- Module構造にbone情報を持たせない

---

[目次](../../目次.md) > 仕様 > VRM入出力 > humanoid Property設計
