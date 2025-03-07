# Vitalis - バイオインフォマティクス API

_[English](README.md)で読む_

遺伝子配列データの解析と可視化のための API

## 機能

- GENBANK ファイルのパース
- FASTA ファイルのパース
- 配列データの保存と管理

## 開発環境のセットアップ

### 前提条件

- Python 3.12
- Poetry
- Docker と Docker Compose（オプション）

### ローカル開発

1. リポジトリをクローン

   ```bash
   git clone <repository-url>
   cd vitalis
   ```

2. 依存関係のインストール

   ```bash
   poetry install
   ```

3. 環境変数の設定

   ```bash
   cp .env.example .env
   # 必要に応じて.envファイルを編集
   ```

4. アプリケーションの実行

   ```bash
   poetry run uvicorn src.interfaces.api.main:app --reload
   ```

5. API ドキュメントへのアクセス
   ```
   http://localhost:8000/docs
   ```

### Docker を使用した実行

1. イメージのビルドと起動

   ```bash
   docker-compose up -d
   ```

2. API ドキュメントへのアクセス
   ```
   http://localhost:8000/docs
   ```

## 環境変数

| 変数名       | 説明                          | デフォルト値              |
| ------------ | ----------------------------- | ------------------------- |
| API_HOST     | API ホスト                    | localhost                 |
| API_PORT     | API ポート                    | 8000                      |
| CORS_ORIGINS | CORS オリジン（カンマ区切り） | ["http://localhost:3000"] |
| UPLOAD_DIR   | アップロードディレクトリ      | uploads                   |
| OUTPUT_DIR   | 出力ディレクトリ              | output                    |

## API エンドポイント

- `GET /`: ウェルカムメッセージ
- `POST /sequence/parse`: 配列ファイルのパース
- `POST /sequence/save/genbank`: GENBANK レコードの保存
- `POST /sequence/save/fasta`: FASTA レコードの保存

詳細な API ドキュメントは `/docs` エンドポイントで確認できます。

## プロジェクト構造

```
vitalis/
├── src/                      # ソースコード
│   ├── application/          # アプリケーション層
│   │   ├── dtos/             # データ転送オブジェクト
│   │   └── services/         # サービス
│   ├── config/               # 設定
│   ├── domain/               # ドメイン層
│   │   ├── models/           # ドメインモデル
│   │   └── repositories/     # リポジトリインターフェース
│   ├── infrastructure/       # インフラストラクチャ層
│   │   ├── parsers/          # パーサー
│   │   ├── repositories/     # リポジトリ実装
│   │   └── utils/            # ユーティリティ
│   └── interfaces/           # インターフェース層
│       └── api/              # API
├── tests/                    # テスト
│   └── data/                 # テストデータ
├── .env.example              # 環境変数サンプル
├── .gitignore                # Gitの除外設定
├── docker-compose.yml        # Docker Compose設定
├── Dockerfile                # Dockerビルド設定
├── poetry.lock               # Poetryロックファイル
├── pyproject.toml            # Poetryプロジェクト設定
└── README.md                 # プロジェクト説明
```

## 技術スタック

- **FastAPI**: 高速な API フレームワーク
- **Pydantic**: データバリデーションとシリアライゼーション
- **pathlib.Path**: モダンなファイルパス操作
- **Poetry**: 依存関係管理
- **Docker**: コンテナ化

## テスト実行

```bash
# すべてのテストを実行
poetry run pytest

# 特定のテストを実行
poetry run pytest tests/test_genbank_parser.py
```

## 貢献方法

1. このリポジトリをフォーク
2. 新しいブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチをプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。
