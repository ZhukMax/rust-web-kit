# Assembly stage
FROM rust:1.84-slim AS builder

WORKDIR /app

# Install dependencies and copy the project
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
COPY . .

# Building a binary file for the release version
RUN cargo build --release && strip target/release/rust-web-kit

# Final image
FROM gcr.io/distroless/cc

WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/release/rust-web-kit /app/rust-web-kit
COPY view /app/view
COPY static /app/static
COPY .env /app/.env

EXPOSE 8080

CMD ["/app/rust-web-kit"]
