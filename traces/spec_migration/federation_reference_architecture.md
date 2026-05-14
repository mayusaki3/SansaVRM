# federation reference architecture

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における federation reference architecture を定義する。

federation reference architecture は、validator、orchestration、execution protocol、state machine、consistency、recovery、governance、observability、release lifecycle、maturity model を統合し、SansaVRM federation の参照構造を示す。

---

## 2. 基本方針

federation reference architecture は以下を行う。

```text
- subsystem の責務境界を整理する
- project-local architecture と federation architecture を分離する
- observer / planner / executor / authority / audit の役割を分離する
- cleanup / release / governance / observability を federation level に接続する
- HLDocS 共通仕様化に向けた抽象化候補を整理する
```

federation reference architecture は以下を行わない。

```text
- subsystem の詳細実装を置き換えない
- validator を executor として扱わない
- dashboard を source of truth として扱わない
- governance approval を validator pass と同一視しない
```

---

## 3. architecture layers

federation reference architecture は以下の layer で構成する。

```text
L1 Source / Project Layer
L2 Index / Manifest Layer
L3 Validation Layer
L4 Orchestration Layer
L5 Execution Protocol Layer
L6 Consistency / State Layer
L7 Recovery Layer
L8 Governance Layer
L9 Observability / Audit Layer
L10 Release / Maturity Layer
```

---

## 4. L1 Source / Project Layer

Source / Project Layer は、各 project の repository、schema、artifact、document、report を保持する。

対象例：

```text
- SansaVRM
- SansaVRM-MuJoCo-Adapter
- SansaVRM Studio AI
- future adapter project
- future export pipeline project
```

各 project は project-local ownership を持つ。

federation は project-local repository を直接所有しない。

---

## 5. L2 Index / Manifest Layer

Index / Manifest Layer は machine-readable index と manifest を提供する。

主な構成：

```text
- migration manifest federation
- canonicalization manifest
- rewrite transaction plan
- external_artifact_index
- hash_index
- federation graph
- orchestration graph
```

この layer は source of truth を正規化するが、state mutation は行わない。

---

## 6. L3 Validation Layer

Validation Layer は observer / gate として動作する。

主な構成：

```text
- manifest validator
- canonicalization validator
- rewrite validator
- cleanup gate validator
- integrity / tamper validator
- federation validator
- dashboard projection validator
```

Validation Layer は rewrite / cleanup / approval を実行しない。

---

## 7. L4 Orchestration Layer

Orchestration Layer は execution planning と coordination planning を行う。

主な構成：

```text
- migration orchestration graph
- validator orchestration
- federation orchestration engine
- downstream action request planning
- rerun planning
- cleanup coordination planning
- rollback coordination planning
```

Orchestration Layer は計画を作るが、project-local repository を直接変更しない。

---

## 8. L5 Execution Protocol Layer

Execution Protocol Layer は実行手順と handshake を扱う。

主な構成：

```text
- canonicalization execution engine
- cleanup execution plan
- federation execution protocol
- distributed checkpoint
- approval boundary
- execution token
- project-local execution report
```

Execution Protocol Layer は validator / governance / audit の結果を参照する。

---

## 9. L6 Consistency / State Layer

Consistency / State Layer は状態遷移と整合性を扱う。

主な構成：

```text
- federation state machine
- federation transaction consistency model
- strong consistency boundary
- eventual consistency boundary
- cleanup consistency
- rollback consistency
- artifact freshness consistency
```

この layer は、allowed / blocked / forbidden transition を定義する。

---

## 10. L7 Recovery Layer

Recovery Layer は failure 後の復旧を扱う。

主な構成：

```text
- federation recovery model
- recovery action request
- stale acknowledgement handling
- orphaned execution handling
- diverged cleanup state handling
- manual recovery required
```

Recovery Layer は project-local recovery ownership を尊重する。

---

## 11. L8 Governance Layer

Governance Layer は authority / approval / ownership を扱う。

主な構成：

