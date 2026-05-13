# post relocation policy

## 1. 目的

本ドキュメントは、SansaVRM 仕様再配置 dry-run 後に必要となる後段ポリシーを定義する。

対象は以下である。

```text
- legacy alias policy
- dry-doc formalization policy
- manifest federation policy
- reorder policy
- cleanup gate
```

---

## 2. 基本方針

仕様再配置は、ファイル移動だけではなく traceability graph の移行として扱う。

以下を原則とする。

```text
1. 旧pathを即削除しない
2. legacy alias phase を設ける
3. dry-doc は暫定IDとして扱う
4. manifest は federation を許可する
5. reorder は dependency ordering として扱う
6. cleanup は verified 後の別フェーズとする
```

Layer番号は dependency 意味論ではなく、ファイルシステム上の並び順制御として扱う。

Layer dependency は、Layer Index、dependency diagram、migration manifest、および本文定義によって定義する。

---

## 3. legacy alias policy

legacy alias は、旧path互換のために残す案内文書である。

legacy alias は以下を目的とする。

```text
- 旧リンク互換
- canonical path 案内
- migration continuity の可視化
- cleanup safety
```

---

## 4. legacy alias template

legacy alias 化する場合、旧path文書は以下の構造へ置換する。

```text
HLDocS block: old_doc_id を維持
canonical_document: false
本文: 新Layer pathへの移行通知
移行情報: old_doc_id / new_doc_id / migration_entry_id / semantic_equivalent / mapping_status
```

---

## 5. legacy alias 適用条件

legacy alias は以下を満たす場合のみ適用できる。

```text
- migration manifest 登録済み
- semantic_equivalent = true
- mapping_status = verified
- placeholder_only ではない
- sec_id が存在する場合は sec_mappings 登録済み
- 目次が新Layer pathを参照済み
- CI validation 通過済み
```

---

## 6. legacy alias 禁止条件

以下の場合は legacy alias 化しない。

```text
- placeholder_only
- migrated_partial
- semantic_equivalent = unknown
- sec_id continuity 未確認
- 正式仕様セット上で旧pathがまだ正本扱い
```

---

## 7. dry-doc formalization policy

`dry-doc-*` は dry-run relocation 用の暫定 doc_id である。

正式化時には以下のいずれかを選択する。

```text
A. dry-doc を維持する
B. 正式 doc_id を再発行する
C. dry-doc を migration session identifier として保持し、canonical doc_id を再発行する
```

推奨は C とする。

---

## 8. canonical doc_id 再発行方針

canonical doc_id を再発行する場合、migration manifest に以下を記録する。

```text
dry_doc_id
canonical_doc_id
old_doc_id
migration_entry_id
semantic_equivalent
formalized_at
```

---

## 9. sec_id policy

sec_id は可能な限り保持する。

ただし、旧文書に sec_id が存在しない場合は以下を許可する。

```text
- new_sec_id generation
- sec_id none record
- semantic block mapping
```

sec_id 不在文書を cleanup する場合でも、migration manifest に `sec_mappings: []` の理由を記録する。

---

## 10. manifest federation policy

大規模 relocation では、manifest を分割できる。

例：

```text
migration_manifest.dry-run.json
migration_manifest.validation-layer.dry-run.json
```

---

## 11. manifest federation requirements

manifest federation を採用する場合、以下を満たす。

```text
- federation root manifest を用意する
- sub-manifest の migration_id を一意にする
- entry_id 衝突を禁止する
- path 衝突を検出する
- doc_id 衝突を検出する
- mapping_status summary を生成する
```

---

## 12. reorder policy

reorder は dependency ordering として扱う。

現在の Layer番号は、dependency 意味論ではなく並び順制御として扱う。

```text
10_Core Semantic Layer
20_Preservation Compatibility Layer
30_Data Model Layer
40_Runtime Integration Layer
50_Validation Layer
60_Import Export Layer
70_Roadmap Layer
```

Layer dependency は番号ではなく、dependency diagram および Layer Index により定義する。

10刻みは、将来の中間 Layer 挿入余地を残すための filesystem ordering である。

---

## 13. cleanup gate

旧path cleanup は以下を満たすまで実施しない。

```text
- manifest federation policy 決定済み
- dry-doc formalization policy 決定済み
- placeholder relocation 解消済み
- legacy alias policy 適用済み
- sec_id continuity 確認済み
- CI validation 通過済み
```

---

## 14. 現時点の判断

```text
reorder: 条件付き可能
legacy alias: 設計完了、適用保留
cleanup: 不可
旧path削除: 不可
```

---

## 15. HLDocS feedback

本ポリシーで得られた知見：

```text
- relocation 後に alias phase が必要
- dry-run doc_id と canonical doc_id を分けるべき
- manifest federation が必要
- reorder は dependency ordering として扱うべき
- Layer番号は dependency 意味論ではなく並び順制御として扱うべき
- cleanup は migration verification 後の別フェーズにすべき
```
