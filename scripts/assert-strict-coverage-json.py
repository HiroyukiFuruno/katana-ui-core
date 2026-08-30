#!/usr/bin/env python3
import json
from pathlib import Path
import sys
import tempfile


def coverage_profile_failures(payload: object) -> list[str]:
    if not isinstance(payload, dict):
        return ["coverage report root must be an object"]
    data = payload.get("data")
    if not isinstance(data, list) or len(data) != 1 or not isinstance(data[0], dict):
        return ["coverage report must contain exactly one data object"]
    totals = data[0].get("totals")
    if not isinstance(totals, dict):
        return ["coverage report totals are missing"]
    failures: list[str] = []
    for metric in ("functions", "lines"):
        summary = totals.get(metric)
        if not isinstance(summary, dict):
            failures.append(f"coverage totals.{metric} is missing")
            continue
        count = summary.get("count")
        covered = summary.get("covered")
        if (
            type(count) is not int
            or type(covered) is not int
            or count <= 0
            or covered < 0
            or covered > count
        ):
            failures.append(f"coverage totals.{metric} has invalid counts")
    return failures


def strict_coverage_failures(payload: object) -> list[str]:
    failures = coverage_profile_failures(payload)
    if failures:
        return failures
    totals = payload["data"][0]["totals"]
    for metric in ("functions", "lines"):
        summary = totals[metric]
        count = summary["count"]
        covered = summary["covered"]
        if covered != count:
            failures.append(
                f"coverage {metric} must be 100%: {covered}/{count}, uncovered={count - covered}"
            )
    return failures


def self_test() -> int:
    good = {
        "data": [
            {
                "totals": {
                    "functions": {"count": 3, "covered": 3},
                    "lines": {"count": 7, "covered": 7},
                }
            }
        ]
    }
    bad = {
        "data": [
            {
                "totals": {
                    "functions": {"count": 3, "covered": 2},
                    "lines": {"count": 7, "covered": 6},
                }
            }
        ]
    }
    malformed = {"data": []}
    impossible = {
        "data": [
            {
                "totals": {
                    "functions": {"count": 3, "covered": 4},
                    "lines": {"count": 7, "covered": -1},
                }
            }
        ]
    }
    if coverage_profile_failures(good):
        print("coverage profile self-test rejected valid totals", file=sys.stderr)
        return 1
    if coverage_profile_failures(bad):
        print("coverage profile self-test rejected a valid incomplete profile", file=sys.stderr)
        return 1
    if not coverage_profile_failures(malformed):
        print("coverage profile self-test accepted malformed data", file=sys.stderr)
        return 1
    if len(coverage_profile_failures(impossible)) != 2:
        print("coverage profile self-test accepted impossible counts", file=sys.stderr)
        return 1
    if strict_coverage_failures(good):
        print("strict coverage JSON self-test rejected valid totals", file=sys.stderr)
        return 1
    if len(strict_coverage_failures(bad)) != 2:
        print("strict coverage JSON self-test accepted uncovered totals", file=sys.stderr)
        return 1
    if not strict_coverage_failures(malformed):
        print("strict coverage JSON self-test accepted malformed data", file=sys.stderr)
        return 1
    with tempfile.TemporaryDirectory() as tmp:
        report = Path(tmp) / "coverage.json"
        report.write_text(json.dumps(good), encoding="utf-8")
        if json.loads(report.read_text(encoding="utf-8")) != good:
            print("strict coverage JSON self-test failed report round-trip", file=sys.stderr)
            return 1
    return 0


def main() -> int:
    if sys.argv[1:] == ["--self-test"]:
        return self_test()
    validate_profile = len(sys.argv) == 3 and sys.argv[1] == "--validate-profile"
    if len(sys.argv) != 2 and not validate_profile:
        print(
            "usage: assert-strict-coverage-json.py [--validate-profile] <coverage.json>",
            file=sys.stderr,
        )
        return 2
    report_path = Path(sys.argv[-1])
    try:
        payload = json.loads(report_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        print(f"strict coverage report could not be read: {error}", file=sys.stderr)
        return 1
    failures = (
        coverage_profile_failures(payload)
        if validate_profile
        else strict_coverage_failures(payload)
    )
    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1
    totals = payload["data"][0]["totals"]
    if validate_profile:
        print(
            "coverage profile is valid: "
            f"functions {totals['functions']['covered']}/{totals['functions']['count']}; "
            f"lines {totals['lines']['covered']}/{totals['lines']['count']}"
        )
        return 0
    print(
        "strict coverage passed: "
        f"functions {totals['functions']['covered']}/{totals['functions']['count']}; "
        f"lines {totals['lines']['covered']}/{totals['lines']['count']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
