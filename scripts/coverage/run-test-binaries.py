#!/usr/bin/env python3
from __future__ import annotations
import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
ALLOWED_PACKAGES = (
    "katana-ui-core",
    "katana-ui-core-storybook",
    "kuc-consumer-app",
)
MAX_SUPPORTED_PARALLEL_BINARIES = 2
HIGH_COST_EXECUTABLE_PREFIXES = (
    "katana_ui_core-",
    "katana_ui_core_storybook-",
    "native_window_contract-",
)
MEDIUM_COST_EXECUTABLE_PREFIXES = ("egui_",)
class RunnerError(RuntimeError):
    pass
def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise RunnerError(message)
def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser()
    p.add_argument("--artifact-json")
    p.add_argument("--metadata-json")
    p.add_argument("--logs-dir")
    p.add_argument("--max-parallel-binaries", type=int)
    p.add_argument("--test-threads", type=int)
    p.add_argument("--self-test", action="store_true")
    return p.parse_args()
def ensure_llvm_profile_file(env: dict[str, str]) -> None:
    value = env.get("LLVM_PROFILE_FILE", "")
    if not value:
        fail("LLVM_PROFILE_FILE is required")
    if "%p" not in value:
        fail("LLVM_PROFILE_FILE must include %p placeholder")
    if not re.search(r"%\d*m", value):
        fail("LLVM_PROFILE_FILE must include %m-like placeholder")
def read_json(path: str) -> object:
    try:
        return json.loads(Path(path).read_text())
    except (OSError, json.JSONDecodeError) as error:
        fail(f"failed to read JSON '{path}': {error}")
def parse_metadata(path: str) -> dict[str, tuple[str, Path]]:
    data = read_json(path)
    if not isinstance(data, dict):
        fail("metadata JSON is not an object")
    packages = data.get("packages")
    if not isinstance(packages, list):
        fail("metadata JSON missing packages list")
    md_dir = Path(path).resolve().parent
    allowed: dict[str, tuple[str, Path]] = {}
    found: set[str] = set()
    for raw in packages:
        if not isinstance(raw, dict):
            fail("metadata package is not an object")
        name = raw.get("name")
        pid = raw.get("id")
        manifest_raw = raw.get("manifest_path")
        if name not in ALLOWED_PACKAGES:
            continue
        if not isinstance(name, str) or not isinstance(pid, str) or not isinstance(manifest_raw, str) or pid in allowed:
            fail("metadata package missing required field")
        if name in found:
            fail(f"duplicate package name '{name}'")
        manifest = Path(manifest_raw)
        if not manifest.is_absolute():
            manifest = (md_dir / manifest).resolve()
        if not manifest.exists():
            fail(f"manifest not found: {manifest}")
        allowed[pid] = (name, manifest.parent)
        found.add(name)
    if len(found) != len(ALLOWED_PACKAGES):
        missing = [n for n in ALLOWED_PACKAGES if n not in found]
        fail(f"metadata missing required packages: {', '.join(missing)}")
    return allowed
