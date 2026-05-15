# rights provenance governance model

## 1. 目的

本ドキュメントは、SansaVRM における rights provenance / governance model を定義する。

本 model は、Studio AI workflow を含む以下を対象とする。

```text
- AI generated asset
- imported asset
- converted asset
- assembled asset
- derived asset
- distribution package
```

特に：

```text
複数 SansaVRM component を assembly し、
新しい SansaVRM asset を生成する workflow
```

を前提とする。

---

## 2. 基本方針

rights provenance governance は以下を扱う。

```text
- provenance chain
- acquisition provenance
- component provenance
- derivation provenance
- conversion provenance
- editor rights
- restriction merge
- tool provenance
- integrity / tamper detection
```

rights provenance governance は以下を行わない。

```text
- 完全な tamper prevention
- 法律上の権利判定
- 自動 license conflict resolution
- 自動 restriction override
- governance approval の代替
```

---

## 3. layer positioning

重要：

```text
rights / provenance / governance
```

は原則として：

```text
Core Semantic Layer
```

へ直接入れない。

主配置候補：

```text
Preservation Compatibility Layer
Validation Layer
future Distribution Governance Layer
```

理由：

```text
rights / provenance / governance
```

は：

```text
semantic identity
```

そのものではなく、distribution / inheritance / validation concern に近いため。

---

## 4. provenance taxonomy

provenance は以下に分類する。

```text
rights_provenance
acquisition_provenance
component_provenance
derivation_provenance
conversion_provenance
policy_provenance
tool_provenance
review_provenance
```

---

## 5. rights provenance

rights provenance は、どの経路で取得・生成・派生したかを扱う。

候補：

```text
self_created
purchase
download
ai_generated
converted
assembled
derived
captured
scanned
```

rights provenance は lineage だけではなく acquisition route を扱う。

---

## 6. acquisition provenance

acquisition provenance は取得元情報を扱う。

候補：

```text
acquisition_method
acquisition_location
acquisition_timestamp
rights_holder
license_snapshot
```

acquisition_location 例：

```text
Booth URL
VRoid Hub URL
GitHub URL
SansaVRM package ID
```

license_snapshot は acquisition 時点の snapshot として扱う。

---

## 7. component provenance

component provenance は assembly 時の component origin を扱う。

例：

```text
hair:
  source=SansaVRM_A

clothing:
  source=SansaVRM_B
```

component provenance は：

```text
- source_asset
- source_component
- merge_rule
- derived_component
```

を扱う。

---

## 8. derivation provenance

derivation provenance は派生 chain を扱う。

ただし重要：

```text
linear chain
```

だけでは不十分。

Studio AI workflow では：

```text
multi-parent derivation graph
```

が必要。

例：

```text
A(body)
B(clothing)
C(hair)
↓ assembly
D(character)
```

つまり：

```text
multiple semantic origins
```

を扱う必要がある。

---

## 9. conversion provenance

conversion provenance は format / workflow conversion を扱う。

例：

```text
VRM → SansaVRM
SansaVRM → FBX
FBX → SansaVRM
```

保持候補：

```text
source_format
target_format
conversion_tool
conversion_version
conversion_timestamp
conversion_loss_note
```

---

## 10. policy provenance

policy provenance は restriction / governance の変更履歴を扱う。

候補：

```text
restriction merge
review result change
distribution state change
commercial rule change
attribution rule change
```

policy provenance は immutable audit trail に近い。

---

## 11. editor rights

assembly / conversion / modification により：

```text
editor
assembler
converter
modifier
```

が発生する。

例：

```text
body:
  original_author=A

clothing:
  original_author=B

assembly:
  editor=C
```

editor rights は以下を扱う。

```text
editor_id
modification_scope
redistribution_rule
commercial_rule
attribution_rule
```

重要：

```text
editor rights
```

は：

```text
original author replacement
```

ではない。

---

## 12. restriction merge

assembly 時、複数 restriction が merge される。

例：

```text
Asset A:
commercial allowed

Asset B:
commercial prohibited
```

この場合：

```text
merged restriction governance
```

が必要。

重要：

