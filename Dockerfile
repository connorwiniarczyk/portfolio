from rust:latest as builder
copy ./server/src /build/src
copy ./server/Cargo.toml /build/Cargo.toml
workdir /build
run cargo build --release

from debian:buster-slim
copy --from=builder /build/target/release/server /usr/bin/server

copy ./production_config.toml /etc/server/config.toml
copy ./Media.toml /etc/server/Media.toml

copy ./public /public

expose 80
cmd /usr/bin/server
