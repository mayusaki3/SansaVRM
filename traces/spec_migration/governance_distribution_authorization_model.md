# governance distribution authorization model

## 1. 目的

本ドキュメントは、SansaVRM federation における governance distribution authorization model を定義する。

governance distribution authorization model は、rights provenance / restriction propagation / rights inheritance / provenance governance / replayability に基づき、distribution を許可する条件を扱う。

---

## 2. 基本方針

governance distribution authorization model は以下を扱う。

```text
- distribution authorization taxonomy
- authorization dependency chain
- restriction propagation
- rights inheritance model
- authorization replayability
- authorization override policy
- authorization invalidation
- authorization debt governance
- cross-project authorization synchronization
```

governance distribution authorization model は以下を行わない。

```text
- provenance ambiguity を silent allow しない
- unresolved restriction conflict を warning として distribution allow しない
- replay-incomplete authorization を distribution evidence としない
- dashboard projection を authorization source of truth としない
```

---

## 3. authorization positioning

distribution authorization は以下に属する。

```text
Distribution Governance Layer
Federated Governance Layer
Operational Traceability Layer
Rights / Provenance Governance Layer
```

authorization は distribution 許可境界であり、単なる package export ではない。

---

## 4. distribution authorization taxonomy

authorization taxonomy：

```text
internal_distribution
preview_distribution
restricted_distribution
public_distribution
commercial_distribution
third_party_distribution
```

scope により review / audit / restriction severity が変わる。

---

## 5. internal distribution

internal_distribution：

```text
- organization internal only
- federation external redistribution不可
- limited provenance review許可
```

internal_distribution は public_distribution ではない。

---

## 6. preview distribution

preview_distribution：

```text
- preview scope only
- rollback path mandatory
- temporary restriction mandatory
- redistribution limitation mandatory
```

preview_distribution は production authorization ではない。

---

## 7. restricted distribution

restricted_distribution：

```text
- restriction-aware distribution
- provenance verification mandatory
- restriction merge review mandatory
- redistribution condition mandatory
```

---

## 8. public distribution

public_distribution：

```text
- public availability
- replayable authorization mandatory
- provenance chain verification mandatory
- rights inheritance verification mandatory
- audit mandatory
```

---

## 9. commercial distribution

commercial_distribution：

```text
- commercial rights verification mandatory
- editor rights verification mandatory
- restriction conflict absence mandatory
- legal-sensitive review mandatory
```

---

## 10. third-party distribution

third_party_distribution：

```text
- third-party redistribution review mandatory
- downstream restriction propagation mandatory
- provenance export mandatory
- distribution policy export mandatory
```

---

## 11. authorization dependency chain

標準 dependency chain：

```text
provenance verified
↓
restriction merge verified
↓
rights inheritance verified
↓
license snapshot verified
↓
distribution authorization
```

hard dependency unresolved は distribution blocker。

---

## 12. authorization dependency requirements

authorization dependency：

```text
- provenance chain verification
- restriction merge verification
- rights inheritance verification
- replayability verification
- audit completeness
- distribution baseline validity
- rollback path availability
```

---

## 13. restriction propagation

restriction propagation は component / assembly / derived asset 間で継承される。

例：

```text
hair restriction
↓
assembled avatar restriction
```

restriction propagation missing は governance risk。

---

## 14. restriction propagation model

restriction propagation model：

```text
component restriction
assembly restriction
derived restriction
distribution restriction
runtime restriction
```

Propagation chain は replayable でなければならない。

---

## 15. restriction merge

restriction merge は複数 component restriction を統合する。

候補：

```text
- most restrictive wins
- conflict requires review
- unresolved conflict blocks distribution
```

silent restriction weakening を禁止する。

---

## 16. rights inheritance model

rights inheritance 候補：

```text
original_author
editor
assembler
converter
tool_provider
runtime_provider
```

rights inheritance は replayable provenance chain を要求する。

---

## 17. rights inheritance propagation

例：

```text
body author
↓
assembled asset

clothing author
↓
assembled asset

assembly editor
↓
distribution policy
```

inheritance ambiguity は review_required または blocker。

---

## 18. tool provenance relation

tool provenance 候補：

```text
SansaVRM Studio AI
conversion pipeline
review pipeline
runtime distribution tool
```

tool provenance は distribution policy に影響しうる。

---

## 19. authorization severity

authorization severity：

```text
informational
warning
review_required
distribution_blocker
legal_blocker
```

legal_blocker は override 不可候補。

---

## 20. authorization replayability

authorization replayability 条件：

```text
- authorization taxonomy recorded
- provenance refs recorded
- restriction refs recorded
- rights inheritance refs recorded
- license snapshot refs recorded
- audit refs recorded
- override refs recorded
```

