# migration archive policy

## 1. 目的

本ポリシーは、SansaVRM 仕様再配置における archive 状態の意味、保持対象、禁止事項、検証条件を定義する。

archive は migration trace の削除ではなく、migration history preservation state として扱う。

---

## 2. 基本方針

archive は migration 完了後の履歴保持状態である。

以下を原則とする。

```text
1. archive 後も migration trace を保持する
2. archive 後も sec_id mapping を保持する
3. archive 後も alias graph を保持する
4. archive 後も verified report を保持する
5. archive は cleanup 後の audit trail として扱う
```

---

## 3. archive の意味

archive は以下を意味する。

```text
- migration lifecycle が完了している
- cleanup decision が完了している
- release 後の履歴として保持されている
- historical traceability が維持されている
```

archive は以下を意味しない。

```text
- manifest 削除
- alias graph 削除
- sec_id mapping 削除
- migration evidence 削除
```

---

## 4. archive 前提条件

archive には以下が必要である。

```text
- verified migration success
- release migration policy pass
- cleanup decision approved
- legacy alias maintenance period completed
- federation validator archive mode pass
```

---

## 5. archive 保持対象

archive 後も以下を保持する。

```text
- root manifest
- sub-manifest
- doc_id alias graph
- sec_id mapping graph
- cleanup classification
- verified migration criteria result
- release migration result
- legacy alias history
- lifecycle history
```

---

## 6. archive 禁止事項

archive 時に以下を行ってはならない。

```text
- migration manifest deletion
- sec_mappings deletion
- alias graph deletion
- verified report deletion
- cleanup classification deletion
```

---

## 7. archive mode validation

federation validator は archive mode を持つ。

archive mode では以下を検査する。

```text
- manifest trace exists
- sub-manifest refs resolvable
- alias graph retained
- sec_id mapping retained
- cleanup decision recorded
- lifecycle state archived
```

---

## 8. archive fail 条件

以下が存在する場合、archive mode は fail とする。

```text
- manifest trace missing
- required sub-manifest missing
- sec_mappings missing
- alias graph missing
- lifecycle history missing
- cleanup decision missing
```

---

## 9. release と archive の違い

release と archive は異なる。

```text
release:
運用可能な安定状態

archive:
移行履歴を保存した完了状態
```

release は operational state であり、archive は historical state である。

---

## 10. cleanup と archive の関係

cleanup は archive の前段である。

```text
verified
↓
aliased
↓
cleanup_candidate
↓
cleanup decision
↓
archived
```

cleanup により旧pathが削除または alias 化されても、migration trace は削除してはならない。

---

## 11. alias history

legacy alias を削除する場合でも、alias history は保持する。

保持内容：

```text
- legacy_path
- target_path
- legacy_doc_id
- target_doc_id
- alias active period
- cleanup decision
```

---

## 12. sec_id history

sec_id mapping は archive 後も保持する。

理由：

```text
- tests / code / diagnostics との historical traceability を維持するため
- 旧仕様から新仕様への検証経路を再構築できるようにするため
```

---

## 13. audit trail

archive は audit trail として扱う。

以下を再構築可能でなければならない。

```text
- どの文書がどこへ移動したか
- どの sec_id がどこへ対応したか
- どの旧pathが alias 化されたか
- どの文書が cleanup されたか
- release 時点で何が verified だったか
```

---

## 14. CI Requirements

CI は archive mode を検査できる。

推奨 check：

```text
federation-validator --archive
```

fail 条件：

```text
- archive target without verified history
- cleanup decision missing
- alias history missing
- sec_id history missing
```

---

## 15. HLDocS feedback

本ポリシーで得られた知見：

```text
- archive は削除ではなく履歴保持状態である
- migration manifest は temporary artifact ではなく audit trail である
- sec_id mapping は archive後も保持すべき
- alias graph は historical traceability として保持すべき
- release と archive は分離すべき
```

---

## 16. 結論

migration archive は、仕様再配置の履歴を保持し、将来の traceability 再構築を可能にするための完了状態である。

archive 後も manifest federation、alias graph、sec_id mapping、cleanup decision、verified report を保持する。
