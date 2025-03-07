"""
配列データの処理サービス
"""

from typing import List

from src.application.dtos.sequence_dto import FastaRecordDTO, GenbankRecordDTO
from src.domain.models.fasta import FastaRecord
from src.domain.models.genbank import GenbankRecord
from src.domain.repositories.sequence_repository import SequenceRepository
from src.infrastructure.utils.file_utils import FileFormat, detect_file_format


class SequenceService:
    """
    配列データの処理サービス
    """

    def __init__(self, repository: SequenceRepository):
        self.repository = repository

    async def parse_file(
        self, file_path: str, format_hint: str | None = None
    ) -> List[GenbankRecordDTO | FastaRecordDTO]:
        """
        配列ファイルをパースする

        Args:
            file_path: パースするファイルのパス
            format_hint: ファイル形式のヒント（指定がない場合は自動判別）

        Returns:
            パースされたレコードのリスト
        """
        # ファイル形式の判別
        if format_hint:
            file_format = FileFormat(format_hint.lower())
        else:
            file_format = detect_file_format(file_path)

        # 形式に応じたパース処理
        if file_format == FileFormat.GENBANK:
            genbank_records = await self.repository.get_genbank_records(file_path)
            return [self._to_genbank_dto(record) for record in genbank_records]
        elif file_format == FileFormat.FASTA:
            fasta_records = await self.repository.get_fasta_records(file_path)
            return [self._to_fasta_dto(record) for record in fasta_records]
        else:
            raise ValueError(f"サポートされていないファイル形式です: {file_path}")

    async def save_record(self, record: GenbankRecordDTO | FastaRecordDTO, file_path: str) -> str:
        """
        レコードをファイルに保存する

        Args:
            record: 保存するレコード
            file_path: 保存先のファイルパス

        Returns:
            保存したファイルのパス
        """
        if isinstance(record, GenbankRecordDTO):
            genbank_record = self._from_genbank_dto(record)
            return await self.repository.save_genbank_record(genbank_record, file_path)
        elif isinstance(record, FastaRecordDTO):
            fasta_record = self._from_fasta_dto(record)
            return await self.repository.save_fasta_record(fasta_record, file_path)
        else:
            raise ValueError(f"サポートされていないレコード型です: {type(record)}")

    # 変換メソッド（既存のサービスから移植）
    def _to_genbank_dto(self, record: GenbankRecord) -> GenbankRecordDTO:
        return GenbankRecordDTO.model_validate(record.model_dump())

    def _from_genbank_dto(self, dto: GenbankRecordDTO) -> GenbankRecord:
        return GenbankRecord.model_validate(dto.model_dump())

    def _to_fasta_dto(self, record: FastaRecord) -> FastaRecordDTO:
        return FastaRecordDTO.model_validate(record.model_dump())

    def _from_fasta_dto(self, dto: FastaRecordDTO) -> FastaRecord:
        return FastaRecord.model_validate(dto.model_dump())
