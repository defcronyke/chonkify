# --- build image ---
FROM rust:latest as build

# Copy local code to the container image.
WORKDIR /
COPY . .

# Install production dependencies and build a release artifact.
RUN echo $PWD && \
	ls -al && \
	cargo build --release && \
	echo $PWD && \
	ls -al


# --- production image ---
FROM rust:slim

WORKDIR /

RUN echo $PWD && \
	ls -al

COPY --from=build target/release/chonkify .

RUN echo $PWD && \
	ls -al

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
CMD ["./chonkify"]
