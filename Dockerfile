FROM rust:1.60-slim AS build
ENV CARGO_HOME=cargo
RUN apt-get update && apt-get install -y upx-ucl
USER nobody
WORKDIR /opt/swarmcret
COPY --chown=nobody . ./
RUN cargo build
RUN cargo test
RUN cargo build --release
RUN upx --best --lzma target/release/swarmcret

FROM scratch AS production
COPY --from=build /opt/swarmcret/target/release/swarmcret /swarmcret
