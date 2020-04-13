FROM ekidd/rust-musl-builder:latest AS build
COPY . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM alpine
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/chronicler /
EXPOSE ${PORT}
CMD ["/chronicler"]