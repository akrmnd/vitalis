"""
ファイルベースの配列リポジトリ実装
"""

from pathlib import Path
from typing import List

from src.domain.models.fasta import FastaRecord
from src.domain.models.genbank import GenbankRecord
from src.domain.repositories.sequence_repository import SequenceRepository
from src.infrastructure.parsers.fasta_parser import FastaParser
from src.infrastructure.parsers.genbank_parser import GenbankParser


class FileSequenceRepository(SequenceRepository):
    """ファイルベースの配列リポジトリ"""

    def __init__(self) -> None:
        self.genbank_parser = GenbankParser()
        self.fasta_parser = FastaParser()

    async def get_genbank_records(self, file_path: str) -> List[GenbankRecord]:
        """GENBANKレコードを取得する"""
        return self.genbank_parser.parse_file(file_path)

    async def get_fasta_records(self, file_path: str) -> List[FastaRecord]:
        """FASTAレコードを取得する"""
        return self.fasta_parser.parse_file(file_path)

    async def save_genbank_record(self, record: GenbankRecord, file_path: str) -> str:
        """GENBANKレコードを保存する"""
        path = Path(file_path)
        path.parent.mkdir(parents=True, exist_ok=True)
        record.to_file(str(path))
        return file_path

    async def save_fasta_record(self, record: FastaRecord, file_path: str) -> str:
        """FASTAレコードを保存する"""
        path = Path(file_path)
        path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(path, "w", encoding="utf-8") as f:
            f.write(f">{record.header}")
            if record.description:
                f.write(f" {record.description}")
            f.write("\n")

            # 配列を60文字ごとに改行
            seq = record.sequence
            for i in range(0, len(seq), 60):
                f.write(f"{seq[i:i+60]}\n")

        return file_path
