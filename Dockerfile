FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /source
COPY . .

RUN cargo install --features binary --path . --root target/install
RUN ldd target/install/bin/woff
RUN target/install/bin/woff tests/fixtures/Roboto-Regular.ttf Roboto-Regular.woff
RUN target/install/bin/woff tests/fixtures/Roboto-Regular.ttf Roboto-Regular.woff2

FROM scratch

COPY --from=builder /source/target/install /
COPY --from=builder /source/tests/fixtures/Roboto-Regular.ttf .
RUN ["woff", "Roboto-Regular.ttf", "Roboto-Regular.woff"]
RUN ["woff", "Roboto-Regular.ttf", "Roboto-Regular.woff2"]

ENTRYPOINT ["/bin/woff"]
