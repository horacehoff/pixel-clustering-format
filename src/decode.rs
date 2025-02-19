use colored::Colorize;
use const_currying::const_currying;
use image::RgbaImage;
use indicatif::ProgressBar;
use mashi_core::Decoder;
use std::fs::File;
use std::io::Read;

pub fn expand_math(s: String) -> String {
    let parts: Vec<String> = s.split('+').map(|x| x.to_string()).collect();
    let mut output: String = String::new();
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
        count += x;
        output.push(count.to_string())
    }
    output
}

#[const_currying]
pub fn decode(path: String,
              #[maybe_const(dispatch = verbose, consts = [true, false])]verbose: bool) {
    let mut input = File::open(path.clone()).unwrap();
    let mut contents = vec![];
    input.read_to_end(&mut contents).unwrap();

    let mut result = String::new();

    //  check if contents is compressed
    let test = String::from_utf8(contents.clone()).unwrap_or(String::from(""));
    if test.contains("%") && test.contains("#") {
        result = test;
    } else {
        let mut decoder = Decoder::new();
        let decompressed = decoder.decode(&contents);
        result = String::from_utf8(decompressed.to_vec()).unwrap();
    }


    // if some letters need to be replaced, replace them
    if result.contains("_") {
        let matcher = result.split("_").collect::<Vec<&str>>();
        let mut output: String = matcher[0].to_string();
        let letters = matcher[1];

        let mut parts: Vec<&str> = letters.split("$").collect();
        parts.retain(|x| !x.is_empty());
        let couples: Vec<(String, String)> = parts.iter().zip(parts.iter().skip(1)).map(|(a, b)| (b.to_string(), a.to_string())).rev().collect();

        for (by, to) in couples {
            output = output.replace(&by, &to);
        }
        result = output;
    }


    let mut colors: Vec<String> = result.split('%').map(|x| x.to_string()).collect();
    let width: u32 = colors.remove(0).parse().unwrap();
    let height: u32 = colors.remove(0).parse().unwrap();
    let bg_color = colors.remove(0);
    let mut output = RgbaImage::from_pixel(width, height, image::Rgba(<[u8; 4]>::from(hex_color::HexColor::parse(&bg_color).unwrap().split_rgba())));


    let mut colors: Vec<String> = colors.remove(0).split("#").map(|x| x.to_string()).collect();
    colors.retain(|x| !x.is_empty());
    println!("Decoding {}...", path.blue());
    let bar = ProgressBar::new(colors.len() as u64);
    for x in colors.iter_mut() {
        let mut is_y = false;
        if x.ends_with("y") {
            is_y = true;
            x.pop().unwrap();
        }
        let split: Vec<&str> = x.split('{').collect();
        // if pixel list is just a plain array of tuples (x,y)
        if split.len() == 1 {
            let split: Vec<&str> = x.split('[').collect();
            let color = format!("#{}", split[0]);
            let mut pixels:Vec<&str> = split[1].trim_end_matches("]").split(")").collect();
            pixels = pixels.iter().map(|x| x.trim_start_matches("(").trim_start_matches(",").trim_end_matches(",").trim_end_matches(")").trim_start_matches("(").trim_start_matches(",").trim_end_matches(",").trim_end_matches(")")).collect();
            pixels.retain(|x| !x.is_empty());
            for pixel in pixels {
                let split: Vec<&str> = pixel.split(',').collect();
                let x = split[0].parse().unwrap();
                let y = split[1].parse().unwrap();
                output.put_pixel(x, y, image::Rgba(<[u8; 4]>::from(hex_color::HexColor::parse(&color.to_string()).unwrap().split_rgba())));
            }
            continue;
        }
        let color = format!("#{}", split[0]);
        let mut pixels = format!("{{{}", split[1]).replace(":", "\":\"").replace(",", "\",\"");
        pixels.insert(1, '"');
        pixels.insert(pixels.len() - 1, '"');

        let parsed = json::parse(&pixels).unwrap();
        for (x, val) in parsed.entries() {
            // de-group and expand each key
            let expanded = expand_math(x.to_string());
            let vecs = math_to_vec(expanded);
            // expand the value
            let expanded = expand_math(val.to_string());
            let vecs2 = math_to_vec(expanded);
            for y in vecs {
                for z in &vecs2 {
                    let color = image::Rgba(<[u8; 4]>::from(hex_color::HexColor::parse(&color.to_string()).unwrap().split_rgba()));
                    if is_y {
                        output.put_pixel(y.parse().unwrap(), z.parse().unwrap(), color);
                    } else {
                        output.put_pixel(z.parse().unwrap(), y.parse().unwrap(), color);
                    }
                }
            }
        }
        if verbose {
            bar.inc(1);
        }
    }

    let output_file = "test.png";
    output.save(output_file).unwrap();
    println!("Saved to {}.", output_file.blue());

}