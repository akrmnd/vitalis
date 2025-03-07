"""
FastAPI依存性注入
"""

from src.application.services.sequence_service import SequenceService
from src.domain.repositories.sequence_repository import SequenceRepository
from src.infrastructure.repositories.file_sequence_repository import (
    FileSequenceRepository,
)


def get_sequence_repository() -> SequenceRepository:
    """配列リポジトリを取得する"""
    return FileSequenceRepository()


def get_sequence_service() -> SequenceService:
    """配列サービスを取得する"""
    repository = get_sequence_repository()
    return SequenceService(repository)
