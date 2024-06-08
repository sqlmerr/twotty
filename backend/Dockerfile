FROM lukemathwalker/cargo-chef:latest-rust-1.78.0 as chef
WORKDIR /app

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime


COPY --from=builder /app/.env .env
COPY --from=builder /app/target/release/axum-template axum-template

EXPOSE 8000
CMD ["./axum-template"]