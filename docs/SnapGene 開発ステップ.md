# Vitalis Studio API仕様 + 開発ステップ（noodles採用版） — ClaudeCode 指示書最適化版

この文書は **ClaudeCode** を用いた実装指示書として整理したものです。ClaudeCode に入力することで、Tauri + React/TypeScript + Rust 環境で API と開発ステップを段階的に実装できます。余計な説明は省き、**具体的な仕様・型・ステップ**を中心に記載しています。

---

## 0. 実装方針
- バックエンド: Rust (`vitalis-core` crate)
- フロント: Tauri + React/TypeScript
- I/O: `noodles`（FASTA/FASTQ対応）+ 自作GenBankパーサ
- データ取得: 巨大配列は `get_window()` による部分アクセス
- 出力形式: FASTA / FASTQ / GenBank / SVG / PDF
- 内部保存形式: SQLite + オプションで独自バイナリ(`.vitalis`)

---

## 1. Rust ドメインモデル
```rust
pub enum Topology { Circular, Linear }

pub struct Range { pub start: usize, pub end: usize }

pub struct Feature {
  pub id: String,
  pub kind: String,
  pub locs: Vec<Range>,
  pub strand: i8,
  pub qualifiers: serde_json::Map<String, serde_json::Value>,
  pub color: Option<String>,
}

pub struct Primer {
  pub id: String,
  pub name: String,
  pub seq: String,
  pub tm_c: f32,
  pub gc: f32,
  pub notes: String,
}

pub struct SequenceMeta {
  pub id: String,
  pub name: String,
  pub length: usize,
  pub topology: Topology,
}
```

---

## 2. 公開 API (Tauri Commands)

### I/O・メタ・ウィンドウ
- `parse_and_import(text: String, fmt: "fasta"|"fastq"|"genbank") -> { seq_id }`
- `export(seq_id, fmt) -> { text }`
- `get_meta(seq_id) -> SequenceMeta`
- `get_window(seq_id, start, end) -> { bases }`
- `stats(seq_id) -> { gc_overall, n_rate, length }`

### 注釈
- `list_features(seq_id) -> Feature[]`
- `add_feature(seq_id, feature) -> { feature_id }`
- `update_feature(seq_id, feature) -> {}`
- `remove_feature(seq_id, feature_id) -> {}`

### 検索・翻訳
- `search(seq_id, pattern, iupac: bool) -> Range[]`
- `translate(seq_id, frame: i8) -> { aa }`
- `find_orf(seq_id, min_len) -> Range[]`

### プライマー
- `design_primers(seq_id, target: Range, params) -> Primer[]`
- `analyze_primer(primer: Primer) -> { hairpin, dimer, notes }`
- `attach_primer(seq_id, primer: Primer) -> { primer_id }`
- `list_primers(seq_id) -> Primer[]`

### 制限酵素（後続）
- `restriction_sites(seq_id, enzymes) -> { enzyme, positions[] }[]`
- `digest(seq_id, enzymes) -> DigestBand[]`

### レンダリング
- `render_linear_svg(seq_id, options) -> { svg_text }`
- `render_plasmid_svg(seq_id, options) -> { svg_text }`
- `export_pdf(svg_texts) -> { pdf_path }`

### プロジェクト/履歴
- `recent(limit) -> { items[] }`
- `tag(seq_id, tag, add) -> {}`
- `star(seq_id, on) -> {}`
- `undo()` / `redo()` / `history_log() -> { entries[] }`

### エラー形式
```rust
AppError {
  code: "InvalidFormat"|"Unsupported"|"OutOfRange"|"Conflict"|"TooLarge"|"Internal",
  message: String,
  details?: any
}
```

---

## 3. 開発ステップ（ClaudeCode 実装単位）

### Phase 0 (W1)
- Tauri + React 雛形生成
- `vitalis-core` crate 雛形作成
- noodles (FASTA/FASTQ) 導入 → `parse_and_import` MVP

### Phase 1 (W2)
- `get_window` 実装（IndexedReader利用）
- `stats` (GC%, N率) 実装
- フロントで仮想スクロールビュー構築

### Phase 2 (W3–4)
- Feature CRUD (`list/add/update/remove_feature`)
- `search` (IUPAC対応)

### Phase 3 (W5–6)
- GenBank 最小パーサ（LOCUS, ORIGIN）
- `translate`, `find_orf`

### Phase 4 (W7–8)
- プライマー API 実装
- 制限酵素 API (sites, digest) P0

### Phase 5 (W9)
- SVG/PDF 出力 API 実装
- フロントで図出力機能確認

### Phase 6 (W10+)
- minimizer/k-mer インデックス導入
- 長大配列性能評価（>5 Mbp）

---

## 4. 受入基準
- **FASTA 100 kb**: 読込 + 先頭表示 < 400ms
- **検索 (10-mer, 1 Mbp)**: 応答 < 300ms
- **スクロール**: 60fps 維持
- **Undo/Redo**: 操作履歴が正しく反映
- **出力**: SVG/PDF の外部確認可能

---

## 5. 実装補足
- **FASTA/FASTQ**: noodles 利用
- **GenBank**: 独自実装（段階拡張）
- **内部保存**: SQLite + `.vitalis` バイナリ
- **描画**: Canvas 2D (LOD/差分描画)
- **索引**: minimizer/k-mer スケッチ

---

この仕様を ClaudeCode に入力することで、各 Phase ごとに実装を進めることができます。

