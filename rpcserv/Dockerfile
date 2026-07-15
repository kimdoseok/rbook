ARG RUST_VERSION=1.75
ARG APP_NAME=grpc-server

################################################################################
# Build stage
################################################################################
FROM rust:${RUST_VERSION}-alpine AS build

ARG APP_NAME
WORKDIR /app

# Install build dependencies for Alpine
RUN apk add --no-cache clang lld musl-dev git protobuf-dev

# Build the application with dependency caching
RUN --mount=type=bind,source=src,target=src \
--mount=type=bind,source=Cargo.toml,target=Cargo.toml \
--mount=type=bind,source=Cargo.lock,target=Cargo.lock \
--mount=type=cache,target=/app/target/ \
--mount=type=cache,target=/usr/local/cargo/git/db \
--mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

################################################################################
# Runtime stage
################################################################################
FROM alpine:3.18

# Create non-privileged user
ARG UID=10001
RUN adduser \
--disabled-password \
--gecos "" \
--home "/nonexistent" \
--shell "/sbin/nologin" \
--no-create-home \
--uid "${UID}" \
appuser

# Copy binary from build stage
COPY --from=build /bin/server /bin/

# gRPC typically uses port 50051
EXPOSE 50051

# Drop privileges
USER appuser

# Run the service
CMD ["/bin/server"]
