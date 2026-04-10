[目次](../目次.md) > ProjectSansa > VRMライブラリ > 要件定義 > 初版

# VRMライブラリ要件定義 初版

## 1. 目的

ProjectSansa で利用可能な、C++ ベースの VRM / glTF ライブラリを整備する。
本ライブラリは、以下を目的とする。

- glTF 2.0 の基礎実装を提供する
- VRM 0.x / 1.x を同一ライブラリ群の中で扱えるようにする
- VRM 0.x で読込、内部共通表現へ変換し、VRM 1.x として保存できるようにする
- O3DE / Unity から利用可能な構成とする
- 将来、Khronos 管理下の glTF 拡張として VRM 関連仕様が再編された場合でも、上位層の追加実装で吸収可能にする

## 2. 開発方針

- 中核は C++ で実装する
- 既存 C# 実装は、移植元というより「参照実装・差分確認元」として扱う
- 仕様依存部を分離し、glTF 基盤と VRM 拡張層を分ける
- バージョン差異は直接アプリ層へ露出させず、共通内部表現で吸収する
- O3DE / Unity 向けには、中核 C++ を利用するためのアダプタ層またはバインディング層を分離して実装する

## 3. システム構成要件

### 3.1 層構造

以下の層に分離する。

1. glTF Core 層
2. VRM Extension 層
3. Meta VRM 層（内部共通表現）
4. Engine Adapter 層（O3DE / Unity）

### 3.2 各層の責務

#### 3.2.1 glTF Core 層

責務:
- glTF 2.0 / GLB の読込・保存
- JSON チャンク / BIN チャンクの処理
- Buffer / BufferView / Accessor / Image / Texture / Material / Mesh / Node / Skin / Scene / Animation の基本表現
- 拡張 (`extensions`) および任意データ (`extras`) の保持
- 未知拡張を破壊せず保持するパススルー機能
- バージョン差異を持たない汎用 glTF 入出力基盤

必須要件:
- 読み込んだ glTF/GLB を、意味を壊さず再保存できること
- 未対応拡張を削除せず保持可能であること
- 参照整合性チェックを行えること

#### 3.2.2 VRM Extension 層

責務:
- VRM 0.x 拡張の読込・保存
- VRM 1.x 拡張の読込・保存
- Khronos 側で将来追加・再編される VRM 相当拡張の実装追加ポイントを提供

必須要件:
- VRM 0.x の `extensions.VRM` を扱えること
- VRM 1.x の `VRMC_vrm`、`VRMC_springBone`、`VRMC_node_constraint`、`VRMC_materials_mtoon` を扱えること
- 拡張単位でモジュール追加できる構造であること
- glTF Core 層に VRM 固有知識を極力持ち込まないこと

#### 3.2.3 Meta VRM 層

責務:
- VRM 0.x / 1.x / 将来拡張差分を吸収する内部共通モデル
- バージョン間変換
- 保存対象バージョンに応じた欠落項目補完・制約適用・警告生成

必須要件:
- VRM 0.x 読込 -> Meta VRM 変換
- VRM 1.x 読込 -> Meta VRM 変換
- Meta VRM -> VRM 0.x 保存
- Meta VRM -> VRM 1.x 保存
- 変換時に情報欠落・非可逆変換がある場合、それを検出して呼び出し元へ通知できること

#### 3.2.4 Engine Adapter 層

責務:
- O3DE での表示・更新・操作用アダプタ
- Unity での表示・更新・操作用アダプタ
- エンジン固有型と Meta VRM / glTF Core の橋渡し

必須要件:
- 中核ロジックはエンジン非依存であること
- O3DE / Unity 依存コードは分離すること
- Viewer 機能はアダプタ層側で提供し、中核へ描画 API 依存を持ち込まないこと

## 4. 機能要件

### 4.1 glTF Core 基本機能

- glTF(.gltf) 読込
- GLB(.glb/.vrm) 読込
- glTF / GLB 保存
- JSON/BIN チャンクの生成
- バッファ参照解決
- 画像リソース解決
- ノード階層の復元
- スキン情報の復元
- マテリアル基本情報の復元
- 拡張データの保持
- 未知拡張データのラウンドトリップ保持

### 4.2 VRM 基本機能

- VRM 0.x 読込
- VRM 1.x 読込
- VRM 0.x 保存
- VRM 1.x 保存
- Meta VRM への統一変換
- Meta VRM から各版への変換
- バージョン差異による変換不能項目の警告出力

### 4.3 Viewer / Runtime 利用機能

- モデルロード
- シーンノード取得
- Humanoid ボーン取得
- メタ情報取得
- 表情情報取得
- 視線制御情報取得
- SpringBone / Constraint 情報取得
- Viewer 実装から利用しやすい API 提供

## 5. 非機能要件

- 中核ライブラリはエンジン非依存
- 例外発生時に原因追跡しやすいエラー体系を持つ
- 破損ファイル、未知拡張、部分的欠落データに対し異常終了ではなく診断可能であること
- 将来拡張を追加しても既存 glTF Core を大きく変更しない構造であること
- 単体テストで Core / VRM 変換の主要パスを網羅すること
- 変換時の警告・情報欠落をログまたは診断オブジェクトとして取得できること

## 6. 初期スコープ

初期スコープは以下とする。

### 6.1 Phase 1
- glTF Core 層の実装
- GLB 読込保存
- 拡張保持
- 基本的な参照整合性検証

### 6.2 Phase 2
- VRM 0.x Reader / Writer
- VRM 1.x Reader / Writer
- Meta VRM 層
- 0.x <-> Meta / 1.x <-> Meta 変換

### 6.3 Phase 3
- O3DE Adapter
- Unity Adapter
- 簡易 Viewer

## 7. 制約・前提

- VRM 0.x と 1.x は同一仕様ではないため、完全可逆変換は保証しない
- 保存先バージョンに存在しない情報は欠落または近似変換となる場合がある
- Khronos 側の将来仕様は未確定のため、現時点では「追加拡張モジュールで吸収可能であること」を要件とし、具体実装は固定しない
- Viewer 機能はライブラリ本体ではなく Adapter 側の責務とする

## 8. 今回確定したい重要方針

- C# 実装の全面移植ではなく、C++ 中核を新設する
- glTF Core / VRM Extension / Meta VRM / Engine Adapter の4層に分離する
- バージョン差異吸収は Meta VRM 層で行う
- Khronos 化対応は VRM Extension 層への追加実装で吸収する
- O3DE / Unity 対応は中核とは別の Adapter 層で行う

---
[目次](../目次.md) > ProjectSansa > VRMライブラリ > 要件定義 > 初版