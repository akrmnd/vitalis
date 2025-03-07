"""
ファイル操作ユーティリティ
"""

import os
from enum import Enum
from typing import BinaryIO

import aiofiles


class FileFormat(Enum):
    """サポートされているファイル形式"""

    GENBANK = "genbank"
    FASTA = "fasta"
    UNKNOWN = "unknown"


def detect_file_format(file_path: str) -> FileFormat:
    """
    ファイルの内容からファイル形式を判別する

    Args:
        file_path: 判別するファイルのパス

    Returns:
        判別されたファイル形式
    """
    try:
        with open(file_path, "r", encoding="utf-8") as f:
            content = f.read(1024)  # 最初の1KBを読み込む

            # FASTAの特徴: >で始まる行があり、その後にシーケンスデータがある
            if ">" in content and any(
                line.strip() and not line.startswith(">") for line in content.split("\n") if line.strip()
            ):
                return FileFormat.FASTA

            # GENBANKの特徴: LOCUSで始まり、特定のセクションがある
            if "LOCUS" in content and any(keyword in content for keyword in ["DEFINITION", "ACCESSION", "VERSION"]):
                return FileFormat.GENBANK

        # より詳細な解析が必要な場合
        with open(file_path, "r", encoding="utf-8") as f:
            lines = [line.strip() for line in f.readlines()[:20]]  # 最初の20行

            # FASTAの特徴をより詳細に確認
            if any(line.startswith(">") for line in lines):
                return FileFormat.FASTA

            # GENBANKの特徴をより詳細に確認
            if any(line.startswith("LOCUS") for line in lines):
                return FileFormat.GENBANK

        return FileFormat.UNKNOWN
    except Exception as e:
        print(f"ファイル形式の判別中にエラーが発生しました: {e}")
        return FileFormat.UNKNOWN


def save_uploaded_file(file: BinaryIO, filename: str, upload_dir: str) -> str:
    """
    アップロードされたファイルを保存する

    Args:
        file: アップロードされたファイルオブジェクト
        filename: ファイル名
        upload_dir: アップロードディレクトリ

    Returns:
        保存されたファイルのパス
    """
    os.makedirs(upload_dir, exist_ok=True)
    file_path = os.path.join(upload_dir, filename)

    with open(file_path, "wb") as f:
        f.write(file.read())

    return file_path


async def save_uploaded_file_async(content: bytes, filename: str, upload_dir: str) -> str:
    """
    アップロードされたファイルを非同期で保存する

    Args:
        content: ファイルの内容
        filename: ファイル名
        upload_dir: アップロードディレクトリ

    Returns:
        保存されたファイルのパス
    """
    os.makedirs(upload_dir, exist_ok=True)
    file_path = os.path.join(upload_dir, filename)

    async with aiofiles.open(file_path, "wb") as f:
        await f.write(content)

    return file_path
