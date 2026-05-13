# SansaVRM Studio AI フィードバック分析

## 1. 目的

本ドキュメントは、SansaVRM Studio AI 側から提示された要件候補を、SansaVRM 本体仕様へ取り込む際の分類整理を行う。

特に以下を区別する。

```text
- Core semantic として本体へ入れるべき内容
- Preservation / Compatibility として保持すべき内容
- Validation Layer 側で扱うべき内容
- Runtime / Export profile 側で扱うべき内容
- authoring state と distribution state の分離
```

---

## 2. 重要な整理

今回の要件は、大きく以下へ分離される。

```text
A. Core semantic
B. Preservation / Compatibility
C. Validation / Integrity
D. Authoring / Distribution workflow
E. Export profile / transformation
F. Provenance / policy / restriction
```

特に重要なのは：

```text
SansaVRM Core へ直接入れるべき内容
```

と、

```text
保持・流通・変換・検証のために保持する内容
```

を分離する点である。

---

## 3. Core semantic に入れる候補

以下は、SansaVRM の semantic identity に関わるため、Core semantic 候補とする。

### 3.1 layered asset 構造

```text
- body
- clothing
- accessory
- hidden
```

理由：

```text
- avatar semantic structure に関わる
- export filtering の基礎になる
- hidden mesh bake の基礎になる
- 着せ替え semantic を定義する
```

ただし、各 Layer の詳細 policy までは Core semantic に入れない。

---

### 3.2 authoring / distribution state separation

以下の状態区分は Core semantic 候補とする。

```text
- authoring state
- distribution state
- export state
```

理由：

```text
- reversible / irreversible export 判定に関わる
- export profile の意味論に関わる
- roundtrip semantic に関わる
```

---

### 3.3 reversible / irreversible export flag

以下は semantic transformation として Core semantic 候補とする。

```json
{
  "export": {
    "reversible": false
  }
}
```

理由：

```text
- roundtrip semantic に関わる
- preserve_only 判定に関わる
- validator 判定に関わる
```

---

## 4. Preservation / Compatibility 側へ入れる候補

以下は Core semantic ではなく、保持・互換・流通・復元のために扱う。

### 4.1 policy / restriction

```text
- adult_content
- violent_content
- grotesque_content
- distribution_restricted
- viewer_warning
```

理由：

```text
- semantic identity ではない
- distribution / viewer / marketplace concern
- platform policy concern
```

これは Preservation Compatibility Layer 候補とする。

---

### 4.2 provenance / generation history

```text
- AI pipeline
- conversion history
- generation history
```

理由：

```text
- source provenance concern
- semantic identity ではない
- validator / marketplace / recovery concern
```

これは Preservation Compatibility Layer 候補とする。

---

### 4.3 source_raw / preserve_only

以下は Preservation Compatibility の中心責務とする。

```text
- adapter raw
- unsupported information
- future recovery
- roundtrip support
```

MuJoCo / robotics / XR / AI workflow との互換保持にも利用する。

---

## 5. Validation Layer 側へ入れる候補

### 5.1 topology_quality

```json
{
  "topology_quality": {
    "manifold": false,
    "watertight": false,
    "uv_valid": true
  }
}
```

これは Validation Layer 側候補とする。

理由：

```text
- validator concern
- runtime suitability concern
- AI quality diagnostics concern
```

---

### 5.2 rigging_state

```json
{
  "rigging_state": {
    "bone_state": "none",
    "humanoid_compatible": false
  }
}
```

これは Validation Layer と Runtime Integration Layer の中間候補とする。

理由：

```text
- runtime compatibility
- avatar platform compatibility
- humanoid validation
```

---

### 5.3 integrity / tamper detection

```json
{
  "integrity": {
    "policy_hash": "...",
    "restriction_hash": "...",
    "signature": "...",
    "validator_version": "..."
  }
}
```

これは Validation Layer 側候補とする。

理由：

```text
- validator concern
- marketplace concern
- upload pipeline concern
- semantic identity ではない
```

---

## 6. Export profile / transformation 側候補

### export_profiles

```json
{
  "export_profiles": {
    "authoring": {},
    "all_ages_distribution": {},
    "adult_distribution": {},
    "vrchat_upload": {}
  }
}
```

これは Import Export Layer と Runtime Integration Layer の中間候補とする。

理由：

```text
- export transformation concern
- platform target concern
- runtime optimization concern
```

---

### policy rewrite support

以下は Export transformation concern として扱う。

```text
- metadata filtering
- policy rewrite
- mesh delete
- hidden mesh bake
- material merge
```

これは Core semantic へ直接入れない。

---

## 7. 重要な境界整理

### Core semantic へ直接入れすぎない

特に以下は注意する。

```text
- distribution policy
- marketplace rule
- viewer warning
- AI provenance detail
- validator diagnostics
```

これらは semantic identity ではない。

---

### authoring state と distribution state を分離する

これはかなり重要である。

```text
authoring:
制作状態

validation:
検査状態

distribution:
流通状態
```

これらを同一 semantic として扱わない。

---

### AI workflow を Preservation concern として扱う

AI生成 pipeline は semantic identity そのものではなく、provenance / recovery / compatibility concern として扱う。

---

## 8. 現時点の推奨分類

### Core semantic 候補

```text
- layered asset structure
- authoring/distribution/export state separation
- reversible/irreversible export state
```

### Preservation Compatibility 候補

```text
- policy/restriction
- provenance
- source_raw
- preserve_only
- adapter raw
```

### Validation Layer 候補

```text
- topology_quality
- integrity
- validator diagnostics
```

### Runtime / Export profile 候補

```text
- export_profiles
- policy rewrite
- mesh filtering
- bake pipeline
```

---

## 9. migration / canonicalization への影響

今回の要件は、現在進行中の migration / canonicalization に影響する。

特に：

```text
- Core semantic と Preservation concern の分離
- Validation concern の分離
- Export profile concern の分離
```

は、Layer dependency と canonicalization policy に影響する。

そのため、cleanup / canonicalization 実施前に分類整理を優先する。

---

## 10. 結論

SansaVRM Studio AI 側の要件は、そのまま Core semantic へ直接投入するのではなく、

```text
Core semantic
Preservation Compatibility
Validation
Runtime/Export transformation
```

へ責務分離して取り込む必要がある。

特に policy / provenance / integrity / export profile は semantic identity そのものではなく、保持・検査・流通・変換 concern として扱う。
