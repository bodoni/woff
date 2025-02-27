FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /source
COPY . .

RUN cargo install --all-features --path . --root target/install

FROM scratch

COPY --from=builder /source/target/install /
COPY --from=builder /source/tests/fixtures/Roboto-Regular.ttf .
RUN woff Roboto-Regular.ttf Roboto-Regular.woff2
