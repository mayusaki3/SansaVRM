# governance VN3 policy extension roadmap

## 1. 目的

本ドキュメントは、VN3系ライセンスおよび conditional policy governance を強化するための roadmap 拡張を定義する。

現在の governance model 群により、VN3ライセンスの多くは replayable governance として扱える。

しかし、以下の強化が必要と考えられる。

```text
- machine-readable conditional policy
- policy evaluation
- federated license reconciliation
- conditional restriction governance
```

---

## 2. 現在の対応状況

既存 model により以下は概ね対応可能。

```text
- rights provenance
- restriction propagation
- rights inheritance
- distribution authorization
- replayable authorization
- provenance snapshot
- audit traceability
```

特に以下は VN3 と相性が良い。

```text
component restriction
assembly restriction
derived restriction
distribution restriction
runtime restriction
```

および：

```text
most restrictive wins
conflict requires review
unresolved conflict blocks distribution
```

---

## 3. 課題

VN3系では以下のような conditional policy が多い。

```text
商用可だが法人禁止
R18可だが政治利用禁止
改変可だが再配布禁止
AI学習禁止
```

現在の normalized boolean model のみでは表現力が不足する可能性がある。

---

## 4. roadmap 追加候補

追加候補：

```text
- governance_policy_expression_model
- governance_policy_evaluation_model
- governance_license_reconciliation_model
- governance_conditional_restriction_model
```

---

## 5. governance_policy_expression_model

目的：

```text
machine-readable policy expression
```

を formalize する。

候補：

```text
- policy_rule
- policy_expression
- restriction_expression
- effect
- condition
- scope
- expiration
```

例：

```json
{
  "rule_id": "vn3-commercial-001",
  "effect": "deny",
  "condition": {
    "commercial_use": true,
    "organization_type": "corporation"
  }
}
```

---

## 6. governance_policy_evaluation_model

目的：

```text
operation-level policy evaluation
```

を formalize する。

対象例：

```text
export
upload
distribution
runtime_load
assembly
ai_training
```

evaluation 結果候補：

```text
allow
conditional_allow
review_required
deny
```

---

## 7. governance_license_reconciliation_model

目的：

```text
federated license reconciliation
```

を formalize する。

対象例：

```text
VN3
独自ライセンス
VRM license
CC license
Booth terms
```

必要候補：

```text
- restriction merge
- semantic license mapping
- license conflict escalation
- replayable reconciliation
```

---

## 8. governance_conditional_restriction_model

目的：

```text
conditional restriction propagation
```

を formalize する。

対象例：

```text
R18 restriction
corporate restriction
political restriction
AI training restriction
redistribution restriction
```

必要候補：

```text
- conditional propagation
- condition-aware restriction merge
- operation-aware restriction evaluation
- restriction escalation
```

---

## 9. 将来方向

将来的には以下との統合を想定。

```text
distribution authorization
rights inheritance
semantic conflict resolution
federated governance
Studio AI policy consumer
```

---

## 10. HLDocS feedback

本 roadmap から、HLDocS 側へ以下をフィードバック候補とする。

```text
- conditional policy governance を formalize すべき
- machine-readable restriction model を formal artifact 化すべき
- policy evaluation / reconciliation を governance layer に含めるべき
- federated license conflict resolution を扱うべき
```

---

## 11. 結論

VN3対応は現在の governance model 群でもかなり実現可能である。

ただし、conditional policy / evaluation / reconciliation layer を追加することで、VN3や複雑な federated license governance に対する対応力を大きく向上できる。
