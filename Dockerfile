FROM alpine:latest

RUN apk update && apk upgrade
RUN apk add bash dcron xz tzdata

RUN mkdir /script
ADD src/setup-cron.sh /script/
ADD src/backup.sh /script/

ENV COMPRESS="gz"
ENV TYPE="multi"
ENV CRON="0 0 * * *"
ENV TZ="UTC"
ENV PREFIX="docker-backup-volume"
ENV PERMISSION="644"

WORKDIR /script
ENTRYPOINT ["/bin/bash", "setup-cron.sh"]
