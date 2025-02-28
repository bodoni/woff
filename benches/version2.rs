#![feature(test)]

extern crate test;

use test::Bencher;

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[bench]
fn compress(bencher: &mut Bencher) {
    let data = ok!(std::fs::read("tests/fixtures/Roboto-Regular.ttf"));
    bencher.iter(|| ok!(woff::version2::compress(&data, 8, "", true)));
}

#[bench]
fn decompress(bencher: &mut Bencher) {
    let data = ok!(std::fs::read("tests/fixtures/Roboto-Regular.ttf.woff2"));
    bencher.iter(|| ok!(woff::version2::decompress(&data)));
}
