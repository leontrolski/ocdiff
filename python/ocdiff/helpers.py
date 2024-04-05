import os
from typing import Any


# From https://github.com/jeffkaufman/icdiff
def get_number_of_cols() -> int:
    if os.name == "nt":
        try:
            import struct
            from ctypes import windll, create_string_buffer  # type: ignore[attr-defined]

            fh = windll.kernel32.GetStdHandle(-12)  # stderr is -12
            csbi = create_string_buffer(22)
            windll.kernel32.GetConsoleScreenBufferInfo(fh, csbi)
            res = struct.unpack("hhhhHhhhhhh", csbi.raw)
            # right - left + 1
            return res[7] - res[5] + 1  # type: ignore[no-any-return]

        except Exception:
            pass

    else:

        def ioctl_GWINSZ(fd: Any) -> tuple[Any, ...] | None:
            try:
                import fcntl
                import termios
                import struct

                cr = struct.unpack(
                    "hh", fcntl.ioctl(fd, termios.TIOCGWINSZ, "1234")  # type: ignore[call-overload]
                )
            except Exception:
                return None
            return cr

        cr = ioctl_GWINSZ(0) or ioctl_GWINSZ(1) or ioctl_GWINSZ(2)
        if cr and cr[1] > 0:
            return cr[1]  # type: ignore[no-any-return]

    return 80


# Other version...
# From: https://gist.github.com/jtriley/1108174
def get_terminal_width() -> int:
    fd = os.open(os.ctermid(), os.O_RDONLY)
    cr = struct.unpack("hh", fcntl.ioctl(fd, termios.TIOCGWINSZ, "1234"))  # type: ignore
    os.close(fd)
    return int(cr[1])
