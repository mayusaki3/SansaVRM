# 旧path cleanup分類表

## 1. 目的

本ドキュメントは、`docs/ja-JP/02_仕様/01_共通/` に残る旧path仕様を、新Layer構成への移行状態に応じて分類する。

本分類は削除指示ではない。

---

## 2. 分類区分

| 分類 | 意味 | cleanup可否 |
|---|---|---|
| keep_in_common | Layer分離後も共通仕様として残す | 削除不可 |
| migrated_complete | 新Layerへ全文移行済み | alias化後に削除検討可 |
| migrated_partial | 新Layerへ一部移行済み | 削除不可 |
| placeholder_only | 新Layerは仮配置のみ | 削除不可 |
| legacy_alias_candidate | 新Layer正式化後に旧pathを案内文へ置換可能 | 削除保留 |
| obsolete_candidate | 将来廃止候補 | 要確認 |

---

## 3. 旧path分類

| 旧path | 新path | 分類 | 理由 |
|---|---|---|---|
| `01_仕様概要.md` | なし | keep_in_common | 仕様全体の概要として共通に残す |
| `02_メタモデル仕様.md` | なし | keep_in_common | Core/Data Model整理後も基礎仕様として残す可能性あり |
| `03_glTF拡張仕様.md` | なし | keep_in_common | glTF拡張の基本仕様として残す可能性あり |
| `04_JSONスキーマ仕様.md` | `50_Validation Layer/5001_JSONスキーマ仕様.md` | placeholder_only | 新pathはplaceholderで全文未移行 |
| `05_Validator実装仕様.md` | `50_Validation Layer/5002_Validator実装仕様.md` | migrated_complete | sec_idを保持して全文移行済み |
| `06_CoreAPI仕様.md` | なし | keep_in_common | Core APIは別Layer化未実施。削除不可 |
| `07_変換仕様.md` | `50_Validation Layer/5003_変換仕様.md` | migrated_complete | sec_idを保持して全文移行済み |
| `08_物理・制御メタモデル仕様.md` | `40_Runtime Integration Layer/4001_物理・制御メタモデル仕様.md` | migrated_complete | 新Layerへ既移行済み |
| `09_MuJoCo連携仕様.md` | `40_Runtime Integration Layer/4002_MuJoCo連携仕様.md` | placeholder_only | 新pathはplaceholderで全文未移行 |
| `10_Core Semantic Definition.md` | `10_Core Semantic Layer/1001_Core Semantic Definition.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `11_Semantic Preservation Matrix.md` | `10_Core Semantic Layer/1002_Semantic Preservation Matrix.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `12_RoundTrip Semantic Criteria.md` | `10_Core Semantic Layer/1003_RoundTrip Semantic Criteria.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `13_Adapter Extension Property Specification.md` | `20_Preservation Compatibility Layer/2001_Adapter Extension Property Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `14_Format Compatibility Preservation Specification.md` | `20_Preservation Compatibility Layer/2002_Format Compatibility Preservation Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `15_Geometry Rig Skinning Extension Specification.md` | `30_Data Model Layer/3001_Geometry Rig Skinning Extension Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `16_Morph Extension Specification.md` | `30_Data Model Layer/3002_Morph Extension Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `17_Animation Extension Specification.md` | `30_Data Model Layer/3003_Animation Extension Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `18_Physics Extension Specification.md` | `30_Data Model Layer/3004_Physics Extension Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `19_Conversion Profile Specification.md` | `20_Preservation Compatibility Layer/2003_Conversion Profile Specification.md` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |

---

## 4. 直近の削除禁止対象

以下は削除禁止とする。

```text
04_JSONスキーマ仕様.md
09_MuJoCo連携仕様.md
01_仕様概要.md
02_メタモデル仕様.md
03_glTF拡張仕様.md
06_CoreAPI仕様.md
```

---

## 5. alias化候補

以下は、manifest verified 後に legacy alias 化を検討する。

```text
05_Validator実装仕様.md
07_変換仕様.md
08_物理・制御メタモデル仕様.md
10_Core Semantic Definition.md
11_Semantic Preservation Matrix.md
12_RoundTrip Semantic Criteria.md
13_Adapter Extension Property Specification.md
14_Format Compatibility Preservation Specification.md
15_Geometry Rig Skinning Extension Specification.md
16_Morph Extension Specification.md
17_Animation Extension Specification.md
18_Physics Extension Specification.md
19_Conversion Profile Specification.md
```

---

## 6. cleanup gate

legacy alias 化または削除判断の前に以下を確認する。

```text
- migration manifest 登録済み
- semantic_equivalent が true
- mapping_status が complete または verified
- sec_id が存在する場合は sec_mappings 登録済み
- 目次が新Layer pathを参照済み
- CIでmigration manifest validation成功
```

---

## 7. 現時点判断

現時点では旧path削除を行わない。

理由：

```text
- placeholder_only 文書が存在する
- sec_id未登録の migrated_complete 文書が存在する
- main manifest と validation-layer manifest が分離状態
- verified 判定が未完了
```

---

## 8. HLDocS feedback

本分類で得られた知見：

```text
- cleanup は relocation と別フェーズにすべき
- legacy alias state が必要
- placeholder_only は削除禁止にすべき
- sec_idなし文書でも cleanup gate が必要
- manifest federation が必要になる可能性がある
```
