import copy
from pathlib import Path
import ocdiff


def _clean(s: str) -> str:
    return "\n".join(s.strip().splitlines()[1:-1])


def test_html_diff() -> None:
    actual = ocdiff.html_diff(
        (Path(__file__).parent / "a.json").read_text(),
        (Path(__file__).parent / "b.json").read_text(),
        context_lines=5,
    )
    # (Path(__file__).parent / "out.html").write_text(actual)
    expected = (Path(__file__).parent / "expected.html").read_text().strip()
    assert _clean(actual) == expected


def test_html_diff_2() -> None:
    actual = ocdiff.html_diff(
        (Path(__file__).parent / "x.json").read_text(),
        (Path(__file__).parent / "y.json").read_text(),
        context_lines=1,
    )
    # (Path(__file__).parent / "out.html").write_text(actual)
    expected = (Path(__file__).parent / "expected_2.html").read_text().strip()
    assert _clean(actual) == expected


def test_html_diff_3() -> None:
    ns = [str(n) for n in range(100)]
    ms = copy.copy(ns)
    ms.insert(50, "999")

    actual = ocdiff.html_diff(
        "\n".join(ns),
        "\n".join(ms),
        context_lines=2,
    )
    expected = """
<span class="ocdiff-lineno"> 50 </span><span class="ocdiff-line ocdiff-equal">49</span>
<span class="ocdiff-lineno">    </span><span class="ocdiff-line ocdiff-none"></span>
<span class="ocdiff-lineno"> 51 </span><span class="ocdiff-line ocdiff-equal">50</span>
<span class="ocdiff-lineno"> 52 </span><span class="ocdiff-line ocdiff-equal">51</span>
</pre><pre class="ocdiff-side"><span class="ocdiff-lineno"> 49 </span><span class="ocdiff-line ocdiff-equal">48</span>
<span class="ocdiff-lineno"> 50 </span><span class="ocdiff-line ocdiff-equal">49</span>
<span class="ocdiff-lineno"> 51 </span><span class="ocdiff-line ocdiff-insert">999</span>
<span class="ocdiff-lineno"> 52 </span><span class="ocdiff-line ocdiff-equal">50</span>
<span class="ocdiff-lineno"> 53 </span><span class="ocdiff-line ocdiff-equal">51</span>
""".strip()
    assert _clean(actual) == expected


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
    )
    expected = """
<span class="ocdiff-lineno"> 2 </span><span class="ocdiff-line ocdiff-equal">315|1900012374324|202</span><span class="ocdiff-line ocdiff-delete">2</span><span class="ocdiff-line ocdiff-equal">0</span><span class="ocdiff-line ocdiff-delete">4</span><span class="ocdiff-line ocdiff-equal">1</span><span class="ocdiff-line ocdiff-delete">9</span><span class="ocdiff-line ocdiff-equal">|F|S|T|F|F|S|S|E|N|ABC 123|OESLMRCYMO|H|_J|</span>
<span class="ocdiff-lineno"> 3 </span><span class="ocdiff-line ocdiff-equal">317|202</span><span class="ocdiff-line ocdiff-delete">2</span><span class="ocdiff-line ocdiff-equal">0</span><span class="ocdiff-line ocdiff-delete">4</span><span class="ocdiff-line ocdiff-equal">1</span><span class="ocdiff-line ocdiff-delete">9</span><span class="ocdiff-line ocdiff-equal">|</span>
<span class="ocdiff-lineno"> 4 </span><span class="ocdiff-line ocdiff-equal">318|0001|0001|</span>
<span class="ocdiff-lineno"> 5 </span><span class="ocdiff-line ocdiff-equal">ZPT|0000000042|3||1|20220418000000|</span>
</pre><pre class="ocdiff-side"><span class="ocdiff-lineno"> 1 </span><span class="ocdiff-line ocdiff-equal">ZHV|0000000042|D0155001|X|MRCY|M|OESL|20220418000000||||OPER|</span>
<span class="ocdiff-lineno"> 2 </span><span class="ocdiff-line ocdiff-equal">315|1900012374324|202</span><span class="ocdiff-line ocdiff-insert">301</span><span class="ocdiff-line ocdiff-equal">01|F|S|T|F|F|S|S|E|N|ABC 123|OESLMRCYMO|H|_J|</span>
<span class="ocdiff-lineno"> 3 </span><span class="ocdiff-line ocdiff-equal">317|202</span><span class="ocdiff-line ocdiff-insert">301</span><span class="ocdiff-line ocdiff-equal">01|</span>
<span class="ocdiff-lineno"> 4 </span><span class="ocdiff-line ocdiff-equal">318|0001|0001|</span>
<span class="ocdiff-lineno"> 5 </span><span class="ocdiff-line ocdiff-equal">ZPT|0000000042|3||1|20220418000000|</span>
    """.strip()
    # (Path(__file__).parent / "out.html").write_text(actual)
    assert _clean(actual) == expected