def parse_artifacts(path: str, allowed: dict[str, tuple[str, Path]]) -> list[tuple[str, Path, Path]]:
    try:
        lines = Path(path).read_text().splitlines()
    except OSError as error:
        fail(f"failed to read artifact stream '{path}': {error}")
    seen: set[Path] = set()
    counts = {name: 0 for name in ALLOWED_PACKAGES}
    bins: list[tuple[str, Path, Path]] = []
    for line_no, raw in enumerate(lines, 1):
        if not raw.strip():
            continue
        try:
            e = json.loads(raw)
        except json.JSONDecodeError as error:
            fail(f"artifact JSON invalid at line {line_no}: {error}")
        if not isinstance(e, dict) or e.get("reason") != "compiler-artifact":
            continue
        profile = e.get("profile")
        is_test_executable = isinstance(profile, dict) and profile.get("test") is True
        repair_bind_executable = os.environ.get("KUC_COVERAGE_BIND_TARGET") == "1"
        if not is_test_executable and not repair_bind_executable:
            continue
        pid = e.get("package_id")
        exe_raw = e.get("executable")
        if not isinstance(pid, str) or pid not in allowed:
            if not is_test_executable:
                continue
            fail(f"artifact line {line_no}: unknown package_id '{pid}'")
        if not isinstance(exe_raw, str) or not exe_raw:
            if not is_test_executable:
                continue
            fail(f"artifact line {line_no}: missing executable")
        name, manifest_dir = allowed[pid]
        exe = Path(exe_raw)
        if not exe.is_absolute():
            exe = manifest_dir / exe
        exe = exe.resolve()
        if not exe.exists():
            fail(f"artifact line {line_no}: executable not found '{exe}'")
        if not os.access(exe, os.X_OK):
            if os.environ.get("KUC_COVERAGE_BIND_TARGET") == "1":
                try:
                    exe.chmod(exe.stat().st_mode | 0o111)
                except OSError as error:
                    fail(f"artifact line {line_no}: failed to restore executable mode '{exe}': {error}")
            if not os.access(exe, os.X_OK):
                fail(f"artifact line {line_no}: executable not runnable '{exe}'")
        if not is_test_executable:
            continue
        if exe in seen:
            fail(f"artifact line {line_no}: duplicate executable '{exe}'")
        seen.add(exe)
        counts[name] += 1
        bins.append((name, exe, manifest_dir))
    if not bins:
        fail("no test executables discovered from artifact stream")
    missing = [name for name in ALLOWED_PACKAGES if counts[name] == 0]
    if missing:
        fail(f"no test executable discovered for package(s): {', '.join(missing)}")
    return bins
def run_binary(task: tuple[str, Path, Path], run_root: Path, index: int, test_threads: int) -> tuple[str, Path, int, Path, Path, float]:
    package, exe, manifest_dir = task
    out_dir = run_root / f"{index:03d}_{package}_{re.sub(r'[^A-Za-z0-9_.-]', '-', exe.name)}"
    out_dir.mkdir(parents=True)
    runtime_dir = Path(tempfile.mkdtemp(prefix=f"kuc-xdg-{index}-"))
    os.chmod(runtime_dir, 0o700)
    if len(os.fsencode(runtime_dir / "wayland-0")) + 1 > 108:
        shutil.rmtree(runtime_dir, ignore_errors=True)
        raise RunnerError(f"XDG runtime socket path is too long: {runtime_dir}")
    stdout = out_dir / "stdout.log"
    stderr = out_dir / "stderr.log"
    env = os.environ.copy()
    env["XDG_RUNTIME_DIR"] = str(runtime_dir)
    env["KUC_STORYBOOK_MOUSE_TRACE"] = str(out_dir / "storybook-mouse-trace.jsonl")
    started = time.monotonic()
    try:
        with stdout.open("wb") as so, stderr.open("wb") as se:
            proc = subprocess.run(
                [str(exe), "--include-ignored", f"--test-threads={test_threads}"],
                cwd=str(manifest_dir),
                env=env,
                stdout=so,
                stderr=se,
            )
    finally:
        shutil.rmtree(runtime_dir, ignore_errors=True)
    return package, exe, proc.returncode, stdout, stderr, time.monotonic() - started
def emit_failures(failed: list[tuple[str, Path, int, Path, Path, float]]) -> None:
    for package, exe, code, so, se, _elapsed in failed:
        print(f"FAILED {package} code={code} exe={exe}", file=sys.stderr)
        print("[stdout]", file=sys.stderr)
        if so.exists():
            print(so.read_text(errors="replace"), file=sys.stderr)
        print("[stderr]", file=sys.stderr)
        if se.exists():
            print(se.read_text(errors="replace"), file=sys.stderr)
