FROM rust:latest

WORKDIR /app

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

COPY . .

# Default to running with cargo watch
CMD ["cargo", "watch", "-x", "run"]
