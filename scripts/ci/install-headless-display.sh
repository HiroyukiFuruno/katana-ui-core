#!/usr/bin/env bash
set -euo pipefail

readonly packages=(
  fonts-noto-cjk
  fonts-noto-color-emoji
  fonts-noto-mono
  xauth
  xvfb
  libwayland-dev
  libx11-dev
  libx11-xcb-dev
  libxcursor-dev
  libxi-dev
  libxinerama-dev
  libxkbcommon-dev
  libxrandr-dev
  pkg-config
)
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
bash "${script_dir}/install-apt-packages.sh" "${packages[@]}"

readonly mono_font=/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf
readonly emoji_font=/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf
readonly expected_emoji_hash=93cdc4ee9aa40e2afceecc63da0ca05ec7aab4bec991ece51a6b52389f48a477
readonly expected_mono_hash=6b692c4b6d15ccf59f1c1fe8d11cb8a92f51960f3e9f1f523781755a3af7e29f

test "$(sha256sum "${emoji_font}" | cut -d ' ' -f 1)" = "${expected_emoji_hash}"
test "$(sha256sum "${mono_font}" | cut -d ' ' -f 1)" = "${expected_mono_hash}"
