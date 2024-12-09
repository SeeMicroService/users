FROM rust:1.83 AS builder
WORKDIR /users
COPY . .
RUN cargo build --release --bin users

FROM debian:bookworm-slim
WORKDIR /users
COPY --from=builder /users/target/release/users /usr/local/bin
CMD ["users"]