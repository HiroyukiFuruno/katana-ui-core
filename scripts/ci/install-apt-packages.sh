#!/usr/bin/env bash
set -euo pipefail

readonly max_attempts=3
readonly default_apt_timeout_seconds=300
readonly apt_timeout_seconds="${KUC_APT_TIMEOUT_SECONDS:-${default_apt_timeout_seconds}}"
readonly apt_acquire_timeout_seconds=30
readonly apt_acquire_retries=2
readonly apt_acquire_options=(
  -o "Acquire::http::Timeout=${apt_acquire_timeout_seconds}"
  -o "Acquire::https::Timeout=${apt_acquire_timeout_seconds}"
  -o "Acquire::Retries=${apt_acquire_retries}"
)
readonly apt_config_root="${KUC_APT_CONFIG_ROOT:-/etc/apt}"

if [ -n "${KUC_APT_CONFIG_ROOT:-}" ]; then
  case "${KUC_APT_CONFIG_ROOT}" in
    /*) ;;
    *)
      echo "KUC apt config root must be an absolute path." >&2
      exit 2
      ;;
  esac
  if [ "${KUC_APT_CONFIG_ROOT}" = "/" ]; then
    echo "KUC apt config root must not be the filesystem root." >&2
    exit 2
  fi
fi

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

prefer_official_ubuntu_mirrors() {
  local mirror_list_file="${apt_config_root}/apt-mirrors.txt"
  local source_file
  local fallback_applied=false

  if [ -f "${mirror_list_file}" ] && grep -Eq '^https?://azure\.archive\.ubuntu\.com/ubuntu/?([[:space:]]|$)' "${mirror_list_file}"; then
    if grep -Eq '^https://(archive|security)\.ubuntu\.com/ubuntu/?([[:space:]]|$)' "${mirror_list_file}"; then
      sudo perl -ni -e 'print unless m{^https?://azure\.archive\.ubuntu\.com/ubuntu/?(?:\s|$)}' "${mirror_list_file}"
    else
      sudo perl -pi -e 's#^https?://azure\.archive\.ubuntu\.com/ubuntu/?#https://archive.ubuntu.com/ubuntu/#' "${mirror_list_file}"
    fi
    fallback_applied=true
  fi

  for source_file in \
    "${apt_config_root}/sources.list" \
    "${apt_config_root}/sources.list.d/ubuntu.sources"; do
    if [ -f "${source_file}" ] && grep -Eq 'https?://azure\.archive\.ubuntu\.com/ubuntu(/|[[:space:]]|$)' "${source_file}"; then
      sudo perl -pi \
        -e 's#http://azure\.archive\.ubuntu\.com/ubuntu(?=/|\s|$)#https://archive.ubuntu.com/ubuntu#g;' \
        -e 's#https://azure\.archive\.ubuntu\.com/ubuntu(?=/|\s|$)#https://archive.ubuntu.com/ubuntu#g' \
        "${source_file}"
      fallback_applied=true
    fi
  done

  if [ "${fallback_applied}" = true ]; then
    echo "KUC apt fallback prefers official Ubuntu HTTPS archives after Azure mirror failure."
  else
    echo "KUC apt fallback found no Azure mirror in the supported Ubuntu source files."
  fi
}

install_packages() {
  local attempt=1

  while [ "${attempt}" -le "${max_attempts}" ]; do
    local log_file
    log_file="${RUNNER_TEMP:-/tmp}/kuc-apt-install-${attempt}.log"
    echo "KUC apt install attempt ${attempt}/${max_attempts}; packages=$*; log=${log_file}"

    if timeout "${apt_timeout_seconds}" sudo apt-get "${apt_acquire_options[@]}" update >"${log_file}" 2>&1 \
      && timeout "${apt_timeout_seconds}" sudo env DEBIAN_FRONTEND=noninteractive \
        apt-get "${apt_acquire_options[@]}" install --yes --no-install-recommends "$@" >>"${log_file}" 2>&1; then
      return 0
    else
      local exit_code=$?
      echo "KUC apt install attempt ${attempt} failed (exit=${exit_code})."
    fi

    tail -n 200 "${log_file}" || true
    if [ "${attempt}" -eq 1 ]; then
      prefer_official_ubuntu_mirrors
    fi
    attempt=$((attempt + 1))
  done

  echo "KUC apt install failed after ${max_attempts} attempts." >&2
  exit 1
}

install_packages "$@"
