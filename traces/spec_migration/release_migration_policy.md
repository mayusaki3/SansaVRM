# release migration policy

## 1. 目的

本ポリシーは、SansaVRM 仕様再配置における release 前の migration 安定化条件を定義する。

release migration policy は、仕様公開前に migration graph が安定していることを確認するための gate として扱う。

---

## 2. 基本方針

release は単なる仕様公開ではなく、migration stabilization boundary として扱う。

以下を原則とする。

```text
1. release 前に federation validator を通過する
2. release 前に verified migration を成立させる
3. release 前に placeholder relocation を残さない
4. release 前に unresolved alias を残さない
5. release 前に pending / unknown sec_id を残さない
6. release 前に dry-doc を formal_doc_id として残さない
```

---

## 3. release mode

release mode は federation validator の厳格モードである。

release mode では以下を fail とする。

```text
- placeholder remains
- partial migration remains
- unresolved alias remains
- pending sec_id state remains
- unknown sec_id state remains
- dry-doc remains as formal_doc_id
- duplicate formal_doc_id exists
- required sub-manifest missing
```

---

## 4. release 前必須条件

release 前に以下を満たす。

```text
- all required manifests resolved
- federation validator success
- all release target entries verified
- formal_doc_id issued
- doc_id_aliases registered
- sec_id states resolved
- cleanup classification complete
- legacy alias policy decided
```

---

## 5. placeholder 禁止

release 対象には placeholder relocation を含めない。

placeholder が残る場合、その対象は release scope から除外するか、全文移行を完了する。

---

## 6. dry-doc 禁止

release 対象の formal_doc_id に `dry-doc-*` を使用してはならない。

`dry-doc-*` は dry-run / placeholder / temporary verification 専用である。

---

## 7. sec_id 解決

release 対象の sec_id state は以下のいずれかでなければならない。

```text
preserved
remapped
newly_assigned
not_required
```

以下は禁止する。

```text
pending
unknown
```

---

## 8. alias graph 解決

release 前に alias graph は解決済みでなければならない。

検査内容：

```text
- alias_target_doc_id exists
- alias_target_path exists
- alias loopなし
- unresolved aliasなし
- legacy alias化対象が明示されている
```

---

## 9. cleanup classification

release 前に旧path文書は以下へ分類済みでなければならない。

```text
keep_in_common
migrated_complete
migrated_partial
placeholder_only
legacy_alias_candidate
obsolete_candidate
```

分類なしの旧path文書を残してはならない。

---

## 10. release scope

release scope は以下を明示する。

```text
- release対象Layer
- release対象文書
- excluded placeholder
- excluded partial migration
- legacy alias candidate
```

---

## 11. release後の状態

release後は以下を保持する。

```text
- migration manifest
- sub-manifest
- doc_id alias graph
- sec_id mapping graph
- cleanup classification
- verified migration report
```

これらは audit trail として扱う。

---

## 12. release禁止条件

以下が存在する場合、releaseしてはならない。

```text
- federation validator failure
- verified migration failure
- placeholder in release scope
- dry-doc formal_doc_id
- pending sec_id state
- unresolved alias
- duplicate formal_doc_id
- missing cleanup classification
```

---

## 13. CI Requirements

release workflow は以下を実行する。

```text
- federation validator --release
- sec_id continuity check
- alias graph check
- cleanup classification check
- dry-doc formalization check
```

---

## 14. HLDocS feedback

本ポリシーで得られた知見：

```text
- release は migration stabilization boundary として扱うべき
- dry-run identity と formal identity は分離すべき
- placeholder は release scope から除外すべき
- release前に cleanup classification が必要
- migration manifest は release後も audit trail として保持すべき
```

---

## 15. 結論

release migration policy は、仕様公開前に migration graph を安定化させるための gate である。

release では verified migration、formal_doc_id、sec_id resolution、alias graph integrity、federation validator success を必須とする。
