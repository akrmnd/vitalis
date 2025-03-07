"""
GENBANKファイルのセクションパーサー
"""

from abc import ABC, abstractmethod
from typing import Any, Dict, List

from pydantic import BaseModel

from src.domain.models.genbank import GenbankFeature
from src.infrastructure.parsers.genbank_constants import (
    GenbankIndent,
    GenbankSectionType,
)


class RecordData(BaseModel):
    """パース中のレコードデータを保持するクラス"""

    locus: str = ""
    size: int = 0
    molecule_type: str = ""
    genbank_division: str = ""
    modification_date: str = ""
    definition: str = ""
    accession: str = ""
    version: str = ""
    keywords: List[str] = []
    source: str = ""
    organism: str = ""
    taxonomy: str = ""
    references: List[Dict[str, Any]] = []
    features: List[GenbankFeature] = []
    sequence: str = ""
    comment: str = ""
    primary: str = ""
    current_section: GenbankSectionType | None = None
    current_feature: GenbankFeature | None = None
    current_reference: Dict[str, Any] = {}
    in_sequence: bool = False
    sequence_lines: List[str] = []


class SectionParser(ABC):
    """セクションパーサーの基底クラス"""

    @abstractmethod
    def can_parse(self, line: str, data: RecordData) -> bool:
        """このパーサーが処理できる行かどうかを判定する"""
        pass

    @abstractmethod
    def parse(self, line: str, data: RecordData) -> RecordData:
        """行を解析して新しいデータオブジェクトを返す"""
        pass


class LocusSectionParser(SectionParser):
    """LOCUSセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.LOCUS.value)

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.LOCUS.value):
            data = data.model_copy(update={"current_section": GenbankSectionType.LOCUS})
            parts = line[len(GenbankSectionType.LOCUS.value) :].strip().split()
            if len(parts) >= 7:
                data = data.model_copy(
                    update={
                        "locus": parts[0],
                        "size": int(parts[1]),
                        "molecule_type": parts[3],
                        "genbank_division": parts[5],
                        "modification_date": parts[6],
                    }
                )
        return data


class DefinitionSectionParser(SectionParser):
    """DEFINITIONセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.DEFINITION.value) or (
            data.current_section == GenbankSectionType.DEFINITION and line.startswith(" ")
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.DEFINITION.value):
            data = data.model_copy(
                update={
                    "current_section": GenbankSectionType.DEFINITION,
                    "definition": line[len(GenbankSectionType.DEFINITION.value) :].strip(),
                }
            )
        else:
            data = data.model_copy(update={"definition": data.definition + " " + line.strip()})
        return data


class AccessionSectionParser(SectionParser):
    """ACCESSIONセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.ACCESSION.value) or (
            data.current_section == GenbankSectionType.ACCESSION and line.startswith(" ")
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.ACCESSION.value):
            data = data.model_copy(
                update={
                    "current_section": GenbankSectionType.ACCESSION,
                    "accession": line[len(GenbankSectionType.ACCESSION.value) :].strip(),
                }
            )
        else:
            data = data.model_copy(update={"accession": data.accession + " " + line.strip()})
        return data


class VersionSectionParser(SectionParser):
    """VERSIONセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.VERSION.value)

    def parse(self, line: str, data: RecordData) -> RecordData:
        data = data.model_copy(
            update={
                "current_section": GenbankSectionType.VERSION,
                "version": line[len(GenbankSectionType.VERSION.value) :].strip(),
            }
        )
        return data


