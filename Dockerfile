FROM rust:1.69-slim AS debian
RUN apt-get update && apt-get install -y upx-ucl
USER nobody
WORKDIR /opt/swarmcret
COPY --chown=nobody . ./
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/swarmcret

FROM rust:1.69.0-alpine3.17 AS alpine
RUN apk add upx
USER nobody
WORKDIR /opt/swarmcret
COPY --chown=nobody . ./
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/swarmcret

FROM scratch AS production
COPY --from=debian /opt/swarmcret/target/release/swarmcret /swarmcret
COPY --from=alpine /opt/swarmcret/target/release/swarmcret /swarmcret-alpine
