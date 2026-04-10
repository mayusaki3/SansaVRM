[目次](../目次.md) > 要件定義 > Runtime動作

# Runtime動作

## 1. 目的

本仕様は、SansaVRM における Runtime 上での動作を定義する。

モデルは静的なデータではなく、
Runtime において動的に構成・評価・更新されることを前提とする。

---

## 2. 基本方針

- モデルは動的に構成可能である
- 判定（適合・権利）は Runtime で実行される
- 構成変更は即時反映可能である
- 部分更新が可能である
- ワールドオブジェクトと同一基盤で扱う

---

## 3. Runtime構成要素

Runtime は以下の要素で構成される。

- Model Instance
- Module Instance
- Slot Manager
- Compatibility Engine
- Rights Engine
- Revenue Engine
- Event System

---

## 4. Model Instance

実行時のモデル実体。

### 4.1 保持情報

- 構成モジュール
- 接続関係
- 状態情報
- 権利状態
- 収益状態

---

## 5. Module Instance

モジュールの Runtime 実体。

### 5.1 保持情報

- attach_point
- target_slot
- 状態（有効 / 無効）
- ローカル設定

---

## 6. Slot Manager

スロットの管理を行う。

### 6.1 機能

- スロット登録
- スロット占有管理
- スロット解放
- スロット競合検出

---

## 7. Compatibility Engine

適合判定を行う。

### 7.1 機能

- 構造判定
- タグ判定
- スロット判定
- 制約判定

---

## 8. Rights Engine

権利・許諾を判定する。

### 8.1 機能

- 利用可否判定
- 条件評価
- 利用制御

---

## 9. Revenue Engine

収益処理を行う。

### 9.1 機能

- 利用イベント取得
- 分配計算
- ログ記録

---

## 10. Event System

Runtime のすべての処理はイベント駆動で行う。

### 10.1 主なイベント

- onModelLoad
- onModuleAttach
- onModuleDetach
- onStructureChange
- onUsage
- onRightsCheck
- onRevenueProcess

---

## 11. 処理フロー

### 11.1 モデルロード

1. モデル読み込み
2. モジュール展開
3. スロット登録
4. 適合判定
5. 権利判定
6. 初期状態確定

---

### 11.2 モジュール装着

1. attach要求
2. スロット確認
3. 適合判定
4. 権利判定
5. attach実行
6. 状態更新

---

### 11.3 モジュール取り外し

1. detach要求
2. 依存関係確認
3. detach実行
4. 状態更新

---

### 11.4 利用イベント

1. 利用検知
2. 権利判定
3. 利用許可
4. 収益計算
5. ログ記録

---

## 12. 状態管理

モデルは以下の状態を持つ。

- ACTIVE
- INACTIVE
- RESTRICTED
- INVALID

---

## 13. 再評価

構成変更時に再評価を行う。

### 13.1 対象

- モジュール追加
- モジュール削除
- 設定変更

---

### 13.2 結果

- 継続可能
- 制限付き
- 無効化

---

## 14. ワールドオブジェクト対応

Runtime はアバターとワールドオブジェクトを区別しない。

- 同一構造
- 同一判定
- 同一収益処理

---

## 15. パフォーマンス要件

- 差分更新を基本とする
- フル再評価を最小化
- 非同期処理を許容
- キャッシュ利用

---

## 16. diagnostics

Runtime 異常を記録する。

- 判定失敗
- attach失敗
- 権利違反
- 収益処理失敗

---

## 17. 将来拡張

- 分散Runtime
- マルチユーザー同期
- AI制御モデル
- 自律動作
- クロスプラットフォーム同期

---

[目次](../目次.md) > 要件定義 > Runtime動作
