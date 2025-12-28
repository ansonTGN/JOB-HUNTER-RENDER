# Etapa 1: Builder
FROM rust:1.84-slim-bookworm as builder
WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# Etapa 2: Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copiamos el binario con el nombre definido en Cargo.toml
COPY --from=builder /usr/src/app/target/release/job_hunter_spain .
COPY --from=builder /usr/src/app/templates ./templates

# El puerto se define por variable de entorno PORT en Render
EXPOSE 3000

CMD ["./job_hunter_spain"]