def test_html_diff_5() -> None:
    a = """
"certification_date": "2015-02-03",
"certification_expiry_date": "2035-02-03",
"current_rating": 80,
"installation_datetime": "2016-02-24T00:00:00Z",
"location": "E",
    """.strip()
    b = """
"certification_date": "2023-05-10",
"certification_expiry_date": "2043-05-09",
"current_rating": 100,
"installation_datetime": "2023-09-18T14:00:00+01:00",
"location": "H",
    """.strip()

    actual = ocdiff.html_diff(
        a,
        b,
    )
    expected = """
<span class="ocdiff-lineno"> 2 </span><span class="ocdiff-line ocdiff-equal">"certification_expiry_date": "203</span><span class="ocdiff-line ocdiff-delete">5</span><span class="ocdiff-line ocdiff-equal">-0</span><span class="ocdiff-line ocdiff-delete">2</span><span class="ocdiff-line ocdiff-equal">-0</span><span class="ocdiff-line ocdiff-delete">3</span><span class="ocdiff-line ocdiff-equal">",</span>
<span class="ocdiff-lineno"> 3 </span><span class="ocdiff-line ocdiff-equal">"current_rating": </span><span class="ocdiff-line ocdiff-delete">8</span><span class="ocdiff-line ocdiff-equal">0,</span>
<span class="ocdiff-lineno"> 4 </span><span class="ocdiff-line ocdiff-equal">"installation_datetime": "20</span><span class="ocdiff-line ocdiff-delete">16</span><span class="ocdiff-line ocdiff-equal">-0</span><span class="ocdiff-line ocdiff-delete">2</span><span class="ocdiff-line ocdiff-equal">-</span><span class="ocdiff-line ocdiff-delete">24</span><span class="ocdiff-line ocdiff-equal">T00:00:00</span><span class="ocdiff-line ocdiff-delete">Z</span><span class="ocdiff-line ocdiff-equal">",</span>
<span class="ocdiff-lineno"> 5 </span><span class="ocdiff-line ocdiff-equal">"location": "</span><span class="ocdiff-line ocdiff-delete">E</span><span class="ocdiff-line ocdiff-equal">",</span>
</pre><pre class="ocdiff-side"><span class="ocdiff-lineno"> 1 </span><span class="ocdiff-line ocdiff-equal">"certification_date": "20</span><span class="ocdiff-line ocdiff-insert">23</span><span class="ocdiff-line ocdiff-equal">-0</span><span class="ocdiff-line ocdiff-insert">5</span><span class="ocdiff-line ocdiff-equal">-</span><span class="ocdiff-line ocdiff-insert">1</span><span class="ocdiff-line ocdiff-equal">0",</span>
<span class="ocdiff-lineno"> 2 </span><span class="ocdiff-line ocdiff-equal">"certification_expiry_date": "20</span><span class="ocdiff-line ocdiff-insert">4</span><span class="ocdiff-line ocdiff-equal">3-0</span><span class="ocdiff-line ocdiff-insert">5</span><span class="ocdiff-line ocdiff-equal">-0</span><span class="ocdiff-line ocdiff-insert">9</span><span class="ocdiff-line ocdiff-equal">",</span>
<span class="ocdiff-lineno"> 3 </span><span class="ocdiff-line ocdiff-equal">"current_rating": </span><span class="ocdiff-line ocdiff-insert">10</span><span class="ocdiff-line ocdiff-equal">0,</span>
<span class="ocdiff-lineno"> 4 </span><span class="ocdiff-line ocdiff-equal">"installation_datetime": "20</span><span class="ocdiff-line ocdiff-insert">23</span><span class="ocdiff-line ocdiff-equal">-0</span><span class="ocdiff-line ocdiff-insert">9</span><span class="ocdiff-line ocdiff-equal">-</span><span class="ocdiff-line ocdiff-insert">18</span><span class="ocdiff-line ocdiff-equal">T</span><span class="ocdiff-line ocdiff-insert">14:</span><span class="ocdiff-line ocdiff-equal">00:00</span><span class="ocdiff-line ocdiff-insert">+01</span><span class="ocdiff-line ocdiff-equal">:00",</span>
<span class="ocdiff-lineno"> 5 </span><span class="ocdiff-line ocdiff-equal">"location": "</span><span class="ocdiff-line ocdiff-insert">H</span><span class="ocdiff-line ocdiff-equal">",</span>
    """.strip()
    # (Path(__file__).parent / "out.html").write_text(actual)
    assert _clean(actual) == expected


