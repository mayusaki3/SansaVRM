# federation bootstrap risk register

## 1. 目的

本ドキュメントは、SansaVRM federation MVP bootstrap 時に発生しやすい risk を整理し、mitigation / detection / blocking condition を定義する。

対象は Preview Federation MVP の bootstrap 段階であり、production readiness の risk register ではない。

---

## 2. 基本方針

bootstrap risk register は以下を扱う。

```text
- early-stage federation failure
- unsafe pass 判定
- stub validator の過信
- generated artifact の source 混入
- unknown / skeleton の誤分類
- cleanup_ready の誤判定
- draft schema の canonical 誤用
- dashboard projection の source of truth 誤用
```

bootstrap risk register は以下を行わない。

```text
- production risk を完全網羅しない
- governance approval を代替しない
- validator を代替しない
- cleanup execution を許可しない
```

---

## 3. risk severity

severity は以下とする。

```text
critical
high
medium
low
```

### critical

destructive cleanup、canonical corruption、stale source of truth、production 誤認につながる risk。

### high

Preview Federation の検証結果を大きく誤らせる risk。

### medium

CI / dashboard / artifact の信頼性を下げる risk。

### low

運用上の混乱や手戻りを増やす risk。

---

## 4. risk status

risk status は以下とする。

```text
open
mitigated
accepted
blocked
superseded
```

`accepted` は risk を無視することではない。

known limitation として記録したうえで継続する状態である。

---

## 5. risk register

### R-001: stub validator pass を安全判定と誤認する

Severity: critical

内容：

```text
stub validator が pass を返したことを、実 validation が成立したと誤認する。
```

影響：

```text
- cleanup_ready 誤判定
- preview を production と誤認
- release_candidate 誤判定
```

Mitigation：

```text
- stub validator report に stub=true を出力する
- dashboard に stub / skeleton status を表示する
- CI で stub pass を production readiness に使わない
```

Blocking condition：

```text
stub validator result が cleanup_ready の根拠に使われている
```

---

### R-002: unknown / skeleton index を pass 扱いする

Severity: critical

内容：

```text
未実装 index、未抽出 reference、未確認 artifact freshness を pass と扱う。
```

影響：

```text
- unresolved references の見逃し
- stale artifact の見逃し
- cleanup gate 誤通過
```

Mitigation：

```text
- unknown は cleanup_pending または cleanup_blocked とする
- skeleton index には skeleton=true を付ける
- CI で unknown-as-pass を検出する
```

Blocking condition：

```text
unknown / skeleton が cleanup_ready 条件に含まれる
```

---

### R-003: dashboard snapshot を source of truth と誤認する

Severity: high

内容：

```text
dashboard_snapshot.json / dashboard_summary.md を validator result や registry の代替として扱う。
```

影響：

```text
- source_of_truth_refs の喪失
- cleanup state の誤解釈
- audit trail 欠落
```

Mitigation：

```text
- dashboard に projection_only=true を含める
- source_of_truth_refs を必須にする
- dashboard generator は registry / validator report を変更しない
```

Blocking condition：

```text
dashboard が cleanup_ready を独自判定している
```

---

### R-004: draft schema を canonical schema として扱う

Severity: high

内容：

```text
0.1-draft schema を canonical boundary や downstream contract として扱う。
```

影響：

```text
- schema drift の見逃し
- downstream project の誤実装
- release lifecycle 誤判定
```

Mitigation：

```text
- schema_version に draft を含める
- declared_stage を必須にする
- federation validator で draft inside canonical boundary を検査する
```

Blocking condition：

```text
draft schema が canonical release / cleanup dependency に使われる
```

---

### R-005: generated reports を repository source として commit する

Severity: medium

内容：

```text
reports/federation の生成物を source file と混同して commit する。
```

影響：

```text
- stale report の混入
- validator source の混乱
- dashboard の古い状態表示
```

Mitigation：

```text
- reports/federation は CI artifact とする
- .gitignore または README で generated を明記する
- commit checklist で generated files を除外する
```

Blocking condition：

```text
stale generated report が source of truth として参照されている
```

---

### R-006: external artifact freshness が unknown のまま cleanup 判定する

Severity: high

内容：

```text
external_artifact_index の freshness_status が unknown のまま cleanup_ready を出す。
```

