"""
GENBANKファイルのドメインモデル
"""

from typing import Any, Dict, List

from pydantic import BaseModel


class GenbankFeature(BaseModel):
    """GENBANKファイルのfeatureを表すクラス"""

    feature_type: str
    location: str
    qualifiers: Dict[str, List[str]]


class GenbankRecord(BaseModel):
    """GENBANKファイルのレコードを表すクラス"""

    locus: str
    size: int
    molecule_type: str
    genbank_division: str
    modification_date: str
    definition: str
    accession: str
    version: str
    keywords: List[str]
    source: str
    organism: str
    taxonomy: str
    references: List[Dict[str, Any]]
    features: List[GenbankFeature]
    sequence: str
    comment: str = ""
    primary: str = ""

    def to_file(self, file_path: str, abbreviate_sequence: bool = True) -> None:
        """
        レコードをJSONファイルに保存する

        Args:
            file_path: 保存先のファイルパス
            abbreviate_sequence: 配列データを省略するかどうか
        """
        import json
        import os

        os.makedirs(os.path.dirname(os.path.abspath(file_path)), exist_ok=True)

        record_dict = self.model_dump()

        record_dict["features"] = [
            {
                "feature_type": f.feature_type,
                "location": f.location,
                "qualifiers": f.qualifiers,
            }
            for f in self.features
        ]

        with open(file_path, "w", encoding="utf-8") as f:
            json.dump(record_dict, f, ensure_ascii=False, indent=2)
