FROM rust:1.79-slim as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked soroban-cli

WORKDIR /app
COPY contracts .

# build contrato wasm
RUN cargo build --release --target wasm32-unknown-unknown

# imagem final minimalista só com o .wasm
FROM debian:bullseye-slim
WORKDIR /contracts
COPY --from=builder /app/poap_badge/target/wasm32-unknown-unknown/release/poap_badge.wasm .
CMD ["ls", "-lh", "/contracts"]
