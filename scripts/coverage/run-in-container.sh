#!/usr/bin/env bash
set -euo pipefail

if [[ "${KUC_COVERAGE_RUNTIME:-}" != "container" ]]; then
  echo "container coverage requires KUC_COVERAGE_RUNTIME=container" >&2
  exit 1
fi
if [[ ! "${KUC_COVERAGE_IMAGE_ID:-}" =~ ^runtime-v1:sha256:[0-9a-f]{64}$ ]]; then
  echo "container coverage requires a validated runtime image identity" >&2
  exit 1
fi

workspace=/tmp/kuc-workspace
readonly linux_emoji_font=/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf
readonly pinned_linux_emoji_sha256=e5899ed38b8ed83e08bd3ac5de09791e9d19d288333a796de1d35ad17396f1ec
actual_linux_emoji_sha256="$(sha256sum "${linux_emoji_font}" | cut -d ' ' -f 1)"
if [[ "${actual_linux_emoji_sha256}" != "${pinned_linux_emoji_sha256}" ]]; then
  echo "coverage image emoji font hash does not match the pinned release input" >&2
  exit 1
fi
export KUC_PINNED_LINUX_EMOJI_SHA256="${pinned_linux_emoji_sha256}"

mkdir -p "${workspace}"
tar \
  --exclude-vcs \
  --exclude=target \
  --exclude='*/target' \
  --create \
  --file=- \
  --directory=/source \
  . \
  | tar --extract --file=- --directory="${workspace}"

cd "${workspace}"
bash scripts/run-strict-coverage.sh
