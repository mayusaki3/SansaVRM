# dry-doc formalization policy

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再配置 dry-run における `dry-doc-*` の扱い、および canonical doc_id への正式化方針を整理する。

本ポリシーは、HLDocS 側へのフィードバック候補を兼ねる。

---

## 2. 基本方針

`dry-doc-*` は canonical doc_id ではなく、dry-run relocation 用の migration identifier として扱う。

filesystem ordering と canonical identity は分離する。

```text
filesystem ordering:
5000 / 6000 / 7000 など

migration identity:
dry-doc-xxxx

canonical identity:
stable canonical doc_id
```

Layer番号や dry-doc 番号は dependency 意味論を持たない。

---

## 3. dry-doc の役割

`dry-doc-*` は以下を目的とする。

```text
- dry-run relocation tracking
- migration grouping
- migration graph tracing
- placeholder relocation tracking
- semantic duplicate tracking
- cleanup gate tracking
```

---

## 4. dry-doc の問題点

`dry-doc-*` を長期 canonical doc_id として扱う場合、以下の問題が発生する。

```text
- migration state が canonical identity に混入する
- Layer reorder の影響を受けやすい
- federation grouping が canonical identity に漏れる
- dry-run concern が長期仕様へ残る
```

---

## 5. canonicalization 方針

現在の推奨方針は以下とする。

```text
migration:
dry-doc

canonical:
stable doc_id
```

つまり、dry-doc は migration concern 用識別子として保持し、正式化後に stable canonical doc_id を割り当てる。

---

## 6. canonicalization lifecycle

```text
planned_relocation
  ↓
dry-doc assigned
  ↓
placeholder/full_copy relocation
  ↓
semantic_equivalent verified
  ↓
canonical doc_id assigned
  ↓
legacy alias ready
  ↓
cleanup_ready
```

canonicalization は cleanup_ready の前に完了する必要がある。

---

## 7. canonical doc_id 条件

stable canonical doc_id は以下を満たす必要がある。

```text
- migration concern を含まない
- Layer番号へ依存しない
- filesystem ordering に依存しない
- long-term stable である
- semantic responsibility を表現できる
```

---

## 8. dry-doc retention

canonicalization 後も dry-doc は migration graph 上に残してよい。

ただし、役割は migration trace 用に限定する。

```text
canonical doc_id:
正式識別子

dry-doc:
migration trace identifier
```

---

## 9. migration manifest との関係

migration manifest は以下を扱う。

```text
dry_doc_id
canonical_doc_id
old_doc_id
migration_entry_id
semantic_equivalent
mapping_status
canonicalization_status
```

validator は temporary dry-doc state と canonicalized state を区別できる必要がある。

---

## 10. validator との関係

validator は以下を判定する。

```text
- dry-doc collision
- canonical doc_id collision
- temporary dual canonical state
- canonicalization readiness
- canonicalization completed
```

---

## 11. cleanup gate との関係

cleanup_ready の前に canonicalization verification が必要である。

```text
dry-doc assigned
  ↓
semantic_equivalent verified
  ↓
canonical doc_id assigned
  ↓
canonical conflict resolved
  ↓
legacy alias ready
  ↓
cleanup_ready
```

---

## 12. numbering separation

本ポリシーでは以下を分離する。

```text
filesystem ordering:
ファイル番号・Layer番号

migration identity:
dry-doc

canonical identity:
stable canonical doc_id

dependency ordering:
Layer Index / dependency diagram

migration ordering:
migration manifest federation

cleanup ordering:
verification-driven cleanup gate
```

番号は意味論を持たない。

---

## 13. HLDocS feedback

本ポリシーから、HLDocS 側へ以下をフィードバックする。

```text
- dry-run relocation identifier が必要
- canonical doc_id と migration identifier を分離すべき
- canonicalization lifecycle を定義すべき
- cleanup_ready 前に canonicalization verification を要求すべき
- filesystem ordering と canonical identity を分離すべき
```

---

## 14. 結論

`dry-doc-*` は canonical doc_id ではなく、migration graph 上の dry-run relocation identifier として扱う。

正式仕様では stable canonical doc_id を別に持ち、cleanup gate 前に canonicalization verification を完了する。
