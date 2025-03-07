"""
配列リポジトリのインターフェース
"""

from abc import ABC, abstractmethod
from typing import List

from src.domain.models.fasta import FastaRecord
from src.domain.models.genbank import GenbankRecord


class SequenceRepository(ABC):
    """配列データのリポジトリインターフェース"""

    @abstractmethod
    async def get_genbank_records(self, file_path: str) -> List[GenbankRecord]:
        """GENBANKレコードを取得する"""
        pass

    @abstractmethod
    async def get_fasta_records(self, file_path: str) -> List[FastaRecord]:
        """FASTAレコードを取得する"""
        pass

    @abstractmethod
    async def save_genbank_record(self, record: GenbankRecord, file_path: str) -> str:
        """GENBANKレコードを保存する"""
        pass

    @abstractmethod
    async def save_fasta_record(self, record: FastaRecord, file_path: str) -> str:
        """FASTAレコードを保存する"""
        pass
