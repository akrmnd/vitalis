"""
アプリケーション設定
"""

from typing import List

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """アプリケーション設定"""

    # APIの設定
    API_HOST: str = "localhost"
    API_PORT: int = 8000

    # CORSの設定
    CORS_ORIGINS: List[str] = ["http://localhost:3000"]

    # ファイルの設定
    UPLOAD_DIR: str = "uploads"
    OUTPUT_DIR: str = "output"

    # 環境変数の設定
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
    )
