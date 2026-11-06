FROM rust:1.85-bookworm AS builder
RUN rustup toolchain install 1.98.1 --profile minimal --target wasm32-unknown-unknown
WORKDIR /app
COPY . .
RUN cargo +1.98.1 install dioxus-cli --locked
RUN cargo +1.98.1 dx build --release --package mw-web

FROM nginx:1.27-alpine
COPY --from=builder /app/target/dx/mw-web/release/web/public /usr/share/nginx/html
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 CMD wget -q --spider http://127.0.0.1/ || exit 1
