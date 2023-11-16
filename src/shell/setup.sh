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
  # Set default SCHEDULE value
  if [ -z "$SCHEDULE" ]; then
    SCHEDULE="0 0 * * *"
  fi
  # Setup crontab
  echo "Setting crontab with SCHEDULE as ${SCHEDULE}"
  crontab -l | { cat; echo "${SCHEDULE} salvage > /proc/1/fd/1 2>/proc/1/fd/2"; } | crontab -

  crond -f
fi
