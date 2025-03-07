"""
FASTAファイルのドメインモデル
"""

from pydantic import BaseModel


class FastaRecord(BaseModel):
    """FASTAファイルのレコードを表すクラス"""

    header: str
    description: str = ""
    sequence: str

    def to_file(self, file_path: str) -> None:
        """FASTAファイルに保存する"""
        with open(file_path, "w", encoding="utf-8") as f:
            f.write(f">{self.header}")
            if self.description:
                f.write(f" {self.description}")
