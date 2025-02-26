ARG APP_NAME={data_loader}
FROM rust:latest AS builder

WORKDIR /usr/src/${APP_NAME}
COPY src .
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo install --path .

FROM postgres:latest

COPY csv .
# Postgres init scripts
COPY docker/initdb /docker-entrypoint-initdb.d
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/bin/${APP_NAME}
