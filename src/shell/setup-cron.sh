#!/usr/bin/env sh

# Check cron variable
if [ -z "$CRON" ]; then
  CRON="0 0 * * *"
fi

docker-volume-backup --validate
RTN_CODE=$?

if [ ! $RTN_CODE = 0 ]; then
  echo $RTN_CODE
  exit $RTN_CODE
fi

# Setup crontab
echo "Setting crontab with CRON as ${CRON}"
crontab -l | { cat; echo "${CRON} docker-volume-backup > /proc/1/fd/1 2>/proc/1/fd/2"; } | crontab -

crond -f
