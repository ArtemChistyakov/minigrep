#![feature(test)]
extern crate test;

use test::Bencher;

use minigrep;
use minigrep::Config;

#[bench]
fn bench_run(b: &mut Bencher) {
    let config = Config {
        query: String::from("to"),
        filename: String::from("voina_i_mir.txt"),
        case_sensitive: false,
    };

    b.iter(|| {
        minigrep::run(&config)
    });
}

#[bench]
fn bench_multithreading_run(b: &mut Bencher) {
    b.iter(|| {
        let config = Config {
            query: String::from("to"),
            filename: String::from("voina_i_mir.txt"),
            case_sensitive: false,
        };
        minigrep::multithreading_run(config)
    });
}

