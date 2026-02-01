FROM rust:1.93 as builder

WORKDIR /app
COPY backend backend
COPY shared shared
COPY Cargo.toml .

RUN cargo build --release -p data-comparer-backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/data-comparer-backend /usr/local/bin/app

ENV HOST=0.0.0.0
ENV PORT=3000

CMD ["app"]
