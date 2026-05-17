FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

RUN apt update && apt install -y\
    protobuf-compiler 

WORKDIR /api

FROM chef AS planner
COPY ./beebo_api/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /api/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./beebo_api/ .
RUN cargo build --release --bin beebo_api

FROM debian:trixie-slim AS runtime

ARG GIT_USER
ARG GIT_EMAIL

RUN apt update && apt install -y \
    ca-certificates \
    git \
    openssh-client

RUN adduser --uid 1000 --disabled-password beebo

RUN mkdir -p /home/beebo/.ssh

RUN ssh-keyscan github.com >> /home/beebo/.ssh/known_hosts

RUN echo "Host github.com\n\
  IdentityFile /home/beebo/.ssh/beebo/id_ed25519\n\
  IdentitiesOnly yes" > /home/beebo/.ssh/config

RUN chown -R beebo:beebo /home/beebo/.ssh

WORKDIR /api

COPY --from=builder /api/target/release/beebo_api /usr/local/bin/beebo_api

RUN chmod +x /usr/local/bin/beebo_api

USER beebo

RUN git config --global --add safe.directory /api \
    && git config --global user.email "$GIT_EMAIL" \
    && git config --global user.name "$GIT_USER"



ENTRYPOINT ["/usr/local/bin/beebo_api"]
