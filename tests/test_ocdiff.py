import ocdiff


def test_part_eq() -> None:
    a = ocdiff.Part("asd", "EQUAL")
    b = ocdiff.Part("asd", "EQUAL")
    assert a == b

    assert a.value
    assert a.kind



def test_diff_eq() -> None:
    a = ocdiff.Diff(
        1,
        [
            ocdiff.Part("asd", "INSERT"),
        ],
        2,
        [
            ocdiff.Part("asd", "EQUAL"),
            ocdiff.Part("asd", "DELETE"),
        ],
    )
    b = ocdiff.Diff(
        1,
        [
            ocdiff.Part("asd", "INSERT"),
        ],
        2,
        [
            ocdiff.Part("asd", "EQUAL"),
            ocdiff.Part("asd", "DELETE"),
        ],
    )
    assert a == b

    assert a.left_lineno
    assert a.left
    assert a.right_lineno
    assert a.right


# def test_mdiff() -> None:
# a = "Hello World\nThis is the second line.\nThis is the third."
# b = "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more"
# assert ocdiff.mdiff(a, b, 5) == [
#     ((1, "Hello World"), (1, "Hallo Welt"), True),
#     ((2, "This is the second line."), (2, "This is the second line."), False),
#     ((3, "This is the third."), (3, "This is life."), True),
#     ((-1, "\n"), (4, "Moar and more"), True),
# ]
