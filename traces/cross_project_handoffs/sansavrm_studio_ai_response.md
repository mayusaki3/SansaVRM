# SansaVRM Studio AI フィードバックへの回答

## 1. 目的

本ドキュメントは、SansaVRM Studio AI フィードバック分析に対する SansaVRM 側の回答を記録する。

本回答は、SansaVRM 側で整理済みの Layer 責務、canonicalization / rewrite / integrity / validation 方針を前提とし、Studio AI 側の要件候補をどの Layer で扱うかを明確化する。

---

## 2. 回答時点の前提

SansaVRM 側では、以下の整理を追加済みである。

```text
- canonicalization_manifest_schema
- canonicalization_report_format
- cleanup_gate_dependency_graph
- rewrite_transaction_model
- dashboard_state_schema
- federation_validator_implementation_model
- canonicalization_execution_plan
- integrity_tamper_model
- sansavrm_mujoco_adapter_response
```

本回答では、以下を前提とする。

```text
- Core semantic へ入れるのは canonical semantic identity に関わるものに限定する
- policy / provenance / integrity proof / diagnostics は Core semantic へ直接入れない
- integrity は identity / proof / validation result に分離する
- authoring / validation / distribution / export は同一 concern として扱わない
- Runtime / Export 由来の transformation は review / rewrite / validation を通す
```

---

## 3. 全体回答

Studio AI 側から提示された要件候補は、SansaVRM 側で以下に分類して扱う。

```text
Core Semantic Layer:
canonical semantic identity に関わる最小限の構造

Preservation Compatibility Layer:
policy / restriction / provenance / source_raw / preserve_only / integrity proof

Validation Layer:
topology quality / diagnostics / integrity validation result / trust diagnostics

Runtime Integration Layer:
runtime compatibility / rigging compatibility / platform execution concern

Import Export Layer:
export profiles / transformation / filtering / bake pipeline / representation generation
```

---

## 4. Core semantic へ入れるもの

### 4.1 layered asset structure

以下は Core semantic 候補として採用する。

```text
- body
- clothing
- accessory
- hidden
```

理由：

```text
- avatar semantic structure に関わる
- 着せ替え semantic に関わる
- export filtering の semantic target になる
- hidden mesh bake の semantic target になる
```

ただし、各 Layer に付随する distribution policy / visibility rule / platform rule は Core semantic へ入れない。

---

### 4.2 authoring / distribution / export state separation

以下の状態分離は Core semantic の周辺概念として採用する。

```text
authoring state
distribution state
export state
```

ただし、状態値そのものを Core semantic にすべて保持するのではなく、Core semantic は state separation の semantic boundary を定義する。

実際の流通条件、viewer warning、marketplace rule、platform policy は Preservation / Runtime / Export 側へ分離する。

---

### 4.3 reversible / irreversible export state

reversible / irreversible export は、semantic transformation 判定に関わるため、Core semantic から参照可能な transformation property として扱う。

ただし、export profile の詳細や platform target は Core semantic へ入れない。

整理：

```text
Core Semantic Layer:
reversible / irreversible の semantic meaning

Import Export Layer:
export profile ごとの具体変換

Validation Layer:
roundtrip / loss / preserve_only 判定
```

---

## 5. Core semantic へ直接入れないもの

以下は Core semantic へ直接入れない。

```text
- adult_content 等の policy / restriction
- distribution_restricted
- viewer_warning
- marketplace rule
- AI pipeline detail
- conversion history detail
- topology_quality result
- validator diagnostics
- representation hash
- signature metadata
- trust score
- export profile detail
- mesh filtering rule
- bake pipeline detail
```

これらは semantic identity そのものではなく、保持・検査・流通・変換 concern である。

---

## 6. Preservation Compatibility Layer へ入れるもの

Preservation Compatibility Layer では、以下を扱う。

```text
- policy / restriction metadata
- provenance / generation history
- source_raw
- preserve_only
- adapter raw
- unsupported information
- integrity proof
- normalized semantic hash
- representation hash
- signature metadata
- import/export chain
- transformation history
```

重要：

```text
integrity proof は Preservation に置く
```

ただし、integrity proof の検証結果は Validation Layer に置く。

---

## 7. Validation Layer へ入れるもの

Validation Layer では、以下を扱う。

```text
- topology_quality
- rigging validation result
- humanoid compatibility validation
- integrity validation result
- tamper validation result
- signature verification result
- validator diagnostics
- trust diagnostics
- conversion diagnostics
```

Studio AI 側の品質評価や AI 生成結果の不完全性は、まず Validation Layer の diagnostics として扱う。

Core semantic へ戻す候補がある場合は、review / rewrite / validation を通す。

---

## 8. Runtime Integration Layer へ入れるもの

Runtime Integration Layer では、以下を扱う。

```text
- rigging runtime compatibility
- humanoid runtime compatibility
- platform runtime requirement
- viewer/runtime execution condition
- runtime-specific adapter contract
```

rigging_state は Core semantic ではなく、Runtime Integration Layer と Validation Layer の境界 concern として扱う。

---

## 9. Import Export Layer へ入れるもの

Import Export Layer では、以下を扱う。

```text
- export_profiles
- reversible / irreversible export execution
- policy rewrite
- metadata filtering
- mesh filtering
- hidden mesh bake
- material merge
- platform export target
```

これらは transformation concern であり、Core semantic そのものではない。

---

## 10. integrity / tamper detection の扱い

Studio AI 側の integrity / tamper detection 要件は、以下のように扱う。

