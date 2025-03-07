"""
vitalis API
"""

import os
from typing import Dict

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from src.config.settings import Settings
from src.interfaces.api.routes import sequence_routes

settings = Settings()

app = FastAPI(
    title="vitalis API",
    description="遺伝子配列データの解析と可視化のためのAPI",
    version="0.1.0",
)

# CORSミドルウェアの設定
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.CORS_ORIGINS,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# ルーターの登録
app.include_router(sequence_routes.router)

# アップロードディレクトリと出力ディレクトリの作成
os.makedirs(settings.UPLOAD_DIR, exist_ok=True)
os.makedirs(settings.OUTPUT_DIR, exist_ok=True)


@app.get("/")
async def root() -> Dict[str, str]:
    """ルートエンドポイント"""
    return {"message": "vitalis APIへようこそ"}


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        "src.interfaces.api.main:app",
        host=settings.API_HOST,
        port=settings.API_PORT,
        reload=True,
    )
