# Stage 1: Build
FROM rust:1.96-alpine AS build


COPY Cargo.toml ./
COPY abi ./abi
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Minimal production image
FROM scratch
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=build /app/target/release/token_scanner_eth /server
EXPOSE 8080
ENTRYPOINT ["/server"]