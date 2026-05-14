# federation capability maturity model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation capability maturity model を定義する。

federation capability maturity model は、PoC / preview / operational / production federation の成熟段階を定義し、各段階で必要な validator、cleanup、governance、observability、release lifecycle の要件を整理する。

---

## 2. 基本方針

capability maturity model は以下を行う。

```text
- federation maturity stage を分類する
- 各 stage の required capability を定義する
- conceptual / experimental / operational boundary を明示する
- preview と production の違いを明示する
- migration roadmap の成熟度指標を提供する
```

capability maturity model は以下を行わない。

```text
- validator を代替しない
- release authority を代替しない
- production readiness を自動判定しない
- roadmap priority を自動決定しない
```

---

## 3. maturity stages

maturity stage は以下とする。

```text
M0 Conceptual
M1 Experimental
M2 Preview Federation
M3 Operational Federation
M4 Production Federation
M5 Long-term Governance Federation
```

---

## 4. M0 Conceptual

`M0 Conceptual` は、architecture / lifecycle / validator concept が定義されている段階である。

特徴：

```text
- conceptual model 中心
- implementation 不完全
- validator は部分的
- cleanup automation なし
- governance は文書中心
- federation graph は設計段階
```

許容：

```text
- manual operation
- draft schema
- incomplete observability
- unstable handoff contract
```

禁止：

```text
- production cleanup
- canonical release dependency
- destructive automation
```

---

## 5. M1 Experimental

`M1 Experimental` は、PoC validator / execution / orchestration が実装され始めた段階である。

特徴：

```text
- partial validator implementation
- dry-run execution available
- federation graph partial implementation
- reconstruction delta handling partially implemented
- draft observability available
```

許容：

```text
- experimental schema
- unstable API
- validator warn-heavy operation
- partial rollback support
```

必要：

```text
- audit trail prototype
- cleanup gate prototype
- governance role definition
```

---

## 6. M2 Preview Federation

`M2 Preview Federation` は、外部 feedback を受けられる federation preview 段階である。

特徴：

```text
- federation validator operational
- orchestration dry-run operational
- release lifecycle defined
- governance authority defined
- observability available
- cleanup consistency partially enforced
```

必要：

```text
- preview release lifecycle
- reconstruction delta rerun policy
- stale artifact detection
- handoff contract management
- federation dashboard projection
```

禁止：

```text
- irreversible destructive cleanup without manual approval
- canonical release from experimental dependency
```

---

## 7. M3 Operational Federation

`M3 Operational Federation` は、multi-project reconstruction が継続運用可能な段階である。

特徴：

```text
- federation validator stable
- orchestration execution protocol operational
- distributed checkpoint available
- rollback coordination available
- cleanup coordination operational
- governance decision traceability enforced
```

必要：

```text
- federation consistency validation
- recovery model operational
- stale acknowledgement handling
- orphaned execution handling
- CI integration
- release candidate flow
```

許容：

```text
- partial manual recovery
- optional eventual consistency outside cleanup scope
```

---

## 8. M4 Production Federation

`M4 Production Federation` は、production-level federation として安定運用可能な段階である。

特徴：

```text
- cleanup consistency enforced
- governance fully traceable
- observability and audit trail stable
- release lifecycle operational
- federation recovery operational
- distributed execution audited
```

必要：

```text
- strong cleanup consistency
- rollback consistency enforcement
- mandatory audit trail retention
- release authority enforcement
- federation execution observability
- stale artifact prevention in cleanup scope
```

禁止：

```text
- unresolved reconstruction delta in production release
- governance missing in destructive scope
- stale validator result in active cleanup scope
```

---

## 9. M5 Long-term Governance Federation

`M5 Long-term Governance Federation` は、長期保守・世代移行・ecosystem governance を扱える段階である。

特徴：

```text
- long-term auditability
- ecosystem-wide governance
- deprecation / obsolete orchestration
- migration lineage tracking
- federation policy evolution
- cross-generation compatibility governance
```

必要：

```text
- long-term audit retention
- governance evolution history
- compatibility lineage
- canonicalization history tracking
- federation-wide deprecation policy
```

---

## 10. capability matrix

```text
Capability                          M0 M1 M2 M3 M4 M5
Conceptual validator                ○  ○  ○  ○  ○  ○
Federation validator                -  △  ○  ○  ○  ○
Execution protocol                  -  △  △  ○  ○  ○
Cleanup coordination                -  -  △  ○  ○  ○
Recovery model                      -  △  △  ○  ○  ○
Governance model                    △  △  ○  ○  ○  ○
Observability                       -  △  ○  ○  ○  ○
Release lifecycle                   -  △  ○  ○  ○  ○
Consistency enforcement             -  -  △  ○  ○  ○
Long-term auditability              -  -  -  △  ○  ○
Cross-generation governance         -  -  -  -  △  ○
```

```text
○ = operational
△ = partial / prototype
- = not available
```

---

## 11. maturity downgrade

maturity stage は downgrade される場合がある。

例：

```text
- governance authority missing
- rollback consistency violation
- audit trail retention failure
- unresolved stale artifact in cleanup scope
- repeated manual recovery without traceability
```

Production release 中に downgrade condition が発生した場合、release lifecycle を再評価する。

---

## 12. reconstruction delta との関係

reconstruction delta は maturity stage に影響する。

例：

```text
major schema rewrite
↓
production federation → preview federation
```

experimental dependency が canonical boundary に混入した場合、maturity downgrade を検討する。

---

## 13. report schema draft

```json
{
  "schema_version": "1.0",
  "maturity_assessment_id": "maturity-YYYYMMDD-NNN",
  "target_scope": "SansaVRM federation",
  "current_stage": "M2 Preview Federation",
  "capabilities": {
    "federation_validator": "operational",
    "execution_protocol": "prototype",
    "cleanup_coordination": "partial"
  },
  "blocking_gaps": [],
  "recommended_next_stage": "M3 Operational Federation"
}
```

---

## 14. dashboard projection

Dashboard は maturity stage を表示できる。

表示対象：

```text
- current maturity stage
- capability gaps
- blocking gaps
- downgrade conditions
- recommended next stage
- unresolved governance / cleanup issues
```

Dashboard は maturity stage を source of truth として決定しない。

---

## 15. CI mapping

CI fail 条件：

```text
- production federation stage with missing governance authority
- production cleanup scope with stale artifact
- operational federation without required validator rerun
- release candidate without cleanup consistency
```

CI warn 条件：

```text
- preview federation with experimental dependency
- prototype observability
- partial rollback coordination
- manual recovery outside cleanup scope
```

---

## 16. 禁止事項

以下を禁止する。

```text
- M0/M1 state を production federation と称すること
- experimental dependency を canonical release に混入すること
- governance missing のまま production cleanup を行うこと
- audit trail missing のまま production release とすること
- maturity downgrade condition を無視すること
```

---

## 17. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction ecosystem には maturity model が必要
- preview / operational / production federation を分離すべき
- cleanup consistency / governance / observability は production readiness 条件に含めるべき
- maturity downgrade condition を formalize すべき
- experimental dependency を canonical boundary に混入させてはならない
```

---

## 18. 結論

federation capability maturity model は、SansaVRM federation を conceptual / experimental / preview / operational / production / long-term governance の成熟段階として整理するモデルである。

これにより、現在どこまで実装・運用できているか、何が production readiness を阻害しているかを traceable に管理できる。
