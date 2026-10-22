#!/usr/bin/env bash
set -euo pipefail

readonly max_attempts=3
readonly default_apt_timeout_seconds=300
readonly apt_timeout_seconds="${KUC_APT_TIMEOUT_SECONDS:-${default_apt_timeout_seconds}}"

case "${apt_timeout_seconds}" in
  '' | *[!0-9]*)
    echo "KUC apt installer timeout must be a positive integer." >&2
    exit 2
    ;;
esac

if [ "${apt_timeout_seconds}" -le 0 ]; then
  echo "KUC apt installer timeout must be a positive integer." >&2
  exit 2
fi

if [ "$#" -eq 0 ]; then
  echo "KUC apt installer requires at least one package." >&2
  exit 2
fi

install_packages() {
  local attempt=1

  while [ "${attempt}" -le "${max_attempts}" ]; do
    local log_file
    log_file="${RUNNER_TEMP:-/tmp}/kuc-apt-install-${attempt}.log"
    echo "KUC apt install attempt ${attempt}/${max_attempts}; packages=$*; log=${log_file}"

    if timeout "${apt_timeout_seconds}" sudo apt-get update >"${log_file}" 2>&1 \
      && timeout "${apt_timeout_seconds}" sudo env DEBIAN_FRONTEND=noninteractive \
        apt-get install --yes --no-install-recommends "$@" >>"${log_file}" 2>&1; then
      return 0
    else
      local exit_code=$?
      echo "KUC apt install attempt ${attempt} failed (exit=${exit_code})."
    fi

    tail -n 200 "${log_file}" || true
    attempt=$((attempt + 1))
  done

  echo "KUC apt install failed after ${max_attempts} attempts." >&2
  exit 1
}

install_packages "$@"
