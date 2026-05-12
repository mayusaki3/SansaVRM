# federation validator specification

## 1. 目的

本仕様は、SansaVRM 仕様再配置で使用する federation validator の責務、入力、検査内容、fail 条件を定義する。

federation validator は単一 manifest checker ではなく、migration federation 全体の graph integrity validator として動作する。

---

## 2. 基本方針

federation validator は以下を原則とする。

```text
1. validator は federation 全体を解決する
2. validator は path だけでなく graph を検査する
3. validator は sec_id continuity を検査する
4. validator は alias graph integrity を検査する
5. validator は verified migration 成立条件を検査する
```

---

## 3. 入力

validator は以下を入力として扱う。

```text
- root manifest
- sub-manifest
- migration entries
- doc_id_aliases
- sec_mappings
- cleanup classification
- alias documents
- specification paths
```

---

## 4. graph 構成要素

validator は以下を graph node / edge として扱う。

### node

```text
- document
- manifest
- section
- alias
- migration entry
```

### edge

```text
- relocation
- alias
- sec_id mapping
- split relocation
- merge relocation
- placeholder transition
```

---

## 5. validator responsibility

validator は以下を検査する。

```text
- federation resolution
- graph integrity
- alias integrity
- sec_id continuity
- migration state validity
- cleanup gate validity
- verified migration validity
- placeholder prohibition
```

---

## 6. federation resolution

validator は root manifest から sub-manifest を解決する。

検査内容：

```text
- required sub-manifest exists
- orphan manifest なし
- duplicate manifest_id なし
- invalid scope なし
```

---

## 7. path validation

validator は federation 全体で path を検査する。

fail 条件：

```text
- duplicate new.path
- unresolved target_path
- alias_target_path missing
```

semantic duplicate は fail としない。

---

## 8. doc_id validation

validator は federation 全体で doc_id を検査する。

fail 条件：

```text
- duplicate formal_doc_id
- unresolved alias
- alias loop
- dry-doc remains in release mode
```

---

## 9. sec_id validation

validator は sec_id graph を検査する。

fail 条件：

```text
- duplicate sec_id conflict
- orphan sec_id
- unresolved sec_mapping
- cleanup target with pending state
```

---

## 10. placeholder validation

validator は placeholder relocation を検査する。

fail 条件：

```text
- placeholder entry exists in release mode
- placeholder marked as verified
- placeholder cleanup attempted
```

---

## 11. alias validation

validator は alias document を検査する。

fail 条件：

```text
- alias_target_doc_id missing
- alias_target_path missing
- canonical_document=true on alias
- alias_document=false on alias
- old specification body remains
```

---

## 12. migration state validation

validator は migration lifecycle state を検査する。

許可される遷移：

```text
draft
→ placeholder
→ partial
→ semantic_verified
→ verified
→ aliased
→ archived
```

不正遷移は fail。

---

## 13. verified validation

validator は verified migration を検査する。

verified fail 条件：

```text
- unresolved alias exists
- placeholder exists
- pending sec_id exists
- federation validator dependency failure
- mapping_status != verified
```

---

## 14. cleanup validation

validator は cleanup gate を検査する。

fail 条件：

```text
- placeholder cleanup
- cleanup before verified
- cleanup before sec_id resolved
- cleanup before alias graph validation
```

---

## 15. release mode

validator は release mode を持つ。

release mode fail 条件：

```text
- dry-doc remains
- placeholder remains
- unresolved alias remains
- pending migration remains
- duplicate formal_doc_id exists
```

---

## 16. archive mode

archive mode では migration trace preservation を検査する。

fail 条件：

```text
- manifest trace deleted
- sec_mappings deleted
- alias graph deleted
```

---

## 17. CI Integration

CI は federation validator を実行できる。

推奨：

```text
pull request:
- federation validation
- alias validation
- sec_id validation

release:
- release mode validation
- verified migration validation
- cleanup gate validation
```

---

## 18. HLDocS feedback

本仕様で得られた知見：

```text
- migration validator は graph validator になる
- placeholder state validation が必要
- alias graph validation が必要
- sec_id graph validation が必要
- migration lifecycle validation が必要
- release mode と archive mode が必要
```

---

## 19. 結論

federation validator は、manifest federation、alias graph、sec_id continuity、migration lifecycle を統合的に検査する migration graph validator である。

verified migration、cleanup、release migration は federation validator success を前提とする。
