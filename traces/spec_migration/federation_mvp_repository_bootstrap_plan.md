# federation MVP repository bootstrap plan

## 1. 目的

本ドキュメントは、SansaVRM federation MVP を repository 上に初回導入するための bootstrap plan を定義する。

対象は Preview Federation MVP の初期実装であり、destructive cleanup、automatic apply execution、downstream repository modification は対象外とする。

---

## 2. 基本方針

Bootstrap では以下を優先する。

```text
- read-only tools を配置する
- schema と sample data を先に配置する
- validator は最小 report を出す
- CI は dry-run と artifact upload のみ行う
- dashboard は summary artifact として生成する
```

Bootstrap では以下を行わない。

```text
- cleanup execution
- canonicalization apply
- downstream repository modification
- governance approval automation
- production release automation
```

---

## 3. 初回追加ディレクトリ

初回 bootstrap で追加する候補：

```text
tools/federation_migration/
  README.md
  schemas/
  examples/
  index_builder/
  validators/
  dashboard/
  reports/.gitkeep
```

CI workflow：

```text
.github/workflows/federation-migration-preview.yml
```

trace registry：

```text
traces/spec_migration/reconstruction_delta_registry.json
traces/spec_migration/external_artifact_registry.json
```

---

## 4. schema bootstrap

初回 schema：

```text
tools/federation_migration/schemas/index_bundle.schema.json
tools/federation_migration/schemas/validator_report.schema.json
tools/federation_migration/schemas/dashboard_snapshot.schema.json
tools/federation_migration/schemas/reconstruction_delta_registry.schema.json
tools/federation_migration/schemas/external_artifact_registry.schema.json
```

初回 schema は draft とする。

canonical schema として扱ってはならない。

---

## 5. example data bootstrap

初回 example：

```text
tools/federation_migration/examples/index_bundle.example.json
tools/federation_migration/examples/validator_report.example.json
tools/federation_migration/examples/dashboard_snapshot.example.json
tools/federation_migration/examples/reconstruction_delta_registry.example.json
tools/federation_migration/examples/external_artifact_registry.example.json
```

example は validator の fixture として使用可能にする。

ただし、example を source of truth として扱ってはならない。

---

## 6. registry bootstrap

### reconstruction_delta_registry.json

最小項目：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "reconstruction_delta_registry",
  "deltas": []
}
```

### external_artifact_registry.json

最小項目：

```json
{
  "schema_version": "0.1-draft",
  "registry_kind": "external_artifact_registry",
  "artifacts": []
}
```

registry は machine-readable source として扱う。

Dashboard は registry を表示できるが、registry を変更してはならない。

---

## 7. tool bootstrap

初回 tool は stub / dry-run としてよい。

候補：

```text
tools/federation_migration/index_builder/build_index.py
tools/federation_migration/validators/run_validators.py
tools/federation_migration/dashboard/build_dashboard.py
```

初期責務：

```text
build_index.py:
example / registry から index_bundle.json を生成

run_validators.py:
index_bundle.json を読み、最小 validator reports を生成

build_dashboard.py:
validator reports から dashboard_summary.md / dashboard_snapshot.json を生成
```

---

## 8. reports bootstrap

CI artifact として以下を生成する。

```text
reports/federation/index_bundle.json
reports/federation/manifest_validator_report.json
reports/federation/canonicalization_validator_report.json
reports/federation/rewrite_validator_report.json
reports/federation/cleanup_gate_validator_report.json
reports/federation/federation_validator_report.json
reports/federation/dashboard_snapshot.json
reports/federation/dashboard_summary.md
```

`reports/federation/` は生成物であり、通常は Git 管理対象外にする。

ただし `.gitkeep` は配置してよい。

---

## 9. CI bootstrap

初回 CI workflow：

```text
.github/workflows/federation-migration-preview.yml
```

実行内容：

```text
1. Python setup
2. build_index.py
3. run_validators.py
4. build_dashboard.py
5. upload reports/federation as artifact
```

CI fail 条件：

```text
- script execution error
- required report missing
- dashboard_snapshot.json missing
- cleanup_ready が unknown index を pass として扱っている
```

CI warn は Markdown summary 内で表示する。

---

## 10. README bootstrap

`tools/federation_migration/README.md` には以下を記載する。

```text
- Preview Federation MVP 用 tool であること
- read-only / dry-run 専用であること
- destructive cleanup を行わないこと
- dashboard は source of truth ではないこと
- schema は draft であること
```

---

## 11. 初回 bootstrap commit scope

初回 commit では以下までに限定する。

```text
- directory skeleton
- draft schemas
- example JSON
- empty registries
- stub tools
- CI dry-run workflow
- README
```

以下は初回 commit では行わない。

```text
- real manifest parsing
- real markdown reference extraction
- real traceability extraction
- cleanup execution
- canonicalization apply
```

---

## 12. bootstrap validation

初回 bootstrap 後の確認：

```text
- CI が成功する
- reports/federation が artifact として生成される
- dashboard_summary.md が存在する
- cleanup_ready が安易に出ない
- unknown / skeleton は pending または blocked になる
```

---

## 13. 次段階への接続

Bootstrap 後の次段階：

```text
1. filesystem_index の real scan
2. migration_index の real extraction
3. manifest_validator の real duplicate check
4. canonicalization_index の real extraction
5. cleanup_gate の stricter validation
6. external_artifact_registry の手動登録
7. federation validator の artifact freshness check
```

---

## 14. 禁止事項

以下を禁止する。

```text
- bootstrap schema を canonical schema として扱うこと
- example JSON を source of truth として扱うこと
- dashboard snapshot を source of truth として扱うこと
- CI で cleanup execution を行うこと
- stub validator の pass を production readiness とみなすこと
```

---

## 15. HLDocS feedback

本 bootstrap plan から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction toolchain は skeleton / schema / example / CI artifact から bootstrap すべき
- 初回実装では destructive operation を含めるべきではない
- draft schema と canonical schema を明示的に分離すべき
- stub validator の結果を production readiness と誤認しない guardrail が必要
- dashboard artifact は source of truth ではなく review artifact として扱うべき
```

---

## 16. 結論

federation MVP repository bootstrap plan は、SansaVRM federation MVP を repository 上に安全に導入するための初回構成計画である。

これにより、schema、example、stub tool、CI artifact、dashboard summary を read-only / dry-run 前提で導入し、Preview Federation の実装開始点を作る。