class KeywordsSectionParser(SectionParser):
    """KEYWORDSセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.KEYWORDS.value)

    def parse(self, line: str, data: RecordData) -> RecordData:
        data = data.model_copy(update={"current_section": GenbankSectionType.KEYWORDS})
        kw_text = line[len(GenbankSectionType.KEYWORDS.value) :].strip().rstrip(".")
        if kw_text:
            data = data.model_copy(update={"keywords": [k.strip() for k in kw_text.split(";") if k.strip()]})
        return data


class SourceSectionParser(SectionParser):
    """SOURCEセクションのパーサー（ORGANISMを含む）"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        # SOURCEセクションの開始行
        if line.startswith(GenbankSectionType.SOURCE.value):
            return True

        # ORGANISMサブセクション
        if line.startswith(f"{GenbankIndent.SECTION.value}ORGANISM"):
            return True

        # 分類情報の行（インデントされた行）
        if data.current_section == GenbankSectionType.SOURCE and line.startswith(GenbankIndent.TAXONOMY.value):
            return True

        # SOURCEセクション内の継続行（他のセクションの開始行でない場合）
        return data.current_section == GenbankSectionType.SOURCE and not any(
            line.startswith(section.value)
            for section in [
                GenbankSectionType.REFERENCE,
                GenbankSectionType.COMMENT,
                GenbankSectionType.PRIMARY,
                GenbankSectionType.FEATURES,
                GenbankSectionType.ORIGIN,
                GenbankSectionType.BASE,
                GenbankSectionType.CONTIG,
            ]
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.SOURCE.value):
            data = data.model_copy(
                update={
                    "current_section": GenbankSectionType.SOURCE,
                    "source": line[len(GenbankSectionType.SOURCE.value) :].strip(),
                }
            )
        elif line.startswith(f"{GenbankIndent.SECTION.value}ORGANISM"):
            data = data.model_copy(update={"organism": line[len(f"{GenbankIndent.SECTION.value}ORGANISM") :].strip()})
        elif data.current_section == GenbankSectionType.SOURCE and line.startswith(GenbankIndent.TAXONOMY.value):
            # 分類情報の行を処理
            taxonomy_line = line.strip()
            if data.taxonomy:
                data = data.model_copy(update={"taxonomy": data.taxonomy + " " + taxonomy_line})
            else:
                data = data.model_copy(update={"taxonomy": taxonomy_line})
        return data


class ReferenceSectionParser(SectionParser):
    """REFERENCEセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        # REFERENCEセクションの開始行
        if line.startswith(GenbankSectionType.REFERENCE.value):
            return True

        # REFERENCEセクション内の継続行（他のセクションの開始行でない場合）
        return data.current_section == GenbankSectionType.REFERENCE and not any(
            line.startswith(section.value)
            for section in [
                GenbankSectionType.FEATURES,
                GenbankSectionType.ORIGIN,
                GenbankSectionType.BASE,
                GenbankSectionType.CONTIG,
                GenbankSectionType.COMMENT,
                GenbankSectionType.PRIMARY,
            ]
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.REFERENCE.value):
            data = data.model_copy(update={"current_section": GenbankSectionType.REFERENCE})
            if data.current_reference:
                data = data.model_copy(update={"references": data.references + [data.current_reference]})
            new_reference = {"citation": line[len(GenbankSectionType.REFERENCE.value) :].strip()}
            data = data.model_copy(update={"current_reference": new_reference})
        elif data.current_section == GenbankSectionType.REFERENCE:
            if line.startswith(f"{GenbankIndent.SECTION.value}AUTHORS"):
                new_reference = dict(data.current_reference)
                new_reference["authors"] = line[len(f"{GenbankIndent.SECTION.value}AUTHORS") :].strip()
                data = data.model_copy(update={"current_reference": new_reference})
            elif line.startswith(f"{GenbankIndent.SECTION.value}TITLE"):
                new_reference = dict(data.current_reference)
                new_reference["title"] = line[len(f"{GenbankIndent.SECTION.value}TITLE") :].strip()
                data = data.model_copy(update={"current_reference": new_reference})
            elif line.startswith(f"{GenbankIndent.SECTION.value}JOURNAL"):
                new_reference = dict(data.current_reference)
                new_reference["journal"] = line[len(f"{GenbankIndent.SECTION.value}JOURNAL") :].strip()
                data = data.model_copy(update={"current_reference": new_reference})
            elif line.startswith(f"{GenbankIndent.PUBMED.value}PUBMED"):
                new_reference = dict(data.current_reference)
                new_reference["pubmed"] = line[len(f"{GenbankIndent.PUBMED.value}PUBMED") :].strip()
                data = data.model_copy(update={"current_reference": new_reference})
        return data


class CommentSectionParser(SectionParser):
    """COMMENTセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.COMMENT.value) or (
            data.current_section == GenbankSectionType.COMMENT and line.startswith(" ")
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.COMMENT.value):
            # COMMENTセクションの開始行
            data = data.model_copy(
                update={
                    "current_section": GenbankSectionType.COMMENT,
                    "comment": line[len(GenbankSectionType.COMMENT.value) :].strip(),
                }
            )
        elif data.current_section == GenbankSectionType.COMMENT:
            # COMMENTセクションの継続行
            if hasattr(data, "comment"):
                data = data.model_copy(update={"comment": data.comment + " " + line.strip()})
            else:
                data = data.model_copy(update={"comment": line.strip()})
        return data