```text
- federation governance model
- canonical authority
- schema authority
- cleanup authority
- rollback authority
- handoff contract authority
- release authority
- governance decision record
```

Governance decision は validator result を代替しない。

---

## 12. L9 Observability / Audit Layer

Observability / Audit Layer は監視・監査・trace correlation を扱う。

主な構成：

```text
- federation observability model
- append-only event model
- audit trail
- telemetry
- freshness observability
- governance observability
- dashboard projection
```

Dashboard は projection であり、source of truth ではない。

---

## 13. L10 Release / Maturity Layer

Release / Maturity Layer は公開段階と運用成熟度を扱う。

主な構成：

```text
- federation release lifecycle model
- federation capability maturity model
- draft / experimental / preview / release_candidate / canonical_release
- deprecated / obsolete / superseded
- M0 Conceptual ～ M5 Long-term Governance Federation
```

Release decision は validator pass、governance approval、audit trail、federation consistency を前提とする。

---

## 14. main data flow

主な data flow：

```text
Source / Project
  ↓
Index / Manifest
  ↓
Validation
  ↓
Orchestration
  ↓
Execution Protocol
  ↓
Validation Rerun
  ↓
State / Consistency
  ↓
Governance / Release
  ↓
Observability / Audit
```

cleanup と release はこの flow の途中結果だけでは実行しない。

cleanup には cleanup gate / cleanup consistency / governance approval が必要である。

release には release authority / audit trail / federation validation が必要である。

---

## 15. responsibility separation

```text
Observer:
validator / dashboard projection validator

Planner:
orchestration engine / validator orchestration

Executor:
canonicalization execution engine / cleanup execution / project-local execution

Authority:
governance model / release authority / cleanup authority

Audit:
observability / audit trail
```

これらを混同してはならない。

---

## 16. project-local vs federation

```text
project-local:
repository ownership / local validator / local cleanup / local rollback

federation:
cross-project consistency / handoff / stale artifact / federation cleanup / coordination
```

project-local pass は federation pass を意味しない。

federation approval は project-local approval を代替しない。

---

## 17. cleanup reference architecture

cleanup の参照流れ：

```text
project-local validator pass
  ↓
cleanup gate validator pass
  ↓
federation validator pass
  ↓
federation cleanup consistency pass
  ↓
governance cleanup approval
  ↓
dry-run cleanup
  ↓
project-local cleanup execution
  ↓
post-cleanup validation
  ↓
audit trail update
```

---

## 18. release reference architecture

release の参照流れ：

```text
release target selected
  ↓
validator pass
  ↓
federation validator pass where applicable
  ↓
governance decision recorded
  ↓
audit trail recorded
  ↓
release lifecycle stage updated
  ↓
dashboard projection generated
```

---

## 19. reconstruction delta reference architecture

reconstruction delta の参照流れ：

```text
delta detected
  ↓
impact analysis
  ↓
state invalidation
  ↓
validator rerun planning
  ↓
replacement execution planning
  ↓
federation propagation
  ↓
cleanup_ready reevaluation
  ↓
audit trail update
```

---

## 20. HLDocS abstraction candidates

HLDocS 共通仕様へ抽象化できる候補：

```text
- reconstruction delta handling
- manifest / index / validator separation
- cleanup gate / cleanup execution separation
- dashboard projection vs audit trail separation
- governance authority model
- release lifecycle model
- maturity model
- federation consistency model
```

---

## 21. 禁止事項

以下を禁止する。

```text
- validator を executor として扱うこと
- dashboard を source of truth として扱うこと
- release authority を validator pass で代替すること
- project-local cleanup_ready を federation_cleanup_ready とみなすこと
- draft artifact を canonical dependency として扱うこと
- audit trail なしに destructive operation を完了扱いすること
```

---

## 22. 結論

federation reference architecture は、SansaVRM federation の validator、orchestration、execution、consistency、recovery、governance、observability、release、maturity を統合する参照アーキテクチャである。

これにより、SansaVRM の大規模仕様再構成を、単一 repository migration ではなく multi-project semantic reconstruction platform として扱える。
