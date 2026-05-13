# HLDocS compatibility audit after common spec update

## 1. 目的

本レポートは、HLDocS 共通仕様の正本更新後に、SansaVRM の dry-run relocation / Layer reorder / migration policy が新HLDocS共通仕様に適合しているかを監査する。

参照するHLDocS正本：

```text
https://github.com/mayusaki3/HLDocS/tree/develop/docs/ja-JP/仕様/00_共通
```

---

## 2. 参照したHLDocS共通仕様

今回の監査では、特に以下を重視する。

```text
03_共通ドキュメント構造.md
04_LLM-MANAGEDブロック規約.md
06_Traceability規約.md
08_再作成時不変条件規約.md
```

---

## 3. 重要な新前提

### 3.1 ファイル名・ディレクトリ・番号は意味論を持たない

新HLDocSでは、ファイル名・ディレクトリ構成・番号は意味論を持たないと明示されている。

### SansaVRMへの影響

現在の `10_Core Semantic Layer` 等の番号付き Layer は、ファイルシステム上の並び順制御として扱う必要がある。

したがって、以下の表現は要注意：

```text
Layer番号が仕様意味を持つ
10/20/30/40/50 が仕様上の意味論そのものを表す
```

今後は以下へ修正する。

```text
Layer番号は並び順制御用
Layer名と本文定義が責務を表す
依存関係は本文またはmanifestで定義する
```

---

## 4. doc_id 方針への影響

### 4.1 HLDocS正本の要点

LLM-MANAGEDブロック規約では、doc_id は恒久的ドキュメント識別子であり、ファイル名・ディレクトリ・番号・内容・意味から独立する。

また、内容更新・移動・改名では doc_id は変更しない。

一方、分割・統合・置換・新規作成の場合は新しい doc_id を割り当ててよい。

---

### 4.2 SansaVRM現状との不一致

現在の dry-run relocation では、多くの移行先文書に `dry-doc-*` を付与している。

これは、単純な移動・改名として扱う場合、HLDocS方針と不一致となる可能性が高い。

対象例：

```text
1001_Core Semantic Definition.md
2001_Adapter Extension Property Specification.md
3001_Geometry Rig Skinning Extension Specification.md
5002_Validator実装仕様.md
6001_VRM 0.x 1.0 差分整理.md
```

---

### 4.3 修正方針

今後の方針は以下とする。

```text
単純 relocation:
旧 doc_id を維持する

dry-run 管理ID:
doc_id ではなく migration manifest 側に dry_run_id / migration_entry_id として保持する

新規仕様:
新しい doc_id を発行してよい

分割・統合:
新しい doc_id を発行してよいが、manifestに old_doc_id / new_doc_id mapping を記録する
```

---

## 5. canonical_document 方針への影響

### 5.1 現状問題

現在、旧pathと新pathの両方に `canonical_document: true` が存在する dual canonical state が発生している。

これは dry-run中の一時状態としては説明可能だが、恒久状態としては危険である。

---

### 5.2 修正方針

```text
旧path:
legacy alias 化後は canonical_document: false

新path:
正式移行後に canonical_document: true

dry-run中:
一時的 dual canonical として manifest に明示
```

ただし、legacy alias 化は placeholder relocation が解消し、semantic_equivalent verified 後にのみ行う。

---

## 6. document_type 方針への影響

### 6.1 HLDocS正本の要点

document_type の許容値は以下のみである。

```text
spec
index
template
prompt
testspec
note
minutes
usage
```

---

### 6.2 SansaVRM現状

現在作成した文書の document_type は概ね `spec` または `index` であり、許容範囲内。

ただし、以下は再確認対象：

```text
旧path cleanup計画
Layer reorder最終計画
post relocation policy
migration integrity check report
```

これらは `spec` より `note` または `usage` の方が適切な可能性がある。

---

## 7. sec_id 方針への影響

### 7.1 HLDocS正本の要点

Traceability規約では、sec_id は testspec を起点として初出・確定する。

spec 単体生成時に sec_id を付与してはならない。

対応する testspec を持たない sec_id を定義してはならない。

