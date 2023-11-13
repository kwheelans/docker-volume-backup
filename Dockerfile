FROM rust:alpine AS builder
RUN apk --no-cache add build-base
WORKDIR /salvage

COPY ./ .

RUN cargo build --release

# Final image
FROM alpine
LABEL ca.wheelans.salvage="true"

RUN apk add --no-cache dcron xz tzdata && mkdir /salvage
WORKDIR /salvage

ENV PATH=/salvage:$PATH \
 CRON="0 0 * * *"

ADD src/shell/*.sh /salvage/

COPY --from=builder /salvage/target/release/salvage /salvage

CMD ["/bin/sh","setup.sh"]
