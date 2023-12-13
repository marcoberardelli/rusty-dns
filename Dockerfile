FROM rust:latest as builder


RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev musl-tools musl-dev gcc-x86-64-linux-gnu pkg-config

WORKDIR /app

COPY . .
RUN cargo build --release

CMD [ "./target/release/rusty-dns" ]

#FROM debian:bullseye-slim


#RUN apt-get update && apt-get install -y libc6-dev

#WORKDIR /app

#COPY --from=builder /app/target/release/rusty-dns .

#CMD [ "./rusty-dns" ]