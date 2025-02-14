use ahash::HashMap;
use lzma_rust::{LZMA2Options, LZMA2Reader};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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
    let mut colors: Vec<String> = result.split('%').map(|x| x.to_string()).collect();
    let width: u32 = colors.remove(0).parse().unwrap();
    let height: u32 = colors.remove(0).parse().unwrap();
    let bg_color = colors.remove(0);


    // if some letters need to be replaced, replace them
    if colors.last().unwrap().contains("$") {
        let replacements = colors.remove(colors.len() - 1);
        let mut parts: Vec<&str> = replacements.split("$").collect();
        parts.retain(|x| !x.is_empty());
        let couples: Vec<(String, String)> = parts.iter().zip(parts.iter().skip(1)).map(|(a, b)| (b.to_string(), a.to_string())).collect();
        for (by, to) in couples {
            colors = colors.par_iter().map(|x| x.replace(&by, &to)).collect();
        }
    }

    let mut colors: Vec<String> = colors.remove(0).split("#").map(|x| x.to_string()).collect();
    colors.retain(|x| !x.is_empty());
    println!("COLORS{colors:?}");
    for x in colors.iter_mut() {
        let mut is_y = false;
        if x.ends_with("y") {
            is_y = true;
            x.pop().unwrap();
        }
        let split: Vec<&str> = x.split('{').into_iter().collect();
        let color = format!("#{}", split[0]);
        let mut pixels = format!("{{{}", split[1]);
        // VERY BAD CODE BUT IT WORKS
        for (loc, (i, _)) in pixels.clone().match_indices(':').enumerate() {
            pixels.insert(loc + i, '"');
        }
        for (loc, (i, _)) in pixels.clone().match_indices(':').enumerate() {
            pixels.insert(loc + i + 1, '"');
        }
        for (loc, (i, _)) in pixels.clone().match_indices(',').enumerate() {
            pixels.insert(loc + i, '"');
        }
        for (loc, (i, _)) in pixels.clone().match_indices(',').enumerate() {
            pixels.insert(loc + i + 1, '"');
        }
        pixels.insert(1, '"');
        pixels.insert(pixels.len() - 1, '"');

        let parsed = json::parse(&pixels).unwrap();
        let mut working: HashMap<String, String> = Default::default();
        for x in parsed.entries() {
            working.insert(x.0.to_string(), x.1.as_str().unwrap().to_string());
        }
        println!("{working:?}\n\n");
    }


}