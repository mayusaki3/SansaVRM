<!--
HLDocS:LLM-MANAGED
doc_id: doc-20260510-000017Z-SV02
lang: ja-JP
canonical_title: 仕様再配置dry-run計画
document_type: note
canonical_document: true
-->

[目次](../../目次.md) > 実装計画 > 共通 > 仕様再配置dry-run計画

# 仕様再配置dry-run計画

## 1. 目的

本ドキュメントは、SansaVRM 仕様群の Layer 化・番号再配置・目次再編を実施する前に、実移行 dry-run を行うための計画を定義する。

本計画では、実ファイル移動はまだ行わない。

以下を事前に固定する。

```text
- 旧path
- 新path
- 新doc_id方針
- migration manifest entry
- sec_id migration方針
- 検査項目
- rollback条件
```

---

## 2. dry-run の目的

dry-run の目的は以下である。

```text
1. ファイル移動前に移行対象を完全列挙する
2. migration manifest を事前生成できる形にする
3. 目次更新範囲を明確化する
4. Traceability断絶を事前検出する
5. validator / CI 更新範囲を明確化する
```

---

## 3. dry-run の非目的

以下は dry-run では行わない。

```text
- 実ファイル移動
- doc_id確定再生成
- sec_id確定再生成
- validator実装修正
- CI本実装
```

---

## 4. dry-run 対象

対象は以下である。

```text
docs/ja-JP/02_仕様/01_共通/
docs/ja-JP/02_仕様/02_VRM入出力/
docs/ja-JP/04_実装計画/01_共通/
docs/ja-JP/05_トレーサビリティ/01_共通/
schemas/traceability/
```

---

## 5. 仕様ファイル再配置候補

## 5.1 Core Semantic Layer

```text
旧path:
docs/ja-JP/02_仕様/01_共通/10_Core Semantic Definition.md
docs/ja-JP/02_仕様/01_共通/11_Semantic Preservation Matrix.md
docs/ja-JP/02_仕様/01_共通/12_RoundTrip Semantic Criteria.md

新path候補:
docs/ja-JP/02_仕様/10_Core Semantic Layer/1001_Core Semantic Definition.md
docs/ja-JP/02_仕様/10_Core Semantic Layer/1002_Semantic Preservation Matrix.md
docs/ja-JP/02_仕様/10_Core Semantic Layer/1003_RoundTrip Semantic Criteria.md
```

---

## 5.2 Preservation / Compatibility Layer

```text
旧path:
docs/ja-JP/02_仕様/01_共通/13_Adapter Extension Property Specification.md
docs/ja-JP/02_仕様/01_共通/14_Format Compatibility Preservation Specification.md
docs/ja-JP/02_仕様/01_共通/19_Conversion Profile Specification.md

新path候補:
docs/ja-JP/02_仕様/20_Preservation Compatibility Layer/2001_Adapter Extension Property Specification.md
docs/ja-JP/02_仕様/20_Preservation Compatibility Layer/2002_Format Compatibility Preservation Specification.md
docs/ja-JP/02_仕様/20_Preservation Compatibility Layer/2003_Conversion Profile Specification.md
```

---

## 5.3 Data Model Layer

```text
旧path:
docs/ja-JP/02_仕様/01_共通/15_Geometry Rig Skinning Extension Specification.md
docs/ja-JP/02_仕様/01_共通/16_Morph Extension Specification.md
docs/ja-JP/02_仕様/01_共通/17_Animation Extension Specification.md
docs/ja-JP/02_仕様/01_共通/18_Physics Extension Specification.md

新path候補:
docs/ja-JP/02_仕様/30_Data Model Layer/3001_Geometry Rig Skinning Extension Specification.md
docs/ja-JP/02_仕様/30_Data Model Layer/3002_Morph Extension Specification.md
docs/ja-JP/02_仕様/30_Data Model Layer/3003_Animation Extension Specification.md
docs/ja-JP/02_仕様/30_Data Model Layer/3004_Physics Extension Specification.md
```

---

## 5.4 Runtime Integration Layer

```text
旧path:
docs/ja-JP/02_仕様/01_共通/08_物理・制御メタモデル仕様.md
docs/ja-JP/02_仕様/01_共通/09_MuJoCo連携仕様.md

新path候補:
docs/ja-JP/02_仕様/40_Runtime Integration Layer/4001_物理・制御メタモデル仕様.md
docs/ja-JP/02_仕様/40_Runtime Integration Layer/4002_MuJoCo連携仕様.md
```

---

## 5.5 Validation Layer

