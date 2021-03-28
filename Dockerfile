FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application! 
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY . .
ENV SQLX_OFFLINE true
# Build our application, leveraging the cached deps!
RUN cargo build --release --bin food-backend

FROM debian:buster-slim AS runtime
WORKDIR app

COPY --from=builder /app/target/release/food-backend food-backend
ENTRYPOINT ["./food-backend"]
