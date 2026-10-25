#!/usr/bin/env python3
"""Cargo生成物の検査と整理を同じ排他境界で実行する。"""

from pathlib import Path
import errno
import os
import signal
import subprocess
import sys
import time


class CargoGuard:
    def __init__(self, root: Path):
        self.path = root / "tmp/maintenance/cargo-target.lock"

    @staticmethod
    def lock(file, blocking: bool) -> None:
        if os.name == "nt":
            import msvcrt

            file.seek(0)
            if not file.read(1):
                file.write(b"\0")
                file.flush()
            file.seek(0)
            while True:
                try:
                    msvcrt.locking(file.fileno(), msvcrt.LK_NBLCK, 1)
                    return
                except OSError as error:
                    if not blocking or error.errno not in (errno.EACCES, errno.EAGAIN):
                        raise
                    file.seek(0)
                    time.sleep(0.1)
        else:
            import fcntl

            fcntl.flock(file, fcntl.LOCK_EX | (0 if blocking else fcntl.LOCK_NB))

    def run(self, command: list[str], cleanup: bool) -> int:
        self.path.parent.mkdir(parents=True, exist_ok=True)
        with self.path.open("a+b") as lock:
            os.set_inheritable(lock.fileno(), False)
            try:
                self.lock(lock, blocking=not cleanup)
            except (BlockingIOError, OSError) as error:
                if cleanup and getattr(error, "errno", None) in (
                    errno.EACCES,
                    errno.EAGAIN,
                    errno.EWOULDBLOCK,
                ):
                    print(f"KUC cleanup refused: an active Cargo operation holds {self.path}", file=sys.stderr)
                    return 75
                raise error
            child: subprocess.Popen[bytes] | None = None
            pending_signals: list[int] = []
            def forward(signum, _frame):
                if child is None:
                    pending_signals.append(signum)
                elif child.poll() is None:
                    if os.name == "nt":
                        child.terminate()
                    else:
                        try:
                            os.killpg(child.pid, signum)
                        except ProcessLookupError:
                            pass

            previous = {number: signal.signal(number, forward) for number in (signal.SIGINT, signal.SIGTERM)}
            try:
                child = subprocess.Popen(command, start_new_session=os.name != "nt")
                for signum in pending_signals:
                    forward(signum, None)
                code = child.wait()
                return code if code >= 0 else 128 - code
            finally:
                for number, handler in previous.items():
                    signal.signal(number, handler)


def main() -> int:
    if len(sys.argv) < 4 or sys.argv[1] not in ("run", "clean") or sys.argv[2] != "--":
        print("usage: cargo_guard.py run|clean -- COMMAND [ARGS...]", file=sys.stderr)
        return 2
    return CargoGuard(Path(__file__).resolve().parents[2]).run(sys.argv[3:], cleanup=sys.argv[1] == "clean")


if __name__ == "__main__":
    sys.exit(main())
