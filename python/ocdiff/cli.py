import argparse

import ocdiff


def main() -> None:
    parser = argparse.ArgumentParser(description="side-by-side diff files")
    parser.add_argument("files", nargs="+", help="Files to diff")
    parser.add_argument(
        "--context-lines",
        type=int,
        default=5,
        help="Number of context lines, defaults to 5, set to -1 for all context",
    )
    parser.add_argument(
        "--max-total-width",
        type=int,
        default=None,
        help="Maximum total width, defaults to existing console width",
    )
    parser.add_argument(
        "--html",
        action="store_true",
        help="Return a html diff",
    )
    args = parser.parse_args()

    if len(args.files) != 2:
        raise RuntimeError("Only specify two files like: ocdiff a.txt b.txt")

    path_a, path_b = args.files
    with open(path_a) as a, open(path_b) as b:
        context_lines = None if args.context_lines == -1 else args.context_lines
        max_total_width = args.max_total_width
        if args.html:
            stdout = ocdiff.html_diff(
                a.read(),
                b.read(),
                context_lines=context_lines,
                max_total_width=max_total_width,
            )
        else:
            stdout = ocdiff.console_diff(
                a.read(),
                b.read(),
                context_lines=context_lines,
                max_total_width=max_total_width,
            )
        print(stdout)