```text
Core Semantic Layer:
改ざん検出対象となる canonical semantic identity

Preservation Compatibility Layer:
integrity descriptors / normalized semantic hash / representation hash / signature metadata / provenance chain

Validation Layer:
tamper validation result / signature verification result / integrity diagnostics
```

重要：

```text
identity
≠
integrity proof
≠
validation result
```

representation hash mismatch のみで semantic_equivalent=false とは判定しない。

normalized semantic hash mismatch は semantic tamper / semantic rewrite / projection failure の候補として扱う。

---

## 11. policy / restriction の扱い

policy / restriction は Core semantic へ直接入れない。

分類：

```text
Preservation Compatibility Layer:
policy / restriction metadata の保持

Import Export Layer:
export profile ごとの policy rewrite / filtering

Validation Layer:
policy validation / diagnostics

Runtime Integration Layer:
runtime platform restriction の判定
```

これにより、distribution policy の変更が Core semantic identity を破壊しないようにする。

---

## 12. provenance / AI generation history の扱い

AI pipeline、generation history、conversion history は provenance として扱う。

分類：

```text
Preservation Compatibility Layer:
provenance chain / generation history / source_raw / transformation history

Validation Layer:
provenance validation / trust diagnostics
```

AI workflow の詳細は semantic identity ではない。

ただし、復元、監査、流通、trust evaluation のために保持してよい。

---

## 13. authoring / validation / distribution / export lifecycle

Studio AI の要件は、SansaVRM が以下の lifecycle を扱う方向で整理する。

```text
authoring state
  ↓
validation state
  ↓
distribution state
  ↓
export state
```

ただし、この lifecycle 全体を Core semantic へ入れるのではなく、Layer を分けて扱う。

```text
authoring boundary:
Core semantic / Preservation

validation boundary:
Validation Layer

distribution boundary:
Preservation / Runtime

export boundary:
Import Export / Runtime
```

---

## 14. reversible / irreversible export の扱い

reversible / irreversible export は、semantic transformation と roundtrip 判定に関わるため、SansaVRM 側で扱う。

ただし責務は分離する。

```text
Core Semantic Layer:
reversible / irreversible の意味論

Preservation Compatibility Layer:
preserve_only / source_raw / transformation history

Import Export Layer:
実際の export transformation

Validation Layer:
loss / roundtrip / irreversible diagnostics
```

---

## 15. canonicalization 前に整理する事項

Studio AI 要件のうち、以下は canonicalization 前に分類判断を行う。

```text
- layered asset structure の Core 採用範囲
- authoring/distribution/export state separation の semantic boundary
- reversible/irreversible export の意味論
- policy / restriction の Preservation 分類
- provenance / AI history の Preservation 分類
- integrity / tamper model の Layer 分離
- export_profiles の Import Export Layer 分類
```

これらは cleanup 後に後付けすると、semantic equivalence と Layer dependency を壊す可能性がある。

---

## 16. Studio AI 側で前提にしてよいこと

Studio AI 側では、以下を前提にしてよい。

```text
- layered asset structure は Core semantic 候補として扱う
- authoring/distribution/export separation は SansaVRM 側で扱う
- policy / restriction は Core ではなく Preservation / Export / Validation concern とする
- provenance / AI generation history は Preservation concern とする
- integrity proof は Preservation concern とする
- integrity validation result は Validation concern とする
- export_profiles / filtering / bake pipeline は Import Export concern とする
- Runtime compatibility は Runtime Integration / Validation concern とする
```

---

## 17. Studio AI 側への注意事項

以下は注意事項とする。

```text
- policy 情報を Core semantic identity として前提化しない
- AI pipeline detail を Core semantic identity として前提化しない
- representation hash を semantic equivalence の唯一条件にしない
- export profile の結果を Core semantic として直接戻さない
- bake / filtering 後の情報を review / rewrite / validation なしに正本扱いしない
```

---

## 18. SansaVRM 側の次タスク

SansaVRM 側では、以下を後続タスクとして扱う。

```text
1. layered asset structure の Core semantic 仕様化
2. authoring/distribution/export state separation の仕様化
3. reversible/irreversible export semantic の仕様化
4. policy / restriction metadata の Preservation 仕様化
5. provenance / generation history の Preservation 仕様化
6. integrity / tamper model の schema 化
7. export_profiles / transformation pipeline の Import Export 仕様化
8. Studio AI 側 fixture / workflow との再突合
```

---

## 19. 回答まとめ

```text
Coreへ入れるもの:
layered asset structure、state separation の semantic boundary、reversible/irreversible の意味論

Preservationへ入れるもの:
policy/restriction、provenance、source_raw、preserve_only、integrity proof

Validationへ入れるもの:
topology_quality、integrity validation、validator diagnostics、trust diagnostics

Runtime/Exportへ入れるもの:
export_profiles、policy rewrite、mesh filtering、bake pipeline、runtime compatibility

Coreへ直接入れないもの:
policy detail、provenance detail、hash/signature、diagnostics、export profile detail
```

---

## 20. 結論

SansaVRM 側では、Studio AI からのフィードバックを受領し、Core semantic / Preservation Compatibility / Validation / Runtime Integration / Import Export の責務分離として反映する。

Studio AI 側は、authoring / distribution / export lifecycle を SansaVRM が扱う前提で進めてよい。

ただし、policy、provenance、integrity proof、diagnostics、export transformation は Core semantic へ直接入れず、Layer 分離したうえで canonicalization / rewrite / validation を通す。
