# 旧path cleanup分類表

## 1. 目的

本ドキュメントは、`docs/ja-JP/02_仕様/01_共通/` に残る旧path仕様を、新Layer構成への移行状態に応じて分類する。

本分類は削除指示ではない。

cleanup ordering は Layer番号ではなく、verification condition、manifest 状態、および migration readiness により決定する。

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

Layer番号は dependency 意味論ではなく filesystem ordering として扱う。

Layer dependency は Layer Index、dependency diagram、migration manifest、および本文定義により定義する。

---

## 3. 旧path分類

| 旧path | 新path | 分類 | 理由 |
|---|---|---|---|
| `仕様概要` | なし | keep_in_common | 仕様全体の概要として共通に残す |
| `メタモデル仕様` | なし | keep_in_common | Core/Data Model整理後も基礎仕様として残す可能性あり |
| `glTF拡張仕様` | なし | keep_in_common | glTF拡張の基本仕様として残す可能性あり |
| `JSONスキーマ仕様` | `Validation Layer/JSONスキーマ仕様` | placeholder_only | 新pathはplaceholderで全文未移行 |
| `Validator実装仕様` | `Validation Layer/Validator実装仕様` | migrated_complete | sec_idを保持して全文移行済み |
| `CoreAPI仕様` | なし | keep_in_common | Core APIは別Layer化未実施。削除不可 |
| `変換仕様` | `Validation Layer/変換仕様` | migrated_complete | sec_idを保持して全文移行済み |
| `物理・制御メタモデル仕様` | `Runtime Integration Layer/物理・制御メタモデル仕様` | migrated_complete | 新Layerへ既移行済み |
| `MuJoCo連携仕様` | `Runtime Integration Layer/MuJoCo連携仕様` | placeholder_only | 新pathはplaceholderで全文未移行 |
| `Core Semantic Definition` | `Core Semantic Layer/Core Semantic Definition` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Semantic Preservation Matrix` | `Core Semantic Layer/Semantic Preservation Matrix` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `RoundTrip Semantic Criteria` | `Core Semantic Layer/RoundTrip Semantic Criteria` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Adapter Extension Property Specification` | `Preservation Compatibility Layer/Adapter Extension Property Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Format Compatibility Preservation Specification` | `Preservation Compatibility Layer/Format Compatibility Preservation Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Geometry Rig Skinning Extension Specification` | `Data Model Layer/Geometry Rig Skinning Extension Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Morph Extension Specification` | `Data Model Layer/Morph Extension Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Animation Extension Specification` | `Data Model Layer/Animation Extension Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Physics Extension Specification` | `Data Model Layer/Physics Extension Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |
| `Conversion Profile Specification` | `Preservation Compatibility Layer/Conversion Profile Specification` | migrated_complete | 新Layerへ移行済み。ただしsec_idなし |

---

## 4. 直近の削除禁止対象

以下は削除禁止とする。

```text
JSONスキーマ仕様
MuJoCo連携仕様
仕様概要
メタモデル仕様
glTF拡張仕様
CoreAPI仕様
```

placeholder relocation 状態の文書は cleanup ordering より優先して削除禁止とする。

---

## 5. alias化候補

以下は、manifest verified 後に legacy alias 化を検討する。

```text
Validator実装仕様
変換仕様
物理・制御メタモデル仕様
Core Semantic Definition
Semantic Preservation Matrix
RoundTrip Semantic Criteria
Adapter Extension Property Specification
Format Compatibility Preservation Specification
Geometry Rig Skinning Extension Specification
Morph Extension Specification
Animation Extension Specification
Physics Extension Specification
Conversion Profile Specification
```

legacy alias readiness は Layer番号ではなく、migration verification と manifest 状態に依存する。

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

cleanup gate は filesystem ordering ではなく verification-driven として扱う。

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
- cleanup ordering は Layer番号ではなく verification-driven にすべき
- Layer番号は dependency 意味論ではなく filesystem ordering として扱うべき
```
