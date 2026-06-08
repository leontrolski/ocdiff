import os


# From https://github.com/jeffkaufman/icdiff
# See: https://github.com/bbhunter/icdiff/commit/173825cc5416377dc5d37d562b54a9854e385370
def terminal_width() -> int:
    if os.name == "nt":
        try:
            import struct
            from ctypes import windll, create_string_buffer  # type: ignore

            fh = windll.kernel32.GetStdHandle(-12)  # stderr is -12
            csbi = create_string_buffer(22)
            windll.kernel32.GetConsoleScreenBufferInfo(fh, csbi)
            res = struct.unpack("hhhhHhhhhhh", csbi.raw)
            return res[7] - res[5] + 1  # type: ignore # right - left + 1
        except Exception:
            pass
    else:
        if width := ioctl_GWINSZ(0) or ioctl_GWINSZ(1) or ioctl_GWINSZ(2):
            return width

    return 80


def ioctl_GWINSZ(fd: int) -> int | None:
    import fcntl
    import struct
    import termios

    try:
        cr = struct.unpack("hhhh", fcntl.ioctl(fd, termios.TIOCGWINSZ, "12345678"))  # type: ignore
    except Exception:
        return None
    return cr[1] if cr else None
