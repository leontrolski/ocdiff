import copy
from pathlib import Path
import ocdiff


def test_html_diff() -> None:
    actual = ocdiff.html_diff(
        (Path(__file__).parent / "a.json").read_text(),
        (Path(__file__).parent / "b.json").read_text(),
        context_lines=5,
        column_limit=80,
    ).strip()
    # (Path(__file__).parent / "out.html").write_text(actual)
    expected = (Path(__file__).parent / "expected.html").read_text().strip()
    assert actual == expected


def test_html_diff_2() -> None:
    actual = ocdiff.html_diff(
        (Path(__file__).parent / "x.json").read_text(),
        (Path(__file__).parent / "y.json").read_text(),
        context_lines=1,
        column_limit=80,
    ).strip()
    # (Path(__file__).parent / "out.html").write_text(actual)
    expected = (Path(__file__).parent / "expected_2.html").read_text().strip()
    assert actual == expected


def test_html_diff_3() -> None:
    ns = [str(n) for n in range(100)]
    ms = copy.copy(ns)
    ms.insert(50, "999")

    actual = ocdiff.html_diff(
        "\n".join(ns),
        "\n".join(ms),
        context_lines=2,
        column_limit=80,
    ).strip()
    expected = """
<div><style>.ocdiff-container { display: flex; background-color: #141414; color: #acacac; }.ocdiff-side { width: 50%; overflow-x: auto; margin: 0; padding: 1rem; }.ocdiff-delete { color: red; }.ocdiff-insert { color: green; }</style><div class="ocdiff-container"><pre class="ocdiff-side"><span class="ocdiff-line ocdiff-equal">48</span>
<span class="ocdiff-line ocdiff-equal">49</span>
<span class="ocdiff-line ocdiff-none"></span>
<span class="ocdiff-line ocdiff-equal">50</span>
<span class="ocdiff-line ocdiff-equal">51</span>
</pre><pre class="ocdiff-side"><span class="ocdiff-line ocdiff-equal">48</span>
<span class="ocdiff-line ocdiff-equal">49</span>
<span class="ocdiff-line ocdiff-insert">999</span>
<span class="ocdiff-line ocdiff-equal">50</span>
<span class="ocdiff-line ocdiff-equal">51</span>
</pre></div></div>
""".strip()
    assert actual == expected


def test_html_diff_4() -> None:
    a = """
ZHV|0000000042|D0155001|X|MRCY|M|OESL|20220418000000||||OPER|
315|1900012374324|20220419|F|S|T|F|F|S|S|E|N|ABC 123|OESLMRCYMO|H|_J|
317|20220419|
318|0001|0001|
ZPT|0000000042|3||1|20220418000000|
    """.strip()
    b = """
ZHV|0000000042|D0155001|X|MRCY|M|OESL|20220418000000||||OPER|
315|1900012374324|20230101|F|S|T|F|F|S|S|E|N|ABC 123|OESLMRCYMO|H|_J|
317|20230101|
318|0001|0001|
ZPT|0000000042|3||1|20220418000000|
    """.strip()

    actual = ocdiff.html_diff(
        a,
        b,
        context_lines=2,
        column_limit=80,
    ).strip()
    expected = """
<div><style>.ocdiff-container { display: flex; background-color: #141414; color: #acacac; }.ocdiff-side { width: 50%; overflow-x: auto; margin: 0; padding: 1rem; }.ocdiff-delete { color: red; }.ocdiff-insert { color: green; }</style><div class="ocdiff-container"><pre class="ocdiff-side"><span class="ocdiff-line ocdiff-equal">ZHV|0000000042|D0155001|X|MRCY|M|OESL|20220418000000||||OPER|</span>
<span class="ocdiff-line ocdiff-equal">315|1900012374324|202</span><span class="ocdiff-line ocdiff-delete">2</span><span class="ocdiff-line ocdiff-equal">0</span><span class="ocdiff-line ocdiff-delete">4</span><span class="ocdiff-line ocdiff-equal">1</span><span class="ocdiff-line ocdiff-delete">9</span><span class="ocdiff-line ocdiff-equal">|F|S|T|F|F|S|S|E|N|ABC 123|OESLMRCYMO|H|_J|</span>
<span class="ocdiff-line ocdiff-equal">317|202</span><span class="ocdiff-line ocdiff-delete">2</span><span class="ocdiff-line ocdiff-equal">0</span><span class="ocdiff-line ocdiff-delete">4</span><span class="ocdiff-line ocdiff-equal">1</span><span class="ocdiff-line ocdiff-delete">9</span><span class="ocdiff-line ocdiff-equal">|</span>
<span class="ocdiff-line ocdiff-equal">318|0001|0001|</span>
<span class="ocdiff-line ocdiff-equal">ZPT|0000000042|3||1|20220418000000|</span>
</pre><pre class="ocdiff-side"><span class="ocdiff-line ocdiff-equal">ZHV|0000000042|D0155001|X|MRCY|M|OESL|20220418000000||||OPER|</span>
<span class="ocdiff-line ocdiff-equal">315|1900012374324|202</span><span class="ocdiff-line ocdiff-insert">301</span><span class="ocdiff-line ocdiff-equal">01|F|S|T|F|F|S|S|E|N|ABC 123|OESLMRCYMO|H|_J|</span>
<span class="ocdiff-line ocdiff-equal">317|202</span><span class="ocdiff-line ocdiff-insert">301</span><span class="ocdiff-line ocdiff-equal">01|</span>
<span class="ocdiff-line ocdiff-equal">318|0001|0001|</span>
<span class="ocdiff-line ocdiff-equal">ZPT|0000000042|3||1|20220418000000|</span>
</pre></div></div>
    """.strip()
    assert actual == expected
