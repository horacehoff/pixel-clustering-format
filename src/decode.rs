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

    let result = String::from_utf8(decompressed).unwrap();
    println!("{result}");
    let mut colors: Vec<&str> = result.split('%').collect();
    let width: u32 = colors.remove(0).parse().unwrap();
    let height: u32 = colors.remove(0).parse().unwrap();

    if colors.last().unwrap().contains("$") {
        let replacements = colors.remove(colors.len() - 1);
        let mut parts: Vec<&str> = replacements.split("$").collect();
        parts.retain(|x| !x.is_empty());
        let couples: Vec<(String, String)> = parts.iter().zip(parts.iter().skip(1)).map(|(a, b)| (b.to_string(), a.to_string())).collect();
        for (by, to) in couples {
            colors.iter().map(|x| x);
        }
        //println!("RPEL {couples:?}");
    }
}