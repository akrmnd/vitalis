"""
GENBANKファイルのパース用定数
"""

from enum import Enum


class GenbankSectionType(Enum):
    """GENBANKファイルのセクションタイプ"""

    LOCUS = "LOCUS"
    DEFINITION = "DEFINITION"
    ACCESSION = "ACCESSION"
    VERSION = "VERSION"
    KEYWORDS = "KEYWORDS"
    SOURCE = "SOURCE"
    REFERENCE = "REFERENCE"
    COMMENT = "COMMENT"
    PRIMARY = "PRIMARY"
    FEATURES = "FEATURES"
    ORIGIN = "ORIGIN"
    BASE = "BASE"
    CONTIG = "CONTIG"
    END = "//"


class GenbankIndent(Enum):
    """GENBANKファイルのインデントレベル"""

    NONE = ""
    SECTION = "  "  # 2スペース
    PUBMED = "   "  # 3スペース（PUBMEDなどで使用）
    SUBSECTION = "    "  # 4スペース
    FEATURE = "     "  # 5スペース
    TAXONOMY = "            "  # 12スペース
    QUALIFIER = "                     "  # 21スペース