class PrimarySectionParser(SectionParser):
    """PRIMARYセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.PRIMARY.value) or (
            data.current_section == GenbankSectionType.PRIMARY and line.startswith(" ")
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.PRIMARY.value):
            # PRIMARYセクションの開始行
            data = data.model_copy(
                update={
                    "current_section": GenbankSectionType.PRIMARY,
                    "primary": line[len(GenbankSectionType.PRIMARY.value) :].strip(),
                }
            )
        elif data.current_section == GenbankSectionType.PRIMARY:
            # PRIMARYセクションの継続行
            if hasattr(data, "primary"):
                data = data.model_copy(update={"primary": data.primary + " " + line.strip()})
            else:
                data = data.model_copy(update={"primary": line.strip()})
        return data


class OriginSectionParser(SectionParser):
    """ORIGINセクションのパーサー"""

    def can_parse(self, line: str, data: RecordData) -> bool:
        return line.startswith(GenbankSectionType.ORIGIN.value) or data.current_section == GenbankSectionType.ORIGIN

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.ORIGIN.value):
            data = data.model_copy(
                update={
                    "current_section": GenbankSectionType.ORIGIN,
                    "in_sequence": True,
                }
            )
        elif data.in_sequence and not line.startswith(GenbankSectionType.END.value):
            sequence_parts = line.strip().split()
            if len(sequence_parts) > 1:
                data = data.model_copy(update={"sequence_lines": data.sequence_lines + ["".join(sequence_parts[1:])]})
        return data


class FeaturesSectionParser(SectionParser):
    """FEATURESセクションのパーサー"""

    def __init__(self) -> None:
        self.in_features = False

    def can_parse(self, line: str, data: RecordData) -> bool:
        if line.startswith(GenbankSectionType.FEATURES.value):
            self.in_features = True
            return True

        # FEATURESセクション内の行（他のセクションの開始行でない場合）
        return self.in_features and not any(
            line.startswith(section.value)
            for section in [
                GenbankSectionType.ORIGIN,
                GenbankSectionType.BASE,
                GenbankSectionType.CONTIG,
            ]
        )

    def parse(self, line: str, data: RecordData) -> RecordData:
        if line.startswith(GenbankSectionType.FEATURES.value):
            data = data.model_copy(update={"current_section": GenbankSectionType.FEATURES})
            return data

        if line.startswith(GenbankSectionType.ORIGIN.value):
            self.in_features = False
            return data

        if line.startswith(GenbankIndent.FEATURE.value) and not line.startswith(GenbankIndent.QUALIFIER.value):
            data = self._process_feature_line(line, data)
        elif line.startswith(GenbankIndent.QUALIFIER.value) and data.current_feature:
            data = self._process_qualifier_line(line, data)
        return data

    def _process_feature_line(self, line: str, data: RecordData) -> RecordData:
        """フィーチャー行を処理する"""
        if data.current_feature:
            data = data.model_copy(update={"features": data.features + [data.current_feature]})

        parts = line[len(GenbankIndent.FEATURE.value) :].strip().split(None, 1)
        if len(parts) >= 2:
            feature_type = parts[0]
            location = parts[1]
            new_feature = GenbankFeature(feature_type=feature_type, location=location, qualifiers={})
            data = data.model_copy(update={"current_feature": new_feature})
        return data

    def _process_qualifier_line(self, line: str, data: RecordData) -> RecordData:
        qualifier_line = line[len(GenbankIndent.QUALIFIER.value) :].strip()
        if qualifier_line.startswith("/"):
            parts = qualifier_line[1:].split("=", 1)
            key = parts[0]

            # 現在のqualifiersをコピー
            if data.current_feature and data.current_feature.qualifiers:
                new_qualifiers = dict(data.current_feature.qualifiers)
            else:
                new_qualifiers = {}

            if len(parts) > 1:
                value = parts[1]
                if value.startswith('"'):
                    value = value[1:]
                    if value.endswith('"'):
                        value = value[:-1]
                # 新しいキーと値を追加
                new_qualifiers[key] = [value]
            else:
                # 値なしのキーを追加
                new_qualifiers[key] = [""]

            # 更新されたqualifiersで現在のフィーチャーを更新
            if data.current_feature:
                data = data.model_copy(
                    update={"current_feature": data.current_feature.model_copy(update={"qualifiers": new_qualifiers})}
                )
        elif data.current_feature and data.current_feature.qualifiers:
            last_key = list(data.current_feature.qualifiers.keys())[-1]
            if data.current_feature.qualifiers[last_key]:
                value = qualifier_line
                if value.endswith('"'):
                    value = value[:-1]
                # 現在のqualifiersをコピー
                new_qualifiers = dict(data.current_feature.qualifiers)
                # 最後のキーの値を更新
                new_qualifiers[last_key] = data.current_feature.qualifiers[last_key] + [value]
                # 更新されたqualifiersで現在のフィーチャーを更新
                data = data.model_copy(
                    update={"current_feature": data.current_feature.model_copy(update={"qualifiers": new_qualifiers})}
                )
        return data
