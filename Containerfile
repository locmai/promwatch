FROM rust:1.81.0 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/promwatch .

CMD ["./promwatch"]
