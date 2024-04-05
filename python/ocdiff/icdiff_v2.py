from dataclasses import dataclass
import difflib
from pathlib import Path
import re
from typing import Iterable, Iterator, Literal
import unicodedata

# Exit code constants
EXIT_CODE_SUCCESS = 0
EXIT_CODE_DIFF = 1
EXIT_CODE_ERROR = 2

color_codes = {
    "black": "\033[0;30m",
    "red": "\033[0;31m",
    "green": "\033[0;32m",
    "yellow": "\033[0;33m",
    "blue": "\033[0;34m",
    "magenta": "\033[0;35m",
    "cyan": "\033[0;36m",
    "white": "\033[0;37m",
    "none": "\033[m",
    "black_bold": "\033[1;30m",
    "red_bold": "\033[1;31m",
    "green_bold": "\033[1;32m",
    "yellow_bold": "\033[1;33m",
    "blue_bold": "\033[1;34m",
    "magenta_bold": "\033[1;35m",
    "cyan_bold": "\033[1;36m",
    "white_bold": "\033[1;37m",
}
color_mapping = {
    "add": "green_bold",
    "subtract": "red_bold",
    "change": "yellow_bold",
    "separator": "blue",
    "description": "blue",
    "permissions": "yellow",
    "meta": "magenta",
    "line-numbers": "white",
}

LineNum = int | Literal["", ">"]
Diff = tuple[tuple[LineNum, str], tuple[LineNum, str], bool | None]


@dataclass
class Options:
    cols: int = 160
    line_numbers: bool = True
    tabsize: int = 4
    whole_file: bool = False
    unified: int = 5
    # set below
    wrapcolumn: int = 0

    def __post_init__(self) -> None:
        if not self.line_numbers:
            self.wrapcolumn = self.cols // 2 - 2
        else:
            self.wrapcolumn = self.cols // 2 - 10


def raw_colorize(s: str, color: str) -> str:
    return "%s%s%s" % (color_codes[color], s, color_codes["none"])


def simple_colorize(s: str, category: str) -> str:
    return raw_colorize(s, color_mapping[category])


def replace_all(replacements: dict[str, str], string: str) -> str:
    for search, replace in replacements.items():
        string = string.replace(search, replace)
    return string


def _display_len(s: str) -> int:
    def width(c: str) -> int:
        if isinstance(c, type("")) and unicodedata.east_asian_width(c) in [
            "F",
            "W",
        ]:
            return 2
        elif c == "\r":
            return 2
        return 1

    return sum(width(c) for c in s)


def _split_line(
    wrapcolumn: int,
    data_list: list[tuple[LineNum, str]],
    line_num: LineNum,
    text: str,
) -> None:
    while True:
        # if blank line or context separator, just add it to the output
        # list
        if not line_num:
            data_list.append((line_num, text))
            return

        # if line text doesn't need wrapping, just add it to the output
        # list
        if _display_len(text) - (text.count("\0") * 3) <= wrapcolumn:
            data_list.append((line_num, text))
            return

        # scan text looking for the wrap point, keeping track if the wrap
        # point is inside markers
        i = 0
        n = 0
        mark = ""
        while n < wrapcolumn and i < len(text):
            if text[i] == "\0":
                i += 1
                mark = text[i]
                i += 1
            elif text[i] == "\1":
                i += 1
                mark = ""
            else:
                n += _display_len(text[i])
                i += 1

        # wrap point is inside text, break it up into separate lines
        line1 = text[:i]
        line2 = text[i:]

        # if wrap point is inside markers, place end marker at end of first
        # line and start marker at beginning of second line because each
        # line will have its own table tag markup around it.
        if mark:
            line1 = line1 + "\1"
            line2 = "\0" + mark + line2

        # tack on first line onto the output list
        data_list.append((line_num, line1))

        # use this routine again to wrap the remaining text
        line_num = ">"
        text = line2


def _line_wrapper(wrapcolumn: int, diffs: Iterable[Diff]) -> Iterator[Diff]:
    """Returns iterator that splits (wraps) mdiff text lines"""

    # pull from/to data and flags from mdiff iterator
    for fromdata, todata, flag in diffs:
        # check for context separators and pass them through
        if flag is None:
            continue

        assert isinstance(fromdata, tuple)
        assert isinstance(todata, tuple)
        (fromline, fromtext), (toline, totext) = fromdata, todata
        # for each from/to line split it at the wrap column to form
        # list of text lines.
        fromlist = list[tuple[LineNum, str]]()
        tolist = list[tuple[LineNum, str]]()
        _split_line(wrapcolumn, fromlist, fromline, fromtext)
        _split_line(wrapcolumn, tolist, toline, totext)
        # yield from/to line in pairs inserting blank lines as
        # necessary when one side has more wrapped lines
        while fromlist or tolist:
            if fromlist:
                fromdata = fromlist.pop(0)
            else:
                fromdata = ("", " ")
            if tolist:
                todata = tolist.pop(0)
            else:
                todata = ("", " ")
            yield fromdata, todata, flag


