# reconstruction comparison workflow

## 1. 目的

本ドキュメントは、SansaVRM の大規模仕様再構成における reconstruction comparison workflow を定義する。

reconstruction comparison workflow は、旧構成と新構成を比較し、cleanup candidate / cleanup blocker / reconstruction completion を判定する workflow である。

重要：

```text
new structure generated
```

だけでは reconstruction completed ではない。

比較調査と cleanup readiness review を経る必要がある。

---

## 2. 基本方針

comparison workflow は以下を扱う。

```text
- old/new comparison
- semantic equivalence
- structural equivalence
- traceability equivalence
- rewrite completeness
- contamination detection
- orphan detection
- cleanup blocker generation
- cleanup candidate generation
```

comparison workflow は以下を行わない。

```text
- semantic equivalence を path equality と混同しない
- rewrite executed を rewrite validated と扱わない
- comparison 前に old structure を削除しない
- cleanup_ready を comparison なしで発行しない
```

---

## 3. comparison stages

comparison stages は以下とする。

```text
C1 scope freeze
C2 structural comparison
C3 semantic comparison
C4 traceability comparison
C5 rewrite completeness comparison
C6 contamination comparison
C7 orphan detection
C8 cleanup blocker generation
C9 cleanup candidate generation
C10 reconstruction completion review
```

---

## 4. C1 scope freeze

比較開始前に comparison scope を freeze する。

freeze 対象：

```text
- old structure scope
- new structure scope
- mixed scope
- comparison target list
- cleanup hold scope
- unresolved reconstruction delta
```

scope freeze なしに comparison を開始してはならない。

---

## 5. C2 structural comparison

structural comparison は以下を比較する。

```text
- document count
- section count
- path mapping
- canonical_doc_id mapping
- migration mapping
- index coverage
```

重要：

```text
structural mismatch
```

だけで semantic failure と断定してはならない。

---

## 6. C3 semantic comparison

semantic comparison は以下を比較する。

```text
- semantic continuity
- lifecycle meaning
- governance meaning
- validator meaning
- cleanup semantics
- provenance semantics
```

重要：

```text
path changed
section split
layer relocation
```

だけで semantic failure としてはならない。

重要なのは：

```text
semantic meaning preserved
```

かどうかである。

---

## 7. equivalence levels

equivalence は以下に分類する。

```text
structural_equivalent
semantic_equivalent
traceability_equivalent
cleanup_equivalent
non_equivalent
```

### structural_equivalent

構造差異がほぼない。

### semantic_equivalent

構造差異はあるが semantic continuity が維持されている。

### traceability_equivalent

traceability mapping が reconstruction 後も維持されている。

### cleanup_equivalent

cleanup 後も reconstruction safety を損なわない。

### non_equivalent

重要 semantic が失われている。

---

## 8. C4 traceability comparison

比較対象：

```text
- doc_id mapping
- sec_id mapping
- ref_id mapping
- testspec linkage
- code linkage
- provenance linkage
```

検査：

```text
- missing mapping
- duplicate mapping
- orphan traceability
- unresolved reference
- temporary bridge dependency
```

traceability mismatch unresolved は cleanup blocker とする。

---

## 9. C5 rewrite completeness comparison

比較対象：

```text
- identity rewrite completeness
- sec_id rewrite completeness
- traceability rewrite completeness
- reference rewrite completeness
- path rewrite completeness
- alias rewrite completeness
```

partial rewrite は cleanup blocker とする。

---

## 10. C6 contamination comparison

検査：

```text
- old reference leakage
- new draft leakage
- mixed namespace collision
- temporary dual state unresolved
- stale canonical evidence
- provenance contamination
```

contamination unresolved は cleanup_hold 条件である。

---

## 11. C7 orphan detection

orphan は以下を含む。

```text
- orphan document
- orphan section
- orphan traceability
- orphan provenance edge
- orphan reference
- orphan cleanup target
```

orphan unresolved は cleanup blocker とする。

---

## 12. comparison evidence

comparison result は evidence artifact として保存する。

保持候補：

```text
comparison_report
semantic_equivalence_report
traceability_equivalence_report
cleanup_blocker_report
cleanup_candidate_report
```

comparison evidence なしに cleanup_ready を出してはならない。

---

## 13. cleanup blocker generation

cleanup blocker 候補：

```text
- semantic_non_equivalent
- traceability_mismatch
- unresolved contamination
- unresolved orphan
- partial rewrite
- unresolved restriction merge
- unresolved provenance chain
- rollback ambiguity
```

cleanup blocker unresolved のまま cleanup execution に進んではならない。

---

## 14. cleanup candidate generation

cleanup candidate は以下を満たす必要がある。

```text
- comparison completed
- semantic_equivalent or cleanup_equivalent
- traceability preserved
- no unresolved contamination
- no unresolved orphan
- rollback scope exists
```

cleanup candidate は cleanup approved を意味しない。

---

## 15. reconstruction completion review

reconstruction completed 判定条件：

```text
- comparison completed
- cleanup blockers resolved
- cleanup execution completed where required
- old structure detached or archived
- no unresolved reconstruction delta
- no unresolved federation dependency
```

new structure generated は reconstruction completed ではない。

---

## 16. comparison registry

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "reconstruction_comparison_registry",
  "comparison_status": "comparison_in_progress",
  "equivalence_summary": {
    "semantic_equivalent": false,
    "traceability_equivalent": false,
    "cleanup_equivalent": false
  },
  "cleanup_blockers": [],
  "cleanup_candidates": []
}
```

---

## 17. validator interaction

comparison workflow は以下 validator と接続する。

```text
manifest_validator
canonicalization_validator
rewrite_validator
cleanup_gate_validator
risk_guard_validator
provenance_validator
```

comparison workflow は validator reports を evidence として使用する。

---

## 18. dashboard display

Dashboard は comparison status を表示する。

表示対象：

```text
- comparison stage
- equivalence summary
- cleanup blockers
- cleanup candidates
- unresolved orphan count
- contamination findings
- reconstruction completion readiness
```

Dashboard は equivalence を独自決定しない。

---

## 19. CI mapping

CI fail 条件：

```text
- cleanup_ready without comparison evidence
- unresolved semantic_non_equivalent
- unresolved traceability mismatch
- unresolved orphan
- partial rewrite treated as completed
- unresolved contamination ignored
```

CI warn 条件：

```text
- comparison_in_progress
- cleanup_hold active
- optional equivalence unresolved outside cleanup scope
```

---

## 20. 禁止事項

以下を禁止する。

```text
- structural equality のみで semantic equivalence とすること
- rewrite executed を rewrite validated と扱うこと
- comparison 前に old structure を削除すること
- comparison evidence なしに cleanup_ready を出すこと
- unresolved cleanup blocker を warning に丸めること
```

---

## 21. HLDocS feedback

本 workflow から、HLDocS 側へ以下をフィードバック候補とする。

```text
- reconstruction comparison workflow を formal phase として持つべき
- semantic equivalence と structural equivalence を分離すべき
- cleanup blocker generation を formal artifact 化すべき
- cleanup candidate generation を cleanup approval と分離すべき
- reconstruction completed は cleanup 後に判定すべき
```

---

## 22. 結論

reconstruction comparison workflow は、旧構成と新構成を比較し、semantic continuity / traceability continuity / cleanup safety を確認する workflow である。

これにより、新構成生成だけで reconstruction completed とせず、comparison evidence と cleanup blocker review を経て安全に cleanup / completion へ進める。
