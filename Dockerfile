FROM ubuntu:22.04 AS builder

RUN apt update && \
    apt install -y curl gcc musl-dev openssl libssl-dev pkg-config cmake \
    && rm -rf /var/lib/apt/lists/*

RUN curl -o /usr/local/bin/rustup-init --proto '=https' --tlsv1.2 -sSf  https://rsproxy.cn/rustup-init.sh
RUN chmod +x /usr/local/bin/rustup-init
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
RUN /usr/local/bin/rustup-init --default-toolchain nightly -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

WORKDIR /app

COPY --from=builder /app/target/release/GitDataOS .
CMD ["/app/GitDataOS"]
