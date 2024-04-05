from typing import TYPE_CHECKING

from . import ocdiff as _ocdiff  # type: ignore

Diff = tuple[tuple[int, str], tuple[int, str], bool | None]

if not TYPE_CHECKING:
    mdiff = _ocdiff.mdiff
    diff_lines = _ocdiff.diff_lines