def _real_len(s: str) -> int:
    s_len = 0
    in_esc = False
    prev = " "
    for c in replace_all({"\0+": "", "\0-": "", "\0^": "", "\1": "", "\t": " "}, s):
        if in_esc:
            if c == "m":
                in_esc = False
        else:
            if c == "[" and prev == "\033":
                in_esc = True
                s_len -= 1  # we counted prev when we shouldn't have
            else:
                s_len += _display_len(c)
        prev = c

    return s_len


def _rpad(s: str, field_width: int) -> str:
    return _pad(s, field_width) + s


def _pad(s: str, field_width: int) -> str:
    return " " * (field_width - _real_len(s))


def _lpad(s: str, field_width: int) -> str:
    return s + _pad(s, field_width)


def _format_line(line_numbers: bool, linenum: LineNum, text: str) -> str:
    text = text.rstrip()
    if not line_numbers:
        return text
    return _add_line_numbers(linenum, text)


def _add_line_numbers(linenum: LineNum, text: str) -> str:
    if linenum == "" or linenum == ">":
        return text

    lid = "%d" % linenum
    return "%s %s" % (
        _rpad(simple_colorize(str(lid), "line-numbers"), 8),
        text,
    )


def _collect_lines(
    line_numbers: bool, diffs: Iterable[Diff]
) -> Iterator[tuple[str, str] | None]:
    for fromdata, todata, _ in diffs:
        assert isinstance(fromdata, tuple)
        assert isinstance(todata, tuple)
        fromlinenum, fromtext = fromdata
        tolinenum, totext = todata
        yield (
            _format_line(line_numbers, fromlinenum, fromtext),
            _format_line(line_numbers, tolinenum, totext),
        )


def _tab_newline_replace(
    tabsize: int, fromlines: Iterable[str], tolines: Iterable[str]
) -> tuple[list[str], list[str]]:
    def expand_tabs(line: str) -> str:
        # hide real spaces
        line = line.replace(" ", "\0")
        # expand tabs into spaces
        line = line.expandtabs(tabsize)
        # replace spaces from expanded tabs back into tab characters
        # (we'll replace them with markup after we do differencing)
        line = line.replace(" ", "\t")
        return line.replace("\0", " ").rstrip("\n")

    fromlines = [expand_tabs(line) for line in fromlines]
    tolines = [expand_tabs(line) for line in tolines]
    return fromlines, tolines


def _colorize(s: str) -> str:
    C_ADD = color_codes[color_mapping["add"]]
    C_SUB = color_codes[color_mapping["subtract"]]
    C_CHG = color_codes[color_mapping["change"]]
    C_NONE = color_codes["none"]

    s = replace_all(
        {
            "\0+": C_ADD,
            "\0-": C_SUB,
            "\0^": C_CHG,
            "\1": C_NONE,
            "\t": " ",
            "\r": "\\r",
        },
        s,
    )
    # If there's a change consisting entirely of whitespace,
    # don't color it.
    return re.sub(
        "\033\\[[01];3([01234567])m(\\s+)(\033\\[)",
        "\033[7;3\\1m\\2\\3",
        s,
    )


def _generate_table(
    diffs: Iterable[tuple[str, str] | None]
) -> Iterator[tuple[str, str]]:
    for i, line in enumerate(diffs):
        if line is None:
            # mdiff yields None on separator lines; skip the bogus ones
            # generated for the first line
            if i > 0:
                yield (
                    simple_colorize("---", "separator"),
                    simple_colorize("---", "separator"),
                )
        else:
            yield line


def make_table(
    options: Options,
    fromlines: list[str],
    tolines: list[str],
    context: bool = False,
    numlines: int = 5,
) -> Iterator[str]:
    if context:
        context_lines = numlines
    else:
        context_lines = None

    fromlines, tolines = _tab_newline_replace(options.tabsize, fromlines, tolines)

    diffs: Iterator[Diff] = difflib._mdiff(  # type: ignore[attr-defined]
        fromlines,
        tolines,
        context_lines,
        linejunk=None,
        charjunk=difflib.IS_CHARACTER_JUNK,
    )

    # Example output from difflib
    # ---------------------------
    # (1, "["),                                   (1, "["),                            False
    # (2, "    {"),                               (2, "    {"),                        False
    # (3, '\x00-        "_ts": 1703071209,\x01'), ("", "\n"),                          True
    # (4, '        "payload": {'),                (3, '        "payload": {'),         False
    # (5, '            "CommonBlock": {'),        (4, '            "CommonBlock": {'), False
    # (6, '                "A0": {'),             (5, '                "A0": {'),      False

    diffs = _line_wrapper(options.wrapcolumn, diffs)
    diff_lines = _collect_lines(options.line_numbers, diffs)

    for left, right in _generate_table(diff_lines):
        yield _colorize(
            "%s %s"
            % (
                _lpad(left, options.cols // 2 - 1),
                _lpad(right, options.cols // 2 - 1),
            )
        )


def diff_files(options: Options, a: Path, b: Path) -> None:
    assert not a.is_dir()
    assert not b.is_dir()

    lines_a = a.read_text().splitlines()
    lines_b = b.read_text().splitlines()

    for line in make_table(
        options=options,
        fromlines=lines_a,
        tolines=lines_b,
        context=(not options.whole_file),
        numlines=int(options.unified),
    ):
        print(line)


if __name__ == "__main__":
    diff_files(
        Options(),
        Path.home() / "a",
        Path.home() / "b",
    )
