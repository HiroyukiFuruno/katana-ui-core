#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import platform
import shlex
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

TITLEBAR_FALLBACK = 32
DEFAULT_FRAMES = "420"
DEFAULT_TRACE_DIR = Path("target/manual-ui-probe/native-77")
DEFAULT_HELPER_DIR = Path("target/manual-ui-probe/native-helper")
DEFAULT_MANIFEST = Path("docs/storybook-77ui-interaction-manifest.json")
DEFAULT_AUDIT = Path("target/storybook-live-interaction-audit.json")
DEFAULT_BINARY = Path("target/release/katana-ui-core-storybook")
PROCESS_NAME = "katana-ui-core-storybook"
MAIN_WINDOW_TITLE = "katana-ui-core Storybook"
MODAL_WINDOW_TITLE = "katana-ui-core Modal"

EXPECTED_TRACE_CONTRACTS: dict[str, list[dict[str, object]]] = {
    "text": [
        {
            "last_action": "select_text",
            "last_event": "text_selection_changed",
            "state_label": "selection=active",
            "text_selection_active": True,
        }
    ],
    "checkbox": [
        {
            "last_action": "checkbox_toggle",
            "last_event": "checked_changed",
            "state_label": "before=false after=true",
            "checkbox_0_checked": True,
            "checkbox_1_checked": True,
        }
    ],
    "progress-bar": [
        {
            "last_action": "progress_tick",
            "last_event": "progress_changed",
        }
    ],
    "tooltip": [
        {
            "last_action": "tooltip_hover",
            "last_event": "tooltip_opened",
            "state_label": "hover=true focus=true",
        },
        {
            "last_action": "tooltip_hover",
            "last_event": "tooltip_closed",
            "state_label": "hover=false focus=false",
        },
    ],
    "modal": [
        {
            "last_action": "modal_escape",
            "last_event": "modal_closed",
            "state_label": "open=false",
        }
    ],
    "tree-view": [
        {
            "last_action": "tree_scroll_retained",
            "last_event": "tree_scroll_offset_kept",
            "state_label": "scroll=retained",
        },
        {
            "last_action": "tree_click_toggle",
            "last_event": "tree_toggled",
        },
    ],
}

MATRIX_SCENARIOS: dict[str, list[str]] = {
    "text": [
        "text_drag_selection",
        "text_keyboard_copy",
        "text_keyboard_paste",
        "text_zero_distance_drag_no_selection",
    ],
    "checkbox": [
        "row_click",
        "checkbox_pointer_checks_both_rows",
        "checkbox_keyboard_toggle",
        "checkbox_keyboard_toggle_off",
        "checkbox_keyboard_focused_secondary_row",
        "checkbox_focus",
        "checkbox_hover_no_click_event",
        "checkbox_hover_secondary_row",
        "disabled_focus_keyboard_block",
        "checkbox_disabled_pointer_block",
    ],
    "progress-bar": [
        "progress_preview_click",
        "progress_timed_tick",
        "progress_timed_cycle",
        "progress_indeterminate_segment_motion",
    ],
    "tooltip": [
        "tooltip_idle_bubble_hidden_until_hover",
        "tooltip_anchor_hover_open",
        "tooltip_hover_idempotent",
        "tooltip_hover_leave_close",
        "tooltip_focus_open",
    ],
    "modal": [
        "modal_keyboard_escape",
        "modal_escape_removes_surface",
        "modal_escape_after_close_idempotent",
        "modal_focus_trap",
    ],
    "tree-view": [
        "preview_click",
        "tree_keyboard_select",
        "tree_focus_item",
        "tree_hover_item",
        "tree_view_context_menu",
        "tree_scroll_retained",
    ],
    "color-picker-rgba": [
        "color_picker_alpha_drag",
    ],
}

MATRIX_PRESETS: dict[tuple[str, str], int] = {
    ("checkbox", "disabled_focus_keyboard_block"): 2,
    ("checkbox", "checkbox_disabled_pointer_block"): 2,
    ("progress-bar", "progress_indeterminate_segment_motion"): 4,
    ("color-picker-rgba", "color_picker_alpha_drag"): 4,
}

FULL_SWEEP_MATRIX_CHECKS: dict[tuple[str, str], list[str]] = {
    ("text", "drag"): ["text_drag_selection"],
    ("text", "keyboard"): ["text_keyboard_copy", "text_keyboard_paste"],
    ("checkbox", "pointer"): ["row_click", "checkbox_pointer_checks_both_rows"],
    ("checkbox", "keyboard"): ["checkbox_keyboard_toggle", "checkbox_keyboard_toggle_off"],
    ("checkbox", "focus"): ["checkbox_focus"],
    ("checkbox", "hover"): ["checkbox_hover_no_click_event", "checkbox_hover_secondary_row"],
    ("progress-bar", "pointer"): ["progress_preview_click"],
    ("progress-bar", "timed_tick"): [
        "progress_timed_tick",
        "progress_timed_cycle",
        "progress_indeterminate_segment_motion",
    ],
    ("tooltip", "hover"): [
        "tooltip_anchor_hover_open",
        "tooltip_hover_idempotent",
        "tooltip_hover_leave_close",
    ],
    ("tooltip", "focus"): ["tooltip_focus_open"],
    ("modal", "pointer"): ["modal_escape_removes_surface"],
    ("modal", "keyboard"): ["modal_keyboard_escape", "modal_escape_after_close_idempotent"],
    ("modal", "focus"): ["modal_focus_trap"],
    ("tree-view", "pointer"): ["preview_click"],
    ("tree-view", "keyboard"): ["tree_keyboard_select"],
    ("tree-view", "focus"): ["tree_focus_item"],
    ("tree-view", "hover"): ["tree_hover_item"],
    ("tree-view", "scroll"): ["tree_scroll_retained"],
    ("tree-view", "context_menu"): ["tree_view_context_menu"],
    ("color-picker-rgba", "drag"): ["color_picker_alpha_drag"],
}

