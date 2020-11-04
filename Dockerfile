
FROM rust

WORKDIR /usr/quoridor

# Download all the dependencies
RUN rustup component add clippy
