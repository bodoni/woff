#![feature(test)]

extern crate test;

use test::Bencher;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[bench]
fn compress(bencher: &mut Bencher) {
    bencher.iter(|| {
        ok!(woff::version2::convert(
            "tests/fixtures/Roboto-Regular.ttf",
            "tests/fixtures/Roboto-Regular.woff2",
            None,
            None,
            None,
        ));
    });
}

#[bench]
fn decompress(bencher: &mut Bencher) {
    bencher.iter(|| {
        ok!(woff::version2::convert(
            "tests/fixtures/Roboto-Regular.woff2",
            "tests/fixtures/Roboto-Regular.otf",
            None,
            None,
            None,
        ));
    });
}
