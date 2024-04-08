from typing import TYPE_CHECKING

from . import ocdiff as _ocdiff  # type: ignore

if not TYPE_CHECKING:
    html_diff = _ocdiff.html_diff
