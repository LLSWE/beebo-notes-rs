FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

RUN apt update && apt install -y protobuf-compiler libprotobuf-dev python3-protobuf

WORKDIR /app

FROM chef AS planner
COPY ./beebo_cron_worker/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./beebo_cron_worker/ .
RUN cargo build --release --bin beebo_cron_worker 

FROM debian:trixie-slim AS runtime


WORKDIR /app
COPY --from=builder /app/target/release/beebo_cron_worker /usr/local/bin
ENTRYPOINT ["/usr/local/bin/beebo_cron_worker"]
