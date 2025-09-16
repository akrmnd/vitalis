# Vitalis Studio

_[English](README.md)で読む_

TauriとRustで構築されたDNA/RNA配列解析・可視化デスクトップアプリケーション

## 機能

- FASTA/FASTQ/GenBankファイルの解析とインポート
- 大規模配列の効率的な処理
- 配列検索と翻訳
- ORF（オープンリーディングフレーム）検出
- プライマー設計と解析
- 制限酵素サイト解析
- 配列の可視化（線形・環状）
- 各種フォーマットへのエクスポート（FASTA、FASTQ、GenBank、SVG、PDF）

## 開発環境のセットアップ

### 前提条件

- Node.js 18+
- Rust 1.70+
- npm または pnpm

### ローカル開発

1. リポジトリをクローン

   ```bash
   git clone <repository-url>
   cd vitalis
   ```

2. 依存関係のインストール

   ```bash
   npm install
   ```

3. 開発サーバーの起動

   ```bash
   npm run tauri dev
   ```

### プロダクションビルド

```bash
npm run tauri build
```

## プロジェクト構造

```
vitalis/
├── src/                      # フロントエンド (React/TypeScript)
│   ├── App.tsx              # メインアプリケーションコンポーネント
│   ├── main.tsx             # アプリケーションエントリーポイント
│   └── styles.css           # グローバルスタイル
├── src-tauri/               # Tauriバックエンド
│   ├── src/
│   │   └── main.rs          # Tauriアプリケーションエントリー
│   ├── Cargo.toml           # Rust依存関係
│   └── tauri.conf.json      # Tauri設定
├── vitalis-core/            # コアRustライブラリ
│   ├── src/
│   │   ├── lib.rs           # ライブラリルート
│   │   ├── sequence.rs      # 配列データ構造
│   │   ├── feature.rs       # 注釈タイプ
│   │   ├── io/              # ファイルI/Oモジュール
│   │   ├── analysis/        # 解析アルゴリズム
│   │   └── visualization/   # レンダリングモジュール
│   └── Cargo.toml           # コアライブラリ依存関係
├── docs/                    # ドキュメント
├── package.json             # Node.js依存関係
├── tsconfig.json            # TypeScript設定
├── vite.config.ts           # Vite設定
└── README.md                # このファイル
```

## 技術スタック

### バックエンド
- **Rust**: コアライブラリ (`vitalis-core`)
- **Tauri**: デスクトップアプリケーションフレームワーク
- **noodles**: バイオインフォマティクスファイル形式（FASTA/FASTQ）
- **SQLite**: ローカルデータストレージ

### フロントエンド
- **React**: UIフレームワーク
- **TypeScript**: 型安全性
- **Vite**: ビルドツール

## コアAPI

### 配列I/O
- `parse_and_import`: ファイルから配列をインポート
- `export`: 各種フォーマットへの配列エクスポート

### 配列操作
- `get_meta`: 配列メタデータの取得
- `get_window`: 大規模ファイル用の配列ウィンドウ取得
- `stats`: 配列統計の計算（GC%、N比率）

### 解析
- `search`: 配列内のパターン検索
- `translate`: DNA/RNAからタンパク質への翻訳
- `find_orf`: オープンリーディングフレームの検出
- `restriction_sites`: 制限酵素サイトの検索

### 可視化
- `render_linear_svg`: 線形配列マップの生成
- `render_plasmid_svg`: 環状プラスミドマップの生成
- `export_pdf`: PDFへの可視化エクスポート

## パフォーマンス目標

- FASTA 100kb読み込み: < 400ms
- 1Mbp内の10-mer検索: < 300ms
- UIスクロール: 60fps
- Undo/Redo: 即座に反映

## テスト実行

```bash
# Rustテスト
cargo test

# フロントエンドテスト
npm test

# リンティング
cargo clippy
npm run lint

# 型チェック
npm run typecheck
```

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。