def test_html_diff_max_total_width() -> None:
    actual = ocdiff.html_diff(
        (Path(__file__).parent / "a.json").read_text(),
        (Path(__file__).parent / "b.json").read_text(),
        max_total_width=80,
    ).strip()
    expected = """
<span class="ocdiff-lineno"> 3  </span><span class="ocdiff-line ocdiff-equal">        "title": "example glossary",</span>
<span class="ocdiff-lineno"> 4  </span><span class="ocdiff-line ocdiff-equal">        "GlossDiv": {</span>
<span class="ocdiff-lineno"> 5  </span><span class="ocdiff-line ocdiff-delete">            "title": "S",</span>
<span class="ocdiff-lineno"> 6  </span><span class="ocdiff-line ocdiff-equal">            "GlossList": {</span>
<span class="ocdiff-lineno"> 7  </span><span class="ocdiff-line ocdiff-equal">                "GlossEntry": {</span>
<span class="ocdiff-lineno"> 8  </span><span class="ocdiff-line ocdiff-equal">                    "ID": "SGML",</span>
<span class="ocdiff-lineno"> 9  </span><span class="ocdiff-line ocdiff-equal">                    "SortAs": "SGML"</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">,</span>
<span class="ocdiff-lineno"> 10 </span><span class="ocdiff-line ocdiff-equal">                    "GlossTerm": "St</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">andard Generalized Markup Language",</span>
<span class="ocdiff-lineno"> 11 </span><span class="ocdiff-line ocdiff-equal">                    "Acronym": "SGML</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">",</span>
<span class="ocdiff-lineno"> 12 </span><span class="ocdiff-line ocdiff-equal">                    "Abbrev": "ISO 8</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">879:1986",</span>
<span class="ocdiff-lineno"> 13 </span><span class="ocdiff-line ocdiff-equal">                    "GlossDef": {</span>
<span class="ocdiff-lineno"> 14 </span><span class="ocdiff-line ocdiff-equal">                        "para": "A </span><span class="ocdiff-line ocdiff-delete">m</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-delete">eta-markup language, used </span><span class="ocdiff-line ocdiff-equal">to create </span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">markup languages such as DocBook.",</span>
<span class="ocdiff-lineno"> 15 </span><span class="ocdiff-line ocdiff-equal">                        "GlossSeeAls</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">o": [</span>
<span class="ocdiff-lineno"> 16 </span><span class="ocdiff-line ocdiff-delete">                            "GML",</span>
<span class="ocdiff-lineno"> 17 </span><span class="ocdiff-line ocdiff-equal">                            "XML"</span>
<span class="ocdiff-lineno"> 18 </span><span class="ocdiff-line ocdiff-equal">                        ]</span>
<span class="ocdiff-lineno"> 19 </span><span class="ocdiff-line ocdiff-equal">                    },</span>
<span class="ocdiff-lineno"> 20 </span><span class="ocdiff-line ocdiff-equal">                    "GlossSee": "mar</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">kup"</span>
<span class="ocdiff-lineno">    </span><span class="ocdiff-line ocdiff-none"></span>
<span class="ocdiff-lineno"> 21 </span><span class="ocdiff-line ocdiff-equal">                }</span>
<span class="ocdiff-lineno"> 22 </span><span class="ocdiff-line ocdiff-equal">            }</span>
<span class="ocdiff-lineno"> 23 </span><span class="ocdiff-line ocdiff-equal">        }</span>
</pre><pre class="ocdiff-side"><span class="ocdiff-lineno"> 2  </span><span class="ocdiff-line ocdiff-equal">    "glossary": {</span>
<span class="ocdiff-lineno"> 3  </span><span class="ocdiff-line ocdiff-equal">        "title": "example glossary",</span>
<span class="ocdiff-lineno"> 4  </span><span class="ocdiff-line ocdiff-equal">        "GlossDiv": {</span>
<span class="ocdiff-lineno">    </span><span class="ocdiff-line ocdiff-none"></span>
<span class="ocdiff-lineno"> 5  </span><span class="ocdiff-line ocdiff-equal">            "GlossList": {</span>
<span class="ocdiff-lineno"> 6  </span><span class="ocdiff-line ocdiff-equal">                "GlossEntry</span><span class="ocdiff-line ocdiff-insert">yyyy</span><span class="ocdiff-line ocdiff-equal">": {</span>
<span class="ocdiff-lineno"> 7  </span><span class="ocdiff-line ocdiff-equal">                    "ID": "SGML",</span>
<span class="ocdiff-lineno"> 8  </span><span class="ocdiff-line ocdiff-equal">                    "SortAs": "SGML"</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">,</span>
<span class="ocdiff-lineno"> 9  </span><span class="ocdiff-line ocdiff-equal">                    "GlossTerm": "St</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">andard Generalized Markup Language",</span>
<span class="ocdiff-lineno"> 10 </span><span class="ocdiff-line ocdiff-equal">                    "Acronym": "SGML</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">",</span>
<span class="ocdiff-lineno"> 11 </span><span class="ocdiff-line ocdiff-equal">                    "Abbrev": "ISO 8</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">879:1986",</span>
<span class="ocdiff-lineno"> 12 </span><span class="ocdiff-line ocdiff-equal">                    "GlossDef": {</span>
<span class="ocdiff-lineno"> 13 </span><span class="ocdiff-line ocdiff-equal">                        "para": "A t</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">o create markup languages such as Do</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">cBook.",</span>
<span class="ocdiff-lineno"> 14 </span><span class="ocdiff-line ocdiff-equal">                        "GlossSeeAls</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">o": [</span>
<span class="ocdiff-lineno">    </span><span class="ocdiff-line ocdiff-none"></span>
<span class="ocdiff-lineno"> 15 </span><span class="ocdiff-line ocdiff-equal">                            "XML"</span>
<span class="ocdiff-lineno"> 16 </span><span class="ocdiff-line ocdiff-equal">                        ]</span>
<span class="ocdiff-lineno"> 17 </span><span class="ocdiff-line ocdiff-equal">                    },</span>
<span class="ocdiff-lineno"> 18 </span><span class="ocdiff-line ocdiff-equal">                    "GlossSee": "mar</span>
<span class="ocdiff-lineno"> …  </span><span class="ocdiff-line ocdiff-equal">kup"</span><span class="ocdiff-line ocdiff-insert">,</span>
<span class="ocdiff-lineno"> 19 </span><span class="ocdiff-line ocdiff-insert">                    "Hullo": 42</span>
<span class="ocdiff-lineno"> 20 </span><span class="ocdiff-line ocdiff-equal">                }</span>
<span class="ocdiff-lineno"> 21 </span><span class="ocdiff-line ocdiff-equal">            }</span>
<span class="ocdiff-lineno"> 22 </span><span class="ocdiff-line ocdiff-equal">        }</span>
    """.strip()
    # (Path(__file__).parent / "out.html").write_text(actual)
    assert _clean(actual) == expected
