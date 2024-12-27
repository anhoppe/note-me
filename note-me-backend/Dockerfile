# Stage 1: Build the Rust binary
FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/note-me-backend /app/note-me-backend
COPY --from=builder /app/target/release/deps /app/deps
EXPOSE 8080
CMD ["./note-me-backend"]