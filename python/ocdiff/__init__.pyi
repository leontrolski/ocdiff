from dataclasses import dataclass
from typing import Literal


def mdiff(
    a: str, b: str, context_lines: int | None
) -> list[tuple[tuple[int, str], tuple[int, str], bool | None]]: ...
def diff_lines(a: str, b: str) -> tuple[str, str]: ...

@dataclass
class Part:
    value: str
    kind: Literal["EQUAL", "DELETE", "INSERT"]

@dataclass
class Diff:
    left_lineno: int
    left: list[Part]
    right_lineno: int
    right: list[Part]
