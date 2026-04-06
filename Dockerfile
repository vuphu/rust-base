FROM rust:slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
LABEL org.opencontainers.image.source=https://github.com/vuphu/rust-base
COPY --from=builder /app/target/release/app .
EXPOSE 3000
ENTRYPOINT ["/app"]
