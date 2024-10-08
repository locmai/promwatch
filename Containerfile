FROM rust:bullseye as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo 'fn main() {}' > src/main.rs

RUN cargo build --release 

COPY src ./src

RUN cargo build --release 

FROM debian:bullseye 

WORKDIR /app

COPY --from=builder /app/target/release/promwatch promwatch

EXPOSE 8080

CMD ["/app/promwatch"]
