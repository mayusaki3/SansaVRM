# federation release lifecycle model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation release lifecycle model を定義する。

federation release lifecycle model は、document、schema、artifact、handoff contract、validator report、dashboard snapshot を、draft / preview / experimental / release_candidate / canonical_release / deprecated / obsolete の公開段階として管理する。

---

## 2. 基本方針

release lifecycle は以下を扱う。

```text
- release stage の分類
- draft / canonical boundary
- preview release の条件
- release candidate の条件
- canonical release の条件
- deprecation / obsolete の条件
- release authority / validation / audit trail の関係
```

release lifecycle は以下を行わない。

```text
- validator を代替しない
- governance approval を代替しない
- cleanup gate を迂回しない
- draft artifact を canonical として扱わない
- stale artifact を release 対象にしない
```

---

## 3. release stage

release stage は以下とする。

```text
draft
experimental
preview
release_candidate
canonical_release
deprecated
obsolete
superseded
```

---

## 4. draft

`draft` は、検討中・作成中の状態である。

特徴：

```text
- canonical ではない
- downstream contract として安定前提にしてはならない
- validator fail を許容する場合がある
- cleanup dependency に使う場合は cleanup_pending または cleanup_blocked
```

---

## 5. experimental

`experimental` は、PoC / 試験的実装用の状態である。

特徴：

```text
- 仕様変更を前提とする
- fixture / schema / artifact の破棄可能性がある
- federation validator では warn 扱いが基本
- canonical contract として扱わない
```

---

## 6. preview

`preview` は、外部 feedback を受けるために公開可能な状態である。

条件：

```text
- 主要 validator が fail ではない
- known limitations が明示されている
- draft / canonical boundary が明示されている
- governance decision が preview として承認している
- audit trail が存在する
```

preview は canonical release ではない。

---

## 7. release_candidate

`release_candidate` は canonical release 前の候補状態である。

条件：

```text
- project-local validator pass
- federation validator pass または applicable scope で pass
- cleanup consistency が成立
- handoff contract consistency が成立
- unresolved reconstruction delta がない
- release authority approval pending または approved
```

release_candidate は destructive cleanup の直前確認対象になり得る。

---

## 8. canonical_release

`canonical_release` は正本公開状態である。

条件：

```text
- release authority approved
- validator pass
- federation validator pass where applicable
- governance decision recorded
- audit trail recorded
- stale required artifact がない
- draft dependency が canonical boundary に混入していない
```

canonical_release は downstream project が安定前提として参照できる。

---

## 9. deprecated

`deprecated` は、互換維持しつつ将来廃止予定の状態である。

条件：

```text
- replacement target が存在する
- legacy alias または migration path が存在する
- downstream consumer へ通知可能である
- cleanup schedule または expiration policy が存在する
```

deprecated は obsolete ではない。

---

## 10. obsolete

`obsolete` は、利用終了・参照解除済みの状態である。

条件：

```text
- unresolved references = 0
- downstream dependency が残っていない
- cleanup gate validator が cleanup_ready を返している
- required alias expiration policy が成立している
```

obsolete 対象は cleanup execution の対象になり得る。

---

## 11. superseded

`superseded` は、後続 release / execution / reconstruction delta により置き換えられた状態である。

特徴：

```text
- active release 判定に使わない
- cleanup_ready 判定に使わない
- audit trail として保持する
- replacement reference が必要
```

---

## 12. release target kinds

release lifecycle の対象は以下とする。

```text
document
schema
artifact
handoff_contract
validator_report
dashboard_snapshot
execution_report
cleanup_report
```

Dashboard snapshot は release target になり得るが、source of truth ではない。

---

## 13. release promotion rules

主な昇格ルール：

```text
draft → experimental:
PoC 実行対象として明示

experimental → preview:
known limitations と validator status を明示

preview → release_candidate:
主要 validator pass / governance review 済

release_candidate → canonical_release:
release authority approval / audit trail / federation validation pass

canonical_release → deprecated:
replacement target と migration path を定義

deprecated → obsolete:
unresolved references = 0 / cleanup gate pass
```

---

## 14. release blocking conditions

以下は release を block する。

```text
- required validator fail
- federation validator fail in applicable scope
- governance decision missing
- release authority missing
- unresolved reconstruction delta
- stale required artifact
- draft dependency inside canonical boundary
- handoff contract pending in release scope
- manual_recovery_required unresolved
```

---

## 15. release record schema draft

```json
{
  "schema_version": "1.0",
  "release_record_id": "release-record-YYYYMMDD-NNN",
  "target_kind": "schema",
  "target_id": "sansavrm_adapter_input.schema",
  "release_stage": "preview",
  "authority_domain": "schema_authority",
  "validator_reports": [],
  "governance_decisions": [],
  "known_limitations": [],
  "supersedes": null
}
```

---

## 16. CI mapping

CI fail 条件：

```text
- canonical_release target に validator fail がある
- release_candidate に unresolved reconstruction delta がある
- canonical boundary に draft dependency が混入している
- stale required artifact が release scope にある
- governance decision missing
```

CI warn 条件：

```text
- preview に known limitations がある
- experimental artifact が downstream で参照されている
- deprecated target に downstream consumer が残っている
```

---

## 17. dashboard projection

Dashboard は release lifecycle を表示する。

表示対象：

```text
- release stage
- target kind
- validator status
- governance approval status
- known limitations
- blocking conditions
- deprecation / obsolete path
- superseded reference
```

Dashboard は release stage を独自変更してはならない。

---

## 18. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction result には draft / preview / release_candidate / canonical_release の lifecycle が必要
- preview は canonical release ではないと明記すべき
- draft dependency を canonical boundary に混入させてはならない
- release authority と validator pass は別条件として扱うべき
- deprecated と obsolete を分離すべき
```

---

## 19. 結論

federation release lifecycle model は、multi-project reconstruction の成果物を draft から canonical release、deprecated、obsolete まで段階管理するモデルである。

これにより、仕様・schema・artifact・handoff contract を、安定度と authority を明示した状態で公開・検証・廃止できる。
