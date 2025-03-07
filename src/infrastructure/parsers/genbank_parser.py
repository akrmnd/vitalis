"""
GENBANKファイルのパーサーモジュール
"""

import re
from typing import List, Optional

from src.domain.models.genbank import GenbankRecord
from src.infrastructure.parsers.genbank_constants import GenbankSectionType
from src.infrastructure.parsers.genbank_section_parsers import (
    AccessionSectionParser,
    CommentSectionParser,
    DefinitionSectionParser,
    FeaturesSectionParser,
    KeywordsSectionParser,
    LocusSectionParser,
    OriginSectionParser,
    PrimarySectionParser,
    RecordData,
    ReferenceSectionParser,
    SectionParser,
    SourceSectionParser,
    VersionSectionParser,
)


class GenbankParser:
    """
    GENBANKファイルをパースするクラス
    """

    def __init__(self) -> None:
        # パーサーの順序が重要
        self.section_parsers: List[SectionParser] = [
            LocusSectionParser(),
            DefinitionSectionParser(),
            AccessionSectionParser(),
            VersionSectionParser(),
            KeywordsSectionParser(),
            SourceSectionParser(),
            ReferenceSectionParser(),
            CommentSectionParser(),
            PrimarySectionParser(),
            FeaturesSectionParser(),
            OriginSectionParser(),
        ]

    def parse_file(self, file_path: str) -> List[GenbankRecord]:
        """
        GENBANKファイルをパースしてGenbankRecordのリストを返す

        Args:
            file_path: パースするGENBANKファイルのパス

        Returns:
            GenbankRecordのリスト
        """
        with open(file_path, "r", encoding="utf-8") as f:
            content = f.read()

        record_contents = re.split(r"//\n", content)
        records = []

        for record_content in record_contents:
            if not record_content.strip():
                continue

            record = self._parse_record(record_content)
            if record:
                records.append(record)

        return records

    def _parse_record(self, content: str) -> Optional[GenbankRecord]:
        """
        GENBANKレコードの内容をパースする

        Args:
            content: GENBANKレコードの内容

        Returns:
            GenbankRecordオブジェクト、パースに失敗した場合はNone
        """
        if not content.strip():
            return None

        data = RecordData()
        lines = content.split("\n")

        # セクションごとに処理するパーサーを決定
        current_parser = None

        for line in lines:
            if not line.strip():
                continue

            # 新しいセクションの開始を検出
            if any(
                line.startswith(section.value)
                for section in [
                    GenbankSectionType.LOCUS,
                    GenbankSectionType.DEFINITION,
                    GenbankSectionType.ACCESSION,
                    GenbankSectionType.VERSION,
                    GenbankSectionType.KEYWORDS,
                    GenbankSectionType.SOURCE,
                    GenbankSectionType.REFERENCE,
                    GenbankSectionType.COMMENT,
                    GenbankSectionType.PRIMARY,
                    GenbankSectionType.FEATURES,
                    GenbankSectionType.ORIGIN,
                ]
            ):
                # 適切なパーサーを見つける
                for parser in self.section_parsers:
                    if parser.can_parse(line, data):
                        print(f"parser: {parser}")
                        current_parser = parser
                        data = parser.parse(line, data)
                        break
            # 現在のセクションの継続行を処理
            elif current_parser and current_parser.can_parse(line, data):
                data = current_parser.parse(line, data)

        data = self._finalize_record_data(data)
        return self._create_record(data)

    def _finalize_record_data(self, data: RecordData) -> RecordData:
        """レコードデータを最終処理する"""
        # 最後のフィーチャーを追加
        if data.current_feature:
            data = data.model_copy(update={"features": data.features + [data.current_feature]})

        # 最後のリファレンスを追加
        if data.current_reference:
            data = data.model_copy(update={"references": data.references + [data.current_reference]})

        # 配列データを結合
        data = data.model_copy(update={"sequence": "".join(data.sequence_lines)})

        # taxonomyがある場合、organismに統合
        if data.taxonomy:
            data = data.model_copy(update={"organism": f"{data.organism} [{data.taxonomy}]"})

        return data

    def _create_record(self, data: RecordData) -> GenbankRecord:
        """GenbankRecordオブジェクトを作成する"""
        return GenbankRecord(
            locus=data.locus,
            size=data.size,
            molecule_type=data.molecule_type,
            genbank_division=data.genbank_division,
            modification_date=data.modification_date,
            definition=data.definition,
            accession=data.accession,
            version=data.version,
            keywords=data.keywords,
            source=data.source,
            organism=data.organism,
            taxonomy=data.taxonomy,
            references=data.references,
            features=data.features,
            sequence=data.sequence,
            comment=data.comment,
            primary=data.primary,
        )


if __name__ == "__main__":
    import os

    parser = GenbankParser()
    records = parser.parse_file("abca4_sequence.gb")

    # 出力ディレクトリを作成
    output_dir = "output"
    os.makedirs(output_dir, exist_ok=True)

    for i, record in enumerate(records):
        # JSONファイルに保存
        output_file = os.path.join(output_dir, f"record_{i + 1}.json")
        record.to_file(output_file)

        print(f"レコード {i + 1} を {output_file} に保存しました")
        print(f"Locus: {record.locus}")
        print(f"Definition: {record.definition}")
        print(f"Features: {len(record.features)} 個")
        print(f"Sequence length: {len(record.sequence)} bp")
        print("-" * 50)
