FROM rust:alpine AS builder
RUN apk --no-cache add build-base
WORKDIR /salvage

COPY ./ .

RUN cargo build --release

# Final image
FROM alpine

RUN apk add --no-cache dcron xz tzdata && mkdir /salvage
WORKDIR /salvage

ENV PATH=/salvage:$PATH \
 COMPRESS="gz" \
 STRATEGY="multiple" \
 CRON="0 0 * * *" \
 TZ="UTC" \
 PREFIX="salvage-backup" \
 GROUP_PERMISSION="read" \
 OTHER_PERMISSION="read"

ADD src/shell/*.sh /salvage/

COPY --from=builder /salvage/target/release/salvage /salvage

ENTRYPOINT ["/bin/sh","setup-cron.sh"]
