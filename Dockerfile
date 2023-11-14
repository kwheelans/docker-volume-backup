FROM lukemathwalker/cargo-chef:latest-rust-alpine as chef

FROM chef AS planner
WORKDIR /recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /salvage
RUN apk --no-cache add build-base

# Build dependencies
COPY --from=planner /recipe/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY ./ .
RUN cargo build --release

# Final image
FROM alpine
LABEL ca.wheelans.salvage="true"

RUN apk add --no-cache dcron xz tzdata && mkdir /salvage
WORKDIR /salvage

ENV PATH=/salvage:$PATH \
 SALVAGE_IS_DOCKER="true" \
 CRON="0 0 * * *"

ADD src/shell/*.sh /salvage/

COPY --from=builder /salvage/target/release/salvage /salvage

CMD ["/bin/sh","setup.sh"]
