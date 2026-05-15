FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY neuroforge .
RUN mkdir -p keys
EXPOSE 8403
ENV RUST_LOG=info
CMD ["./neuroforge", "--serve", "--port", "8403", "--demo"]
