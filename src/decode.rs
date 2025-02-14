use ahash::HashMap;
use image::RgbaImage;
use lzma_rust::{LZMA2Options, LZMA2Reader};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::File;
use std::io::Read;

pub fn expand_math(s: String) -> String {
    let mut current = '0';
    let mut parts: Vec<String> = s.split('+').map(|x| x.to_string()).collect();
    let mut output: String = String::new();
    //println!("{parts:?}");
    for x in parts {
        if !x.contains("*") {
            output.push('+');
            output.push_str(&x);
        } else {
            let split: Vec<&str> = x.split('*').collect();
            let count: u32 = split[1].parse().unwrap();
            let number = split[0];
            for _ in 0..count {
                output.push('+');
                output.push_str(number);
            }
        }
    }
    output.trim_start_matches('+').to_string()
}

pub fn math_to_vec(s: String) -> Vec<String> {
    let splits: Vec<u32> = s.split('+').collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let mut output: Vec<String> = Vec::with_capacity(splits.len());
    let mut count: u32 = 0;
    for x in splits {
        count = count + x;
        output.push(count.to_string())
    }
    output
}

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
    let mut output = RgbaImage::from_pixel(width, height, image::Rgba(<[u8; 4]>::from(hex_color::HexColor::parse(&bg_color).unwrap().split_rgba())));


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

        for x in working.keys() {
            let expanded = expand_math(x.to_string());

            let expanded = math_to_vec(expanded);
            println!("{expanded:?}");
        }



        println!("{working:?}\n\n");
    }

    output.save("test.png").unwrap();
}