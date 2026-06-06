FROM rust:latest

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    cmake \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy

RUN cargo fmt --all -- --check

RUN cargo clippy --all-targets --all-features -- \
    -W clippy::all \
    -W clippy::pedantic \
    -W clippy::nursery \
    -D warnings

    RUN cargo test --all -- -q

    CMD ["cargo", "test", "--all"]
