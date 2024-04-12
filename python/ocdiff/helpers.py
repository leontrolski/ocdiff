import fcntl
import os
import struct
import termios


# From https://github.com/jeffkaufman/icdiff
def terminal_width() -> int:
    try:
        if os.name == "nt":
            from ctypes import windll, create_string_buffer  # type: ignore[attr-defined]

            fh = windll.kernel32.GetStdHandle(-12)  # stderr is -12
            csbi = create_string_buffer(22)
            windll.kernel32.GetConsoleScreenBufferInfo(fh, csbi)
            res = struct.unpack("hhhhHhhhhhh", csbi.raw)
            # right - left + 1
            return res[7] - res[5] + 1  # type: ignore[no-any-return]
        else:
            # From: https://gist.github.com/jtriley/1108174
            fd = os.open(os.ctermid(), os.O_RDONLY)
            cr = struct.unpack("hh", fcntl.ioctl(fd, termios.TIOCGWINSZ, "1234"))  # type: ignore
            os.close(fd)
            return int(cr[1])
    except Exception:
        return 80
