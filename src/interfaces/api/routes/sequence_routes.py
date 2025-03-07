"""
配列データのAPIルート
"""

import os
from typing import Dict, List, Optional

from fastapi import (
    APIRouter,
    BackgroundTasks,
    Depends,
    File,
    Form,
    HTTPException,
    UploadFile,
)

from src.application.dtos.sequence_dto import FastaRecordDTO, GenbankRecordDTO
from src.application.services.sequence_service import SequenceService
from src.config.settings import Settings
from src.infrastructure.utils.file_utils import save_uploaded_file_async
from src.interfaces.api.dependencies import get_sequence_service

router = APIRouter(prefix="/sequence", tags=["sequence"])
settings = Settings()
sequence_service_dependency = Depends(get_sequence_service)
file_dependency = File(...)
form_none_dependency = Form(None)


@router.post("/parse", response_model=List[GenbankRecordDTO | FastaRecordDTO])
async def parse_sequence_file(
    background_tasks: BackgroundTasks,
    file: UploadFile = file_dependency,
    file_format: Optional[str] = form_none_dependency,  # ファイル形式を明示的に指定できるオプション
    service: SequenceService = sequence_service_dependency,
) -> List[GenbankRecordDTO | FastaRecordDTO]:
    """
    配列ファイルをパースする

    - **file**: アップロードするファイル（GENBANKまたはFASTA形式）
    - **file_format**: ファイル形式（"genbank"または"fasta"、指定がない場合は自動判別）
    """
    # ファイル形式の検証
    if file_format and file_format.lower() not in ["genbank", "fasta"]:
        raise HTTPException(
            status_code=400,
            detail="サポートされていないファイル形式です。'genbank'または'fasta'を指定してください。",
        )

    # 一時ファイルに保存
    if not file.filename:
        raise HTTPException(status_code=400, detail="ファイル名が指定されていません")

    temp_file_path = os.path.join(settings.UPLOAD_DIR, file.filename)
    await save_uploaded_file_async(await file.read(), file.filename, settings.UPLOAD_DIR)

    # バックグラウンドタスクで一時ファイルを削除
    background_tasks.add_task(os.remove, temp_file_path)

    try:
        records = await service.parse_file(temp_file_path, file_format)
        return records
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"ファイルのパースに失敗しました: {str(e)}") from e


@router.post("/save/genbank", response_model=Dict[str, str])
async def save_genbank_record(
    record: GenbankRecordDTO,
    service: SequenceService = sequence_service_dependency,
) -> Dict[str, str]:
    """GENBANKレコードを保存する"""
    try:
        file_path = os.path.join(settings.OUTPUT_DIR, f"{record.locus}.json")
        saved_path = await service.save_record(record, file_path)
        return {"file_path": saved_path}
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"レコードの保存に失敗しました: {str(e)}") from e


@router.post("/save/fasta", response_model=Dict[str, str])
async def save_fasta_record(
    record: FastaRecordDTO,
    service: SequenceService = sequence_service_dependency,
) -> Dict[str, str]:
    """FASTAレコードを保存する"""
    try:
        file_path = os.path.join(settings.OUTPUT_DIR, f"{record.header}.fasta")
        saved_path = await service.save_record(record, file_path)
        return {"file_path": saved_path}
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"レコードの保存に失敗しました: {str(e)}") from e
