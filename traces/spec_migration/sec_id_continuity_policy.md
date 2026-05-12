# sec_id continuity policy

## 1. 目的

本ポリシーは、SansaVRM 仕様再配置 dry-run における `sec_id` の継続性、欠落、再採番、cleanup gate での扱いを定義する。

本ポリシーは文書削除や sec_id の即時付与を指示するものではない。

---

## 2. 背景

仕様再配置では、文書単位の移動だけでなく、仕様・テスト・実装の traceability を維持する必要がある。

HLDocS では traceability の実質的な接続点は文書単位ではなく section 単位であるため、`doc_id` よりも `sec_id` continuity が重要になる。

---

## 3. 基本方針

`sec_id` は cleanup / verified migration の gate として扱う。

以下を原則とする。

```text
1. sec_id continuity は doc_id continuity より優先する
2. sec_id が存在する文書は、移行時に保持または明示 remap する
3. sec_id が存在しない文書は、cleanup 前に policy 判定する
4. placeholder relocation は sec_id continuity 未確認として扱う
5. sec_id remap は migration manifest に記録する
```

---

## 4. sec_id 状態分類

| 状態 | 意味 |
|---|---|
| preserved | 旧sec_idを新文書で保持 |
| remapped | 旧sec_idから新sec_idへ明示対応 |
| newly_assigned | 旧文書にsec_idがなく、新文書で新規付与 |
| not_required | 当該文書ではsec_id不要 |
| pending | 判定未完了 |
| unknown | 旧文書確認未完了 |

---

## 5. preserved

旧文書の sec_id を新Layer文書でも保持する状態。

対象例：

```text
5002_Validator実装仕様.md
5003_変換仕様.md
```

---

## 6. remapped

旧 sec_id と新 sec_id が異なるが、semantic equivalence が確認されている状態。

migration manifest には以下を記録する。

```json
{
  "old_sec_id": "sec_old",
  "new_sec_id": "sec_new",
  "semantic_equivalent": true,
  "mapping_reason": "remapped"
}
```

---

## 7. newly_assigned

旧文書に sec_id が存在しないが、正式化時に新規 sec_id を付与する状態。

対象候補：

```text
1001_Core Semantic Definition.md
1002_Semantic Preservation Matrix.md
1003_RoundTrip Semantic Criteria.md
2001_Adapter Extension Property Specification.md
2002_Format Compatibility Preservation Specification.md
2003_Conversion Profile Specification.md
3001_Geometry Rig Skinning Extension Specification.md
3002_Morph Extension Specification.md
3003_Animation Extension Specification.md
3004_Physics Extension Specification.md
4001_物理・制御メタモデル仕様.md
```

---

## 8. not_required

index、計画、alias、trace補助文書など、仕様検証単位を持たない文書は `not_required` とできる。

対象例：

```text
5000_Validation Layer Index.md
legacy alias 文書
cleanup計画
reorder計画
```

---

## 9. pending / unknown

placeholder relocation または旧文書未確認のものは、`pending` または `unknown` とする。

対象例：

```text
4002_MuJoCo連携仕様.md
5001_JSONスキーマ仕様.md
```

---

## 10. cleanup gate

旧path cleanup または legacy alias 化の前に、対象文書は以下のいずれかでなければならない。

```text
preserved
remapped
newly_assigned
not_required
```

以下の場合は cleanup 禁止とする。

```text
pending
unknown
```

---

## 11. sec_id assignment policy

正式仕様化時に新規 sec_id を付与する場合、以下を満たす。

```text
- semantic unit ごとに付与する
- 章番号には依存しない
- doc_id 再発行に影響されない
- migration manifest に newly_assigned として記録する
```

---

## 12. sec_id と alias 文書

legacy alias 文書は原則として仕様検証単位を持たない。

そのため sec_id は `not_required` とする。

ただし、alias 自体の integrity は CI で検査する。

---

## 13. sec_id と placeholder 文書

placeholder 文書は旧本文の正式移行が未完了であるため、sec_id continuity を `pending` とする。

placeholder 文書に新規 sec_id を付与してはならない。

---

## 14. sec_id と split relocation

split relocation では、旧sec_idが複数新文書へ分配される可能性がある。

この場合、sec_mappings には split boundary を記録する。

```text
old_sec_id
↓
new_doc_id + new_sec_id
```

---

## 15. sec_id と merge relocation

merge relocation では、複数旧文書の sec_id が単一新文書へ集約される可能性がある。

この場合、sec_id collision を検査する。

---

## 16. CI Requirements

CI は以下を検査できる。

```text
- cleanup対象に pending / unknown が含まれない
- preserved / remapped / newly_assigned / not_required のいずれかに分類済み
- sec_id duplicate が存在しない
- sec_id remap に semantic_equivalent が設定されている
```

---

## 17. HLDocS feedback

本ポリシーで得られた知見：

```text
- doc_id continuity より sec_id continuity が重要
- placeholder relocation では sec_id を付与しない方がよい
- alias 文書は sec_id not_required として扱うべき
- sec_id status が cleanup gate に必要
- split / merge relocation では sec_id graph が必要
```

---

## 18. 結論

SansaVRM 仕様再配置では、sec_id continuity を cleanup / verified migration の重要 gate として扱う。

placeholder 文書は sec_id pending とし、legacy alias 文書は sec_id not_required とする。

正式仕様化時には、sec_id の preserved / remapped / newly_assigned / not_required を明示し、migration manifest に記録する。
