#![allow(unused_imports)]
extern crate test;

use std::path::PathBuf;
use std::mem::replace;
use test::test::Bencher;
use prelude::*;

// this one doesn't need any special considerations
#[bench]
fn bench_extension_regex(b: &mut Bencher) {
    let p = PathBuf::from("target/release/libdoggo.rlib");
    b.iter(|| is_artifact(&p, None) )
}

#[bench]
fn bench_parser(b:&mut Bencher) {
    let cli_input = "1M";
    b.iter(|| threshhold(Some(cli_input)))
}
