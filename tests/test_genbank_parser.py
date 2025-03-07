"""
GENBANKパーサーのテスト
"""

import sys
from pathlib import Path

# プロジェクトのルートディレクトリをパスに追加
sys.path.insert(0, str(Path(__file__).parent.parent))

from src.domain.models.genbank import GenbankRecord
from src.infrastructure.parsers.genbank_parser import GenbankParser


def test_parse_abca4_genbank_file() -> None:
    """ABCA4のGENBANKファイルを正しくパースできるかテストする"""
    file_path = Path(__file__).parent / "data" / "abca4_sequence.gb"

    content = file_path.read_text(encoding="utf-8")

    accession_index = content.find("ACCESSION")
    if accession_index >= 0:
        # ACCESSIONセクションの最初の100文字を表示
        accession_section = content[accession_index : accession_index + 100]
        print(f"ACCESSIONセクションの一部:\n{accession_section}")

    # ACCESSIONセクションの行を直接確認
    accession_lines = []
    in_accession = False
    for line in content.split("\n"):
        if line.startswith("ACCESSION"):
            in_accession = True
            accession_lines.append(line)
        elif in_accession and line.startswith(" "):
            accession_lines.append(line)
        elif in_accession:
            in_accession = False
            break

    print("ACCESSIONセクションの行:")
    for line in accession_lines:
        print(f"  '{line}'")

    # パーサーの作成
    parser = GenbankParser()

    # ファイルのパース
    records = parser.parse_file(str(file_path))

    # 結果の検証
    print(f"レコード数: {len(records)}")
    if records:
        print(f"フィーチャー数: {len(records[0].features)}")

    # 結果の検証
    assert len(records) == 1
    record = records[0]

    # 型の検証
    assert isinstance(record, GenbankRecord)

    # 基本情報の検証
    assert record.locus == "NG_009073"
    assert record.size == 128315
    assert record.molecule_type == "DNA"
    assert record.genbank_division == "PRI"
    assert record.modification_date == "03-OCT-2024"
    assert "Homo sapiens ATP binding cassette subfamily A member 4" in record.definition
    assert record.accession == "NG_009073 REGION: 5000..133314"
    assert record.version == "NG_009073.2"
    assert "RefSeq" in record.keywords
    assert "Homo sapiens" in record.source

    # フィーチャーの検証
    assert len(record.features) > 0
    gene_feature = next((f for f in record.features if f.feature_type == "gene"), None)
    assert gene_feature is not None
    assert gene_feature.location == "1..128315"
    assert "ABCA4" in gene_feature.qualifiers.get("gene", [""])[0]

    # 配列の検証
    assert len(record.sequence) > 0
    assert record.sequence.startswith("ggacacagcg")
    assert record.sequence.endswith("aacta")

    # ファイルの内容を確認
    content = file_path.read_text(encoding="utf-8")

    # FEATURESセクションを探す
    features_index = content.find("FEATURES")
    if features_index >= 0:
        # FEATURESセクションの最初の500文字を表示
        features_section = content[features_index : features_index + 500]
        print(f"FEATURESセクションの一部:\n{features_section}")

        # 最初のフィーチャー行を探す
        lines = content[features_index:].split("\n")
        for i, line in enumerate(lines[:10]):
            print(f"行 {i}: '{line}'")
            if i > 0 and line.startswith("     ") and not line.startswith("                     "):
                print(f"最初のフィーチャー行: '{line}'")
                parts = line[5:].strip().split(None, 1)
                print(f"分割後のパーツ: {parts}")
                break
