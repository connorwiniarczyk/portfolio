from rust:latest as builder

run git clone https://github.com/connorwiniarczyk/serv.git
workdir serv

# Use the MUSL Libc target to compile the binary without dynamic linking
# idea from here:
# https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
run rustup target add x86_64-unknown-linux-musl
run rustup update
run cargo install --path . --target x86_64-unknown-linux-musl

from alpine:latest

copy --from=builder /usr/local/cargo/bin/serv /usr/bin/serv

workdir /www
copy . .

expose 80
cmd /usr/bin/serv -p 80
