from typing import TYPE_CHECKING

from . import ocdiff as _ocdiff  # type: ignore

if not TYPE_CHECKING:
    foo = _ocdiff.foo
