# WOFF [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a converter for Web Open Font Format.

## Usage

```shell
cargo install --features binary woff

woff Roboto-Regular.ttf Roboto-Regular.woff
woff Roboto-Regular.woff Roboto-Regular.ttf

woff Roboto-Regular.ttf Roboto-Regular.woff2
woff Roboto-Regular.woff2 Roboto-Regular.ttf
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/bodoni/woff/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/woff/actions/workflows/build.yml
[documentation-img]: https://docs.rs/woff/badge.svg
[documentation-url]: https://docs.rs/woff
[package-img]: https://img.shields.io/crates/v/woff.svg
[package-url]: https://crates.io/crates/woff
