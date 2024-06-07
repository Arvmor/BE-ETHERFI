FROM rust:latest AS rust-builder

WORKDIR /app

COPY ./ ./

# Build the Rust program for release
RUN cargo build --release

# Expose the port
EXPOSE 1337

CMD ["/bin/sh", "-c", "/app/target/release/backend"]