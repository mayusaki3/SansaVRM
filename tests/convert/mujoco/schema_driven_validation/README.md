# MuJoCo Schema-Driven Validation Golden Tests

## 1. 目的

本ディレクトリは、MuJoCo Schema-Driven Validation PoC の fixture、golden data、golden test runner を管理する。

以下を検証する。

- Schema Registry 読み込み
- Adapter Capability 読み込み
- Validation Error Code catalog 読み込み
- io_scope routing
- fallback emission
- Diagnostics generation
- Conversion Report generation
- strict mode output_allowed 判定
- golden data 一致

---

## 2. ディレクトリ構成

```text
schema_driven_validation/
  capability/
  expected_diagnostics/
  expected_reports/
  inputs/
  registry/
  test_golden_validator.py
```

---

## 3. 実行方法

PowerShell:

```powershell
$env:PYTHONPATH="tools"
python -m unittest `
  tests.convert.mujoco.schema_driven_validation.test_golden_validator
```

Linux/macOS:

```bash
export PYTHONPATH=tools
python -m unittest \
  tests.convert.mujoco.schema_driven_validation.test_golden_validator
```

---

## 4. 現在の fixture

### 正常系

- `mjcf`
- `adapter_artifact`
- `source_raw`
- `fallback(warn)`

### Golden data

- expected diagnostics
- expected conversion report

---

## 5. 今後追加予定

- strict mode failure fixture
- permissive mode fixture
- unsupported fixture
- lossy conversion fixture
- Adapter Capability conflict fixture
- runtime version mismatch fixture
- CI exit code golden test
- diagnostics summary golden test
- artifact generation golden test

---

## 6. 注意点

- golden test は部分一致で比較する
- 実装内部詳細ではなく仕様保証対象を比較する
- Validation Error Code が変更された場合、golden data を更新する必要がある
- traceability matrix と不整合な fixture を追加してはならない
