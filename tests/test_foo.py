import ocdiff


def test_foo() -> None:
    assert ocdiff.diff() == [
        (("0", "Hello World\n"), ("0", "Hallo Welt\n"), True),
        (
            ("1", "This is the second line.\n"),
            ("1", "This is the second line.\n"),
            True,
        ),
        (("2", "This is the third."), ("2", "This is life.\n"), True),
        (("", ""), ("3", "Moar and more"), True),
    ]
