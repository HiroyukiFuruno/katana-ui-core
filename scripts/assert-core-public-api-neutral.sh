#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_SRC="$ROOT_DIR/crates/katana-ui-core/src"
CORE_API_PATHS=(
  "$CORE_SRC/facade.rs"
  "$CORE_SRC/runtime"
  "$CORE_SRC/window"
  "$CORE_SRC/surface"
  "$CORE_SRC/panel"
  "$CORE_SRC/style"
  "$CORE_SRC/theme"
  "$CORE_SRC/render_model"
  "$CORE_SRC/widget"
  "$CORE_SRC/adapter_contract"
  "$CORE_SRC/raster_host"
)

violations="$(
  rg -n "\\b(View|ViewId|Element)\\b" \
    "${CORE_API_PATHS[@]}" \
    --glob '*.rs'
)" || true

if [ -n "$violations" ]; then
  echo "core public API exposes framework-native symbols"
  echo "$violations"
  exit 1
fi

platform_violations="$(
  rg -n "(/System/Library/Fonts|/Library/Fonts|/usr/share/fonts|C:\\\\Windows\\\\Fonts|\\b(AppKit|UIKit|CoreText|DirectWrite|fontconfig|eframe|egui|winit|tao|x11rb|wayland_client|cocoa)::)" \
    "${CORE_API_PATHS[@]}" \
    --glob '*.rs'
)" || true

if [ -n "$platform_violations" ]; then
  echo "core public API exposes OS-specific font paths or backend symbols"
  echo "$platform_violations"
  exit 1
fi

app_specific_violations="$(
  rg -n "\"viewer\\.|\\bVIEWER_" \
    "${CORE_API_PATHS[@]}" \
    --glob '*.rs'
)" || true

if [ -n "$app_specific_violations" ]; then
  echo "core public API exposes app-specific viewer host action symbols"
  echo "$app_specific_violations"
  exit 1
fi

legacy_production_name_violations="$(
  rg -n -i "^[[:space:]]*///?.*(katana-chat-ui|katana-language-editor|katana-document-viewer|katana-render-runtime|\\b(KCU|KLE|KDV|KRR)\\b)" \
    "$CORE_SRC" --glob '*.rs' --glob '!**/*test*' || true
)"
if [ -n "$legacy_production_name_violations" ]; then
  echo "production API comments expose an external consumer name"
  echo "$legacy_production_name_violations"
  exit 1
fi

production_name_violations="$(python3 - "$ROOT_DIR/crates" "$ROOT_DIR/examples" <<'PY'
from pathlib import Path
import re
import sys
roots = [Path(value) for value in sys.argv[1:]]
forbidden = re.compile(r"katana-(?!(?:ui-core|ui-widget|widget-parity-backlog)(?a:\b))[a-z][a-z0-9-]*|(?:\.\./|github\.com/[^/\s]+/)katana(?![a-z0-9_-])|\b(?:KCU|KLE|KDV|KRR|KML|KCF|KDR)\b", re.I)
def comments(text):
    i = line = depth = 0
    while i < len(text):
        if depth:
            start_line, start = line + 1, i
            while i < len(text):
                if text.startswith("/*", i): depth += 1; i += 2
                elif text.startswith("*/", i):
                    depth -= 1; i += 2
                    if not depth: break
                else: line += text[i] == "\n"; i += 1
            yield start_line, text[start:i]
        elif text.startswith("//", i):
            end = text.find("\n", i)
            if end < 0: end = len(text)
            yield line + 1, text[i:end]; i = end
        elif text.startswith("/*", i): depth = 1; i += 2
        elif text[i] == "r" and i + 1 < len(text):
            marker = i + 1
            while marker < len(text) and text[marker] == "#": marker += 1
            if marker < len(text) and text[marker] == '"':
                hashes = text[i + 1:marker]
                end = text.find('"' + hashes, marker + 1)
                if end >= 0:
                    line += text[i:end + len(hashes) + 1].count("\n")
                    i = end + len(hashes) + 1
                    continue
            line += text[i] == "\n"; i += 1
        elif text[i] == '"' or (text[i] == "'" and i + 2 < len(text) and text[i + 2] == "'"):
            quote = text[i]; i += 1
            while i < len(text):
                if text[i] == "\\": i += 2
                elif text[i] == quote: i += 1; break
                else: line += text[i] == "\n"; i += 1
        else: line += text[i] == "\n"; i += 1
for path in sorted(path for root in roots for path in root.rglob("*.rs")):
    if "target" in path.parts: continue
    for start, comment in comments(path.read_text(encoding="utf-8")):
        if forbidden.search(comment): print(f"{path}:{start}:{comment.splitlines()[0]}")
PY
)"
if [ -n "$production_name_violations" ]; then
  echo "production API comments expose an external consumer name"
  echo "$production_name_violations"
  exit 1
fi

# 現行の公開文書と運用スキルを検査し、過去のrelease/ADR記録は履歴として保持する。
PUBLIC_DOC_PATHS=(
  "$ROOT_DIR/README.md"
  "$ROOT_DIR/docs/selection-dialogs.md"
  "$ROOT_DIR/docs/ui-selection-snapshots.md"
  "$ROOT_DIR/docs/live-unicode-observation.md"
  "$ROOT_DIR/docs/tab-strip-host-proposals.md"
  "$ROOT_DIR/docs/context-menu-enabled-resolver.md"
  "$ROOT_DIR/docs/ui-separation-plan.md"
  "$ROOT_DIR/docs/architecture/ui-separation/owned-ui-task-map.md"
  "$ROOT_DIR/docs/architecture/ui-separation/fallible-text-command-root-design.md"
  "$ROOT_DIR/docs/architecture/ui-separation/root-plan-source.md"
  "$ROOT_DIR/docs/architecture/ui-separation/ast-lint-remediation-plan.md"
)
doc_violations="$(
  python3 - "$ROOT_DIR" "${PUBLIC_DOC_PATHS[@]}" <<'PY'
from pathlib import Path
import re
import sys
forbidden = re.compile(r"katana-(?!(?:ui-core|ui-widget|widget-parity-backlog)(?a:\b))[a-z][a-z0-9-]*|(?:\.\./|github\.com/[^/\s]+/)katana(?![a-z0-9_-])|\b(?:KCU|KLE|KDV|KRR|KML|KCF|KDR)\b", re.I)
repo_root = Path(sys.argv[1])
paths = [Path(value) for value in sys.argv[2:]]
for directory in (".codex/skills", ".agents/skills", ".agent/skills", ".claude/skills"):
    paths.extend(sorted((repo_root / directory).glob("*/SKILL.md")))
for directory in (".codex/workflows", ".agent/workflows", ".claude/commands"):
    paths.extend(sorted((repo_root / directory).glob("*.md")))
for path in paths:
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if forbidden.search(line): print(f"{path}:{number}:{line}")
PY
  )"
if [ -n "$doc_violations" ]; then
  echo "current public guidance exposes an external consumer name"
  echo "$doc_violations"
  exit 1
fi