```text
旧path:
docs/ja-JP/02_仕様/01_共通/04_JSONスキーマ仕様.md
docs/ja-JP/02_仕様/01_共通/05_Validator実装仕様.md
docs/ja-JP/02_仕様/01_共通/07_変換仕様.md

新path候補:
docs/ja-JP/02_仕様/50_Validation Layer/5001_JSONスキーマ仕様.md
docs/ja-JP/02_仕様/50_Validation Layer/5002_Validator実装仕様.md
docs/ja-JP/02_仕様/50_Validation Layer/5003_変換仕様.md
```

---

## 5.6 Import / Export Layer

```text
旧path:
docs/ja-JP/02_仕様/02_VRM入出力/01_VRM 0.x 1.0 差分整理.md
docs/ja-JP/02_仕様/02_VRM入出力/03_VRM 0.x import詳細設計.md
docs/ja-JP/02_仕様/02_VRM入出力/04_VRM 1.0 import詳細設計.md

新path候補:
docs/ja-JP/02_仕様/60_Import Export Layer/6001_VRM 0.x 1.0 差分整理.md
docs/ja-JP/02_仕様/60_Import Export Layer/6002_VRM 0.x import詳細設計.md
docs/ja-JP/02_仕様/60_Import Export Layer/6003_VRM 1.0 import詳細設計.md
```

---

## 5.7 Roadmap Layer

```text
旧path:
docs/ja-JP/04_実装計画/01_共通/01_初版実装ロードマップ.md
docs/ja-JP/04_実装計画/01_共通/02_ロードマップ再整理.md
docs/ja-JP/04_実装計画/01_共通/03_仕様依存マップ.md
docs/ja-JP/04_実装計画/01_共通/04_仕様再配置計画.md
docs/ja-JP/04_実装計画/01_共通/05_仕様再配置dry-run計画.md

新path候補:
docs/ja-JP/04_実装計画/70_Roadmap Layer/7001_初版実装ロードマップ.md
docs/ja-JP/04_実装計画/70_Roadmap Layer/7002_ロードマップ再整理.md
docs/ja-JP/04_実装計画/70_Roadmap Layer/7003_仕様依存マップ.md
docs/ja-JP/04_実装計画/70_Roadmap Layer/7004_仕様再配置計画.md
docs/ja-JP/04_実装計画/70_Roadmap Layer/7005_仕様再配置dry-run計画.md
```

---

## 6. migration manifest dry-run

実移行前に以下の manifest を生成する。

```text
traces/spec_migration/migration_manifest.dry-run.json
```

本移行時は以下へ昇格する。

```text
traces/spec_migration/migration_manifest.json
```

---

## 7. doc_id / sec_id方針

## 7.1 doc_id

再配置時は doc_id 再生成を許可する。

ただし dry-run では仮doc_idを使用する。

```text
dry_doc_id
```

---

## 7.2 sec_id

sec_id 再生成を許可する。

ただし dry-run では section 単位の対応方針のみを記録する。

```text
sec_mapping_policy
```

---

## 8. dry-run 検査項目

以下を検査する。

```text
- 移動元pathが存在する
- 移動先pathが重複しない
- 目次リンクが生成可能
- migration manifest schemaに適合する
- orphan doc_idが発生しない
- orphan sec_idが発生しない設計である
- semantic_equivalent が明示されている
```

---

## 9. rollback条件

以下の場合、実移行へ進まない。

```text
- 移動先path重複
- migration manifest不整合
- semantic_equivalent未記入
- 目次生成不可
- validator参照更新範囲不明
- Traceability参照更新範囲不明
```

---

## 10. 実移行前の確認事項

実移行前に以下を確認する。

```text
1. dry-run manifest が schema validation を通過する
2. 目次新構成が確定している
3. migration対象が全列挙されている
4. Traceability更新方針が確定している
5. validator更新方針が確定している
```

---

## 11. 実移行時の推奨順序

```text
1. 新ディレクトリ作成
2. ファイル移動
3. LLM-MANAGED block更新
4. hierarchy link更新
5. 目次更新
6. migration_manifest.json生成
7. Traceability更新
8. validator参照更新
9. CI実行
```

---

## 12. 関連仕様

本計画は以下と連携する。

```text
仕様依存マップ
仕様再配置計画
Traceability Migration Specification
Migration Manifest Specification
```

---

## 13. 結論

仕様再配置は semantic continuity を維持しながら実施する必要がある。

そのため、実ファイル移動前に dry-run を行い、移動先、migration manifest、目次、Traceability、validator 参照更新を事前検証する。

---

[目次](../../目次.md) > 実装計画 > 共通 > 仕様再配置dry-run計画
