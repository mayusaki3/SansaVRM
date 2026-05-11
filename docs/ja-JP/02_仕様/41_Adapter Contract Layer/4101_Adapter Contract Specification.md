<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-4101-adapter-contract-specification
lang: ja-JP
canonical_title: Adapter Contract Specification
document_type: spec
canonical_document: true
-->

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Adapter Contract Specification

# Adapter Contract Specification

## 1. 目的

本仕様は、SansaVRM における Adapter の共通契約を定義する。

本仕様は以下を対象とする。

```text
- VRM adapter
- FBX adapter
- MMD adapter
- URDF adapter
- MuJoCo adapter
- runtime adapter
```

---

## 2. 基本方針

Adapter は format/runtime 固有処理を担当し、Core Semantic を汚染しない。

以下を原則とする。

```text
1. Adapter は import/export 実行責務を持つ
2. Core Semantic は SansaVRM 本体が保持する
3. Adapter は diagnostics / loss_report を返却できる
4. Adapter は preservation_only を破壊しない
5. Adapter は source_raw を保持可能である
6. Runtime-specific optimization は Adapter 側責務とする
```

---

## 3. Adapter Scope

Adapter は以下を扱う。

```text
- format parse
- format export
- runtime conversion
- runtime binding
- proprietary handling
- optimization policy
```

---

## 4. 非対象範囲

以下は Adapter の責務外とする。

```text
- Core Semantic 定義
- validator rule 定義
- traceability policy
- generic semantic abstraction
```

---

## 5. Adapter Input

Adapter は以下を入力として扱う。

```text
- SansaVRM document
- Conversion Profile
- Preservation Layer
- Runtime Binding
- external artifact
```

---

## 6. Adapter Output

Adapter は以下を出力できる。

```text
- converted format
- diagnostics
- conversion_report
- loss_report
- adapter artifact
- runtime artifact
```

---

## 7. Diagnostics Contract

Adapter diagnostics は以下を含められる。

```text
- unsupported semantic
- approximation
- fallback
- runtime warning
- preservation status
```

---

## 8. Artifact Contract

Adapter artifact は以下を許可する。

```text
- cache
- baked runtime data
- optimization data
- runtime metadata
- binary blob
```

Artifact は外部化可能である。

---

## 9. Preservation Requirements

Adapter は以下を満たす。

```text
- preservation_only を破壊しない
- source_raw を削除しない
- raw_binary_ref を保持する
- unsupported semantic を silent loss しない
```

---

## 10. Runtime Binding

Adapter は Runtime Binding と接続可能である。

対象例：

```text
- MuJoCo runtime
- Unity runtime
- O3DE runtime
- HIL runtime
```

---

## 11. Validator Connection

Adapter は validator と接続できなければならない。

Validator は以下を検査する。

```text
- adapter output integrity
- diagnostics consistency
- artifact reference integrity
- loss_report consistency
```

---

## 12. 関連仕様

本仕様は以下と連携する。

```text
Runtime Integration Layer
Custom Parameter Registry Layer
Format Compatibility Preservation Specification
Conversion Profile Specification
```

---

## 13. 結論

Adapter Contract は runtime / format 固有処理を隔離するための共通契約である。

SansaVRM 本体は semantic integration platform を維持し、実際の import/export/runtime 処理は Adapter 側へ分離する。

---

[目次](../目次.md) > 仕様 > Adapter Contract Layer > Adapter Contract Specification