Replay 不可能 authorization は public/commercial distribution に使用してはならない。

---

## 21. authorization override policy

override 候補：

```text
temporary legal waiver
temporary preview release
critical rollback redistribution
```

override は以下を要求する。

```text
- override reason
- affected restriction
- affected provenance ambiguity
- approver
- expiration
- rollback plan
- follow-up review
```

---

## 22. non-overridable authorization blockers

override 不可候補：

```text
- unresolved rights holder conflict
- unresolved restriction merge conflict
- missing provenance chain for public distribution
- unresolved commercial rights ambiguity
- unresolved privacy/security restriction
```

---

## 23. authorization invalidation

以下は authorization invalidation を発生させる。

```text
- rights holder changed
- restriction changed
- license snapshot invalidated
- provenance chain invalidated
- unresolved restriction conflict discovered
- replayability invalidated
```

invalidated authorization を active distribution evidence として扱ってはならない。

---

## 24. authorization debt

authorization debt 候補：

```text
temporary restriction bridge
temporary provenance ambiguity waiver
legacy license compatibility shim
post-distribution review task
```

authorization debt は visible governance artifact とする。

---

## 25. authorization debt governance

authorization debt governance：

```text
- debt registration
- debt severity
- debt expiration
- debt owner
- debt replay impact
- debt cleanup target
```

hidden authorization debt を禁止する。

---

## 26. cross-project authorization synchronization

対象例：

```text
SansaVRM
SansaVRM Studio AI
distribution pipeline
review pipeline
```

同期対象：

```text
- authorization taxonomy
- restriction policy
- provenance baseline
- rights inheritance policy
- distribution severity taxonomy
```

unsynchronized authorization は federation distribution risk。

---

## 27. authorization lifecycle

authorization lifecycle：

```text
authorization_pending
authorization_review_required
authorization_approved
authorization_blocked
authorization_overridden
authorization_invalidated
authorization_superseded
authorization_archived
```

invalidated authorization を active distribution evidence として扱ってはならない。

---

## 28. authorization report

最小構造：

```json
{
  "schema_version": "0.1-draft",
  "artifact_kind": "governance_distribution_authorization_report",
  "authorization_taxonomy": "public_distribution",
  "authorization_status": "authorization_review_required",
  "restriction_refs": [],
  "provenance_refs": [],
  "rights_inheritance_refs": [],
  "source_of_truth_refs": []
}
```

---

## 29. reason codes

候補 reason code：

```text
authorization_provenance_missing
authorization_restriction_conflict
authorization_rights_inheritance_ambiguous
authorization_license_snapshot_invalid
authorization_replayability_missing
authorization_override_without_audit
authorization_distribution_blocker_unresolved
authorization_invalidated_but_active
```

---

## 30. orchestration relation

federation execution orchestration は以下を block する。

```text
- public/commercial distribution without replayable authorization
- distribution with unresolved legal blocker
- distribution with invalidated authorization
- distribution with unresolved provenance ambiguity
- distribution with unresolved restriction conflict
```

---

## 31. dashboard relation

Dashboard は authorization summary を表示できる。

表示対象：

```text
- authorization taxonomy
- authorization lifecycle status
- restriction summary
- provenance summary
- rights inheritance summary
- override summary
- debt summary
```

Dashboard は authorization approval を独自決定しない。

---

## 32. CI mapping

CI fail 条件：

```text
- public/commercial distribution without replayable authorization
- unresolved legal blocker in distribution scope
- invalidated authorization used as active evidence
- unresolved provenance ambiguity in production distribution
- hidden authorization debt detected
```

CI warn 条件：

```text
- authorization_review_required
- preview distribution active
- authorization debt cleanup pending
- cross-project authorization acknowledgment pending outside production scope
```

---

## 33. 禁止事項

以下を禁止する。

```text
- provenance ambiguity を silent allow すること
- unresolved restriction conflict を warning に丸めること
- replay-incomplete authorization を public/commercial distribution に使うこと
- invalidated authorization を active evidence と扱うこと
- dashboard projection を authorization source of truth と扱うこと
```

---

## 34. HLDocS feedback

本 model から、HLDocS 側へ以下をフィードバック候補とする。

```text
- distribution authorization model を formalize すべき
- restriction propagation / rights inheritance を governance layer に含めるべき
- authorization replayability を mandatory 化すべき
- authorization debt / override / invalidation を formal artifact 化すべき
- cross-project authorization synchronization を formalize すべき
```

---

## 35. 結論

governance distribution authorization model は、SansaVRM federation における distribution 許可を replayable に管理する model である。

これにより、rights provenance・restriction propagation・rights inheritance・tool provenance を含めた distribution governance を安全に運用できる。
