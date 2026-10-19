FROM rust:1 as BUILD

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

COPY --from=build /app/target/release/http-server ./http-server
COPY --from=build /app/src/server/public ./src/server/public

EXPOSE 8080

CMD ["./http-server"]