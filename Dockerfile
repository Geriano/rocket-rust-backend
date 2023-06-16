FROM rustlang/rust:nightly
WORKDIR /app
COPY . /app
EXPOSE 8000
RUN rm -rf target
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch