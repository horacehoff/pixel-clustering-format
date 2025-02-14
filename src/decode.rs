use lzma_rust::{LZMA2Options, LZMA2Reader};
use std::fs::File;
use std::io::Read;

pub fn decode(path: String) {
    let mut input = File::open(path).unwrap();
    let mut contents = vec![];
    input.read_to_end(&mut contents).unwrap();

    let mut options = LZMA2Options::with_preset(6);
    options.dict_size = LZMA2Options::DICT_SIZE_DEFAULT;

    let mut decompressed = Vec::new();

    let mut r = LZMA2Reader::new(&contents[..], options.dict_size, None);
    r.read_to_end(&mut decompressed).unwrap();

    println!("RESULT {}", String::from_utf8(decompressed).unwrap())
}