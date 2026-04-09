# ── Stage 1: Rust builder ─────────────────────────────────────────────────────
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /build

# Cache dependencies by building dummy src first
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null; rm -rf src/

# Real build
COPY src/ src/
RUN cargo build --release

# ── Stage 2: Minimal runtime ──────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl libgomp1 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/klother-measure ./klother-measure

# ONNX Runtime shared libs are bundled by the `ort` crate with download-binaries feature
COPY --from=builder /build/target/release/build/ort-*/out/lib* /usr/local/lib/
RUN ldconfig

ENV RUST_LOG="klother_measure=info,tower_http=warn"
ENV MODEL_DIR="/models"
ENV PORT="9090"

EXPOSE 9090

ENTRYPOINT ["./klother-measure"]
