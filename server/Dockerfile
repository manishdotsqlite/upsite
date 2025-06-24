# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.86.0
ARG APP_NAME=server

# ---- BUILD STAGE ----
FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME

WORKDIR /app

# Install required build packages
RUN apk add --no-cache clang lld musl-dev git

# Copy the source code
COPY . .

# Build the Rust binary in release mode
RUN cargo build --release && \
    cp ./target/release/$APP_NAME /bin/server

# ---- FINAL STAGE ----
FROM alpine:3.18 AS final

ARG UID=10001

# Create a non-root user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Use non-root user
USER appuser

# Copy the built binary
COPY --from=build /bin/server /bin/

# Expose port
EXPOSE 8000

# Run the server
CMD ["/bin/server"]
