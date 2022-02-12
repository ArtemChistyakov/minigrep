#![feature(test)]
extern crate test;

use test::Bencher;

use minigrep;
use minigrep::Config;

#[bench]
fn empty(b: &mut Bencher) {
    let config = Config {
        query: String::from("to"),
        filename: String::from("voina_i_mir.txt"),
        case_sensitive: false,
    };

    b.iter(|| {
        minigrep::run(&config)
    });
}
