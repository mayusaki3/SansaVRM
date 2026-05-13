# migration federation validator draft

## 1. 目的

本ドラフトは、SansaVRM の大規模仕様再配置 dry-run における manifest federation を検証する validator 構想を定義する。

本ドラフトは実装仕様ではなく、HLDocS 側へフィードバックするための実地検証メモを兼ねる。

---

## 2. 基本方針

migration federation validator は、単一 manifest の JSON 検査ではなく、複数 manifest をまたいだ migration graph validation を行う。

検査対象は filesystem ordering ではなく、migration graph、semantic state、cleanup readiness である。

Layer番号は dependency 意味論ではなく filesystem ordering として扱う。

---

## 3. 入力

validator の入力は以下とする。

```text
- root manifest
- sub-manifest list
- migration entry set
- cleanup classification
- integrity check report
- layer dependency diagram
```

---

## 4. manifest federation 構成

manifest federation は以下の構成を許可する。

```text
root manifest
  ├ sub-manifest: core / preservation / data model / runtime
  ├ sub-manifest: validation
  ├ sub-manifest: import export
  └ sub-manifest: roadmap
```

sub-manifest の分割単位は Layer番号ではなく、review unit / migration concern / conflict isolation によって決定してよい。

---

## 5. 検査対象状態

validator は以下の migration state を扱う。

```text
new_document
path_relocation
full_copy_relocation
placeholder_relocation
migrated_partial
migrated_complete
verified
legacy_alias
cleanup_blocked
cleanup_ready
canonical_transition
```

---

## 6. 一意性検査

### 6.1 migration_id

すべての manifest の `migration_id` は federation 内で一意でなければならない。

### 6.2 entry_id

すべての entry_id は federation 内で一意でなければならない。

### 6.3 path

同一 new path の重複は禁止する。

ただし、old path と new path の semantic duplicate は、manifest 上で migration relation が明示されている場合に限り許可する。

### 6.4 doc_id

doc_id duplicate は以下のように分類する。

```text
valid_same_document:
同一文書の移行または alias として明示されている

invalid_collision:
無関係な文書で doc_id が衝突している

temporary_dual_canonical:
dry-run relocation 中の一時的状態として明示されている
```

---

## 7. duplicate分類

validator は duplicate を以下に分類する。

```text
path duplicate
doc_id duplicate
semantic duplicate
canonical duplicate
sec_id collision
```

semantic duplicate は dry-run relocation / legacy alias phase では許容される場合がある。

canonical duplicate は、temporary dual canonical state として manifest に明示されていない限り NG とする。

---

## 8. placeholder relocation 検査

placeholder relocation は migration complete ではない。

placeholder relocation が残っている場合、以下を NG とする。

```text
- cleanup_ready と判定すること
- legacy alias 化すること
- canonical switch すること
- old path を削除対象にすること
```

---

## 9. sec_id continuity 検査

sec_id continuity は存在する sec_id のみを対象にする。

sec_id が存在しない文書は、欠落ではなく none / not_applicable として扱える。

validator は以下を検査する。

```text
- preserved sec_id が old / new で対応していること
- sec_id collision がないこと
- sec_mappings の old_sec_id / new_sec_id が形式整合していること
- sec_id missing を理由に spec 側で新規生成していないこと
```

---

## 10. cleanup readiness 判定

cleanup readiness は verification-driven とする。

cleanup_ready には以下が必要である。

```text
- semantic_equivalent = true
- mapping_status = verified
- placeholder relocation が残っていない
- legacy alias policy が適用済みまたは不要である
- sec_id continuity が確認済みまたは not_applicable である
- manifest federation の一意性検査が通過している
- canonical duplicate が解消済みまたは temporary として明示されている
- CI validation が通過している
```

---

## 11. cleanup blocked 判定

以下の場合は cleanup_blocked とする。

```text
- placeholder_only
- migrated_partial
- semantic_equivalent = unknown
- mapping_status が pending / partial
- manifest 未登録
- sec_id continuity 未確認
- canonical conflict 未解消
- root manifest から sub-manifest が辿れない
```

---

## 12. ordering 分離

validator は以下を分離して扱う。

```text
filesystem ordering:
ファイル名・フォルダ名の番号

dependency ordering:
Layer Index / dependency diagram / 本文定義

migration ordering:
manifest federation / migration graph

cleanup ordering:
cleanup gate / verification condition
```

番号は意味論を持たない。

---

## 13. 出力

validator は以下を出力する。

```text
- federation summary
- manifest list
- entry count
- migration state summary
- duplicate report
- placeholder report
- sec_id continuity report
- cleanup readiness report
- canonical conflict report
- orphan report
```

---

## 14. 判定結果

判定結果は以下とする。

```text
PASS:
cleanup / canonical switch に進める

WARN:
dry-run 継続可能だが、cleanup 不可

FAIL:
migration graph が不整合。cleanup / alias / canonical switch 禁止
```

---

## 15. HLDocS feedback

本 validator 構想から、HLDocS 側へ以下をフィードバックする。

```text
- manifest federation validator が必要
- duplicate は種類別に分類すべき
- placeholder relocation は正式 state として扱うべき
- cleanup readiness は verification-driven にすべき
- temporary dual canonical state を定義すべき
- filesystem ordering と migration / dependency / cleanup ordering を分離すべき
```

---

## 16. 結論

migration federation validator は、大規模再構成において cleanup や canonical switch を安全に行うための前提である。

validator は filesystem ordering ではなく、migration graph と verification condition を基準に判定する。
