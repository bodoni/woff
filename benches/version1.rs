#![feature(test)]

extern crate test;

use test::Bencher;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[bench]
fn compress(bencher: &mut Bencher) {
    bencher.iter(|| {
        ok!(woff::version1::convert(
            "tests/fixtures/Roboto-Regular.ttf",
            "tests/fixtures/Roboto-Regular.ttf.woff",
        ));
    });
}

#[bench]
fn decompress(bencher: &mut Bencher) {
    bencher.iter(|| {
        ok!(woff::version1::convert(
            "tests/fixtures/Roboto-Regular.ttf.woff",
            "tests/fixtures/Roboto-Regular.ttf",
        ));
    });
}
