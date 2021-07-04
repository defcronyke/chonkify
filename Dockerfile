# --- build image ---
FROM rust:latest as build

# Copy local code to the container image.
WORKDIR /
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo build --release


# --- production image ---
FROM rust:slim

WORKDIR /

COPY --from=build target/release/chonkify .

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
CMD ["./chonkify"]
