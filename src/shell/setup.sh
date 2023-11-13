#!/usr/bin/env sh

salvage --validate
RTN_CODE=$?

if [ ! $RTN_CODE = 0 ]; then
  echo $RTN_CODE
  exit $RTN_CODE
fi
echo "$SALVAGE_RUN_ONCE"
if [ "$SALVAGE_RUN_ONCE" = "true" ]; then
  salvage
else
  # Check cron variable
  if [ -z "$CRON" ]; then
    CRON="0 0 * * *"
  fi
  # Setup crontab
  echo "Setting crontab with CRON as ${CRON}"
  crontab -l | { cat; echo "${CRON} salvage > /proc/1/fd/1 2>/proc/1/fd/2"; } | crontab -

  crond -f
fi
