# Vitalis Studio - 開発指示書

## プロジェクト概要
DNA/RNA配列解析ツール「Vitalis Studio」の開発プロジェクト。Tauri + React/TypeScript + Rustで実装。

## 技術スタック
- **バックエンド**: Rust (`vitalis-core` crate)
- **フロントエンド**: Tauri + React/TypeScript
- **I/O**: noodles（FASTA/FASTQ対応）+ 独自GenBankパーサ
- **データ保存**: SQLite + 独自バイナリ形式（.vitalis）
- **出力形式**: FASTA / FASTQ / GenBank / SVG / PDF

## 主要機能
1. シーケンスI/O（FASTA/FASTQ/GenBank形式の読み書き）
2. 大規模配列の効率的な処理（部分ウィンドウアクセス）
3. 注釈（Feature）の管理
4. 配列検索・翻訳・ORF探索
5. プライマー設計・解析
6. 制限酵素サイト解析
7. 配列の可視化（SVG/PDF出力）
8. プロジェクト管理・操作履歴

## 開発フェーズ

### 現在のフェーズ: Phase 0-1
- Tauri + React環境のセットアップ
- vitalis-coreクレートの基本実装
- noodlesによるFASTA/FASTQ読み込み

### 次のステップ
1. `get_window`実装（部分配列取得）
2. 統計情報API（GC%, N率）
3. フロントエンドの仮想スクロールビュー

## コマンド

### 開発サーバー起動
```bash
npm run tauri dev
```

### ビルド
```bash
npm run tauri build
```

### テスト
```bash
cargo test
npm test
```

### リント・型チェック
```bash
cargo clippy
cargo fmt --check
npm run lint
npm run typecheck
```

## API仕様

### コアデータ型（Rust）
- `Topology`: Circular | Linear
- `Range`: 配列の範囲（start, end）
- `Feature`: 注釈情報（ID、種類、位置、ストランド、属性）
- `Primer`: プライマー情報（配列、Tm、GC%）
- `SequenceMeta`: 配列メタ情報

### 主要Tauriコマンド
- I/O: `parse_and_import`, `export`
- メタ情報: `get_meta`, `get_window`, `stats`
- 注釈: `list_features`, `add_feature`, `update_feature`, `remove_feature`
- 検索・翻訳: `search`, `translate`, `find_orf`
- プライマー: `design_primers`, `analyze_primer`, `attach_primer`
- 制限酵素: `restriction_sites`, `digest`
- 可視化: `render_linear_svg`, `render_plasmid_svg`, `export_pdf`

## パフォーマンス基準
- FASTA 100kb読み込み: < 400ms
- 10-mer検索（1Mbp配列）: < 300ms
- スクロール: 60fps維持
- Undo/Redo: 即座に反映

## ディレクトリ構造
```
vitalis/
├── src/              # フロントエンド（React/TypeScript）
├── src-tauri/        # Tauriバックエンド
│   └── src/
│       └── main.rs
├── vitalis-core/     # Rustコアライブラリ
│   └── src/
│       └── lib.rs
└── package.json
```

## 注意事項
- 巨大配列（>5Mbp）は部分アクセスで処理
- GenBankパーサーは段階的に実装
- minimizer/k-merインデックスは後期フェーズで導入