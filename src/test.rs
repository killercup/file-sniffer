#![allow(unused_imports)]
extern crate test;
extern crate regex;

use std::fs;
use regex::Regex;
use std::path::PathBuf;
use std::mem::replace;
use test::test::Bencher;
use prelude::*;
use gitignore::*;

#[bench]
fn bench_gitignore_parse(b: &mut Bencher) {
    let file_contents = include_str!("testdata/.gitignore");
    b.iter(|| process_to_vector(file_contents))
}

#[bench]
fn bench_gitignore(b: &mut Bencher) {
    let file_contents = include_str!("testdata/.gitignore");
    b.iter(|| file_contents_to_regex(file_contents))
}

#[bench]
fn bench_traversal(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, None, true, &None, false, false))
}

#[bench]
fn bench_traversal_gitignore(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, None, true, &None, true, true))
}
#[bench]
fn bench_traversal_sort (b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| { let v = read_all(&p, 4, None, None, None, true, &None, false, true); v.sort(None, 2) })
}

#[bench]
fn bench_traversal_artifacts(b: &mut Bencher) {
    let p = PathBuf::from("src/testdata");
    b.iter(|| read_all(&p, 4, None, None, None, true, &None, false, true))
}


#[bench]
fn bench_extension_regex(b: &mut Bencher) {
    let metadata = fs::metadata("src/main.rs").unwrap();
    b.iter(|| is_artifact("target/release/libdoggo.rlib", None, &metadata, &None) )
}

#[bench]
fn bench_extension_regex_long(b: &mut Bencher) {
    let metadata = fs::metadata("src/main.rs").unwrap();
    b.iter(|| is_artifact("./target/release/.fingerprint/kernel32-sys-5ee1259db1228dbc/build-script-build_script_build-5ee1259db1228dbc.json", 
                          None, &metadata, &None) )
}

#[bench]
fn bench_parser(b:&mut Bencher) {
    let cli_input = "1M";
    b.iter(|| threshold(Some(cli_input)))
}
