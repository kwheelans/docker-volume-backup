#!/usr/bin/env bash

# Check compression variable
case "${COMPRESS,,}" in
  'gz' | 'xz' | 'bz2' | 'lzma')
  echo "Compressing with ${COMPRESS,,}"
  ;;

  *)
   COMPRESS='gz'
   echo "TYPE value no recognized, setting to ${COMPRESS,,}"
   ;;
esac

# Check type variable
case "${TYPE,,}" in
'single' | 'multi')
  echo "Archive type set to ${TYPE,,}"
  ;;

*)
  TYPE='multi'
  echo "TYPE value not recognized, setting to ${TYPE,,}"
  ;;
esac

# Check cron variable
if [[ -z "$CRON" ]]; then
  CRON="0 0 * * *"
fi

# Checking that expected directory mounts are in place
if [[ ! -d "/backup" ]]; then
  echo "No volume mounted at /backup"
  exit 1
elif [[ ! -d "/data" ]]; then
  echo "Nothing mounted under /data"
  exit 1
fi

# Setup crontab
echo "Setting crontab with CRON as ${CRON}"
crontab -l | { cat; echo "${CRON} bash /script/backup.sh"; } | crontab -

crond -f
