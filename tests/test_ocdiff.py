import ocdiff


def test_mdiff() -> None:
    a = "Hello World\nThis is the second line.\nThis is the third."
    b = "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more"
    assert ocdiff.mdiff(a, b, 5) == [
        ((1, "Hello World"), (1, "Hallo Welt"), True),
        ((2, "This is the second line."), (2, "This is the second line."), False),
        ((3, "This is the third."), (3, "This is life."), True),
        ((-1, "\n"), (4, "Moar and more"), True),
    ]


def test_diff_lines() -> None:
    a = '"para": "A meta-markup language, used to create markup languages such as DocBook.",'
    b = '"para": "A to create markup languages such as DocBook."'
    x = '"para": "A \x00^m\x01et\x00^a-\x01markup\x00- language, used to create markup\x01 languages such as DocBook.",'
    y = '"para": "A \x00^to cr\x01e\x00+a\x01t\x00^e \x01markup languages such as DocBook.",'

    x = '"para": "A \x00-meta-markup\x01\x00- \x01\x00-language,\x01\x00- \x01\x00-used\x01\x00- \x01to create markup languages such as \x00-DocBook.",\x01',
    y = '"para": "A to create markup languages such as \x00^DocBook."\x01'
    assert ocdiff.diff_lines(a, b) == (a, b)
