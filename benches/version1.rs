#![feature(test)]

extern crate test;

use test::Bencher;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[bench]
fn compress(bencher: &mut Bencher) {
    let data = ok!(std::fs::read("tests/fixtures/Roboto-Regular.ttf"));
    bencher.iter(|| ok!(woff::version1::compress(&data, 1, 0)));
}

#[bench]
fn decompress(bencher: &mut Bencher) {
    let data = ok!(std::fs::read("tests/fixtures/Roboto-Regular.ttf.woff"));
    bencher.iter(|| ok!(woff::version1::decompress(&data)));
}
