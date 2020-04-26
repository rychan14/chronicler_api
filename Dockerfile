FROM ekidd/rust-musl-builder:latest AS build
COPY . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release