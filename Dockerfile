
FROM rust

WORKDIR /usr/quoridor

# Download all the dependencies
RUN rustup component add clippy
RUN rustup component add rustfmt

COPY quoridor/Cargo.lock /usr/quoridor/
COPY quoridor/Cargo.toml /usr/quoridor/
COPY quoridor/src/ /usr/quoridor/src/

RUN cargo fetch