```text
automatic override
```

は危険。

MVP では：

```text
restriction merge result
restriction conflict
review required
```

を扱う方向が望ましい。

---

## 13. tool provenance

tool provenance は生成・編集・変換 workflow を扱う。

例：

```text
generated_by:
  tool=SansaVRM Studio AI
  version=0.x.x
```

保持候補：

```text
tool_name
tool_version
workflow_id
execution_mode
generated_timestamp
```

重要：

```text
AI workflow
```

では tool provenance が rights / distribution に影響する可能性がある。

---

## 14. review provenance

review provenance は governance review を扱う。

候補：

```text
reviewer
review_scope
review_timestamp
review_result
review_reason
```

review provenance は distribution governance と接続される可能性がある。

---

## 15. tamper detection

SansaVRM は file format であるため：

```text
perfect tamper prevention
```

は困難。

そのため重視するのは：

```text
tamper detection
integrity validation
provenance chain validation
```

である。

---

## 16. integrity validation

integrity validation は以下を扱う。

```text
- provenance chain consistency
- missing source provenance
- invalid merge provenance
- editor rights inconsistency
- restriction merge inconsistency
- provenance graph break
```

integrity validation は Validation Layer concern に近い。

---

## 17. provenance graph

rights provenance は graph structure を持つ。

特に：

```text
multi-parent derivation
```

を扱う必要がある。

候補 graph edge：

```text
acquired_from
converted_from
assembled_from
derived_from
modified_from
reviewed_from
```

---

## 18. provenance chain validation

provenance chain validation は以下を検査する。

```text
- unresolved source
- invalid provenance edge
- cycle detection
- missing acquisition provenance
- restriction conflict unresolved
- editor attribution missing
```

unresolved provenance chain は distribution-ready にしてはならない。

---

## 19. governance positioning

rights provenance governance は以下に接続される。

```text
distribution governance
restriction inheritance
review traceability
assembly governance
conversion governance
federation provenance validation
```

将来的には：

```text
runtime royalty
commercial restriction
usage fee
```

などとも接続される可能性がある。

---

## 20. reconstruction interaction

rights provenance governance は reconstruction governance とも接続される。

例：

```text
old provenance
new provenance
partial overwrite provenance
mixed provenance scope
```

mixed reconstruction では provenance contamination に注意する必要がある。

---

## 21. validator positioning

将来 validator 候補：

```text
provenance_validator
restriction_merge_validator
editor_rights_validator
tool_provenance_validator
integrity_validator
```

MVP では reason code / dashboard / registry レベルでもよい。

---

## 22. dashboard display

Dashboard は provenance / governance を表示できる。

候補：

```text
- provenance graph
- restriction merge state
- review required state
- editor rights summary
- tool provenance summary
- integrity warnings
```

Dashboard は provenance を source of truth と混同してはならない。

---

## 23. CI mapping

CI fail 候補：

```text
- unresolved provenance chain
- invalid provenance edge
- unresolved restriction conflict
- missing editor attribution in required scope
- invalid provenance graph cycle
```

CI warn 候補：

```text
- provenance incomplete outside distribution scope
- review pending
- partial provenance snapshot
```

---

## 24. 禁止事項

以下を禁止する。

```text
- rights provenance を Core semantic identity と混同すること
- editor rights を original author replacement と扱うこと
- automatic restriction override
- unresolved provenance chain を distribution-ready と扱うこと
- temporary provenance bridge を canonical provenance と扱うこと
```

---

## 25. HLDocS feedback candidate

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- provenance/governance は semantic identity と分離すべき
- reconstruction governance と provenance governance は接続される
- multi-parent derivation graph を扱う必要がある
- temporary bridge provenance を source of truth として扱うべきではない
- integrity validation と provenance validation を分離すべき
```

---

## 26. 結論

rights provenance governance model は、SansaVRM における acquisition / assembly / derivation / conversion / restriction / editor rights / tool provenance を扱う governance model である。

特に、Studio AI workflow における multi-parent derivation graph と restriction merge を扱うことで、distribution governance / provenance validation / integrity validation の基盤を形成する。
