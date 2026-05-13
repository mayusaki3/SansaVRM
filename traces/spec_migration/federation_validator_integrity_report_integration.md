# federation validator and integrity report integration

## 1. 目的

本ドキュメントは、`migration_federation_validator_draft` と `migration_integrity_check_report` の役割分担、および統合方針を整理する。

大規模仕様再配置では、検査仕様と検査結果を分離する必要がある。

---

## 2. 基本方針

以下の役割分担とする。

```text
migration_federation_validator_draft:
検査仕様・判定ルール・入力/出力形式を定義する

migration_integrity_check_report:
現時点のSansaVRM再配置状態に対する検査結果を記録する
```

---

## 3. ordering 分離

両文書で以下の ordering を分離する。

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

Layer番号は dependency 意味論ではなく filesystem ordering として扱う。

---

## 4. validator draft の責務

`migration_federation_validator_draft` は以下を定義する。

```text
- validator の入力
- manifest federation 構成
- migration state
- duplicate分類
- placeholder relocation 検査
- sec_id continuity 検査
- cleanup readiness 判定
- cleanup blocked 判定
- 出力形式
- PASS / WARN / FAIL 判定
```

---

## 5. integrity report の責務

`migration_integrity_check_report` は以下を記録する。

```text
- 現時点の manifest 状態
- duplicate path / semantic duplicate 状態
- duplicate doc_id 状態
- orphan specification 状態
- sec_id continuity 状態
- placeholder relocation 状態
- cleanup block list
- legacy alias 候補
- reorder readiness
```

---

## 6. 統合時の関係

将来的に validator を実装した場合、以下の関係とする。

```text
validator spec
  ↓
validator execution
  ↓
integrity report generation
```

つまり、integrity report は validator 出力の human-readable report として扱う。

---

## 7. report generation model

validator 実装時は、以下を出力する。

```text
- machine-readable JSON report
- human-readable Markdown report
- cleanup readiness summary
- failure reason list
- warning list
```

Markdown report が `migration_integrity_check_report` に相当する。

---

## 8. 状態遷移

状態遷移は以下とする。

```text
planned_relocation
  ↓
placeholder_relocation または full_copy_relocation
  ↓
migrated_partial または migrated_complete
  ↓
verified
  ↓
legacy_alias_ready
  ↓
cleanup_ready
```

ただし、placeholder_relocation が残る場合は cleanup_blocked とする。

---

## 9. canonical transition との関係

integrity report は canonical conflict の結果のみを記録する。

validator draft は canonical conflict の判定ルールを定義する。

分類：

```text
temporary_dual_canonical:
dry-run中に明示された一時状態

invalid_canonical_conflict:
明示されていない恒久的衝突

canonical_resolved:
legacy alias / canonical switch により解消済み
```

---

## 10. cleanup gate との関係

cleanup gate は validator が判定し、integrity report に結果を出力する。

```text
validator:
cleanup_ready / cleanup_blocked を判定

integrity report:
cleanup_ready / cleanup_blocked の理由を表示
```

---

## 11. HLDocS feedback

本統合方針から、HLDocS 側へ以下をフィードバックする。

```text
- 検査仕様と検査結果を分離すべき
- migration validator は machine-readable report と human-readable report を出力すべき
- integrity report は validator 出力として扱える
- cleanup gate は validator の判定対象にすべき
- canonical transition は validator rule と report result に分離すべき
```

---

## 12. 結論

`migration_federation_validator_draft` は検査仕様であり、`migration_integrity_check_report` は検査結果である。

今後は validator 仕様を整備し、その出力として integrity report を生成する構成へ整理する。
