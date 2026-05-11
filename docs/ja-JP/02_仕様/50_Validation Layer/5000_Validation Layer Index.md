<!--
HLDocS:LLM-MANAGED
doc_id: dry-doc-5000-validation-layer-index
lang: ja-JP
canonical_title: Validation Layer Index
document_type: index
canonical_document: true
-->

[目次](../../目次.md) > 仕様 > Validation Layer > Validation Layer Index

# Validation Layer Index

## 1. 目的

本ドキュメントは、SansaVRM における Validation Layer の責務、構成、依存順序を定義する。

Validation Layer は、SansaVRM の仕様・変換・互換性・移行・検証結果の観測可能性を扱う層である。

---

## 2. Validation Layer の責務

Validation Layer は以下を扱う。

```text
- schema validation
- semantic validation
- conversion validity
- diagnostics
- migration traceability
- coverage analysis
- loss report
- compatibility analysis
- roundtrip verification
```

---

## 3. 非責務

Validation Layer は以下を直接扱わない。

```text
- runtime execution
- physics simulation execution
- adapter implementation detail
- renderer implementation
- UI workflow
```

---

## 4. Validation Core

Validation Core は、検証の中核となる仕様群である。

```text
5001_JSONスキーマ仕様
5002_Validator実装仕様
5003_変換仕様
```

### 4.1 依存順序

```text
JSON Schema
↓
Validator
↓
Conversion validity
```

---

## 5. Validation Observability Stack

Validation Observability Stack は、検証結果、移行状態、損失、互換性、RoundTrip を観測可能にする仕様群である。

```text
5004_Diagnostics仕様
5005_Traceability Migration仕様
5006_Coverage Analysis仕様
5007_Loss Report仕様
5008_Compatibility Analysis仕様
5009_RoundTrip Verification仕様
```

### 5.1 依存順序

```text
Diagnostics
↓
Migration Traceability
↓
Coverage Analysis
↓
Loss Report
↓
Compatibility Analysis
↓
RoundTrip Verification
```

---

## 6. 構成一覧

| 番号 | 文書 | 責務 |
|---|---|---|
| 5001 | JSONスキーマ仕様 | 構造 validation contract |
| 5002 | Validator実装仕様 | 意味・参照・整合 validation |
| 5003 | 変換仕様 | conversion validity / validator linkage |
| 5004 | Diagnostics仕様 | validation / conversion / migration の観測可能性 |
| 5005 | Traceability Migration仕様 | migration graph / sec_id continuity |
| 5006 | Coverage Analysis仕様 | validation coverage / orphan detection |
| 5007 | Loss Report仕様 | semantic loss / approximation / preservation state |
| 5008 | Compatibility Analysis仕様 | semantic compatibility continuum |
| 5009 | RoundTrip Verification仕様 | semantic continuity verification |

---

## 7. Layer 間依存

Validation Layer は以下の Layer を参照する。

```text
Core Semantic Layer
Preservation Compatibility Layer
Data Model Layer
Runtime Integration Layer
```

Validation Layer は runtime execution に依存しない。

---

## 8. Core Semantic Layer との関係

Core Semantic Layer は意味定義を提供する。

Validation Layer は、その意味が保持・検証されているかを検査する。

---

## 9. Preservation Compatibility Layer との関係

Preservation Compatibility Layer は、preservation / passthrough / conversion profile を定義する。

Validation Layer は、それらの保持状態、損失、互換性、RoundTrip を検査する。

---

## 10. Data Model Layer との関係

Data Model Layer は、Geometry / Morph / Animation / Physics などの実体データ構造を定義する。

Validation Layer は、それらの構造・参照・semantic continuity を検査する。

---

## 11. Runtime Integration Layer との関係

Runtime Integration Layer は runtime binding や runtime semantic を定義する。

Validation Layer は runtime semantic が保持・損失・互換・RoundTrip 可能かを検査する。

---

## 12. Migration 状態

本 Layer では dry-run relocation 中の文書を含む。

以下の状態を許可する。

```text
pending
partial
complete
verified
```

placeholder relocation は migration completion を意味しない。

---

## 13. 並び順の根拠

Validation Layer の番号は dependency ordering に基づく。

```text
5000 index
5001 schema
5002 validator
5003 conversion
5004 diagnostics
5005 migration traceability
5006 coverage
5007 loss
5008 compatibility
5009 roundtrip
```

---

## 14. HLDocS feedback

本 Layer 整理で得られた知見：

```text
- Validation は Core と Observability に分離できる
- Traceability / Coverage / Loss / Compatibility / RoundTrip は Observability Family として扱える
- relocation は migration graph として扱うべき
- placeholder relocation は巨大仕様移行に有効
- sec_id continuity は長期 traceability に重要
```

---

## 15. 結論

Validation Layer は、SansaVRM の semantic preservation を検証し、検証結果を観測可能にするための責務層である。

本 Layer により、schema validation、semantic validation、migration traceability、coverage、loss、compatibility、roundtrip verification を体系的に管理できる。

---

[目次](../../目次.md) > 仕様 > Validation Layer > Validation Layer Index
