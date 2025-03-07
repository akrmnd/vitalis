FROM python:3.12-slim

WORKDIR /opt/app

# 依存関係のインストール
COPY pyproject.toml poetry.lock* README.md /opt/app/
# アプリケーションのコピー
COPY ./src /opt/app/src

RUN pip install poetry && \
    poetry config virtualenvs.create false && \
    poetry install --only main

# 環境変数の設定
ENV PYTHONPATH=/opt/app

# ディレクトリの作成
RUN mkdir -p /opt/app/uploads /opt/app/output

# 実行ユーザーの設定
RUN useradd -m appuser && \
    chown -R appuser:appuser /opt/app
USER appuser

# アプリケーションの実行
CMD uvicorn src.interfaces.api.main:app --host ${API_HOST} --port ${API_PORT}