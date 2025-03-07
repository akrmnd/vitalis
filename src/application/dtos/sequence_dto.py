"""
配列データのDTO
"""

from typing import Any, Dict, List, Optional

from pydantic import BaseModel


class GenbankFeatureDTO(BaseModel):
    """GENBANKフィーチャーのDTO"""

    feature_type: str
    location: str
    qualifiers: Dict[str, List[str]]


class GenbankRecordDTO(BaseModel):
    """GENBANKレコードのDTO"""

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
    features: List[GenbankFeatureDTO]
    sequence: str
    comment: Optional[str] = None
    primary: Optional[str] = None


class FastaRecordDTO(BaseModel):
    """FASTAレコードのDTO"""

    header: str
    description: str
    sequence: str
