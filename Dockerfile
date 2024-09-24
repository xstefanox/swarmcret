FROM rust:1.81-slim-bullseye AS build-default
RUN apt-get update && apt-get install -y upx-ucl
USER nobody
WORKDIR /opt/swarmcret
COPY --chown=nobody . ./
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/swarmcret

FROM rust:1.81.0-alpine3.20 AS build-alpine
RUN apk add upx
USER nobody
WORKDIR /opt/swarmcret
COPY --chown=nobody . ./
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/swarmcret

FROM scratch AS default
COPY --from=build-default /opt/swarmcret/target/release/swarmcret /swarmcret

FROM scratch AS alpine
COPY --from=build-alpine /opt/swarmcret/target/release/swarmcret /swarmcret
