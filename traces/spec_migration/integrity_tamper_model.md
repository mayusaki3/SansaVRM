# integrity tamper model

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成および将来の asset lifecycle における integrity / tamper model を定義する。

改ざん検出情報は、semantic identity、integrity proof、validation result を混同しない形で扱う。

本モデルは、Core Semantic Layer、Preservation Compatibility Layer、Validation Layer の責務境界を明確にする。

---

## 2. 基本方針

改ざん検出情報は、Core semantic に直接格納しない。

ただし、改ざん検出の対象となる canonical semantic identity は Core semantic に属する。

以下を分離する。

```text
identity:
何であるかを示す semantic identity

integrity proof:
その identity または representation が改ざんされていないことを示す証跡

validation result:
integrity proof を検証した結果
```

---

## 3. Layer responsibility

### Core Semantic Layer

Core Semantic Layer は、改ざん検出の対象となる semantic identity を保持する。

保持対象：

```text
- canonical semantic identity
- canonical semantic graph
- semantic dependency
- normalized semantic projection target
```

非保持対象：

```text
- representation hash
- signature metadata
- provenance chain
- validator result
- trust score
- tamper diagnostics
```

### Preservation Compatibility Layer

Preservation Compatibility Layer は、integrity proof と provenance を保持する。

保持対象：

```text
- integrity descriptors
- normalized semantic hash
- representation hash
- archive hash
- signature metadata
- provenance chain
- import/export chain
- transformation history
```

### Validation Layer

Validation Layer は、integrity proof の検証結果を保持する。

保持対象：

```text
- tamper validation result
- signature verification result
- integrity diagnostics
- trust diagnostics
- mismatch reason
- warning / failure classification
```

---

## 4. integrity classes

integrity は以下に分類する。

```text
semantic_integrity
representation_integrity
package_integrity
provenance_integrity
signature_integrity
runtime_integrity
```

### semantic_integrity

canonical semantic projection に対する integrity。

semantic-preserving rewrite に耐える必要がある。

### representation_integrity

JSON、glTF、VRM、MJCF、FBX、MMD などの具体 representation に対する integrity。

whitespace、ordering、encoding、compression、export setting の違いで変化し得る。

### package_integrity

配布 archive、bundle、外部依存ファイルを含む package に対する integrity。

### provenance_integrity

作成者、変換履歴、export/import chain、adapter chain に対する integrity。

### signature_integrity

署名、証明書、署名対象範囲、署名アルゴリズムに対する integrity。

### runtime_integrity

runtime 読み込み後の状態、adapter 出力、検証結果に対する integrity。

---

## 5. normalized semantic hash

normalized semantic hash は、canonical semantic projection に対して計算する hash である。

目的：

```text
- representation rewrite に耐える
- canonicalization 後も semantic equivalence を確認する
- semantic tamper を検出する
```

normalized semantic hash の対象は、Core semantic の projection である。

hash 値そのものは Preservation Compatibility Layer に保持する。

---

## 6. representation hash

representation hash は、特定ファイルまたは byte representation に対する hash である。

対象例：

```text
- JSON file hash
- VRM file hash
- glTF file hash
- MJCF file hash
- archive hash
```

representation hash は representation concern であり、Core semantic identity ではない。

representation hash の不一致は、直ちに semantic non-equivalence を意味しない。

---

## 7. signature metadata

signature metadata は Preservation Compatibility Layer に保持する。

保持例：

```text
- signature_id
- signature_algorithm
- signed_target_kind
- signed_target_id
- signer
- signed_at
- certificate_reference
- signature_value
```

signature verification result は Validation Layer に保持する。

署名値そのものを Core semantic equivalence の条件にしてはならない。

---

## 8. provenance chain

provenance chain は Preservation Compatibility Layer に保持する。

保持例：

```text
- source_format
- source_tool
- source_adapter
- import_timestamp
- transformation_steps
- export_profile
- export_timestamp
- adapter_artifacts
```

provenance は semantic identity ではない。

ただし、semantic equivalence 判断や trust evaluation の補助情報として使用してよい。

---

## 9. validation result

Validation Layer は integrity / tamper validation の結果を保持する。

例：

```json
{
  "integrity_validation": {
    "status": "warn",
    "reason": "representation hash changed but normalized semantic hash matched",
    "semantic_integrity": "pass",
    "representation_integrity": "warn",
    "signature_integrity": "not_applicable"
  }
}
```

status の許容値：

```text
pass
warn
fail
blocked
not_applicable
```

---

## 10. semantic equivalence との関係

semantic equivalence は normalized semantic projection に基づいて判定する。

representation hash の不一致のみで semantic_equivalent=false にしてはならない。

semantic_equivalent=false とする候補：

```text
- normalized semantic hash mismatch
- canonical semantic graph mismatch
- required semantic dependency missing
- semantic role changed
- topology semantic changed beyond allowed threshold
```

representation mismatch は、Validation Layer の warning または fail として扱う。

---

## 11. canonicalization / rewrite との関係

canonicalization / rewrite 中は representation が変化する可能性がある。

