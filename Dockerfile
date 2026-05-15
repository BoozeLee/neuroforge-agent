FROM rust:1.78-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --bin neuroforge

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/neuroforge .
COPY keys/ keys/
EXPOSE 8403
ENV RUST_LOG=info
CMD ["./neuroforge", "--serve", "--port", "8403"]
