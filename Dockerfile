FROM messense/rust-musl-cross:x86_64-musl

WORKDIR /source
COPY . .

RUN cargo test
