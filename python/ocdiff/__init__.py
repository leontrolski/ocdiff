from typing import TYPE_CHECKING

from . import ocdiff as _ocdiff  # type: ignore

if not TYPE_CHECKING:
    mdiff = _ocdiff.mdiff
    diff_lines = _ocdiff.diff_lines
    Part = _ocdiff.Part
    Diff = _ocdiff.Diff
