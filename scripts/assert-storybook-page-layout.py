#!/usr/bin/env python3
from pathlib import Path
import re
import sys

PAGES_DIR = Path("storybook/src/pages")

BESPOKE_PAGES = {
    "accordion.rs",
    "breadcrumb.rs",
    "command_palette.rs",
    "loading_dots.rs",
    "menu_button.rs",
    "modal_overlay.rs",
    "progress_bar.rs",
    "side_menu.rs",
    "slide_control.rs",
    "status_bar.rs",
    "theme_tokens.rs",
    "toolbar.rs",
    "tooltip.rs",
    "tree_view.rs",
    "welcome.rs",
}

TITLE_WORDS = ("Samples", "Primitive", "Button", "Spinner")

EXTRA_MENU_PAGES = {"Overview", "ThemeTokens"}
GROUP_DIRS = {
    Path("composite/button"),
    Path("composite/indicator"),
    Path("composite/input"),
    Path("composite/selector"),
}

WIDGET_PAGE_ALIASES = {
    "primitive/text": "Text",
    "primitive/icon": "Icon",
    "primitive/spinner": "Spinner",
    "primitive/loading_dots": "LoadingDots",
    "composite/button/svg": "SvgButton",
    "composite/button/text": "TextButton",
    "composite/button/icon_text": "IconTextButton",
    "composite/input/toggle": "Toggle",
    "composite/selector/toggle": "Toggle",
    "composite/selector/segmented": "SegmentedToggle",
    "composite/selector/select": "SelectBox",
    "composite/combo_box": "ComboBox",
    "composite/selector/color": "ColorSwatch",
    "composite/selector/color_picker": "ColorPickerRgba",
    "composite/input/text": "TextInput",
    "composite/input/search": "SearchBox",
    "composite/indicator/tooltip": "Tooltip",
    "composite/indicator/badge": "Badge",
    "composite/indicator/key_cap": "KeyCap",
    "layout/card": "Card",
    "layout/accordion": "Accordion",
    "composite/menu_button": "MenuButton",
    "layout/side_menu": "SideMenu",
    "composite/command_palette": "CommandPalette",
    "layout/split": "SplitPane",
    "layout/modal": "Modal",
    "layout/popover": "Popover",
    "layout/align_center": "AlignCenter",
    "composite/tabs": "Tabs",
    "composite/tree_view": "TreeView",
    "layout/toolbar": "Toolbar",
    "composite/breadcrumb": "Breadcrumb",
    "composite/progress_bar": "ProgressBar",
    "layout/status_bar": "StatusBar",
    "composite/selection_list": "SelectionList",
    "composite/notification_toast": "NotificationToast",
    "composite/slide_control": "SlideControl",
    "composite/dynamic_array_editor": "DynamicArrayEditor",
    "composite/code_diff": "CodeDiff",
}


def extract_function_body(source: str, name_pattern: str) -> str | None:
    match = re.search(name_pattern, source)
    if match is None:
        return None

    open_index = source.find("{", match.end())
    if open_index < 0:
        return None

    depth = 0
    for index in range(open_index, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[open_index + 1 : index]
    return None


def check_standard_page(path: Path, source: str) -> list[str]:
    errors: list[str] = []
    page_content = extract_function_body(source, r"fn\s+page_content\s*\(")
    if page_content is None:
        errors.append("標準ページは fn page_content(...) を持つ必要があります")
        return errors

    public_page = extract_function_body(source, r"pub\s+fn\s+\w+_page\s*\(")
    if public_page is None:
        errors.append("pub fn *_page(...) がありません")
        return errors

    if "page_content" in public_page and "v_stack((" in public_page:
        errors.append("pub fn *_page が page_content の前に独自 v_stack を置いています")

    return errors


def widget_dirs() -> dict[str, str]:
    source_root = Path("crates/katana-ui-core/src")
    results: dict[str, str] = {}
    for root in ("primitive", "layout", "composite"):
        for directory in sorted((source_root / root).glob("*")):
            if not directory.is_dir():
                continue
            relative = directory.relative_to(source_root)
            if relative in GROUP_DIRS:
                for child in sorted(directory.glob("*")):
                    if child.is_dir() and (child / "mod.rs").exists():
                        key = child.relative_to(source_root).as_posix()
                        results[key] = WIDGET_PAGE_ALIASES.get(key, "")
            elif (directory / "mod.rs").exists():
                key = relative.as_posix()
                results[key] = WIDGET_PAGE_ALIASES.get(key, "")
    return results


def sidebar_pages() -> dict[str, str]:
    source = Path("storybook/src/sidebar.rs").read_text()
    pages: dict[str, str] = {}
    for label, page in re.findall(r'entry\("([^"]+)",\s*Page::(\w+)\)', source):
        if page not in EXTRA_MENU_PAGES:
            pages[page] = label
    return pages


def check_widget_menu_parity() -> list[str]:
    errors: list[str] = []
    expected_by_dir = widget_dirs()
    unmapped = sorted(key for key, page in expected_by_dir.items() if not page)
    if unmapped:
        errors.append("Storybook 対応先が未定義の widget directory: " + ", ".join(unmapped))

    expected = {page: key for key, page in expected_by_dir.items() if page}
    actual = sidebar_pages()
    missing = sorted(set(expected) - set(actual))
    extra = sorted(set(actual) - set(expected))
    if missing:
        errors.append("Storybook menu に不足している widget: " + ", ".join(missing))
    if extra:
        errors.append("widget directory と対応しない Storybook menu: " + ", ".join(extra))

    mismatched_labels = [
        f"{page}: expected {page}, actual {actual[page]}"
        for page in sorted(set(expected) & set(actual))
        if actual[page] != page
    ]
    if mismatched_labels:
        errors.append("Storybook menu label が Page 名と一致しません: " + "; ".join(mismatched_labels))
    return errors


def main() -> int:
    failures: list[str] = []
    for path in sorted(PAGES_DIR.glob("*.rs")):
        if path.name == "mod.rs":
            continue
        source = path.read_text()
        if path.name in BESPOKE_PAGES:
            continue
        for error in check_standard_page(path, source):
            failures.append(f"{path}: {error}")
    for error in check_widget_menu_parity():
        failures.append(error)

    if failures:
        print("storybook page layout lint failed", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
