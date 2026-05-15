# SansaVRM Studio AI governance feedback

## 1. 目的

本ドキュメントは、SansaVRM 再構成から得られた governance / provenance / reconstruction に関する知見を、SansaVRM Studio AI 側へフィードバックするための整理である。

---

## 2. 重要認識

現在の再構成から、以下が重要と判明した。

```text
- provenance は単純 lineage では不足
- multi-parent derivation graph が必要
- restriction merge は危険領域
- editor rights は original author replacement ではない
- provenance/governance は Core semantic と分離すべき
- temporary bridge を source of truth にしてはならない
```

---

## 3. provenance は graph として扱うべき

Studio AI workflow では：

```text
A(body)
B(clothing)
C(hair)
↓ assembly
D(character)
```

が普通に発生する。

そのため：

```text
linear lineage chain
```

ではなく：

```text
multi-parent derivation graph
```

として扱う必要がある。

---

## 4. provenance と semantic identity を混同しない

重要：

```text
rights/provenance/governance
```

は：

```text
semantic identity
```

そのものではない。

そのため：

```text
Core Semantic Layer
```

へ直接入れすぎない方が良い。

推奨：

```text
Preservation
Validation
Distribution Governance
```

寄りで扱う。

---

## 5. restriction merge は review-required 前提が望ましい

例：

```text
Asset A:
commercial allowed

Asset B:
commercial prohibited
```

自動 merge / override は危険。

推奨：

```text
restriction merge result
restriction conflict
review required
```

中心で扱う。

---

## 6. editor rights は独立概念

assembly / conversion / modification により：

```text
editor
assembler
converter
modifier
```

が発生する。

重要：

```text
editor rights
```

は：

```text
original author replacement
```

ではない。

Studio AI 側でも：

```text
editor attribution
modification scope
redistribution rule
```

を独立管理した方が良い。

---

## 7. tool provenance は重要

AI workflow では：

```text
tool_name
tool_version
workflow_id
execution_mode
```

が distribution / rights へ影響する可能性がある。

そのため：

```text
tool provenance
```

は重要。

---

## 8. temporary bridge を source of truth にしない

例：

```text
legacy alias
temporary redirect
mapping table
compatibility bridge
```

これらは：

```text
temporary bridge
```

であり：

```text
canonical provenance
```

ではない。

cleanup 前に整理する必要がある。

---

## 9. tamper prevention より tamper detection

SansaVRM は file format であるため：

```text
perfect tamper prevention
```

は困難。

そのため：

```text
tamper detection
integrity validation
provenance chain validation
```

中心で考える方が現実的。

---

## 10. reconstruction governance と provenance governance は接続される

重要：

```text
mixed reconstruction
partial overwrite
temporary dual state
```

では：

```text
provenance contamination
```

も発生しうる。

そのため：

```text
reconstruction governance
```

と：

```text
provenance governance
```

は将来的に接続される。

---

## 11. distribution-ready は provenance validation を要求する

推奨：

```text
unresolved provenance chain
restriction conflict unresolved
editor attribution missing
```

の状態では：

```text
distribution-ready
```

にしない。

---

## 12. 将来 validator 候補

候補：

```text
provenance_validator
restriction_merge_validator
editor_rights_validator
tool_provenance_validator
integrity_validator
```

MVP では reason code / dashboard 表示でもよい。

---

## 13. reconstruction completion と provenance

重要：

```text
new structure generated
```

だけでは reconstruction completed ではない。

比較調査・cleanup・traceability review が必要。

provenance についても：

```text
provenance equivalence
restriction equivalence
editor attribution continuity
```

などを将来的に comparison workflow に含める可能性がある。

---

## 14. 現時点推奨

現時点推奨：

```text
- provenance/governance を Core semantic へ直接入れすぎない
- graph-based provenance を前提にする
- restriction merge は review-required 前提にする
- temporary bridge を source of truth にしない
- provenance validation を distribution governance と接続する
```

---

## 15. 結論

SansaVRM Studio AI workflow は、単純 asset lineage ではなく multi-parent derivation / assembly / restriction merge / editor rights を扱う governance problem に近い。

そのため、provenance/governance を semantic identity と分離しつつ、Validation / Distribution Governance 側で provenance chain validation と integrity validation を行う方向が望ましい。