EXPECTED_MATRIX_CONTRACTS: dict[tuple[str, str], list[dict[str, object]]] = {
    ("text", "text_drag_selection"): EXPECTED_TRACE_CONTRACTS["text"],
    ("text", "text_keyboard_copy"): [
        {
            "last_action": "copy_selection",
            "last_event": "clipboard_copy",
            "state_label": "clipboard=selected_text",
            "clipboard_text_len__min": 1,
        }
    ],
    ("checkbox", "checkbox_pointer_checks_both_rows"): EXPECTED_TRACE_CONTRACTS["checkbox"],
    ("checkbox", "row_click"): [
        {
            "last_action": "checkbox_toggle",
            "last_event": "checked_changed",
            "state_label": "before=false after=true",
            "checkbox_0_checked": True,
            "checkbox_focused_index": 0,
        }
    ],
    ("checkbox", "checkbox_keyboard_toggle"): [
        {
            "last_action": "checkbox_keyboard_toggle",
            "last_event": "checked_changed",
            "state_label": "before=false after=true",
            "checkbox_0_checked": True,
            "checkbox_focused_index": 0,
        }
    ],
    ("checkbox", "checkbox_keyboard_toggle_off"): [
        {
            "last_action": "checkbox_keyboard_toggle",
            "last_event": "checked_changed",
            "state_label": "before=true after=false",
            "checkbox_0_checked": False,
            "checkbox_focused_index": 0,
        }
    ],
    ("checkbox", "checkbox_keyboard_focused_secondary_row"): [
        {
            "last_action": "checkbox_keyboard_toggle",
            "last_event": "checked_changed",
            "state_label": "before=true after=false",
            "checkbox_0_checked": False,
            "checkbox_1_checked": False,
            "checkbox_focused_index": 1,
        }
    ],
    ("checkbox", "checkbox_focus"): [
        {
            "last_action": "checkbox_focus",
            "last_event": "checkbox_focused",
            "state_label": "focused=true",
            "checkbox_focused_index": 0,
        }
    ],
    ("checkbox", "checkbox_hover_no_click_event"): [
        {
            "last_action": "none",
            "last_event": "none",
            "action_count": 0,
            "checkbox_0_checked": False,
            "checkbox_hovered_index": 0,
        }
    ],
    ("checkbox", "checkbox_hover_secondary_row"): [
        {
            "last_action": "none",
            "last_event": "none",
            "action_count": 0,
            "checkbox_0_checked": False,
            "checkbox_1_checked": False,
            "checkbox_hovered_index": 1,
        }
    ],
    ("checkbox", "disabled_focus_keyboard_block"): [
        {
            "preset_index": 2,
            "last_action": "checkbox_keyboard_blocked",
            "last_event": "checkbox_keyboard_ignored",
            "state_label": "disabled=true",
            "checkbox_0_checked": False,
            "checkbox_1_checked": False,
        }
    ],
    ("checkbox", "checkbox_disabled_pointer_block"): [
        {
            "preset_index": 2,
            "last_action": "none",
            "last_event": "none",
            "action_count": 0,
            "checkbox_0_checked": False,
            "checkbox_1_checked": False,
        }
    ],
    ("progress-bar", "progress_preview_click"): [
        {
            "last_action": "progress_change",
            "last_event": "progress_changed",
        }
    ],
    ("progress-bar", "progress_timed_tick"): [
        {
            "last_action": "progress_tick",
            "last_event": "progress_changed",
            "progress_percent__min": 82,
        }
    ],
    ("progress-bar", "progress_timed_cycle"): [
        {
            "last_action": "progress_tick",
            "last_event": "progress_changed",
            "progress_percent": 0,
        }
    ],
    ("progress-bar", "progress_indeterminate_segment_motion"): [
        {
            "preset_index": 4,
            "last_action": "progress_tick",
            "last_event": "progress_changed",
            "progress_segment_width": 52,
        }
    ],
    ("tooltip", "tooltip_idle_bubble_hidden_until_hover"): [
        {
            "last_action": "none",
            "last_event": "none",
            "action_count": 0,
        }
    ],
    ("tooltip", "tooltip_anchor_hover_open"): [
        {
            "last_action": "tooltip_hover",
            "last_event": "tooltip_opened",
            "state_label": "hover=true focus=true",
        }
    ],
    ("tooltip", "tooltip_hover_idempotent"): [
        {
            "last_action": "tooltip_hover",
            "last_event": "tooltip_opened",
            "state_label": "hover=true focus=true",
            "action_count": 1,
        }
    ],
    ("tooltip", "tooltip_hover_leave_close"): EXPECTED_TRACE_CONTRACTS["tooltip"],
    ("tooltip", "tooltip_focus_open"): [
        {
            "last_action": "tooltip_focus",
            "last_event": "tooltip_focused",
            "state_label": "hover=true focus=true",
        }
    ],
    ("modal", "modal_keyboard_escape"): EXPECTED_TRACE_CONTRACTS["modal"],
    ("modal", "modal_escape_removes_surface"): EXPECTED_TRACE_CONTRACTS["modal"],
    ("modal", "modal_escape_after_close_idempotent"): [
        {
            "last_action": "modal_escape",
            "last_event": "modal_closed",
            "state_label": "open=false",
            "action_count": 1,
        }
    ],
    ("modal", "modal_focus_trap"): [
        {
            "last_action": "modal_focus_trap",
            "last_event": "modal_focused",
            "state_label": "focus=trapped",
        }
    ],
    ("tree-view", "preview_click"): [
        {
            "last_action": "tree_click_toggle",
            "last_event": "tree_toggled",
        }
    ],
    ("tree-view", "tree_keyboard_select"): [
        {
            "last_action": "tree_keyboard_select",
            "last_event": "tree_selected",
        }
    ],
    ("tree-view", "tree_focus_item"): [
        {
            "last_action": "tree_focus_item",
            "last_event": "tree_item_focused",
        }
    ],
    ("tree-view", "tree_hover_item"): [
        {
            "last_action": "tree_hover_item",
            "last_event": "hover_start",
        }
    ],
    ("tree-view", "tree_view_context_menu"): [
        {
            "last_action": "tree_context_menu",
            "last_event": "tree_context_opened",
            "state_label": "context_menu=open",
        }
    ],
    ("tree-view", "tree_scroll_retained"): [
        {
            "last_action": "tree_scroll_retained",
            "last_event": "tree_scroll_offset_kept",
            "state_label": "scroll=retained",
        }
    ],
    ("color-picker-rgba", "color_picker_alpha_drag"): [
        {
            "preset_index": 4,
            "last_action": "color_alpha_drag",
            "last_event": "alpha_changed",
            "state_label": "color_picker.alpha=188",
        }
    ],
}

NO_STATE_MATRIX_SCENARIOS = {
    ("text", "text_keyboard_paste"),
    ("text", "text_zero_distance_drag_no_selection"),
}

C_SOURCE = r"""
#include <ApplicationServices/ApplicationServices.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void move_to(double x, double y, bool down) {
    CGPoint p = CGPointMake(x, y);
    CGPostMouseEvent(p, true, 1, down);
}

static void post_mouse(CGMouseButton button, CGEventType type, double x, double y) {
    CGPoint p = CGPointMake(x, y);
    CGEventRef event = CGEventCreateMouseEvent(NULL, type, p, button);
    CGEventPost(kCGHIDEventTap, event);
    CFRelease(event);
}

static int key_code(const char *name) {
    if (strcmp(name, "space") == 0) return 49;
    if (strcmp(name, "tab") == 0) return 48;
    if (strcmp(name, "enter") == 0) return 36;
    if (strcmp(name, "escape") == 0) return 53;
    if (strcmp(name, "backspace") == 0) return 51;
    if (strcmp(name, "a") == 0) return 0;
    if (strcmp(name, "b") == 0) return 11;
    if (strcmp(name, "c") == 0) return 8;
    if (strcmp(name, "d") == 0) return 2;
    if (strcmp(name, "e") == 0) return 14;
    if (strcmp(name, "v") == 0) return 9;
    if (strcmp(name, "w") == 0) return 13;
    if (strcmp(name, "1") == 0) return 18;
    if (strcmp(name, "2") == 0) return 19;
    return -1;
}

static void post_key_code(CGKeyCode key, bool down) {
    CGEventRef event = CGEventCreateKeyboardEvent(NULL, key, down);
    CGEventPost(kCGHIDEventTap, event);
    CFRelease(event);
}

int main(int argc, char **argv) {
    if (argc < 2) return 2;
    const char *mode = argv[1];
    if (strcmp(mode, "key") == 0) {
        if (argc < 3) return 2;
        int code = key_code(argv[2]);
        if (code < 0) return 2;
        post_key_code((CGKeyCode)code, true);
        usleep(80000);
        post_key_code((CGKeyCode)code, false);
        usleep(180000);
        return 0;
    }

    if (strcmp(mode, "cmdkey") == 0) {
        if (argc < 3) return 2;
        int code = key_code(argv[2]);
        if (code < 0) return 2;
        post_key_code((CGKeyCode)55, true);
        usleep(30000);
        post_key_code((CGKeyCode)code, true);
        usleep(80000);
        post_key_code((CGKeyCode)code, false);
        usleep(30000);
        post_key_code((CGKeyCode)55, false);
        usleep(180000);
        return 0;
    }

    if (argc < 4) return 2;
    double x = atof(argv[2]);
    double y = atof(argv[3]);

    if (strcmp(mode, "move") == 0) {
        move_to(x, y, false);
        usleep(120000);
        return 0;
    }

    if (strcmp(mode, "click") == 0) {
        int hold = argc > 4 ? atoi(argv[4]) : 180000;
        move_to(x, y, false);
        usleep(60000);
        move_to(x, y, true);
        usleep(hold);
        move_to(x, y, false);
        usleep(120000);
        return 0;
    }

    if (strcmp(mode, "rightclick") == 0) {
        move_to(x, y, false);
        usleep(60000);
        post_mouse(kCGMouseButtonRight, kCGEventRightMouseDown, x, y);
        usleep(100000);
        post_mouse(kCGMouseButtonRight, kCGEventRightMouseUp, x, y);
        usleep(120000);
        return 0;
    }

    if (strcmp(mode, "scroll") == 0) {
        int dy = argc > 4 ? atoi(argv[4]) : -8;
        move_to(x, y, false);
        usleep(60000);
        CGEventRef scroll = CGEventCreateScrollWheelEvent(NULL, kCGScrollEventUnitLine, 1, dy);
        CGEventPost(kCGHIDEventTap, scroll);
        CFRelease(scroll);
        usleep(120000);
        return 0;
    }

    if (strcmp(mode, "hscroll") == 0) {
        int dx = argc > 4 ? atoi(argv[4]) : -8;
        move_to(x, y, false);
        usleep(60000);
        CGEventRef scroll = CGEventCreateScrollWheelEvent(NULL, kCGScrollEventUnitLine, 2, 0, dx);
        CGEventPost(kCGHIDEventTap, scroll);
        CFRelease(scroll);
        usleep(120000);
        return 0;
    }

    if (strcmp(mode, "drag") == 0) {
        if (argc < 6) return 2;
        double x2 = atof(argv[4]);
        double y2 = atof(argv[5]);
        int steps = argc > 6 ? atoi(argv[6]) : 12;
        if (steps < 1) steps = 1;
        move_to(x, y, false);
        usleep(60000);
        move_to(x, y, true);
        usleep(60000);
        for (int i = 1; i <= steps; i++) {
            double t = (double)i / (double)steps;
            move_to(x + (x2 - x) * t, y + (y2 - y) * t, true);
            usleep(25000);
        }
        usleep(80000);
        move_to(x2, y2, false);
        usleep(120000);
        return 0;
    }

    return 2;
}
"""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Drive the visible KUC Storybook window with native macOS input.",
    )
    parser.add_argument("pages", nargs="*", help="Optional Storybook page ids to probe.")
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--audit", type=Path, default=DEFAULT_AUDIT)
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_TRACE_DIR)
    parser.add_argument("--helper-dir", type=Path, default=DEFAULT_HELPER_DIR)
    parser.add_argument("--binary", type=Path, default=DEFAULT_BINARY)
    parser.add_argument("--frames", default=DEFAULT_FRAMES)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument(
        "--matrix",
        action="store_true",
        help="Run per-acceptance-check native scenarios for unresolved Storybook UI.",
    )
    parser.add_argument(
        "--full-sweep",
        action="store_true",
        help="Run every manifest required operation through a native Storybook window.",
    )
    return parser.parse_args()