---

### 7.2 SansaVRM現状との不一致

現在の migration manifest では、既存文書から移行した sec_id を保持した箇所がある。

既存 sec_id の保持は許容される可能性があるが、以下は新HLDocS上で要注意：

```text
specのみの判断で新sec_idを追加する
sec_id missingを理由にspec側へ新規sec_idを生成する
```

これは禁止する。

---

### 7.3 修正方針

```text
既存 sec_id:
変更せず保持する

sec_id がない spec:
新規生成しない

sec_id continuity:
存在するsec_idのみmappingする

sec_id missing:
欠落ではなく none / not_applicable としてmanifestに記録する
```

---

## 8. 参照形式への影響

### 8.1 HLDocS正本の要点

HLDocS成果物本文における機械参照は `doc_id#sec_id` のみ許可される。

ただし、人間向けナビゲーションMarkdownリンクは別目的であり、本制約の対象外である。

---

### 8.2 SansaVRM現状

現在の生成文書には、関連仕様を自然文や text block として列挙している箇所がある。

これは機械参照ではないため、ただちに違反とは限らない。

ただし、機械参照として扱う場合は、相対パスやURLではなく `doc_id#sec_id` が必要になる。

---

## 9. 再作成・patch 方針への影響

### 9.1 HLDocS正本の要点

recreate / patch は、既存 doc_id を保持した同一文書の改版であり、修正前正本の本文実体が必要。

修正前正本がない場合、new に読み替えてはいけない。

---

### 9.2 SansaVRM現状

現在の `dry-run relocation` は、実態として以下が混在している。

```text
- new document
- path relocation
- placeholder relocation
- full copy relocation
```

これらを operation として明確に分離する必要がある。

---

## 10. 番号付き並び替えへの影響

### 10.1 HLDocS正本の要点

番号はファイルシステム上の並び順制御目的に限り付与可能。

番号は意味・役割・仕様上の区別を一切持たない。

---

### 10.2 SansaVRM修正方針

以下の表現を修正する。

```text
NG:
10_Core Semantic Layer は番号10なので上流である

OK:
Layer dependency diagram により Core Semantic Layer を上流と定義する
番号10は並び順制御用であり意味論を持たない
```

---

## 11. 直近修正が必要なSansaVRM文書

以下は監査・修正対象とする。

```text
traces/spec_migration/post_relocation_policy.md
docs/ja-JP/04_実装計画/01_共通/07_Layer_reorder最終計画.md
traces/spec_migration/migration_integrity_check_report.md
traces/spec_migration/legacy_path_cleanup_classification.md
traces/spec_migration/migration_manifest.dry-run.json
traces/spec_migration/migration_manifest.validation-layer.dry-run.json
```

---

## 12. 現時点の判定

```text
Layer構造案:
継続可能。ただし番号意味論は禁止。

dry-doc方針:
要修正。単純relocationでは旧doc_id維持が原則。

sec_id方針:
要修正。spec側で新規生成禁止。存在するsec_idのみ保持。

legacy alias:
継続可能。ただし canonical_document=false とする。

cleanup:
引き続き禁止。placeholder解消・manifest整備・HLDocS適合後に判断。
```

---

## 13. 次アクション

```text
1. post_relocation_policy.md を新HLDocSに合わせて改定
2. Layer_reorder最終計画.md の番号意味論を修正
3. migration manifest の dry-doc 方針を見直し
4. sec_id missing を欠落ではなく not_applicable として扱う
5. HLDocS feedback issue draft を新仕様前提で作成
```

---

## 14. 結論

新HLDocS共通仕様により、SansaVRM の dry-run relocation 方針は以下を修正する必要がある。

```text
- doc_id は移動・改名では維持する
- dry-doc は LLM-MANAGED の doc_id ではなく migration管理IDへ移す
- sec_id は spec単体で生成しない
- 番号付きLayerは意味論ではなく並び順制御として扱う
- legacy alias は canonical_document=false とする
```

これらを反映した後に、Import Export Layer / Roadmap Layer の移行を継続する。
