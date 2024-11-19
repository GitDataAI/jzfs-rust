FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --bin jzfs-api --release

FROM ubuntu
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl\
  && rm -rf /var/lib/apt/lists/*


WORKDIR /app
COPY --from=builder /app/target/release/jzfs-api ./
COPY ./config ./config

EXPOSE 34513
CMD ["/app/jzfs-api"]