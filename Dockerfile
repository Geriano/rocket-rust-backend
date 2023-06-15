FROM rustlang/rust:nightly
WORKDIR /app
COPY . /app
EXPOSE 8000
RUN rm -rf target
RUN cargo install cargo-watch