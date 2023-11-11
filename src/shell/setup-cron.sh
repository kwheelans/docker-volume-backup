#!/usr/bin/env bash

# Check cron variable
if [[ -z "$CRON" ]]; then
  CRON="0 0 * * *"
fi

/docker-volume-backup/docker-volume-backup --validate
RTN_CODE=$?

if [ ! $RTN_CODE = 0 ]; then
  echo $RTN_CODE
  exit $RTN_CODE
fi

# Setup crontab
echo "Setting crontab with CRON as ${CRON}"
crontab -l | { cat; echo "${CRON} /docker-volume-backup/docker-volume-backup"; } | crontab -

crond -f
