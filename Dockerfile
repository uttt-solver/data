# Use a Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml to cache dependencies
COPY Cargo.toml ./

# Build the project with dummy files to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features "postgres"

# Copy the rest of your source code
COPY src ./src
COPY migrations ./migrations

# Set environment variables for PostgreSQL connection
ENV DATABASE_URL=postgres://uttt:supersecretpassword@db/uttt

# Define the startup command
CMD ["sh", "-c", "diesel migration run && cargo run --release"]
