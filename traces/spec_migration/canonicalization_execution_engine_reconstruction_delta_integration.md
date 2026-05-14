# canonicalization execution engine reconstruction delta integration

## 1. 目的

本ドキュメントは、`canonicalization_execution_engine.md` に対して、reconstruction delta handling を接続する補足文書である。

既存の canonicalization execution engine は、scope freeze 後に入力が変わった場合、現在の execution を superseded とし、新しい execution_id を発行する方針を持つ。

本ドキュメントでは、その入力変化が「再構築中の要件追加・方針変更・cross-project feedback」である場合の具体的な扱いを定義する。

---

## 2. 基本方針

再構築中に要件追加・方針変更・cross-project feedback が入った場合は、通常 patch ではなく reconstruction delta として扱う。

reconstruction delta は current execution に直接混入しない。

特に scope freeze 後の delta は、必ず superseded / replacement execution として扱う。

---

## 3. freeze 前の reconstruction delta

freeze 前の reconstruction delta は active planning scope へ統合できる。

条件：

```text
- scope freeze 前
- checkpoint creation 前
- dry-run execution 前
- approval boundary 前
```

処理：

```text
- reconstruction_delta_id を記録する
- orchestration graph を更新する
- rewrite transaction plan を更新する
- validator rerun scope を更新する
- dashboard projection input を更新する
```

---

## 4. freeze 後の reconstruction delta

freeze 後の reconstruction delta は current execution に直接混入してはならない。

処理：

```text
- current execution を superseded とする
- reconstruction_delta_id を記録する
- delta impact analysis を行う
- replacement canonicalization_id / execution_id を発行する
- rewrite transaction plan を再生成する
- validator rerun scope を再計算する
- cleanup_ready を再評価する
```

---

## 5. stale execution 禁止条件

以下の delta が発生した場合、既存 execution を completed / cleanup_ready 判定に使ってはならない。

```text
- semantic_delta
- validation_delta
- traceability_delta
- cleanup_delta
- cross_project_delta affecting execution scope
- external_artifact_delta affecting validator scope
```

---

## 6. execution status 追加扱い

canonicalization execution engine は、reconstruction delta により以下の状態遷移を扱う。

```text
approved
→ superseded

dry_run_completed
→ superseded

validator_handoff_ready
→ rerun_required

completed
→ superseded_for_reconstruction_delta
```

`superseded_for_reconstruction_delta` は、過去には有効だったが、再構築中の追加要件により現在の判断には使えない execution を表す。

---

## 7. validator rerun policy

reconstruction delta 後は validator rerun を必須とする。

最低限再実行する validator：

```text
- manifest_validator
- canonicalization_validator
- rewrite_validator
- cleanup_gate_validator
- dashboard_projection_validator
```

必要に応じて再実行する validator：

```text
- integrity_tamper_validator
- reference_validator
- traceability_validator
- sec_id_validator
- cross_project validation
```

---

## 8. cleanup_ready 再評価

reconstruction delta 発生後は cleanup_ready を再評価する。

以下は cleanup_ready を無効化する。

```text
- semantic_delta
- validation_delta
- traceability_delta
- cleanup_delta
- cross_project_delta affecting cleanup scope
```

representation_delta のみで semantic_integrity=pass の場合は、cleanup_ready を維持できる場合がある。

ただし、その場合も integrity / tamper validator の再確認を必要とする。

---

## 9. cross-project feedback の扱い

MuJoCo Adapter / Studio AI 等からの feedback は cross_project_delta として扱う。

例：

```text
- Adapter JSON schema change
- updated_extension_properties policy change
- diagnostics / conversion_report classification change
- export profile requirement change
- Studio AI workflow / fixture change
```

cross-project delta は localized patch として扱わない。

---

## 10. HLDocS feedback

本 integration から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction phase の変更要求は通常 patch と分離すべき
- freeze 後の reconstruction delta は current execution に直接混入してはならない
- reconstruction delta は superseded / replacement execution として traceable に扱うべき
- reconstruction delta 発生後は validator rerun と cleanup_ready 再評価を必須にすべき
- cross-project feedback は reconstruction delta として扱い、localized patch として扱ってはならない
```

---

## 11. 結論

canonicalization execution engine は、再構築中の要件追加・方針変更・cross-project feedback を reconstruction delta として扱う。

これにより、freeze 後 delta の unsafe mixing を防ぎ、superseded execution、replacement execution、validator rerun、cleanup_ready reevaluation を traceable に管理できる。