影響：

```text
- MuJoCo Adapter / Studio AI との不整合
- downstream artifact stale の見逃し
- federation cleanup inconsistency
```

Mitigation：

```text
- required artifact の unknown は cleanup_blocked
- optional artifact の unknown は cleanup_pending / warn
- dashboard に freshness unknown を表示する
```

Blocking condition：

```text
required external artifact freshness unknown with cleanup_ready
```

---

### R-007: reconstruction delta registry を更新せずに再実行する

Severity: high

内容：

```text
要件追加・方針変更が発生したが reconstruction_delta_registry に記録されない。
```

影響：

```text
- stale execution の再利用
- validator rerun 漏れ
- cleanup_ready 再評価漏れ
```

Mitigation：

```text
- reconstruction_delta_registry を CI 入力にする
- rerun_required を dashboard に表示する
- delta 未記録の方針変更は known risk として扱う
```

Blocking condition：

```text
known delta が registry 未記録のまま cleanup / release scope に入る
```

---

### R-008: CI dry-run を apply 実行と誤認する

Severity: medium

内容：

```text
CI が dry-run artifact を生成したことを、実 execution 完了と誤認する。
```

影響：

```text
- execution status の誤表示
- approval boundary の省略
- cleanup / release 誤判定
```

Mitigation：

```text
- CI report に dry_run_only=true を含める
- execution_status を applied にしない
- dashboard に dry-run only を表示する
```

Blocking condition：

```text
CI dry-run result が apply completed として扱われる
```

---

### R-009: handoff response の存在を contract acceptance と誤認する

Severity: medium

内容：

```text
MuJoCo Adapter / Studio AI への response document があるだけで、handoff contract が accepted と誤認される。
```

影響：

```text
- pending decision の見逃し
- downstream action request 漏れ
- federation cleanup 誤通過
```

Mitigation：

```text
- response_exists と contract_accepted を分離する
- pending_decisions を registry / dashboard に表示する
- federation validator で handoff pending を検出する
```

Blocking condition：

```text
handoff pending のまま cleanup dependency に入る
```

---

### R-010: Preview Federation を Production Federation と誤認する

Severity: critical

内容：

```text
MVP / Preview 段階の validator / dashboard を production readiness と誤認する。
```

影響：

```text
- destructive cleanup の早期実行
- canonical release 誤公開
- governance / audit 不足
```

Mitigation：

```text
- maturity_stage を dashboard に表示する
- MVP report に preview_only=true を含める
- production readiness checklist を別途要求する
```

Blocking condition：

```text
MVP state で production cleanup / canonical release を実行しようとしている
```

---

## 6. bootstrap CI risk checks

CI で検出すべき risk：

```text
- unknown-as-pass
- skeleton-as-pass
- missing source_of_truth_refs
- dashboard cleanup_ready independent decision
- draft schema inside canonical boundary
- required artifact freshness unknown
- generated report committed as source
```

---

## 7. dashboard risk view

Dashboard は risk view を持つ。

表示対象：

```text
- open critical risks
- open high risks
- accepted known limitations
- blocking conditions
- preview_only flag
- dry_run_only flag
```

Dashboard は risk status を独自変更しない。

---

## 8. risk register schema draft

```json
{
  "schema_version": "0.1-draft",
  "risk_register_id": "bootstrap-risk-register",
  "risks": [
    {
      "risk_id": "R-001",
      "severity": "critical",
      "status": "open",
      "title": "stub validator pass を安全判定と誤認する",
      "blocking_condition": "stub validator result が cleanup_ready の根拠に使われている"
    }
  ]
}
```

---

## 9. HLDocS feedback

本 risk register から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction bootstrap には risk register が必要
- stub / skeleton / unknown を pass と誤認しない guardrail が必要
- dashboard projection / generated artifact / source of truth を分離すべき
- preview と production の誤認を risk として扱うべき
- risk register は CI / dashboard と接続すべき
```

---

## 10. 結論

federation bootstrap risk register は、Preview Federation MVP bootstrap における early-stage risk を管理するための risk register である。

これにより、read-only / dry-run / validator-first の原則を崩さず、unsafe pass、source of truth 混同、draft/canonical 混同、preview/production 誤認を早期に検出できる。
