ARG BASE_IMAGE=ekidd/rust-musl-builder:nightly-2019-06-08
FROM ${BASE_IMAGE} AS builder
ADD . ./
RUN sudo chown -R rust:rust /home/rust
RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-rest \
    /usr/local/bin/
WORKDIR /app/
CMD /usr/local/bin/rust-rest
