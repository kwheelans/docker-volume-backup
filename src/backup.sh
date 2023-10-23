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

if [[ "$TYPE" == 'single' ]]; then
  backup_single "$EXTENSION"
else
  backup_multi "$EXTENSION"
fi
