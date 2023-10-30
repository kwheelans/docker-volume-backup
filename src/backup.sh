#!/usr/bin/env bash

backup_multi() {
  for data in *; do
    local timestamp="$(date +%Y-%m-%d_%H-%M-%S)"
    local ext="$1"
    local output="/backup/${PREFIX}_${data}_${timestamp}.tar.${ext}"

    tar -caf "$output" "$data"
    chmod "$PERMISSION" "$output"
  done
}

backup_single() {
  local timestamp="$(date +%Y-%m-%d_%H-%M-%S)"
  local ext="$1"
  local output="/backup/${PREFIX}_${timestamp}.tar.${ext}"
  local data="*"

  tar -caf "$output" "$data"
  chmod "$PERMISSION" "$output"
}

TYPE=$2
cd /data || exit 1

RUNTIME="$(date +%Y-%m-%d_%H-%M-%S)"

if [[ "$TYPE" == 'single' ]]; then
  echo "Running backup_single at ${RUNTIME}"
  backup_single "${COMPRESS,,}"
else
  echo "Running backup_multi at ${RUNTIME}"
  backup_multi "${COMPRESS,,}"
fi
