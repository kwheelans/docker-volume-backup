#!/usr/bin/env bash

backup_multi() {
  for data in *; do
    local timestamp="$(date +%Y-%m-%d_%H-%M-%S)"
    local ext="$1"
    local output="/backup/docker-backup-volume_${data}_${timestamp}.tar.${ext}"

    tar -caf "$output" "$data"
  done
}

backup_single() {
  local timestamp="$(date +%Y-%m-%d_%H-%M-%S)"
  local ext="$1"
  local output="/backup/docker-backup-volume_all_${timestamp}.tar.${ext}"
  local data="*"

  tar -caf "$output" "$data"
}

EXTENSION=$1
TYPE=$2
cd /data || exit 1

RUNTIME="$(date +%Y-%m-%d_%H-%M-%S)"

if [[ "$TYPE" == 'single' ]]; then
  echo "Running backup_single at ${RUNTIME}"
  backup_single "$EXTENSION"
else
  echo "Running backup_muli at ${RUNTIME}"
  backup_multi "$EXTENSION"
fi