def resolve(root: Path, path: Path) -> Path:
    return path if path.is_absolute() else root / path


def run(root: Path, cmd: list[str], **kwargs: Any) -> subprocess.CompletedProcess[str]:
    return subprocess.run(cmd, cwd=root, text=True, **kwargs)


def require_macos() -> None:
    if platform.system() != "Darwin":
        raise SystemExit("storybook native window probe requires macOS native input APIs")


def ensure_helper(root: Path, helper_dir: Path) -> Path:
    helper_dir.mkdir(parents=True, exist_ok=True)
    helper = helper_dir / "kuc_native_input"
    source = helper.with_suffix(".c")
    current_source = source.read_text() if source.exists() else None
    if current_source != C_SOURCE:
        source.write_text(C_SOURCE)
    if helper.exists() and current_source == C_SOURCE:
        return helper
    result = run(
        root,
        [
            "clang",
            "-Wno-deprecated-declarations",
            "-framework",
            "ApplicationServices",
            str(source),
            "-o",
            str(helper),
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        print(result.stdout)
        raise SystemExit(result.returncode)
    return helper


def cargo_command() -> list[str]:
    cargo = os.environ.get("CARGO", "cargo")
    return shlex.split(cargo)


def ensure_binary(root: Path, binary: Path, skip_build: bool) -> None:
    if skip_build:
        return
    result = run(
        root,
        [
            *cargo_command(),
            "build",
            "--release",
            "-p",
            "katana-ui-core-storybook",
            "--locked",
        ],
    )
    if result.returncode != 0:
        raise SystemExit(result.returncode)


def manifest_pages(manifest_path: Path) -> list[str]:
    data = json.loads(manifest_path.read_text())
    return [entry["page"] for entry in manifest_entries(data)]


def manifest_entries(data: dict[str, Any]) -> list[dict[str, Any]]:
    entries = data.get("ui", [])
    if not isinstance(entries, list):
        raise SystemExit("manifest `ui` must be an array")
    for entry in entries:
        if not isinstance(entry, dict) or not isinstance(entry.get("page"), str):
            raise SystemExit("manifest ui entries must contain page strings")
    return entries


def required_operations_for_manifest_entry(
    manifest: dict[str, Any],
    entry: dict[str, Any],
) -> list[str]:
    explicit = entry.get("required_operations")
    if isinstance(explicit, list) and explicit:
        return [str(item) for item in explicit]
    engine = entry.get("engine")
    defaults = manifest.get("defaults_by_engine", {}).get(engine, {})
    operations = defaults.get("required_operations", [])
    return [str(item) for item in operations]


def window_bounds(root: Path) -> tuple[int, int, int, int] | None:
    script = (
        f'tell application "System Events" to tell process "{PROCESS_NAME}" '
        "to get {position, size} of window 1"
    )
    result = run(root, ["osascript", "-e", script], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return parse_bounds(result.stdout) if result.returncode == 0 else None


def parse_bounds(output: str) -> tuple[int, int, int, int] | None:
    parts = [part.strip() for part in output.replace("\n", "").split(",")]
    try:
        nums = [int(part) for part in parts if part]
    except ValueError:
        return None
    if len(nums) != 4:
        return None
    return (nums[0], nums[1], nums[2], nums[3])


def window_names(root: Path) -> list[str]:
    script = (
        f'tell application "System Events" to tell process "{PROCESS_NAME}" '
        "to get name of windows"
    )
    result = run(root, ["osascript", "-e", script], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode != 0:
        return []
    return [part.strip() for part in result.stdout.replace("\n", "").split(",") if part.strip()]


def named_window_bounds(root: Path, title: str) -> tuple[int, int, int, int] | None:
    script = f'''
tell application "System Events"
    tell process "{PROCESS_NAME}"
        repeat with currentWindow in windows
            if name of currentWindow is "{title}" then
                set windowPosition to position of currentWindow
                set windowSize to size of currentWindow
                return (item 1 of windowPosition as text) & "," & (item 2 of windowPosition as text) & "," & (item 1 of windowSize as text) & "," & (item 2 of windowSize as text)
            end if
        end repeat
    end tell
end tell
'''
    result = run(root, ["osascript", "-e", script], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return parse_bounds(result.stdout) if result.returncode == 0 else None


def wait_for_modal_window(root: Path, timeout: float = 12.0) -> dict[str, Any] | None:
    deadline = time.time() + timeout
    while time.time() < deadline:
        names = window_names(root)
        modal_bounds = named_window_bounds(root, MODAL_WINDOW_TITLE)
        if MAIN_WINDOW_TITLE in names and MODAL_WINDOW_TITLE in names and modal_bounds:
            return {
                "names": names,
                "front_window_name": names[0] if names else None,
                "modal_bounds": modal_bounds,
            }
        time.sleep(0.1)
    names = window_names(root)
    modal_bounds = named_window_bounds(root, MODAL_WINDOW_TITLE)
    if not names and not modal_bounds:
        return None
    return {
        "names": names,
        "front_window_name": names[0] if names else None,
        "modal_bounds": modal_bounds,
    }


def read_rows(trace_path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not trace_path.exists():
        return rows
    for line in trace_path.read_text().splitlines():
        try:
            value = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(value, dict):
            rows.append(value)
    return rows


def wait_for_window(
    root: Path,
    trace_path: Path,
    timeout: float = 12.0,
) -> tuple[tuple[int, int, int, int] | None, dict[str, Any] | None]:
    deadline = time.time() + timeout
    bounds = None
    rows: list[dict[str, Any]] = []
    while time.time() < deadline:
        bounds = window_bounds(root)
        rows = read_rows(trace_path)
        if bounds and rows:
            return bounds, rows[-1]
        time.sleep(0.1)
    return bounds, rows[-1] if rows else None


def activate_storybook_window(root: Path) -> None:
    script = f'''
tell application "System Events"
    if exists process "{PROCESS_NAME}" then
        set frontmost of process "{PROCESS_NAME}" to true
    end if
end tell
'''
    run(root, ["osascript", "-e", script], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    time.sleep(0.2)


def screen_point(
    bounds: tuple[int, int, int, int],
    row: dict[str, Any],
    canvas_x: int,
    canvas_y: int,
) -> tuple[int, int]:
    left, top, _width, height = bounds
    canvas_height = int(row.get("height") or 920)
    chrome_height = max(0, height - canvas_height) or TITLEBAR_FALLBACK
    return (left + canvas_x, top + chrome_height + canvas_y)


def input_cmd(root: Path, helper: Path, mode: str, *args: object) -> None:
    result = run(
        root,
        [str(helper), mode, *[str(arg) for arg in args]],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stdout)


def set_clipboard(root: Path, text: str) -> None:
    result = run(
        root,
        ["pbcopy"],
        input=text,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stdout)


def click_modal_center(root: Path, helper: Path, modal_bounds: tuple[int, int, int, int]) -> None:
    left, top, width, height = modal_bounds
    input_cmd(root, helper, "click", left + width // 2, top + height // 2, 80000)


def click_modal_and_escape(root: Path, helper: Path, modal_bounds: tuple[int, int, int, int]) -> None:
    click_modal_center(root, helper, modal_bounds)
    input_cmd(root, helper, "key", "escape")


def parse_modal_summary(output: str) -> dict[str, Any]:
    prefix = "katana-ui-core-storybook-modal-window:"
    flags: dict[str, Any] = {}
    for line in output.splitlines():
        if not line.startswith(prefix):
            continue
        for token in line.removeprefix(prefix).strip().split():
            if "=" not in token:
                continue
            key, value = token.split("=", 1)
            if value == "true":
                flags[key] = True
            elif value == "false":
                flags[key] = False
            else:
                try:
                    flags[key] = int(value)
                except ValueError:
                    flags[key] = value
    return flags


def modal_window_contract_failures(
    check: str,
    window_info: dict[str, Any],
    summary: dict[str, Any],
    exited_after_escape: bool,
    front_window_after_tab: str | None = None,
) -> list[str]:
    failures: list[str] = []
    names = window_info.get("names") or []
    if MAIN_WINDOW_TITLE not in names or MODAL_WINDOW_TITLE not in names:
        failures.append("expected separate Storybook and Modal windows")
    if window_info.get("front_window_name") != MODAL_WINDOW_TITLE:
        failures.append("expected modal window to be the front window")
    for key in [
        "modal_window_opened",
        "same_display",
        "frontmost",
        "state_reflected",
        "overlay_rendered",
    ]:
        if summary.get(key) is not True:
            failures.append(f"expected modal summary {key}=true")
    if check in {
        "modal_keyboard_escape",
        "modal_escape_removes_surface",
        "modal_escape_after_close_idempotent",
    } and not exited_after_escape:
        failures.append("expected Escape in modal window to close the modal run")
    if check == "modal_focus_trap" and front_window_after_tab != MODAL_WINDOW_TITLE:
        failures.append("expected Tab to keep modal window frontmost")
    return failures


def operate(
    root: Path,
    helper: Path,
    page: str,
    bounds: tuple[int, int, int, int],
    row: dict[str, Any],
) -> None:
    def pt(x: int, y: int) -> tuple[int, int]:
        return screen_point(bounds, row, x, y)

    if page == "tooltip":
        input_cmd(root, helper, "move", *pt(482, 298))
        time.sleep(0.35)
        return
    if page == "progress-bar":
        time.sleep(1.2)
        return
    if page == "menu":
        input_cmd(root, helper, "click", *pt(381, 261), 160000)
        return
    if page == "tree-view":
        input_cmd(root, helper, "scroll", *pt(359, 253), -8)
        input_cmd(root, helper, "scroll", *pt(359, 253), -8)
        time.sleep(0.2)
        input_cmd(root, helper, "click", *pt(402, 260), 180000)
        return
    if page == "checkbox":
        input_cmd(root, helper, "click", *pt(484, 276), 160000)
        input_cmd(root, helper, "click", *pt(484, 320), 160000)
        return
    if page == "radio":
        input_cmd(root, helper, "click", *pt(484, 276), 160000)
        return
    if page == "toggle":
        input_cmd(root, helper, "click", *pt(632, 275), 160000)
        input_cmd(root, helper, "click", *pt(632, 275), 160000)
        return
    if page == "text":
        input_cmd(root, helper, "click", *pt(345, 223), 160000)
        return
    if page in {"button", "text-button"}:
        input_cmd(root, helper, "click", *pt(361, 273), 160000)
        return
    if page == "svg-button":
        input_cmd(root, helper, "click", *pt(367, 273), 160000)
        return
    if page == "icon-text-button":
        input_cmd(root, helper, "click", *pt(365, 273), 160000)
        return
    input_cmd(root, helper, "click", *pt(345, 223), 160000)


def operate_matrix_check(
    root: Path,
    helper: Path,
    page: str,
    check: str,
    bounds: tuple[int, int, int, int],
    row: dict[str, Any],
) -> None:
    def pt(x: int, y: int) -> tuple[int, int]:
        return screen_point(bounds, row, x, y)

    if page == "text" and check == "text_drag_selection":
        input_cmd(root, helper, "drag", *pt(438, 254), *pt(522, 254), 12)
        return
    if page == "text" and check == "text_keyboard_copy":
        input_cmd(root, helper, "drag", *pt(438, 254), *pt(522, 254), 24)
        time.sleep(0.4)
        activate_storybook_window(root)
        time.sleep(0.15)
        input_cmd(root, helper, "cmdkey", "c")
        time.sleep(0.25)
        return
    if page == "text" and check == "text_keyboard_paste":
        set_clipboard(root, "native paste")
        activate_storybook_window(root)
        time.sleep(0.15)
        input_cmd(root, helper, "cmdkey", "v")
        return
    if page == "text" and check == "text_zero_distance_drag_no_selection":
        input_cmd(root, helper, "drag", *pt(438, 254), *pt(438, 254), 1)
        return
    if page == "checkbox" and check == "checkbox_pointer_checks_both_rows":
        input_cmd(root, helper, "click", *pt(484, 276), 160000)
        input_cmd(root, helper, "click", *pt(484, 320), 160000)
        return
    if page == "checkbox" and check == "row_click":
        input_cmd(root, helper, "click", *pt(484, 276), 160000)
        return
    if page == "checkbox" and check == "checkbox_keyboard_toggle":
        activate_storybook_window(root)
        time.sleep(0.15)
        input_cmd(root, helper, "key", "tab")
        time.sleep(0.35)
        input_cmd(root, helper, "key", "space")
        time.sleep(0.25)
        return
    if page == "checkbox" and check == "checkbox_keyboard_toggle_off":
        activate_storybook_window(root)
        time.sleep(0.15)
        input_cmd(root, helper, "key", "tab")
        time.sleep(0.35)
        input_cmd(root, helper, "key", "space")
        time.sleep(0.35)
        input_cmd(root, helper, "key", "space")
        time.sleep(0.25)
        return
    if page == "checkbox" and check == "checkbox_keyboard_focused_secondary_row":
        input_cmd(root, helper, "click", *pt(484, 320), 160000)
        input_cmd(root, helper, "key", "space")
        return
    if page == "checkbox" and check == "checkbox_focus":
        activate_storybook_window(root)
        time.sleep(0.15)
        input_cmd(root, helper, "key", "tab")
        return
    if page == "checkbox" and check == "checkbox_hover_no_click_event":
        input_cmd(root, helper, "move", *pt(484, 276))
        return
    if page == "checkbox" and check == "checkbox_hover_secondary_row":
        input_cmd(root, helper, "move", *pt(484, 320))
        return
    if page == "checkbox" and check == "disabled_focus_keyboard_block":
        activate_storybook_window(root)
        time.sleep(0.15)
        input_cmd(root, helper, "key", "tab")
        time.sleep(0.25)
        input_cmd(root, helper, "key", "space")
        time.sleep(0.15)
        input_cmd(root, helper, "key", "space")
        return
    if page == "checkbox" and check == "checkbox_disabled_pointer_block":
        input_cmd(root, helper, "click", *pt(484, 276), 160000)
        return
    if page == "progress-bar" and check == "progress_preview_click":
        input_cmd(root, helper, "click", *pt(484, 276), 160000)
        return
    if page == "progress-bar" and check == "progress_timed_tick":
        time.sleep(0.35)
        return
    if page == "progress-bar" and check == "progress_timed_cycle":
        time.sleep(0.85)
        return
    if page == "progress-bar" and check == "progress_indeterminate_segment_motion":
        time.sleep(0.6)
        return
    if page == "tooltip" and check == "tooltip_idle_bubble_hidden_until_hover":
        time.sleep(0.35)
        return
    if page == "tooltip" and check == "tooltip_anchor_hover_open":
        input_cmd(root, helper, "move", *pt(482, 298))
        time.sleep(0.35)
        return
    if page == "tooltip" and check == "tooltip_hover_idempotent":
        input_cmd(root, helper, "move", *pt(482, 298))
        time.sleep(0.2)
        input_cmd(root, helper, "move", *pt(482, 298))
        time.sleep(0.2)
        return
    if page == "tooltip" and check == "tooltip_hover_leave_close":
        input_cmd(root, helper, "move", *pt(482, 298))
        time.sleep(0.35)
        input_cmd(root, helper, "move", *pt(348, 226))
        return
    if page == "tooltip" and check == "tooltip_focus_open":
        input_cmd(root, helper, "key", "tab")
        return
    if page == "modal" and check == "modal_keyboard_escape":
        input_cmd(root, helper, "key", "tab")
        input_cmd(root, helper, "key", "escape")
        return
    if page == "modal" and check == "modal_escape_removes_surface":
        input_cmd(root, helper, "key", "tab")
        input_cmd(root, helper, "key", "escape")
        return
    if page == "modal" and check == "modal_escape_after_close_idempotent":
        input_cmd(root, helper, "key", "tab")
        input_cmd(root, helper, "key", "escape")
        input_cmd(root, helper, "key", "escape")
        return
    if page == "modal" and check == "modal_focus_trap":
        input_cmd(root, helper, "key", "tab")
        return
    if page == "tree-view" and check == "preview_click":
        input_cmd(root, helper, "click", *pt(402, 260), 180000)
        return
    if page == "tree-view" and check == "tree_keyboard_select":
        input_cmd(root, helper, "key", "tab")
        time.sleep(0.35)
        input_cmd(root, helper, "key", "enter")
        return
    if page == "tree-view" and check == "tree_focus_item":
        input_cmd(root, helper, "key", "tab")
        return
    if page == "tree-view" and check == "tree_hover_item":
        input_cmd(root, helper, "move", *pt(402, 260))
        return
    if page == "tree-view" and check == "tree_view_context_menu":
        input_cmd(root, helper, "rightclick", *pt(348, 226))
        return
    if page == "tree-view" and check == "tree_scroll_retained":
        input_cmd(root, helper, "scroll", *pt(359, 253), -8)
        input_cmd(root, helper, "scroll", *pt(359, 253), -8)
        time.sleep(0.2)
        input_cmd(root, helper, "click", *pt(402, 260), 180000)
        return
    if page == "color-picker-rgba" and check == "color_picker_alpha_drag":
        input_cmd(root, helper, "drag", *pt(438, 254), *pt(522, 254), 12)
        return
    raise RuntimeError(f"unsupported matrix scenario {page}:{check}")


def operate_required_operation(
    root: Path,
    helper: Path,
    page: str,
    operation: str,
    bounds: tuple[int, int, int, int],
    row: dict[str, Any],
) -> None:
    def pt(x: int, y: int) -> tuple[int, int]:
        return screen_point(bounds, row, x, y)

    if operation == "pointer":
        operate(root, helper, page, bounds, row)
        return
    if operation == "hover":
        if page == "tooltip":
            input_cmd(root, helper, "move", *pt(482, 298))
        elif page in {"checkbox", "radio"}:
            input_cmd(root, helper, "move", *pt(484, 276))
        elif page == "tree-view":
            input_cmd(root, helper, "move", *pt(402, 260))
        elif page == "breadcrumb":
            input_cmd(root, helper, "move", *pt(594, 278))
        elif page == "status-bar":
            input_cmd(root, helper, "move", *pt(390, 292))
        else:
            input_cmd(root, helper, "move", *pt(345, 223))
        time.sleep(0.25)
        return
    if operation == "focus":
        if page in {"text-input", "text-area"}:
            input_cmd(root, helper, "click", *pt(410, 276), 120000)
            return
        input_cmd(root, helper, "key", "tab")
        return
    if operation == "keyboard":
        if page in {"closeable-tab-strip", "tabs"}:
            input_cmd(root, helper, "cmdkey", "2")
            return
        if page in {"text-input", "text-area"}:
            input_cmd(root, helper, "click", *pt(410, 276), 120000)
            input_cmd(root, helper, "key", "a")
            input_cmd(root, helper, "key", "b")
            return
        if page == "search-box":
            input_cmd(root, helper, "key", "tab")
            input_cmd(root, helper, "key", "a")
            input_cmd(root, helper, "key", "enter")
            return
        input_cmd(root, helper, "key", "tab")
        time.sleep(0.18)
        input_cmd(root, helper, "key", "space")
        time.sleep(0.18)
        input_cmd(root, helper, "key", "enter")
        return
    if operation == "scroll":
        if page in {"closeable-tab-strip", "tabs"}:
            input_cmd(root, helper, "hscroll", *pt(430, 280), -8)
            return
        if page == "drag-and-drop":
            input_cmd(root, helper, "click", *pt(388, 327), 120000)
            return
        if page == "scroll-area":
            input_cmd(root, helper, "scroll", *pt(380, 280), -8)
            input_cmd(root, helper, "scroll", *pt(380, 280), -8)
            return
        input_cmd(root, helper, "scroll", *pt(359, 253), -8)
        input_cmd(root, helper, "scroll", *pt(359, 253), -8)
        return
    if operation == "drag":
        if page in {"closeable-tab-strip", "tabs"}:
            input_cmd(root, helper, "drag", *pt(390, 280), *pt(500, 280), 14)
            return
        if page == "drag-and-drop":
            input_cmd(root, helper, "drag", *pt(461, 279), *pt(739, 291), 14)
            return
        if page == "scroll-area":
            input_cmd(root, helper, "drag", *pt(624, 271), *pt(624, 315), 12)
            return
        if page == "split-pane":
            input_cmd(root, helper, "drag", *pt(455, 259), *pt(500, 259), 12)
            return
        input_cmd(root, helper, "drag", *pt(360, 258), *pt(540, 258), 14)
        return
    if operation == "resize":
        if page == "drag-and-drop":
            input_cmd(root, helper, "click", *pt(806, 301), 120000)
            return
        if page == "panel":
            input_cmd(root, helper, "drag", *pt(1014, 560), *pt(1038, 584), 12)
            return
        if page == "scroll-area":
            input_cmd(root, helper, "drag", *pt(626, 318), *pt(650, 342), 12)
            return
        if page == "split-pane":
            input_cmd(root, helper, "drag", *pt(603, 303), *pt(630, 330), 12)
            return
        input_cmd(root, helper, "drag", *pt(681, 348), *pt(720, 382), 12)
        return
    if operation == "context_menu":
        if page in {"closeable-tab-strip", "tabs"}:
            input_cmd(root, helper, "rightclick", *pt(390, 280))
            return
        if page == "menu":
            input_cmd(root, helper, "click", *pt(382, 260), 120000)
            input_cmd(root, helper, "rightclick", *pt(692, 226))
            return
        if page == "menu-button":
            input_cmd(root, helper, "rightclick", *pt(392, 268))
            return
        input_cmd(root, helper, "rightclick", *pt(348, 226))
        return
    if operation == "timed_tick":
        time.sleep(0.8)
        return
    raise RuntimeError(f"unsupported required operation {page}:{operation}")


def state_rows(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [
        row
        for row in rows
        if row.get("action_count", 0) > 0
        or row.get("settings_revision", 0) > 0
        or row.get("last_action") != "none"
        or row.get("last_event") != "none"
    ]


def trace_matches(row: dict[str, Any], expected: dict[str, object]) -> bool:
    for key, value in expected.items():
        if key.endswith("__min"):
            actual = row.get(key.removesuffix("__min"))
            if not isinstance(actual, (int, float)) or not isinstance(value, (int, float)):
                return False
            if actual < value:
                return False
            continue
        actual = row.get(key)
        if isinstance(value, bool):
            if actual is not value:
                return False
            continue
        if isinstance(value, int):
            if actual != value:
                return False
            continue
        if str(actual) != str(value):
            return False
    return True


def trace_contract_failures(page: str, rows: list[dict[str, Any]]) -> list[str]:
    contracts = EXPECTED_TRACE_CONTRACTS.get(page, [])
    failures: list[str] = []
    for expected in contracts:
        if not any(trace_matches(row, expected) for row in rows):
            expected_label = ", ".join(f"{key}={value}" for key, value in expected.items())
            failures.append(f"missing trace contract: {expected_label}")
    return failures


def matrix_contract_failures(page: str, check: str, rows: list[dict[str, Any]]) -> list[str]:
    if (page, check) in NO_STATE_MATRIX_SCENARIOS:
        changed = state_rows(rows)
        if changed:
            last = changed[-1]
            return [
                "expected no state change, got "
                f"{last.get('last_action')}/{last.get('last_event')}/{last.get('state_label')}"
            ]
        return []
    if (page, check) == ("progress-bar", "progress_indeterminate_segment_motion"):
        segment_positions = [
            row.get("progress_segment_x")
            for row in rows
            if isinstance(row.get("progress_segment_x"), int)
        ]
        if len(set(segment_positions)) < 2:
            return ["expected indeterminate progress segment x to move across frames"]
    contracts = EXPECTED_MATRIX_CONTRACTS.get((page, check), [])
    if not contracts:
        return [f"missing matrix expectation for {page}:{check}"]
    failures: list[str] = []
    for expected in contracts:
        if not any(trace_matches(row, expected) for row in rows):
            expected_label = ", ".join(f"{key}={value}" for key, value in expected.items())
            failures.append(f"missing trace contract: {expected_label}")
    return failures


def result_summary(page: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    changed_rows = state_rows(rows)
    last = rows[-1] if rows else {}
    contract_failures = trace_contract_failures(page, rows)
    if page in EXPECTED_TRACE_CONTRACTS:
        ok = not contract_failures
        reason = "trace_contract_matched" if ok else "; ".join(contract_failures)
    else:
        ok = bool(changed_rows)
        reason = "state_changed" if changed_rows else "no_state_change"
    return {
        "page": page,
        "ok": ok,
        "reason": reason,
        "last": {
            key: last.get(key)
            for key in [
                "frame",
                "action_count",
                "settings_revision",
                "last_action",
                "last_event",
                "state_label",
                "last_setting",
                "last_setting_value",
                "text_selection_active",
                "clipboard_text_len",
                "progress_percent",
                "progress_segment_x",
                "progress_segment_width",
                "checkbox_0_checked",
                "checkbox_1_checked",
                "checkbox_focused_index",
                "checkbox_hovered_index",
            ]
        },
        "state_tail": [
            {
                key: row.get(key)
                for key in [
                    "frame",
                    "action_count",
                    "settings_revision",
                    "last_action",
                    "last_event",
                    "state_label",
                    "text_selection_active",
                    "clipboard_text_len",
                    "progress_percent",
                    "progress_segment_x",
                    "progress_segment_width",
                    "checkbox_0_checked",
                    "checkbox_1_checked",
                    "checkbox_focused_index",
                    "checkbox_hovered_index",
                ]
            }
            for row in changed_rows[-3:]
        ],
    }


def matrix_result_summary(page: str, check: str, rows: list[dict[str, Any]]) -> dict[str, Any]:
    last = rows[-1] if rows else {}
    failures = matrix_contract_failures(page, check, rows)
    ok = not failures
    return {
        "page": page,
        "check": check,
        "ok": ok,
        "reason": "trace_contract_matched" if ok else "; ".join(failures),
        "last": {
            key: last.get(key)
            for key in [
                "frame",
                "action_count",
                "settings_revision",
                "last_action",
                "last_event",
                "state_label",
                "text_selection_active",
                "clipboard_text_len",
                "progress_percent",
                "progress_segment_x",
                "progress_segment_width",
                "checkbox_0_checked",
                "checkbox_1_checked",
                "checkbox_focused_index",
                "checkbox_hovered_index",
            ]
        },
        "state_tail": [
            {
                key: row.get(key)
                for key in [
                    "frame",
                    "action_count",
                    "settings_revision",
                    "last_action",
                    "last_event",
                    "state_label",
                    "text_selection_active",
                    "clipboard_text_len",
                    "progress_percent",
                    "progress_segment_x",
                    "progress_segment_width",
                    "checkbox_0_checked",
                    "checkbox_1_checked",
                    "checkbox_focused_index",
                    "checkbox_hovered_index",
                ]
            }
            for row in state_rows(rows)[-3:]
        ],
    }


def passed_audit_scenarios_by_operation(
    audit_path: Path,
) -> dict[tuple[str, str], list[dict[str, Any]]]:
    data = json.loads(audit_path.read_text())
    scenarios = data.get("scenarios", [])
    if not isinstance(scenarios, list):
        raise SystemExit("audit `scenarios` must be an array")
    by_operation: dict[tuple[str, str], list[dict[str, Any]]] = {}
    for scenario in scenarios:
        if not isinstance(scenario, dict) or not scenario.get("passed"):
            continue
        page = scenario.get("page")
        operation_kind = scenario.get("operation_kind")
        if not isinstance(page, str) or not isinstance(operation_kind, str):
            continue
        by_operation.setdefault((page, operation_kind), []).append(scenario)
    return by_operation


def trace_matches_audit_scenario(row: dict[str, Any], scenario: dict[str, Any]) -> bool:
    action = str(scenario.get("action", "none"))
    event = str(scenario.get("event", "none"))
    state = str(scenario.get("state", "idle"))
    if action == "none" and event == "none":
        return (
            row.get("action_count", 0) == 0
            and row.get("last_action") == "none"
            and row.get("last_event") == "none"
        )
    if row.get("last_action") != action:
        return False
    if row.get("last_event") != event:
        if row.get("state_label") == state and action.endswith("context_pin"):
            return True
        return False
    if state != "idle" and not state_label_matches(row.get("state_label"), state):
        return False
    return True


def state_label_matches(actual: object, expected: str) -> bool:
    actual_value = str(actual)
    if actual_value == expected:
        return True
    expected_tokens = expected.split()
    if not expected_tokens:
        return False
    actual_tokens = set(actual_value.split())
    return all(token in actual_tokens for token in expected_tokens)


def visual_only_audit_scenarios(scenarios: list[dict[str, Any]]) -> bool:
    return bool(scenarios) and all(
        scenario.get("action") == "none" and scenario.get("event") == "none"
        for scenario in scenarios
    )


def full_sweep_contract_failures(
    page: str,
    operation: str,
    rows_after_operation: list[dict[str, Any]],
    scenarios: list[dict[str, Any]],
) -> list[str]:
    if not scenarios:
        return [f"missing passed headless audit scenario for {page}:{operation}"]
    if visual_only_audit_scenarios(scenarios):
        changed = state_rows(rows_after_operation)
        if changed:
            last = changed[-1]
            return [
                "expected visual-only native operation without Storybook action, got "
                f"{last.get('last_action')}/{last.get('last_event')}/{last.get('state_label')}"
            ]
        return []
    if any(
        trace_matches_audit_scenario(row, scenario)
        for row in rows_after_operation
        for scenario in scenarios
    ):
        return []
    expected = [
        f"{scenario.get('operation')}:{scenario.get('action')}/"
        f"{scenario.get('event')}/{scenario.get('state')}"
        for scenario in scenarios[:4]
    ]
    return [f"native trace did not match passed audit scenario: expected one of {expected}"]


def full_sweep_result_summary(
    page: str,
    operation: str,
    rows_after_operation: list[dict[str, Any]],
    scenarios: list[dict[str, Any]],
) -> dict[str, Any]:
    failures = full_sweep_contract_failures(page, operation, rows_after_operation, scenarios)
    last = rows_after_operation[-1] if rows_after_operation else {}
    return {
        "page": page,
        "operation": operation,
        "ok": not failures,
        "reason": "native_required_operation_matched"
        if not failures
        else "; ".join(failures),
        "audit_scenarios": [
            {
                "operation": scenario.get("operation"),
                "action": scenario.get("action"),
                "event": scenario.get("event"),
                "state": scenario.get("state"),
                "body_pixel_diff": scenario.get("body_pixel_diff"),
            }
            for scenario in scenarios
        ],
        "last": {
            key: last.get(key)
            for key in [
                "frame",
                "action_count",
                "settings_revision",
                "last_action",
                "last_event",
                "state_label",
                "text_selection_active",
                "clipboard_text_len",
                "progress_percent",
                "progress_segment_x",
                "progress_segment_width",
                "checkbox_0_checked",
                "checkbox_1_checked",
                "checkbox_focused_index",
                "checkbox_hovered_index",
            ]
        },
        "state_tail": [
            {
                key: row.get(key)
                for key in [
                    "frame",
                    "action_count",
                    "settings_revision",
                    "last_action",
                    "last_event",
                    "state_label",
                    "text_selection_active",
                    "clipboard_text_len",
                    "progress_percent",
                    "progress_segment_x",
                    "progress_segment_width",
                    "checkbox_0_checked",
                    "checkbox_1_checked",
                    "checkbox_focused_index",
                    "checkbox_hovered_index",
                ]
            }
            for row in state_rows(rows_after_operation)[-3:]
        ],
    }


def probe_page(
    root: Path,
    binary: Path,
    helper: Path,
    output_dir: Path,
    frames: str,
    page: str,
) -> dict[str, Any]:
    trace_path = output_dir / f"{page}.jsonl"
    if trace_path.exists():
        trace_path.unlink()
    env = os.environ.copy()
    env["KUC_STORYBOOK_MOUSE_TRACE"] = str(trace_path)
    env["KUC_STORYBOOK_SCALE"] = "1"
    proc = subprocess.Popen(
        [str(binary), "--open-window", frames, page],
        cwd=root,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    try:
        bounds, first = wait_for_window(root, trace_path)
        if not bounds or not first:
            return {"page": page, "ok": False, "reason": "window_or_trace_not_ready"}
        activate_storybook_window(root)
        operate(root, helper, page, bounds, first)
        time.sleep(0.5)
        return result_summary(page, read_rows(trace_path))
    finally:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=2)
            except subprocess.TimeoutExpired:
                proc.kill()


def probe_modal_window_check(
    root: Path,
    binary: Path,
    helper: Path,
    frames: str,
    check: str,
) -> dict[str, Any]:
    proc = subprocess.Popen(
        [str(binary), "--open-modal-window", frames],
        cwd=root,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    try:
        window_info = wait_for_modal_window(root)
        if not window_info or not window_info.get("modal_bounds"):
            return {
                "page": "modal",
                "check": check,
                "ok": False,
                "reason": "modal_window_not_ready",
                "last": {"window_names": window_names(root)},
            }
        front_window_after_tab = None
        modal_bounds = window_info["modal_bounds"]
        if check == "modal_focus_trap":
            click_modal_center(root, helper, modal_bounds)
            input_cmd(root, helper, "key", "tab")
            time.sleep(0.2)
            names_after_tab = window_names(root)
            front_window_after_tab = names_after_tab[0] if names_after_tab else None
        click_modal_and_escape(root, helper, modal_bounds)
        if check == "modal_escape_after_close_idempotent" and proc.poll() is None:
            time.sleep(0.05)
            if proc.poll() is None:
                input_cmd(root, helper, "key", "escape")
        try:
            stdout, _stderr = proc.communicate(timeout=4)
            exited_after_escape = proc.returncode == 0
        except subprocess.TimeoutExpired:
            proc.kill()
            stdout, _stderr = proc.communicate(timeout=2)
            exited_after_escape = False
        summary = parse_modal_summary(stdout or "")
        failures = modal_window_contract_failures(
            check,
            window_info,
            summary,
            exited_after_escape,
            front_window_after_tab,
        )
        return {
            "page": "modal",
            "check": check,
            "ok": not failures,
            "reason": "separate_modal_window_contract_matched"
            if not failures
            else "; ".join(failures),
            "last": {
                "window_names": window_info.get("names"),
                "front_window_name": window_info.get("front_window_name"),
                "front_window_after_tab": front_window_after_tab,
                "exited_after_escape": exited_after_escape,
                **summary,
            },
        }
    finally:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=2)
            except subprocess.TimeoutExpired:
                proc.kill()


def probe_matrix_check(
    root: Path,
    binary: Path,
    helper: Path,
    output_dir: Path,
    frames: str,
    page: str,
    check: str,
) -> dict[str, Any]:
    if page == "modal":
        return probe_modal_window_check(root, binary, helper, frames, check)
    trace_path = output_dir / f"{page}--{check}.jsonl"
    if trace_path.exists():
        trace_path.unlink()
    env = os.environ.copy()
    env["KUC_STORYBOOK_MOUSE_TRACE"] = str(trace_path)
    env["KUC_STORYBOOK_SCALE"] = "1"
    command = [str(binary), "--open-window", frames, page]
    if preset_index := MATRIX_PRESETS.get((page, check)):
        command.extend(["--preset", str(preset_index)])
    proc = subprocess.Popen(
        command,
        cwd=root,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    try:
        bounds, first = wait_for_window(root, trace_path)
        if not bounds or not first:
            return {
                "page": page,
                "check": check,
                "ok": False,
                "reason": "window_or_trace_not_ready",
            }
        activate_storybook_window(root)
        operate_matrix_check(root, helper, page, check, bounds, first)
        time.sleep(0.5)
        return matrix_result_summary(page, check, read_rows(trace_path))
    finally:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=2)
            except subprocess.TimeoutExpired:
                proc.kill()


def probe_matrix_check_with_retries(
    root: Path,
    binary: Path,
    helper: Path,
    output_dir: Path,
    frames: str,
    page: str,
    check: str,
    attempts: int = 3,
) -> dict[str, Any]:
    last_result: dict[str, Any] = {}
    for attempt in range(1, attempts + 1):
        result = probe_matrix_check(root, binary, helper, output_dir, frames, page, check)
        result["attempt"] = attempt
        last_result = result
        if result.get("ok"):
            return result
        time.sleep(0.35)
    last_result["attempts"] = attempts
    return last_result


def probe_full_sweep_check(
    root: Path,
    binary: Path,
    helper: Path,
    output_dir: Path,
    frames: str,
    page: str,
    operation: str,
    audit_scenarios: dict[tuple[str, str], list[dict[str, Any]]],
) -> dict[str, Any]:
    matrix_checks = FULL_SWEEP_MATRIX_CHECKS.get((page, operation), [])
    if matrix_checks:
        child_results = [
            probe_matrix_check_with_retries(root, binary, helper, output_dir, frames, page, check)
            for check in matrix_checks
        ]
        failures = [result for result in child_results if not result.get("ok")]
        return {
            "page": page,
            "operation": operation,
            "ok": not failures,
            "reason": "native_matrix_operation_matched"
            if not failures
            else "; ".join(
                f"{result.get('check')}: {result.get('reason')}" for result in failures
            ),
            "checks": child_results,
        }

    trace_path = output_dir / f"{page}--{operation}.jsonl"
    if trace_path.exists():
        trace_path.unlink()
    env = os.environ.copy()
    env["KUC_STORYBOOK_MOUSE_TRACE"] = str(trace_path)
    env["KUC_STORYBOOK_SCALE"] = "1"
    proc = subprocess.Popen(
        [str(binary), "--open-window", frames, page],
        cwd=root,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    try:
        bounds, first = wait_for_window(root, trace_path)
        if not bounds or not first:
            return {
                "page": page,
                "operation": operation,
                "ok": False,
                "reason": "window_or_trace_not_ready",
            }
        activate_storybook_window(root)
        before_rows = read_rows(trace_path)
        operate_required_operation(root, helper, page, operation, bounds, first)
        time.sleep(0.55)
        rows = read_rows(trace_path)
        rows_after_operation = rows[len(before_rows) :]
        return full_sweep_result_summary(
            page,
            operation,
            rows_after_operation,
            audit_scenarios.get((page, operation), []),
        )
    finally:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=2)
            except subprocess.TimeoutExpired:
                proc.kill()


def selected_matrix_checks(pages: list[str]) -> list[tuple[str, str]]:
    selected_pages = pages or sorted(MATRIX_SCENARIOS)
    checks: list[tuple[str, str]] = []
    for page in selected_pages:
        for check in MATRIX_SCENARIOS.get(page, []):
            checks.append((page, check))
    return checks


def selected_full_sweep_checks(
    manifest: dict[str, Any],
    pages: list[str],
) -> list[tuple[str, str]]:
    selected_pages = set(pages) if pages else None
    checks: list[tuple[str, str]] = []
    for entry in manifest_entries(manifest):
        page = entry["page"]
        if selected_pages is not None and page not in selected_pages:
            continue
        for operation in required_operations_for_manifest_entry(manifest, entry):
            checks.append((page, operation))
    return checks


def run_probe(args: argparse.Namespace) -> int:
    require_macos()
    root = args.root.resolve()
    manifest_path = resolve(root, args.manifest)
    output_dir = resolve(root, args.output_dir)
    helper_dir = resolve(root, args.helper_dir)
    binary = resolve(root, args.binary)
    output_dir.mkdir(parents=True, exist_ok=True)
    helper = ensure_helper(root, helper_dir)
    ensure_binary(root, binary, args.skip_build)
    if args.full_sweep:
        manifest = json.loads(manifest_path.read_text())
        audit_scenarios = passed_audit_scenarios_by_operation(resolve(root, args.audit))
        selected_checks = selected_full_sweep_checks(manifest, args.pages)
        results: list[dict[str, Any]] = []
        for index, (page, operation) in enumerate(selected_checks, 1):
            result = probe_full_sweep_check(
                root,
                binary,
                helper,
                output_dir,
                args.frames,
                page,
                operation,
                audit_scenarios,
            )
            results.append(result)
            status = "ok" if result.get("ok") else "FAIL"
            reason = result.get("reason", "unknown")
            print(
                f"{index:03d}/{len(selected_checks):03d} {status} {page}:{operation} {reason}",
                flush=True,
            )
        summary_path = output_dir / "summary.json"
        summary_path.write_text(json.dumps(results, ensure_ascii=False, indent=2))
        failures = [result for result in results if not result.get("ok")]
        print(
            f"summary ok={len(results) - len(failures)} fail={len(failures)} path={summary_path}"
        )
        for failure in failures:
            print(
                f"FAIL {failure.get('page')}:{failure.get('operation')}: "
                f"{failure.get('reason')} {failure.get('last')}"
            )
        return 1 if failures else 0
    if args.matrix:
        selected_checks = selected_matrix_checks(args.pages)
        results: list[dict[str, Any]] = []
        for index, (page, check) in enumerate(selected_checks, 1):
            result = probe_matrix_check_with_retries(
                root,
                binary,
                helper,
                output_dir,
                args.frames,
                page,
                check,
            )
            results.append(result)
            status = "ok" if result.get("ok") else "FAIL"
            reason = result.get("reason", "unknown")
            print(
                f"{index:02d}/{len(selected_checks):02d} {status} {page}:{check} {reason}",
                flush=True,
            )
        summary_path = output_dir / "summary.json"
        summary_path.write_text(json.dumps(results, ensure_ascii=False, indent=2))
        failures = [result for result in results if not result.get("ok")]
        print(
            f"summary ok={len(results) - len(failures)} fail={len(failures)} path={summary_path}"
        )
        for failure in failures:
            print(
                f"FAIL {failure.get('page')}:{failure.get('check')}: "
                f"{failure.get('reason')} {failure.get('last')}"
            )
        return 1 if failures else 0
    selected_pages = args.pages or manifest_pages(manifest_path)
    results: list[dict[str, Any]] = []
    for index, page in enumerate(selected_pages, 1):
        result = probe_page(root, binary, helper, output_dir, args.frames, page)
        results.append(result)
        status = "ok" if result.get("ok") else "FAIL"
        reason = result.get("reason", "unknown")
        print(f"{index:02d}/{len(selected_pages):02d} {status} {page} {reason}", flush=True)
    summary_path = output_dir / "summary.json"
    summary_path.write_text(json.dumps(results, ensure_ascii=False, indent=2))
    failures = [result for result in results if not result.get("ok")]
    print(f"summary ok={len(results) - len(failures)} fail={len(failures)} path={summary_path}")
    for failure in failures:
        print(f"FAIL {failure.get('page')}: {failure.get('reason')} {failure.get('last')}")
    return 1 if failures else 0


def self_test() -> int:
    bounds = (10, 20, 1400, 952)
    row = {"height": 920}
    if screen_point(bounds, row, 5, 7) != (15, 59):
        print("screen_point failed")
        return 1
    rows = [
        {"frame": 1, "action_count": 0, "last_action": "none", "last_event": "none"},
        {
            "frame": 2,
            "action_count": 1,
            "last_action": "toggle_change",
            "last_event": "toggle_changed",
        },
    ]
    summary = result_summary("toggle", rows)
    if not summary["ok"] or summary["reason"] != "state_changed":
        print("result_summary failed")
        return 1
    text_summary = result_summary(
        "text",
        [
            {
                "frame": 1,
                "action_count": 1,
                "last_action": "select_text",
                "last_event": "text_selection_changed",
                "state_label": "selection=active",
                "text_selection_active": True,
            }
        ],
    )
    if not text_summary["ok"] or text_summary["reason"] != "trace_contract_matched":
        print("trace contract positive failed")
        return 1
    bad_text_summary = result_summary(
        "text",
        [
            {
                "frame": 1,
                "action_count": 1,
                "last_action": "style_apply",
                "last_event": "text_style_changed",
                "state_label": "role=heading",
            }
        ],
    )
    if bad_text_summary["ok"] or "missing trace contract" not in bad_text_summary["reason"]:
        print("trace contract negative failed")
        return 1
    copy_summary = matrix_result_summary(
        "text",
        "text_keyboard_copy",
        [
            {
                "frame": 1,
                "action_count": 2,
                "last_action": "copy_selection",
                "last_event": "clipboard_copy",
                "state_label": "clipboard=selected_text",
                "clipboard_text_len": 4,
            }
        ],
    )
    if not copy_summary["ok"]:
        print("matrix min contract failed")
        return 1
    paste_summary = matrix_result_summary(
        "text",
        "text_keyboard_paste",
        [{"frame": 1, "action_count": 0, "last_action": "none", "last_event": "none"}],
    )
    if not paste_summary["ok"]:
        print("matrix no-state contract failed")
        return 1
    return 0


def main() -> int:
    args = parse_args()
    if args.self_test:
        return self_test()
    return run_probe(args)


if __name__ == "__main__":
    raise SystemExit(main())