def schedule_batches(binaries: list[tuple[str, Path, Path]], max_parallel: int) -> list[list[tuple[str, Path, Path]]]:
    # 実測で長い library/native target を最初に開始し、短い egui contract 群が
    # 13分級の Storybook library の開始を遅らせないようにする。
    def cost(task: tuple[str, Path, Path]) -> int:
        name = task[1].name
        if name.startswith(HIGH_COST_EXECUTABLE_PREFIXES):
            return 0
        if name.startswith(MEDIUM_COST_EXECUTABLE_PREFIXES):
            return 1
        return 2

    ordered = sorted(
        enumerate(binaries),
        key=lambda item: (cost(item[1]), item[0]),
    )
    tasks = [binary for _index, binary in ordered]
    return [tasks[offset:offset + max_parallel] for offset in range(0, len(tasks), max_parallel)]
def run_suite(binaries: list[tuple[str, Path, Path]], logs_dir: Path, max_parallel: int, test_threads: int) -> int:
    logs_dir.mkdir(parents=True, exist_ok=True)
    run_root = logs_dir / f"run-{time.time_ns()}"
    run_root.mkdir()
    results: list[tuple[str, Path, int, Path, Path, float]] = []
    failed: list[tuple[str, Path, int, Path, Path, float]] = []
    start = time.monotonic()
    ordered = [task for batch in schedule_batches(binaries, max_parallel) for task in batch]
    cursor = 0
    with ThreadPoolExecutor(max_workers=max_parallel) as ex:
        futures = {}
        while cursor < min(max_parallel, len(ordered)):
            task = ordered[cursor]
            futures[ex.submit(run_binary, task, run_root, cursor, test_threads)] = task
            cursor += 1
        while futures:
            future = next(as_completed(tuple(futures)))
            futures.pop(future)
            try:
                result = future.result()
            except Exception as error:
                fail(f"failed to run test binary: {error}")
            results.append(result)
            if result[2] != 0:
                failed.append(result)
            if not failed and cursor < len(ordered):
                task = ordered[cursor]
                futures[ex.submit(run_binary, task, run_root, cursor, test_threads)] = task
                cursor += 1
    elapsed = time.monotonic() - start
    if failed:
        print(f"run-test-binaries: failed {len(failed)} binaries after {elapsed:.2f}s", file=sys.stderr)
        emit_failures(failed)
        return 1
    counts = {name: 0 for name in ALLOWED_PACKAGES}
    for package, _e, _c, so, se, _elapsed in results:
        counts[package] += 1
    manifest = {"total": len(results), "elapsed_sec": round(elapsed, 2), "logs_root": str(run_root), "max_parallel_binaries": max_parallel, "test_threads_per_binary": test_threads, "package_counts": counts, "binaries": [{"package": p, "executable": str(e), "elapsed_sec": round(duration, 2), "stdout": str(so), "stderr": str(se)} for p, e, _c, so, se, duration in results]}
    print(json.dumps(manifest))
    return 0
def write_binary(path: Path, body: str) -> None:
    path.write_text("#!/usr/bin/env python3\n" + body)
    path.chmod(0o755)
def parse_max_active(path: Path) -> int:
    if not path.exists():
        return 0
    active = max_active = 0
    for raw in path.read_text().splitlines():
        parts = raw.split("\t")
        if len(parts) != 3:
            continue
        if parts[2] == "start":
            active += 1
            max_active = max(max_active, active)
        elif parts[2] == "end":
            active -= 1
    return max_active
def run_case(script: Path, artifact: Path, metadata: Path, logs: Path, env: dict[str, str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(script), "--artifact-json", str(artifact), "--metadata-json", str(metadata), "--logs-dir", str(logs), "--max-parallel-binaries", "2", "--test-threads", "1"],
        env={**os.environ, **env}, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True
    )
