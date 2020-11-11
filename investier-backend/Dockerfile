FROM rust:1.47 AS developer

RUN cargo install cargo-watch
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build

COPY ./src src

RUN rm ./target/debug/deps/investier*
CMD cargo run

ENV PORT=8081
ENV FRONTEND_ORIGIN=http://localhost:8080
EXPOSE 8081

FROM ekidd/rust-musl-builder:stable AS builder

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release

COPY ./src src

RUN cargo build --release && \
    strip /home/rust/src/target/x86_64-unknown-linux-musl/release/investier

FROM scratch

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/investier /

CMD ["/investier"]
