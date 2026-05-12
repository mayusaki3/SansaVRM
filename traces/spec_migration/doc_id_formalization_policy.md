# doc_id formalization policy

## 1. 目的

本ポリシーは、SansaVRM 仕様再配置 dry-run で導入した `dry-doc-*` doc_id を、正式移行時にどのように扱うかを定義する。

本ポリシーは削除・rename を指示するものではない。

---

## 2. 背景

仕様Layer再配置では、新Layer path に dry-run 文書を作成するため、暫定 doc_id として `dry-doc-*` を使用した。

例：

```text
dry-doc-1001-core-semantic-definition
dry-doc-5000-validation-layer-index
dry-doc-5009-roundtrip-verification-specification
```

---

## 3. 基本方針

正式化前に以下を決定する。

```text
- dry-doc を正式 doc_id として採用するか
- 新 doc_id を再採番するか
- old_doc_id と new_doc_id の alias を残すか
- sec_id continuity をどのように保持するか
```

---

## 4. doc_id 状態分類

| 状態 | 意味 |
|---|---|
| dry | dry-run 用の暫定 doc_id |
| formalized | 正式 doc_id として採用済み |
| reissued | 正式 doc_id を再採番済み |
| aliased | 旧doc_idとの対応を保持 |
| deprecated | 旧doc_idとして参照のみ保持 |

---

## 5. 選択肢A: dry-doc を正式化

### 内容

`dry-doc-*` をそのまま正式 doc_id として採用する。

### メリット

```text
- 追加 migration が不要
- 新Layer path と doc_id が一致しやすい
- migration manifest が単純
```

### デメリット

```text
- dry-run 由来であることが doc_id に残る
- HLDocS の正式doc_id命名規則と整合しない可能性がある
```

### 判定

原則として採用しない。

---

## 6. 選択肢B: 正式 doc_id を再採番

### 内容

正式移行時に HLDocS 準拠の doc_id を新規発行する。

### メリット

```text
- 正式仕様として整った doc_id になる
- dry-run と正式版を明確に分離できる
```

### デメリット

```text
- migration manifest 更新が必要
- old_doc_id / dry_doc_id / formal_doc_id の三者対応が必要
```

### 判定

正式採用候補。

---

## 7. 選択肢C: alias doc_id を導入

### 内容

正式 doc_id を発行しつつ、旧 doc_id / dry doc_id を alias として保持する。

### メリット

```text
- traceability continuity を維持しやすい
- 旧参照が壊れにくい
- migration graph と相性がよい
```

### デメリット

```text
- doc_id 管理が複雑になる
- alias 解決規則が必要
```

### 判定

推奨。

---

## 8. 推奨方針

正式移行では以下を推奨する。

```text
1. formal_doc_id を新規発行する
2. old_doc_id と dry_doc_id を alias として保持する
3. migration manifest に doc_id_aliases を追加する
4. sec_id continuity を doc_id alias より優先する
```

---

## 9. doc_id_aliases

migration manifest には以下を追加できる。

```json
{
  "doc_id_aliases": [
    {
      "alias_type": "legacy",
      "doc_id": "doc-20260510-000003Z-SV02"
    },
    {
      "alias_type": "dry_run",
      "doc_id": "dry-doc-1001-core-semantic-definition"
    }
  ]
}
```

---

## 10. sec_id との関係

正式移行では doc_id よりも sec_id continuity を優先する。

理由：

```text
- traceability は section-level で機能する
- doc_id は document container の識別子
- sec_id は検証・テスト・実装との接続点
```

---

## 11. dry-doc の扱い

`dry-doc-*` は以下の用途に限定する。

```text
- dry-run relocation
- placeholder relocation
- temporary layer verification
```

正式仕様セットには原則として含めない。

---

## 12. cleanup gate

旧path cleanup 前に以下を確認する。

```text
- formal_doc_id 発行済み
- doc_id_aliases 登録済み
- sec_mappings 登録済み
- migration_status = verified
```

---

## 13. HLDocS feedback

本ポリシーで得られた知見：

```text
- dry-run doc_id と formal doc_id は分離すべき
- doc_id alias が必要
- doc_id continuity より sec_id continuity が重要
- migration manifest に alias 解決機構が必要
```

---

## 14. 結論

`dry-doc-*` は正式 doc_id として採用せず、正式移行時に formal_doc_id を発行する。

旧 doc_id および dry doc_id は alias として migration manifest に保持し、sec_id continuity を優先して traceability を維持する。
