FROM rustlang/rust:nightly
ARG ROCKET_VERSION
ENV ROCKET_VERSION=v0.4.0

RUN rustup default nightly && rustup update

COPY . . 
RUN cargo build --release
EXPOSE 8000

CMD target/release/portfolio
