FROM alpine:latest

RUN apk update && apk upgrade
RUN apk add bash dcron xz

RUN mkdir /script
ADD src/setup-cron.sh /script/
ADD src/backup.sh /script/

ENV COMPRESS="gz"
ENV TYPE="multi"
ENV CRON="0 0 * * *"

WORKDIR /script
ENTRYPOINT ["/bin/bash", "setup-cron.sh"]
