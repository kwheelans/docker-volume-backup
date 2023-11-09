FROM rust:alpine AS builder
RUN apk --no-cache add build-base
WORKDIR /docker-volume-backup

COPY ./ .

RUN cargo build --release

# Final image
FROM alpine

RUN apk add --no-cache bash dcron xz tzdata && mkdir /docker-volume-backup
WORKDIR /docker-volume-backup

ENV COMPRESS="gz" \
 STRATEGY="multiple" \
 CRON="0 0 * * *" \
 TZ="UTC" \
 PREFIX="docker-volume-backup" \
 PERMISSION="644"

ADD src/shell/*.sh /docker-volume-backup/

COPY --from=builder /docker-volume-backup/target/release/docker-volume-backup /docker-volume-backup

ENTRYPOINT ["/bin/bash", "setup-cron.sh"]
