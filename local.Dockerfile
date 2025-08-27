FROM rust:slim-bookworm AS chef
WORKDIR /app
RUN apt update
RUN apt install -y libssl-dev pkg-config
RUN cargo install cargo-chef

FROM chef AS planner
COPY ./backend_axum backend_axum
COPY ./database_migrations ./database_migrations
COPY ./database_seed ./database_seed
COPY ./Cargo.* .

RUN cargo chef prepare --recipe-path recipe.json --bin backend_axum

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin backend_axum
COPY ./backend_axum backend_axum
COPY ./Cargo.* .
RUN cargo build --release --bin backend_axum

FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/backend_axum backend_axum
EXPOSE 8080
ENTRYPOINT ["/app/backend_axum"]
