#!/usr/bin/env python3
"""cargo_guard の実プロセス排他契約を検証する。"""

from pathlib import Path
import os
import shutil
import signal
import subprocess
import sys
import tempfile
import time


ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/maintenance/cargo_guard.py"


def fixture_guard() -> tuple[tempfile.TemporaryDirectory[str], Path, Path]:
    temporary = tempfile.TemporaryDirectory(prefix="kuc-cargo-guard-")
    root = Path(temporary.name)
    copied_guard = root / "scripts/maintenance/cargo_guard.py"
    copied_guard.parent.mkdir(parents=True)
    shutil.copy2(GUARD, copied_guard)
    return temporary, root, copied_guard


def run_guard(guard: Path, root: Path, mode: str, *command: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(guard), mode, "--", *command],
        cwd=root,
        text=True,
        capture_output=True,
        check=False,
    )


def wait_for_marker(marker: Path) -> None:
    deadline = time.monotonic() + 5.0
    while not marker.exists() and time.monotonic() < deadline:
        time.sleep(0.01)
    assert marker.exists(), f"producer did not signal readiness: {marker}"


def test_cleanup_refuses_active_producer_then_succeeds_after_release() -> None:
    temporary, root, guard = fixture_guard()
    try:
        ready = root / "producer-ready"
        child_pid = root / "producer-child-pid"
        marker = root / "cleanup-marker"
        producer = subprocess.Popen(
            [
                sys.executable,
                str(guard),
                "run",
                "--",
                sys.executable,
                "-c",
                "from pathlib import Path; import os, sys; Path(sys.argv[1]).write_text('ready'); Path(sys.argv[2]).write_text(str(os.getpid())); sys.stdin.buffer.read()",
                str(ready),
                str(child_pid),
            ],
            cwd=root,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            wait_for_marker(ready)
            cleanup = run_guard(
                guard,
                root,
                "clean",
                sys.executable,
                "-c",
                "from pathlib import Path; Path(__import__('sys').argv[1]).write_text('cleaned')",
                str(marker),
            )
            assert cleanup.returncode == 75, cleanup.stdout + cleanup.stderr
            assert not marker.exists(), "cleanup command ran while producer held the lock"

            assert producer.stdin is not None
            producer.stdin.close()
            assert producer.wait(timeout=5) == 0
            cleanup = run_guard(
                guard,
                root,
                "clean",
                sys.executable,
                "-c",
                "from pathlib import Path; Path(__import__('sys').argv[1]).write_text('cleaned')",
                str(marker),
            )
            assert cleanup.returncode == 0, cleanup.stdout + cleanup.stderr
            assert marker.read_text(encoding="utf-8") == "cleaned"
        finally:
            if producer.poll() is None:
                if producer.stdin is not None and not producer.stdin.closed:
                    producer.stdin.close()
                try:
                    producer.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    if child_pid.exists() and os.name != "nt":
                        try:
                            os.killpg(int(child_pid.read_text(encoding="utf-8")), signal.SIGKILL)
                        except ProcessLookupError:
                            pass
                    producer.kill()
                    producer.wait(timeout=5)
    finally:
        temporary.cleanup()


def test_run_propagates_child_exit_code() -> None:
    temporary, root, guard = fixture_guard()
    try:
        result = run_guard(guard, root, "run", sys.executable, "-c", "raise SystemExit(37)")
        assert result.returncode == 37, result.stdout + result.stderr
    finally:
        temporary.cleanup()


def test_signal_is_forwarded_to_a_running_child_process_group() -> None:
    if os.name == "nt":
        return
    temporary, root, guard = fixture_guard()
    try:
        ready = root / "signal-ready"
        child_pid = root / "signal-child-pid"
        producer = subprocess.Popen(
            [
                sys.executable,
                str(guard),
                "run",
                "--",
                sys.executable,
                "-c",
                "from pathlib import Path; import os, signal, sys; Path(sys.argv[1]).write_text('ready'); Path(sys.argv[2]).write_text(str(os.getpid())); signal.pause()",
                str(ready),
                str(child_pid),
            ],
            cwd=root,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            wait_for_marker(ready)
            producer.send_signal(signal.SIGTERM)
            assert producer.wait(timeout=5) == 143
        finally:
            if producer.poll() is None:
                if child_pid.exists():
                    try:
                        os.killpg(int(child_pid.read_text(encoding="utf-8")), signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                producer.kill()
                producer.wait(timeout=5)
    finally:
        temporary.cleanup()


def test_invalid_arguments_are_rejected() -> None:
    temporary, root, guard = fixture_guard()
    try:
        result = subprocess.run(
            [sys.executable, str(guard), "clean", "not-a-separator", "true"],
            cwd=root,
            text=True,
            capture_output=True,
            check=False,
        )
        assert result.returncode == 2
        assert "usage: cargo_guard.py" in result.stderr
    finally:
        temporary.cleanup()


if __name__ == "__main__":
    test_cleanup_refuses_active_producer_then_succeeds_after_release()
    test_run_propagates_child_exit_code()
    signal_tests = 0
    if os.name != "nt":
        test_signal_is_forwarded_to_a_running_child_process_group()
        signal_tests = 1
    test_invalid_arguments_are_rejected()
    print(f"cargo guard tests: {3 + signal_tests} passed")
