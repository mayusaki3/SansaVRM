# cleanup gate dependency graph

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再配置 dry-run における cleanup gate の依存関係を整理する。

cleanup は filesystem ordering ではなく verification-driven により判定する。

---

## 2. 基本方針

cleanup は relocation とは別フェーズとする。

cleanup 可否は Layer番号やファイル番号では判定しない。

cleanup 可否は以下の状態により判定する。

```text
- migration manifest 登録状態
- semantic_equivalent
- mapping_status
- placeholder relocation state
- legacy alias state
- sec_id continuity
- canonical conflict state
- CI validation state
```

---

## 3. ordering 分離

cleanup gate では以下を分離する。

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

## 4. cleanup gate dependency diagram

```text
migration manifest registered
  ↓
semantic_equivalent checked
  ↓
mapping_status complete or verified
  ↓
placeholder relocation resolved
  ↓
sec_id continuity checked or not_applicable
  ↓
legacy alias policy applied or not_required
  ↓
canonical conflict resolved or temporary state declared
  ↓
CI validation passed
  ↓
cleanup_ready
```

---

## 5. cleanup blocked diagram

```text
placeholder_only
  ↓
cleanup_blocked

migrated_partial
  ↓
cleanup_blocked

semantic_equivalent unknown
  ↓
cleanup_blocked

mapping_status pending or partial
  ↓
cleanup_blocked

manifest missing
  ↓
cleanup_blocked

canonical conflict unresolved
  ↓
cleanup_blocked
```

---

## 6. cleanup_ready 条件

cleanup_ready は以下をすべて満たす場合のみ成立する。

```text
- manifest に登録されている
- semantic_equivalent = true
- mapping_status = verified
- placeholder relocation が残っていない
- sec_id continuity が確認済みまたは not_applicable
- legacy alias が適用済みまたは不要
- canonical conflict が解消済みまたは temporary として明示済み
- federation validator が PASS
- CI validation が PASS
```

---

## 7. cleanup_blocked 条件

以下のいずれかに該当する場合は cleanup_blocked とする。

```text
- placeholder_only
- migrated_partial
- semantic_equivalent = unknown
- mapping_status = pending
- mapping_status = partial
- manifest 未登録
- sec_id continuity 未確認
- canonical conflict 未解消
- federation validator が FAIL
- CI validation が FAIL
```

---

## 8. legacy alias との関係

legacy alias は cleanup 前の互換フェーズである。

legacy alias readiness は以下で判定する。

```text
- semantic_equivalent = true
- mapping_status = verified
- placeholder relocation がない
- canonical path が確定している
- migration_entry_id が存在する
```

legacy alias が必要な文書では、cleanup_ready の前に legacy alias phase を通過する。

---

## 9. placeholder relocation との関係

placeholder relocation は migration complete ではない。

placeholder relocation が残っている場合、以下は禁止する。

```text
- cleanup_ready 判定
- legacy alias 化
- canonical switch
- old path 削除
```

---

## 10. canonical transition との関係

大規模 dry-run relocation 中は temporary dual canonical state が発生し得る。

ただし、恒久状態としては canonical conflict を解消する必要がある。

cleanup 前に以下を確認する。

```text
- canonical source が明示されている
- legacy alias 側は canonical_document=false へ移行可能
- new path 側が canonical_document=true として verified 済み
```

---

## 11. sec_id continuity との関係

sec_id continuity は存在する sec_id のみを対象とする。

sec_id が存在しない文書は none / not_applicable として扱える。

cleanup gate では以下を確認する。

```text
- preserved sec_id が manifest に記録されている
- sec_id collision がない
- sec_id missing を理由に spec 側で新規生成していない
```

---

## 12. HLDocS feedback

本 dependency graph から、HLDocS 側へ以下をフィードバックする。

```text
- cleanup は relocation と別フェーズにすべき
- cleanup ordering は filesystem ordering ではなく verification-driven にすべき
- placeholder relocation は cleanup_blocked state として扱うべき
- legacy alias phase を cleanup 前に定義すべき
- canonical transition state を扱うべき
- federation validator を cleanup gate に含めるべき
```

---

## 13. 結論

cleanup gate は、大規模再構成後の旧path削除・legacy alias 化・canonical switch を安全に判断するための検証ゲートである。

cleanup gate は Layer番号やファイル番号に依存せず、migration graph と verification condition に依存する。
