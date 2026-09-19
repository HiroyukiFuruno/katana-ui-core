#!/usr/bin/env bash
set -euo pipefail

readonly packages=(
  ffmpeg
  fonts-noto-cjk
  fonts-noto-color-emoji
  fonts-noto-mono
  xauth
  xvfb
)
readonly max_attempts=3
readonly apt_timeout_seconds=300

install_packages() {
  local attempt=1

  while [ "${attempt}" -le "${max_attempts}" ]; do
    local log_file
    log_file="${RUNNER_TEMP:-/tmp}/kuc-apt-install-${attempt}.log"
    echo "KUC headless-display install attempt ${attempt}/${max_attempts}; log=${log_file}"

    if timeout "${apt_timeout_seconds}" sudo apt-get update >"${log_file}" 2>&1 \
      && timeout "${apt_timeout_seconds}" sudo env DEBIAN_FRONTEND=noninteractive \
        apt-get install --yes --no-install-recommends "${packages[@]}" >>"${log_file}" 2>&1; then
      return 0
    fi

    local exit_code=$?
    echo "KUC headless-display install attempt ${attempt} failed (exit=${exit_code})."
    tail -n 200 "${log_file}" || true
    attempt=$((attempt + 1))
  done

  echo "KUC headless-display install failed after ${max_attempts} attempts." >&2
  exit 1
}

install_packages

readonly mono_font=/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf
readonly emoji_font=/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf
readonly expected_emoji_hash=93cdc4ee9aa40e2afceecc63da0ca05ec7aab4bec991ece51a6b52389f48a477
readonly expected_mono_hash=6b692c4b6d15ccf59f1c1fe8d11cb8a92f51960f3e9f1f523781755a3af7e29f

test "$(sha256sum "${emoji_font}" | cut -d ' ' -f 1)" = "${expected_emoji_hash}"
test "$(sha256sum "${mono_font}" | cut -d ' ' -f 1)" = "${expected_mono_hash}"
echo "KUC_PINNED_LINUX_EMOJI_SHA256=${expected_emoji_hash}" >> "${GITHUB_ENV}"
