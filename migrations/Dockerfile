FROM rust:1.31
ADD . /app/migration
WORKDIR /app
RUN cargo install --force --root tmp --no-default-features --features postgres,sqlite diesel_cli
CMD diesel migration run
