#!/usr/bin/env python3
"""Xvfbのnative coverage環境がX11 server resetで接続を失わない契約。"""

from pathlib import Path
import ctypes
import os
import shlex
import shutil
import subprocess
import time
import unittest


ROOT = Path(__file__).resolve().parents[1]
RUN_STRICT = ROOT / "scripts/run-strict-coverage.sh"


def coverage_xvfb_arguments() -> list[str]:
    commands = [
        line for line in RUN_STRICT.read_text(encoding="utf-8").splitlines()
        if line.lstrip().startswith("Xvfb ")
    ]
    if len(commands) != 1:
        raise AssertionError("coverage must define exactly one Xvfb command")
    return shlex.split(commands[0].split(" >", 1)[0], comments=True)


class CoverageXvfbTest(unittest.TestCase):
    def test_run_strict_coverage_uses_noreset(self) -> None:
        self.assertIn("-noreset", coverage_xvfb_arguments())

    @unittest.skipUnless(
        os.name == "posix"
        and hasattr(os, "uname")
        and os.uname().sysname == "Linux"
        and os.environ.get("KUC_XVFB_REAL_TEST") == "1",
        "real Xvfb probe requires Linux and KUC_XVFB_REAL_TEST=1",
    )
    def test_real_xvfb_preserves_atom_generation_across_connections(self) -> None:
        xvfb = shutil.which("Xvfb")
        self.assertIsNotNone(xvfb, "Xvfb is required for the explicit real probe")
        x11 = ctypes.CDLL("libX11.so.6")
        x11.XOpenDisplay.argtypes = [ctypes.c_char_p]
        x11.XOpenDisplay.restype = ctypes.c_void_p
        x11.XInternAtom.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
        x11.XInternAtom.restype = ctypes.c_ulong
        x11.XSync.argtypes = [ctypes.c_void_p, ctypes.c_int]
        x11.XCloseDisplay.argtypes = [ctypes.c_void_p]

        display_number = 99
        while (
            Path(f"/tmp/.X{display_number}-lock").exists()
            or Path(f"/tmp/.X11-unix/X{display_number}").exists()
        ):
            display_number += 1
        display = f":{display_number}".encode()
        arguments = coverage_xvfb_arguments()
        arguments[0] = xvfb
        arguments[1] = f":{display_number}"
        server = subprocess.Popen(
            arguments,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
        )
        try:
            deadline = time.monotonic() + 10.0
            while True:
                handle = x11.XOpenDisplay(display)
                if handle:
                    x11.XCloseDisplay(handle)
                    break
                if time.monotonic() >= deadline:
                    stderr = (server.stderr.read().decode(errors="replace") if server.poll() is not None and server.stderr else "server did not accept an X11 connection")
                    self.fail(f"Xvfb display did not become ready: {stderr}")
                time.sleep(0.01)

            failures = 0
            generations = 0
            for _ in range(100):
                handle = x11.XOpenDisplay(display)
                if not handle:
                    failures += 1
                    continue
                if x11.XInternAtom(handle, b"KUC_SERVER_GENERATION_PROBE", 1) == 0:
                    generations += 1
                x11.XInternAtom(handle, b"KUC_SERVER_GENERATION_PROBE", 0)
                x11.XSync(handle, 0)
                x11.XCloseDisplay(handle)
            self.assertEqual(failures, 0)
            self.assertEqual(generations, 1)
        finally:
            server.terminate()
            try:
                server.wait(timeout=10)
            except subprocess.TimeoutExpired:
                server.kill()
                server.wait(timeout=10)
            finally:
                if server.stderr is not None:
                    server.stderr.close()


if __name__ == "__main__":
    unittest.main()
