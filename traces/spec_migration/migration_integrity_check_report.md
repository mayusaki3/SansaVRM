# migration integrity check report

## 1. 目的

本レポートは、SansaVRM 仕様再配置 dry-run における orphan / duplicate / sec_id continuity の確認結果を整理する。

本レポートは削除指示ではない。

---

## 2. 検査対象

検査対象は以下とする。

```text
- docs/ja-JP/02_仕様/01_共通/
- docs/ja-JP/02_仕様/10_Core Semantic Layer/
- docs/ja-JP/02_仕様/20_Preservation Compatibility Layer/
- docs/ja-JP/02_仕様/30_Data Model Layer/
- docs/ja-JP/02_仕様/40_Runtime Integration Layer/
- docs/ja-JP/02_仕様/50_Validation Layer/
- traces/spec_migration/migration_manifest.dry-run.json
- traces/spec_migration/migration_manifest.validation-layer.dry-run.json
- traces/spec_migration/legacy_path_cleanup_classification.md
```

---

## 3. manifest 状態

現在、manifest は以下に分離されている。

```text
main manifest:
traces/spec_migration/migration_manifest.dry-run.json

validation layer manifest:
traces/spec_migration/migration_manifest.validation-layer.dry-run.json
```

### 判定

```text
状態: federated manifest
```

単一 manifest 未統合のため、最終cleanup前に統合または federation validator が必要。

---

## 4. duplicate path 検査

### 4.1 新path側

新Layer path 上では、同一 path の重複は確認されていない。

### 4.2 旧path / 新path 並存

旧pathと新pathには、semantic duplicate が意図的に存在する。

例：

```text
01_共通/05_Validator実装仕様.md
50_Validation Layer/5002_Validator実装仕様.md
```

### 判定

```text
状態: intentional semantic duplication
cleanup前にlegacy alias化が必要
```

---

## 5. duplicate doc_id 検査

### 5.1 旧path / 新path

新Layer側では dry-doc-* を使用しており、旧path doc_id とは重複していない。

### 5.2 注意点

dry-doc-* は暫定 doc_id であり、正式化時に doc_id policy を決定する必要がある。

### 判定

```text
状態: duplicateなし。ただしdry-doc正式化方針未決
```

---

## 6. orphan specification 検査

### 6.1 新Layer目次登録

以下の新Layerは `docs/ja-JP/目次.md` に登録済み。

```text
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
40_Runtime Integration Layer
50_Validation Layer
```

### 6.2 manifest未登録

以下は main manifest には未登録だが、validation-layer manifest に登録済み。

```text
5000_Validation Layer Index.md
5001_JSONスキーマ仕様.md
5002_Validator実装仕様.md
5003_変換仕様.md
5004_Diagnostics仕様.md
5005_Traceability Migration仕様.md
5006_Coverage Analysis仕様.md
5007_Loss Report仕様.md
5008_Compatibility Analysis仕様.md
5009_RoundTrip Verification仕様.md
```

### 判定

```text
状態: orphanではない。ただしmanifest federation状態
```

---

## 7. sec_id continuity 検査

### 7.1 sec_id preserved

以下は sec_id を保持して移行済み。

```text
5002_Validator実装仕様.md
5003_変換仕様.md
```

### 7.2 sec_id missing / none

以下は旧仕様または新仕様に sec_id がない、または sec_mappings が空である。

```text
1001_Core Semantic Definition.md
1002_Semantic Preservation Matrix.md
1003_RoundTrip Semantic Criteria.md
2001_Adapter Extension Property Specification.md
2002_Format Compatibility Preservation Specification.md
2003_Conversion Profile Specification.md
3001_Geometry Rig Skinning Extension Specification.md
3002_Morph Extension Specification.md
3003_Animation Extension Specification.md
3004_Physics Extension Specification.md
4001_物理・制御メタモデル仕様.md
```

### 7.3 placeholder のため未確認

以下は placeholder relocation のため sec_id continuity 未確認。

```text
4002_MuJoCo連携仕様.md
5001_JSONスキーマ仕様.md
```

### 判定

```text
状態: cleanup前にsec_id policy確認が必要
```

---

## 8. placeholder relocation 検査

以下は placeholder relocation として扱う。

```text
4002_MuJoCo連携仕様.md
5001_JSONスキーマ仕様.md
```

### 判定

```text
状態: 旧path削除禁止
```

---

## 9. cleanup block list

以下は現時点で cleanup 禁止。

```text
01_仕様概要.md
02_メタモデル仕様.md
03_glTF拡張仕様.md
04_JSONスキーマ仕様.md
06_CoreAPI仕様.md
09_MuJoCo連携仕様.md
```

---

## 10. legacy alias 候補

以下は verified 後に alias 化を検討できる。

```text
05_Validator実装仕様.md
07_変換仕様.md
08_物理・制御メタモデル仕様.md
10_Core Semantic Definition.md
11_Semantic Preservation Matrix.md
12_RoundTrip Semantic Criteria.md
13_Adapter Extension Property Specification.md
14_Format Compatibility Preservation Specification.md
15_Geometry Rig Skinning Extension Specification.md
16_Morph Extension Specification.md
17_Animation Extension Specification.md
18_Physics Extension Specification.md
19_Conversion Profile Specification.md
```

---

## 11. reorder readiness

### 準備済み

```text
- 新Layer path 作成済み
- 目次登録済み
- Validation Layer index 作成済み
- cleanup分類表作成済み
```

### 未完了

```text
- main manifest / validation manifest federation policy
- sec_id policy
- placeholder全文移行
- legacy alias policy
```

---

## 12. 現時点判定

```text
reorder: 条件付き可能
cleanup: 不可
legacy alias: 設計可能、適用は保留
旧path削除: 不可
```

---

## 13. HLDocS feedback

本検査で得られた知見：

```text
- manifest federation が必要
- semantic duplicate と path duplicate を区別すべき
- dry-doc の正式化 policy が必要
- cleanup block list が必要
- placeholder relocation は旧path削除禁止にすべき
- sec_id continuity は cleanup gate に含めるべき
```
