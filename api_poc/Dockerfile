# Rust multi-stage image
# See https://dev.to/mattdark/rust-docker-image-optimization-with-multi-stage-builds-4b6c

####################################################################################################
## Builder
####################################################################################################
# use official rust image for building rust app
# see https://hub.docker.com/_/rust
FROM rust:1.80 AS builder
WORKDIR /app

# download and build app dependencies (this enables layer caching)
COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# copy real app source data and mark main.rs as modified and then build full app
COPY src src
RUN touch src/main.rs
RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
# see https://github.com/GoogleContainerTools/distroless/blob/main/examples/rust/Dockerfile
# image runs as non-root user by default
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/api /

# default port is defined in code
EXPOSE 8000

CMD ["./api"]