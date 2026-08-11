import codecs
from pathlib import Path
import sys

# source = Path("research_20260805.m.csv")
# data = source.read_bytes().removeprefix(codecs.BOM_UTF8)
# source.write_bytes(codecs.BOM_UTF8 + data)

# source = Path("research_20260805.u.csv")
# data = source.read_bytes().removeprefix(codecs.BOM_UTF8)
# source.write_bytes(codecs.BOM_UTF8 + data)

source = Path(sys.argv[1])
data = source.read_bytes().removeprefix(codecs.BOM_UTF8)
source.write_bytes(codecs.BOM_UTF8 + data)
