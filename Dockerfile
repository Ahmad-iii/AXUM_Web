# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy the static files
COPY static ./static

# Install necessary tools
RUN apt-get update && apt-get install -y gcc

# Build the application
RUN cargo build --release

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["./target/release/axum_web_app"]
