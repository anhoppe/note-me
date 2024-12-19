# Stage 1: Build the Rust binary
FROM rust:1.74-slim-bullseye AS builder
WORKDIR /app
COPY note-me-backend ./
RUN cargo fetch
COPY src ./src
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/note-me-backend /app/note-me-backend
COPY --from=builder /app/target/release/deps /app/deps # Copy dependencies
EXPOSE 3000
CMD ["./note-me-backend"]