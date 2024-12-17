FROM ubuntu
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata openssl git\
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY  target/release/SSHServer ./
COPY config.yaml .
COPY ed25519 .
EXPOSE 2222
CMD ["/app/SSHServer"]