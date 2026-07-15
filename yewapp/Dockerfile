ARG RUST_VERSION=1.75
ARG APP_NAME=yewapp

################################################################################
# Build stage
################################################################################
FROM rust:${RUST_VERSION}-alpine AS build

ARG APP_NAME
WORKDIR /app

# Install build dependencies for Alpine
RUN apk add --no-cache clang lld musl-dev git npm

# Install trunk (Yew's build tool)
RUN cargo install trunk

# Add WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html ./

# Build the Yew app (produces dist/ directory)
RUN trunk build --release

################################################################################
# Runtime stage (serve static files)
################################################################################
FROM alpine:3.18

# Install lightweight web server
RUN apk add --no-cache http-server

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

# Copy built assets from build stage
COPY --from=build /app/dist /app/dist

WORKDIR /app

EXPOSE 8080

USER appuser

# Serve static files
CMD ["http-server", "-p", "8080", "-g"]
