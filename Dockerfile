# ---------- BUILD STAGE ----------
FROM rust:1.93-alpine3.20 AS builder
WORKDIR /app

# Cache deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source
COPY src ./src
COPY migrations ./migrations
ENV SQLX_OFFLINE=true
RUN cargo build --release

# ---------- RUNTIME STAGE ----------
FROM busybox:1.37.0-glibc

COPY --from=builder /app/target/release/tudu /app

RUN mkdir -p /data

EXPOSE 3000
ENTRYPOINT ["/app"]