def self_test() -> int:
    root = Path(tempfile.mkdtemp(prefix="kuc-run-test-binaries-"))
    try:
        script = Path(__file__).resolve()
        md = root / "metadata.json"
        ids, dirs = {}, {}
        for i, name in enumerate(ALLOWED_PACKAGES):
            d = root / f"pkg{i:02d}"
            d.mkdir()
            (d / "Cargo.toml").write_text("[package]\nname='x'\nversion='0.1.0'\n")
            ids[name] = f"pkg-{i}"
            dirs[name] = d
        md.write_text(json.dumps({"packages": [{"id": ids[n], "name": n, "manifest_path": str(dirs[n] / "Cargo.toml")} for n in ALLOWED_PACKAGES]}))
        def exe(name: str, file: str, body: str) -> str:
            p = dirs[name] / file
            write_binary(p, body)
            return str(p)
        ok = {}
        slow_body = "import os,time\nn=__file__\nf=os.getenv('KUC_SELFTEST_EVENT_FILE')\nif f: open(f,'a',encoding='utf-8').write(f'{time.time()}\\t{n}\\tstart\\n')\ntime.sleep(DELAY)\nif f: open(f,'a',encoding='utf-8').write(f'{time.time()}\\t{n}\\tend\\n')\n"
        for i, name in enumerate(ALLOWED_PACKAGES):
            ok[name] = exe(name, f"ok-{i}.py", "print('ok')")
        fail = exe(ALLOWED_PACKAGES[1], "fail.py", "import sys;print('bad', file=sys.stderr); sys.exit(1)")
        slow: dict[str, str] = {}
        for i, name in enumerate(ALLOWED_PACKAGES):
            delay = "0.8" if i == 0 else "0.1"
            slow[name] = exe(name, f"slow-{i}.py", slow_body.replace("DELAY", delay))
        def write_artifact(items: list[tuple[str, str]]) -> Path:
            p = root / f"artifact-{time.time_ns()}.json"
            p.write_text("".join(json.dumps({"reason": "compiler-artifact", "package_id": ids[n], "executable": e, "profile": {"test": True}}) + "\n" for n, e in items))
            return p
        normal = run_case(script, write_artifact([(n, ok[n]) for n in ALLOWED_PACKAGES]), md, root / "normal", {"LLVM_PROFILE_FILE": "cov-%p-%m.profraw"})
        if normal.returncode != 0 or not normal.stdout:
            print("self-test failed: normal", file=sys.stderr)
            return 1
        m = json.loads(normal.stdout)
        if m.get("total") != len(ALLOWED_PACKAGES):
            print("self-test failed: manifest", file=sys.stderr)
            return 1
        bad_file = root / "bad-json.json"
        bad_file.write_text("{")
        bad = run_case(script, bad_file, md, root / "bad-json", {"LLVM_PROFILE_FILE": "cov-%p-%m.profraw"})
        if bad.returncode == 0 or "artifact JSON invalid" not in (bad.stdout or ""):
            print("self-test failed: malformed artifact", file=sys.stderr)
            return 1
        bind_target_exe = dirs[ALLOWED_PACKAGES[0]] / "bind-target.py"
        bind_target_exe.write_text("#!/usr/bin/env python3\nprint('bind target')\n")
        bind_target_child = dirs[ALLOWED_PACKAGES[0]] / "bind-target-child.py"
        bind_target_child.write_text("#!/usr/bin/env python3\nprint('bind target child')\n")
        bind_target_items = [
            (name, str(bind_target_exe) if index == 0 else ok[name])
            for index, name in enumerate(ALLOWED_PACKAGES)
        ]
        bind_target_artifact = write_artifact(bind_target_items)
        with bind_target_artifact.open("a") as artifact_file:
            artifact_file.write(
                json.dumps(
                    {
                        "reason": "compiler-artifact",
                        "package_id": ids[ALLOWED_PACKAGES[0]],
                        "executable": str(bind_target_child),
                        "profile": {"test": False},
                    }
                )
                + "\n"
            )
        bind_target = run_case(
            script,
            bind_target_artifact,
            md,
            root / "bind-target",
            {
                "LLVM_PROFILE_FILE": "cov-%p-%m.profraw",
                "KUC_COVERAGE_BIND_TARGET": "1",
            },
        )
        if (
            bind_target.returncode != 0
            or not os.access(bind_target_exe, os.X_OK)
            or not os.access(bind_target_child, os.X_OK)
        ):
            print("self-test failed: bind target executable repair", file=sys.stderr)
            return 1
        fail_case = [(n, fail if n == ALLOWED_PACKAGES[1] else ok[n]) for n in ALLOWED_PACKAGES]
        f = run_case(script, write_artifact(fail_case), md, root / "fail-log", {"LLVM_PROFILE_FILE": "cov-%p-%m.profraw"})
        if f.returncode != 1 or "bad" not in (f.stdout or ""):
            print("self-test failed: failure log", file=sys.stderr)
            return 1
        event = root / "parallel.log"
        parallel = [(name, slow[name]) for name in ALLOWED_PACKAGES]
        par = run_case(script, write_artifact(parallel), md, root / "parallel", {"LLVM_PROFILE_FILE": "cov-%p-%4m.profraw", "KUC_SELFTEST_EVENT_FILE": str(event)})
        if par.returncode != 0 or parse_max_active(event) != MAX_SUPPORTED_PARALLEL_BINARIES:
            print("self-test failed: parallel", file=sys.stderr)
            return 1
        event_times = {}
        for raw in event.read_text().splitlines():
            timestamp, executable, kind = raw.split("\t")
            event_times[(Path(executable).name, kind)] = float(timestamp)
        if event_times[("slow-2.py", "start")] >= event_times[("slow-0.py", "end")]:
            print("self-test failed: continuous refill", file=sys.stderr)
            return 1
        scheduled = schedule_batches([
            (ALLOWED_PACKAGES[0], Path("short"), root),
            (ALLOWED_PACKAGES[1], Path("egui_host_root_facade_contract-heavy"), root),
            (ALLOWED_PACKAGES[2], Path("katana_ui_core_storybook-heavy"), root),
            (ALLOWED_PACKAGES[0], Path("katana_ui_core-heavy"), root),
            (ALLOWED_PACKAGES[0], Path("native_window_contract-heavy"), root),
        ], MAX_SUPPORTED_PARALLEL_BINARIES)
        if [task[1].name for task in scheduled[0]] != ["katana_ui_core_storybook-heavy", "katana_ui_core-heavy"]:
            print("self-test failed: long-running schedule", file=sys.stderr)
            return 1
        print("self-test passed: 3/3")
        return 0
    finally:
        shutil.rmtree(root, ignore_errors=True)
def main() -> int:
    args = parse_args()
    if args.self_test:
        return self_test()
    if not args.artifact_json or not args.metadata_json or not args.logs_dir or not args.max_parallel_binaries or not args.test_threads:
        print("usage: run-test-binaries.py --artifact-json <path> --metadata-json <path> --logs-dir <path> --max-parallel-binaries <count> --test-threads <count>", file=sys.stderr)
        return 1
    if not 1 <= args.max_parallel_binaries <= MAX_SUPPORTED_PARALLEL_BINARIES:
        print(f"parallel binaries must be between 1 and {MAX_SUPPORTED_PARALLEL_BINARIES}", file=sys.stderr)
        return 1
    if not 1 <= args.test_threads <= 12:
        print("test threads must be between 1 and 12", file=sys.stderr)
        return 1
    try:
        ensure_llvm_profile_file(os.environ)
        allowed = parse_metadata(args.metadata_json)
        binaries = parse_artifacts(args.artifact_json, allowed)
        return run_suite(binaries, Path(args.logs_dir), args.max_parallel_binaries, args.test_threads)
    except RunnerError:
        return 1
if __name__ == "__main__":
    raise SystemExit(main())
