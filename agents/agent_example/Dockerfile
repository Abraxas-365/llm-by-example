# Use the official Rust image from the Docker Hub
FROM rust:latest

# Create a new a directory in your image filesystem
WORKDIR /usr/src/agent_example

# Copy your source code into the image
COPY . .

# Build your application
# --release flag is for to build it in release mode which is optimized and smaller.
RUN set -eux; \
    cargo build --release; 


# Specify the binary to run when the image is run as a container
CMD ["./target/release/agent_example"]
