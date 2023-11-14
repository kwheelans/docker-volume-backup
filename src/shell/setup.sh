#!/usr/bin/env sh

salvage --validate
RTN_CODE=$?

if [ ! $RTN_CODE = 0 ]; then
  echo $RTN_CODE
  exit $RTN_CODE
fi

#Wait to ensure container status is running
sleep 1

if [ "$SALVAGE_RUN_ONCE" = "true" ]; then
  salvage
else
  # Set default CRON value
  if [ -z "$CRON" ]; then
    CRON="0 0 * * *"
  fi
  # Setup crontab
  echo "Setting crontab with CRON as ${CRON}"
  crontab -l | { cat; echo "${CRON} salvage > /proc/1/fd/1 2>/proc/1/fd/2"; } | crontab -

  crond -f
fi
