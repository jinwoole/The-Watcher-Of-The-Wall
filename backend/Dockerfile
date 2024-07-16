# 빌드 스테이지
FROM rust:1.75-buster as builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y libpq-dev

COPY ./actix/Cargo.toml ./actix/Cargo.lock ./

COPY ./actix/src ./src

RUN cargo build --release

# 실행 스테이지
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/auth-only .

EXPOSE 8080

CMD ["./auth-only"]