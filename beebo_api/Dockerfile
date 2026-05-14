FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /api

FROM chef AS planner
COPY ./beebo_api/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /api/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./beebo_api/ .
RUN cargo build --release --bin beebo-notes-rs

FROM debian:trixie-slim AS runtime

ARG GIT_USER
ARG GIT_EMAIL

RUN apt update && apt install -y \
    ca-certificates \
    git \
    openssh-client

RUN mkdir -p /root/.ssh && \
    ssh-keyscan github.com >> /root/.ssh/known_hosts

RUN git config --global --add safe.directory /api \
      && git config --global user.email "$GIT_EMAIL"\
        && git config --global user.name "$GIT_USER"

RUN echo "Host github.com\n\
  IdentityFile /root/.ssh/beebo/id_ed25519\n\
  IdentitiesOnly yes" > /root/.ssh/config

WORKDIR /api
COPY --from=builder /api/target/release/beebo-notes-rs /usr/local/bin
ENTRYPOINT ["/usr/local/bin/beebo-notes-rs"]
