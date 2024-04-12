from typing import TYPE_CHECKING

from . import ocdiff as _ocdiff  # type: ignore
from . import helpers

if not TYPE_CHECKING:
    html_diff = _ocdiff.html_diff


def console_diff(
    a: str,
    b: str,
    context_lines: int | None = None,
    max_total_width: int | None = None,
) -> str:
    if max_total_width is None:
        max_total_width = helpers.terminal_width()
    return _ocdiff.console_diff(
        a=a,
        b=b,
        context_lines=context_lines,
        max_total_width=max_total_width,
    )
