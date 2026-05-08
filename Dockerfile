FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /api

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /api/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin beebo-notes-rs

FROM debian:trixie-slim AS runtime
WORKDIR /api
COPY --from=builder /api/target/release/beebo-notes-rs /usr/local/bin
ENTRYPOINT ["/usr/local/bin/beebo-notes-rs"]