そのため、以下を分離して検証する。

```text
pre_rewrite_semantic_hash
post_rewrite_semantic_hash
pre_rewrite_representation_hash
post_rewrite_representation_hash
```

判定方針：

```text
semantic hash match + representation hash mismatch:
semantic-preserving rewrite として扱える

semantic hash mismatch:
semantic tamper または semantic rewrite として要判断

representation hash match + semantic hash mismatch:
projection error または semantic index error として fail 候補
```

---

## 12. cross-project artifacts との関係

### MuJoCo Adapter

Adapter artifacts、conversion_report、diagnostics、updated_extension_properties は、Core semantic へ直接戻さない。

これらは以下に分類する。

```text
adapter artifacts:
Preservation / Runtime concern

conversion_report:
Validation Layer concern

diagnostics:
Validation Layer concern

updated_extension_properties:
Runtime / Import Export boundary concern
```

updated_extension_properties に semantic candidate が含まれる場合は、Core へ直接反映せず、rewrite / review / validation を通す。

### Studio AI

Studio AI の authoring/export/distribution 状態、policy rewrite、mesh filtering、bake pipeline は、Core semantic ではなく Runtime / Export / Preservation / Validation concern として扱う。

ただし、normalized semantic projection の対象にする Core semantic candidate は、canonicalization 前に分離判断する。

---

## 13. schema draft

```json
{
  "integrity": {
    "schema_version": "1.0",
    "target_identity": {
      "canonical_doc_id": "doc-example",
      "semantic_target_id": "semantic-target-example"
    },
    "semantic_integrity": {
      "normalized_projection_id": "projection-example",
      "hash_algorithm": "sha256",
      "normalized_semantic_hash": "..."
    },
    "representation_integrity": {
      "representation_kind": "json",
      "hash_algorithm": "sha256",
      "representation_hash": "..."
    },
    "signature": {
      "signature_id": "sig-example",
      "signature_algorithm": "example",
      "signed_target_kind": "normalized_semantic_projection",
      "signature_value": "..."
    },
    "provenance": {
      "source_format": "vrm",
      "source_tool": "example-tool",
      "transformation_steps": []
    }
  }
}
```

---

## 14. validator requirements

integrity / tamper validator は以下を検査する。

```text
- target_identity が Core semantic identity に解決できる
- normalized projection が再生成可能である
- normalized semantic hash が一致する
- representation hash が対象 representation と一致する
- signature target が存在する
- signature verification result が記録されている
- provenance chain が循環していない
- transformation history が rewrite transaction と矛盾しない
- validation result が dashboard state と矛盾しない
```

---

## 15. dashboard との関係

Dashboard は integrity / tamper validation を以下として表示する。

```text
integrity_state:
pass | warn | fail | blocked | not_applicable

tamper_state:
not_checked | no_tamper_detected | semantic_mismatch | representation_mismatch | signature_invalid | provenance_broken
```

Dashboard は integrity proof を変更しない。

Dashboard は validation result を表示するだけである。

---

## 16. cleanup gate との関係

cleanup gate では、semantic integrity を優先して確認する。

cleanup_ready 条件候補：

```text
- semantic_integrity = pass または not_applicable
- representation_integrity = pass または warn または not_applicable
- signature_integrity = pass または warn または not_applicable
- provenance_integrity = pass または warn または not_applicable
- integrity validator が fail ではない
```

semantic_integrity=fail の場合、cleanup_ready にしてはならない。

---

## 17. 禁止事項

以下を禁止する。

```text
- representation hash を Core semantic identity として扱うこと
- signature metadata を semantic equivalence の必須条件にすること
- provenance chain を Core semantic として扱うこと
- validator result を Core semantic に格納すること
- representation hash mismatch のみで semantic_equivalent=false と断定すること
- updated_extension_properties を review / rewrite / validation なしに Core へ反映すること
```

---

## 18. cross-project handoff への回答方針

MuJoCo Adapter / Studio AI への回答では、以下を明示する。

```text
- 改ざん検出の対象 identity は Core semantic に属する
- 改ざん検出情報そのものは Preservation に保持する
- 検証結果は Validation Layer に保持する
- Runtime / Export 由来の integrity 情報は Core へ直接戻さない
- Core へ入れる候補は canonicalization / rewrite / validation を通す
```

---

## 19. HLDocS feedback

本モデルから、HLDocS 側へ以下をフィードバックする。

```text
- semantic identity と integrity proof と validation result を分離すべき
- normalized semantic hash と representation hash を区別すべき
- signature / provenance は Core semantic ではなく Preservation concern として扱うべき
- integrity validation result は Validation concern として扱うべき
- representation rewrite を semantic tamper と誤判定しない仕組みが必要
```

---

## 20. 結論

integrity / tamper model では、改ざん検出の対象となる canonical semantic identity を Core semantic に置き、integrity proof を Preservation Compatibility Layer、validation result を Validation Layer に分離する。

これにより、semantic-preserving rewrite、canonicalization、legacy alias、cross-project adapter artifacts を扱いながら、改ざん検出と semantic equivalence を両立できる。
