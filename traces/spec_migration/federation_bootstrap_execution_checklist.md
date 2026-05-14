# federation bootstrap execution checklist

## 1. 目的

本ドキュメントは、SansaVRM federation MVP repository bootstrap を実作業として実行するための checklist を定義する。

本 checklist は、初回 bootstrap commit を安全に作成するための作業順、確認順、commit 境界、禁止事項を整理する。

---

## 2. 基本方針

bootstrap execution は以下を守る。

```text
- read-only / dry-run 専用で開始する
- schema / example / registry / stub tool / CI を小さく導入する
- destructive operation を含めない
- unknown / skeleton を pass 扱いしない
- CI artifact を最初から出す
```

---

## 3. execution phases

Bootstrap execution phase は以下とする。

```text
B0 pre-check
B1 directory skeleton
B2 draft schemas
B3 example JSON
B4 registries
B5 stub tools
B6 CI workflow
B7 local dry-run
B8 commit
B9 CI artifact review
```

---

## 4. B0 pre-check

確認：

```text
- develop branch 上で作業している
- destructive cleanup を含めない
- apply execution を含めない
- schema は draft として扱う
- reports/federation は generated artifact として扱う
```

NG 条件：

```text
- cleanup execution を含めようとしている
- canonicalization apply を含めようとしている
- downstream repository modification を含めようとしている
```

---

## 5. B1 directory skeleton

作成：

```text
tools/federation_migration/README.md
tools/federation_migration/schemas/.gitkeep
tools/federation_migration/examples/.gitkeep
tools/federation_migration/index_builder/.gitkeep
tools/federation_migration/validators/.gitkeep
tools/federation_migration/dashboard/.gitkeep
tools/federation_migration/reports/.gitkeep
```

確認：

```text
- directory が責務ごとに分離されている
- reports は生成物用であることが README に明記されている
```

---

## 6. B2 draft schemas

作成：

```text
tools/federation_migration/schemas/index_bundle.schema.json
tools/federation_migration/schemas/validator_report.schema.json
tools/federation_migration/schemas/dashboard_snapshot.schema.json
tools/federation_migration/schemas/reconstruction_delta_registry.schema.json
tools/federation_migration/schemas/external_artifact_registry.schema.json
```

確認：

```text
- schema_version は 0.1-draft
- canonical schema として扱わない旨が明記されている
- minimum required fields のみ定義している
```

---

## 7. B3 example JSON

作成：

```text
tools/federation_migration/examples/index_bundle.example.json
tools/federation_migration/examples/validator_report.example.json
tools/federation_migration/examples/dashboard_snapshot.example.json
tools/federation_migration/examples/reconstruction_delta_registry.example.json
tools/federation_migration/examples/external_artifact_registry.example.json
```

確認：

```text
- example は source of truth ではない
- cleanup_ready は安易に出していない
- unknown は pending / blocked / unknown のまま表現している
```

---

## 8. B4 registries

作成：

```text
traces/spec_migration/reconstruction_delta_registry.json
traces/spec_migration/external_artifact_registry.json
```

初期値：

```text
- registry_kind を持つ
- schema_version は 0.1-draft
- entries は空配列でよい
```

確認：

```text
- registry は machine-readable source として扱う
- dashboard は registry を変更しない
```

---

## 9. B5 stub tools

作成：

```text
tools/federation_migration/index_builder/build_index.py
tools/federation_migration/validators/run_validators.py
tools/federation_migration/dashboard/build_dashboard.py
```

最低動作：

```text
build_index.py:
examples / registries から reports/federation/index_bundle.json を生成

run_validators.py:
index_bundle.json から minimal validator reports を生成

build_dashboard.py:
validator reports から dashboard_snapshot.json / dashboard_summary.md を生成
```

確認：

```text
- source file を変更しない
- cleanup / apply を実行しない
- unknown を pass 扱いしない
- script error 時は non-zero exit
```

---

## 10. B6 CI workflow

作成：

```text
.github/workflows/federation-migration-preview.yml
```

実行順：

```text
1. checkout
2. setup python
3. build_index.py
4. run_validators.py
5. build_dashboard.py
6. upload reports/federation
```

確認：

```text
- destructive operation がない
- downstream repository modification がない
- artifact upload がある
- required report missing で fail する
```

---

## 11. B7 local dry-run

実行例：

```text
python tools/federation_migration/index_builder/build_index.py
python tools/federation_migration/validators/run_validators.py
python tools/federation_migration/dashboard/build_dashboard.py
```

確認：

```text
- reports/federation/index_bundle.json が生成される
- validator report が生成される
- dashboard_snapshot.json が生成される
- dashboard_summary.md が生成される
- cleanup_ready が不明情報だけで出ていない
```

---

## 12. B8 commit

commit 対象：

```text
- tools/federation_migration/**
- .github/workflows/federation-migration-preview.yml
- traces/spec_migration/reconstruction_delta_registry.json
- traces/spec_migration/external_artifact_registry.json
```

commit 対象外：

```text
- reports/federation/generated files
- local temp files
- downstream project files
```

commit message 例：

```text
Add federation migration MVP bootstrap
```

---

## 13. B9 CI artifact review

確認：

```text
- GitHub Actions が成功する
- reports/federation artifact がアップロードされる
- dashboard_summary.md を確認できる
- validator report に source_of_truth_refs がある
- cleanup_ready / blocked / pending が確認できる
```

NG 条件：

```text
- report missing
- source_of_truth_refs missing
- unknown を pass として扱っている
- generated reports を repository に誤 commit している
```

---

## 14. bootstrap completion criteria

Bootstrap 完了条件：

```text
- directory skeleton が存在する
- draft schemas が存在する
- example JSON が存在する
- empty registries が存在する
- stub tools が local dry-run で動く
- CI が reports artifact を生成する
- destructive operation が含まれていない
```

---

## 15. 次の作業への接続

Bootstrap 完了後の推奨順：

```text
1. filesystem_index real scan
2. migration_index real extraction
3. manifest_validator real duplicate check
4. dashboard blocking reason view improvement
5. external_artifact_registry manual registration
6. federation_validator minimal freshness check
```

---

## 16. 禁止事項

以下を禁止する。

```text
- bootstrap 中に cleanup execution を入れること
- bootstrap 中に canonicalization apply を入れること
- generated reports を source of truth とすること
- dashboard summary を validation result の代替にすること
- stub validator pass を production readiness とみなすこと
```

---

## 17. HLDocS feedback

本 checklist から、HLDocS 側へ以下をフィードバックする。

```text
- reconstruction toolchain の bootstrap は operator checklist 化すべき
- 初回導入は schema / example / registry / stub / CI artifact に限定すべき
- local dry-run と CI artifact review を分けるべき
- generated reports と source files を明確に分離すべき
- unknown を pass として扱わない確認項目が必要
```

---

## 18. 結論

federation bootstrap execution checklist は、SansaVRM federation MVP を repository に初回導入するための実作業 checklist である。

これにより、read-only / dry-run / validator-first の原則を守りながら、Preview Federation の bootstrap commit を安全に作成できる。
