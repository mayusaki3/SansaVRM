# federation governance model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation governance model を定義する。

federation governance model は、multi-project reconstruction における approval authority、canonical authority、cleanup owner、rollback owner、handoff contract authority、draft / canonical boundary を整理する。

---

## 2. 基本方針

federation governance は以下を扱う。

```text
- approval authority
- canonical authority
- cleanup ownership
- rollback ownership
- handoff contract authority
- draft / canonical authority boundary
- cross-project decision responsibility
- manual recovery responsibility
```

federation governance は以下を行わない。

```text
- validator 判定を代替しない
- execution protocol を代替しない
- repository owner の承認を省略しない
- downstream project の責務を upstream project が勝手に決定しない
```

---

## 3. authority domains

authority domain は以下に分離する。

```text
canonical_authority
schema_authority
artifact_authority
validation_authority
cleanup_authority
rollback_authority
handoff_contract_authority
release_authority
```

1つの project が複数 authority を持ってよい。

ただし、authority domain は明示されなければならない。

---

## 4. canonical authority

canonical authority は、canonical_doc_id、canonical schema、canonical semantic boundary を決定する権限である。

SansaVRM 本体仕様に関する canonical authority は SansaVRM project が持つ。

他 project は draft / proposal / feedback を提示できるが、SansaVRM canonical identity を直接確定できない。

---

## 5. schema authority

schema authority は、schema の canonical / draft / experimental 状態を決定する権限である。

例：

```text
SansaVRM:
canonical Adapter input boundary schema authority

MuJoCo Adapter:
draft adapter fixture / adapter-local schema authority

Studio AI:
Studio AI fixture / workflow schema authority
```

他 project の draft schema を canonical schema として扱ってはならない。

---

## 6. artifact authority

artifact authority は、生成 artifact の鮮度・正当性・再生成責任を持つ。

対象例：

```text
- Adapter fixture
- diagnostics.json
- conversion_report.json
- updated_extension_properties.json
- Studio AI fixture
- export profile sample
```

artifact producer project が primary artifact authority を持つ。

consumer project は artifact validation result を持てるが、producer artifact の正本性を勝手に変更しない。

---

## 7. validation authority

validation authority は validator result の責任を持つ。

```text
project-local validator:
project-local authority

federation validator:
federation-level consistency authority
```

federation validator は project-local validator を代替しない。

project-local validator pass だけでは federation validation pass を意味しない。

---

## 8. cleanup authority

cleanup authority は destructive cleanup を承認する責任を持つ。

```text
project-local cleanup:
対象 repository owner / project owner

federation cleanup coordination:
federation orchestration owner
```

project-local cleanup_ready は federation cleanup approval を意味しない。

federation cleanup approval は project-local cleanup approval を代替しない。

---

## 9. rollback authority

rollback authority は rollback 実行と rollback 結果確認の責任を持つ。

```text
project-local rollback:
各 project owner

federation rollback coordination:
federation orchestration owner
```

federation protocol は他 project の rollback を直接実行しない。

rollback_failed の場合、manual_recovery_owner を明示する。

---

## 10. handoff contract authority

handoff contract authority は、project 間 contract の合意・更新・失効を管理する。

handoff contract には以下を明示する。

```text
- producer authority
- consumer authority
- accepted assumptions
- pending decisions
- draft / canonical status
- expiration / superseded condition
- cleanup dependency
```

handoff pending のまま downstream cleanup を承認してはならない。

---

## 11. release authority

release authority は、reconstruction result を release / preview / draft として扱う権限である。

release authority は以下を確認する。

```text
- validator pass
- federation validator pass
- cleanup consistency
- handoff contract consistency
- unresolved reconstruction delta がない
```

release authority は validator を代替しない。

---

## 12. governance decision record

governance decision は traceable に記録する。

schema draft：

```json
{
  "governance_decision_id": "governance-decision-YYYYMMDD-NNN",
  "decision_kind": "cleanup_approval",
  "authority_domain": "cleanup_authority",
  "owner_project": "SansaVRM",
  "decision_status": "approved",
  "related_validator_reports": [],
  "related_handoff_contracts": []
}
```

---

## 13. approval matrix

```text
対象                         必要 authority
canonical_doc_id decision      canonical_authority
canonical schema publication   schema_authority
adapter fixture regeneration   artifact_authority
project-local cleanup          cleanup_authority
federation cleanup             cleanup_authority + federation orchestration owner
project-local rollback         rollback_authority
federation rollback            rollback_authority + federation orchestration owner
handoff contract acceptance    handoff_contract_authority
release publication            release_authority
```

---

## 14. draft / canonical boundary

Draft artifact は canonical artifact として扱ってはならない。

Draft を canonical に昇格する条件：

```text
- schema_authority approval
- validation pass
- handoff contract update
- downstream compatibility check
- federation validator pass where applicable
```

Draft のまま cleanup dependency に入る場合は cleanup_blocked または cleanup_pending とする。

---

## 15. manual recovery responsibility

manual_recovery_required の場合、manual recovery owner を明示する。

記録対象：

```text
- failure kind
- affected project
- responsible authority domain
- recovery action request
- required validation after recovery
```

manual recovery owner 未定義の failure は federation completed にできない。

---

## 16. CI mapping

CI fail 条件：

```text
- required authority decision missing
- cleanup approval missing
- rollback owner missing
- handoff contract authority mismatch
- draft artifact treated as canonical
- manual recovery owner missing
```

CI warn 条件：

```text
- optional governance decision pending
- draft artifact outside cleanup scope
- release authority pending outside release scope
```

---

## 17. dashboard projection

Dashboard は governance state を表示する。

表示対象：

```text
- authority domain
- owner project
- approval status
- pending decisions
- handoff authority state
- cleanup approval state
- rollback owner state
- manual recovery owner
```

Dashboard は governance decision を実行しない。

---

## 18. 禁止事項

以下を禁止する。

```text
- federation approval だけで project-local approval を省略すること
- project-local validator pass を federation validator pass とみなすこと
- draft schema を authority approval なしに canonical とすること
- handoff pending のまま destructive cleanup を承認すること
- rollback owner 未定義のまま rollback_required を解消すること
```

---

## 19. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- multi-project reconstruction には governance model が必要
- approval authority / cleanup owner / rollback owner を明示すべき
- draft / canonical authority boundary を定義すべき
- handoff contract authority を traceable に管理すべき
- governance decision は validator result を代替しないと明記すべき
```

---

## 20. 結論

federation governance model は、multi-project reconstruction における権限・承認・責任境界を定義する governance model である。

これにより、technical validation / execution / recovery と、誰が承認・所有・復旧責任を持つかを分離して管理できる。
