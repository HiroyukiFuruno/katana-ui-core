#!/usr/bin/env bash
set -euo pipefail

workspace=/tmp/kuc-workspace
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
