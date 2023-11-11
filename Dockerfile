FROM rust:alpine AS builder
RUN apk --no-cache add build-base
WORKDIR /docker-volume-backup

COPY ./ .

RUN cargo build --release

# Final image
FROM alpine

RUN apk add --no-cache dcron xz tzdata && mkdir /docker-volume-backup
WORKDIR /docker-volume-backup

ENV PATH=/docker-volume-backup:$PATH \
 COMPRESS="gz" \
 STRATEGY="multiple" \
 CRON="0 0 * * *" \
 TZ="UTC" \
 PREFIX="docker-volume-backup" \
 GROUP_PERMISSION="read" \
 OTHER_PERMISSION="read"

ADD src/shell/*.sh /docker-volume-backup/

COPY --from=builder /docker-volume-backup/target/release/docker-volume-backup /docker-volume-backup

ENTRYPOINT ["/bin/sh","setup-cron.sh"]
