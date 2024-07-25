# Use the private Rust image from docker.aleksiwork.com as the build environment
FROM docker.aleksiwork.com/rust:1.79.0-alpine3.20 as builder

# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Use a faster mirror for Alpine packages
RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories

# Install necessary dependencies for building the application
RUN apk add --no-cache musl-dev pkgconfig openssl-dev git

# Configure Cargo to use the USTC mirror for crates.io
RUN mkdir -p /root/.cargo && \
    echo '[source.crates-io]' > /root/.cargo/config && \
    echo 'replace-with = "ustc"' >> /root/.cargo/config && \
    echo '[source.ustc]' >> /root/.cargo/config && \
    echo 'registry = "https://mirrors.ustc.edu.cn/crates.io-index"' >> /root/.cargo/config

# Set the workdir and copy the source into it
WORKDIR /app
COPY .seajob-server /app

# Do a release build
RUN cargo build --release

# Use a plain Alpine image from the same private registry as the runtime environment
FROM docker.aleksiwork.com/alpine:3.20

# Install necessary runtime dependencies
RUN apk add --no-cache libgcc

# Copy the binary into the final image
WORKDIR /app
COPY --from=builder /app/target/release/seajob-server .

# Set the binary as entrypoint
ENTRYPOINT ["./seajob-server"]
