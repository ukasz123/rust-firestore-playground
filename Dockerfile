# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:1.27.0

# Firebase project id.
ARG project_id

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# Install production dependencies and build a release artifact.
RUN cargo install

ENV PROJECT_ID project_id

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
CMD ["rust-firestore-playground"]
