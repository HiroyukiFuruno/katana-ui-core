#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KUC_APT_TIMEOUT_SECONDS=600 exec bash "${script_dir}/install-apt-packages.sh" ffmpeg
