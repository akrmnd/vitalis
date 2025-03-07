"""
FASTAファイルのパーサーモジュール
"""

from typing import List

from src.domain.models.fasta import FastaRecord


class FastaParser:
    """
    FASTAファイルをパースするクラス
    """

    def parse_file(self, file_path: str) -> List[FastaRecord]:
        """
        FASTAファイルをパースしてFastaRecordのリストを返す

        Args:
            file_path: パースするFASTAファイルのパス

        Returns:
            FastaRecordのリスト
        """
        records = []

        with open(file_path, "r", encoding="utf-8") as f:
            current_header = ""
            current_description = ""
            current_sequence: List[str] = []

            for line in f:
                line = line.strip()
                if not line:
                    continue

                if line.startswith(">"):
                    # 新しいレコードの開始
                    if current_header:
                        # 前のレコードを保存
                        records.append(
                            FastaRecord(
                                header=current_header,
                                description=current_description,
                                sequence="".join(current_sequence),
                            )
                        )

                    # 新しいヘッダーを解析
                    header_parts = line[1:].split(" ", 1)
                    current_header = header_parts[0]
                    current_description = header_parts[1] if len(header_parts) > 1 else ""
                    current_sequence = []
                else:
                    # 配列行
                    current_sequence.append(line)

            # 最後のレコードを追加
            if current_header:
                records.append(
                    FastaRecord(
                        header=current_header,
                        description=current_description,
                        sequence="".join(current_sequence),
                    )
                )

        return records
