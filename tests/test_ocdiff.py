from pathlib import Path
import ocdiff


def test_html_diff() -> None:
    actual = ocdiff.html_diff(
        (Path(__file__).parent / "a.json").read_text(),
        (Path(__file__).parent / "b.json").read_text(),
        context_lines=5,
        column_limit=80,
    )
    expected = (Path(__file__).parent / "expected.html").read_text()
    assert actual == expected
