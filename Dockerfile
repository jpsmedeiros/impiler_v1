FROM rust:1.28.0

WORKDIR /src
COPY . .

RUN cargo install

CMD cargo run