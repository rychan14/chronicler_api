FROM ekidd/rust-musl-builder AS build
COPY . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM scratch
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/pocket-pick /
EXPOSE ${PORT}
CMD ["/pocket-pick"]