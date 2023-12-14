FROM rust:latest as builder


RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev musl-tools musl-dev gcc-x86-64-linux-gnu pkg-config

WORKDIR /app

COPY . .
RUN cargo build --release

CMD [ "./target/release/rusty-dns" ]
