use colored::Colorize;
use image::RgbaImage;
use kdam::tqdm;
use mashi_core::Decoder;
use std::fs::File;
use std::io::Read;

pub fn expand_math(s: &str) -> String {
    let parts: Vec<String> = s.split('+').map(ToString::to_string).collect();
    let mut output: String = String::new();
    for x in parts {
        if x.contains('*') {
            let split: Vec<&str> = x.split('*').collect();
            let count: u32 = split[1].parse().unwrap();
            let number = split[0];
            for _ in 0..count {
                output.push('+');
                output.push_str(number);
            }
        } else {
            output.push('+');
            output.push_str(&x);
        }
    }
    output.trim_start_matches('+').to_string()
}

pub fn math_to_vec(s: &str) -> Vec<String> {
    let splits: Vec<u32> = s
        .split('+')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut output: Vec<String> = Vec::with_capacity(splits.len());
    let mut count: u32 = 0;
    for x in splits {
        count += x;
        output.push(count.to_string());
    }
    output
}

pub fn decode(
    path: String,
    output_file: &str,
) {
    println!("PCF -- Decoding {}...", path.blue());
    let mut input = File::open(path).unwrap();
    let mut contents = vec![];
    input.read_to_end(&mut contents).unwrap();

    let mut result = String::new();

    //  check if contents is compressed
    let test = String::from_utf8(contents.clone()).unwrap_or_default();
    if test.contains('%') && test.contains('#') {
        result = test;
    } else {
        let mut decoder = Decoder::new();
        let decompressed = decoder.decode(&contents);
        result = String::from_utf8(decompressed.to_vec()).unwrap();
    }

    result = result.replace("é", "13")
        .replace("à", "69")
        .replace("@", "28")
        .replace("ç", "18")
        .replace(";", "19")
        .replace("µ", "37")
        .replace("|", "10")
        .replace("¤", "16")
        .replace("£", "24")
        .replace("\\", "14")
        .replace("=", "21")
        .replace("`", "22")
        .replace("&", "27")
        .replace(".", "12")
        .replace("§", "23")
        .replace("!", "17")
        .replace("/", "42")
        .replace("è", "11")
        .replace("~", "01")
        .replace("^", "00");
    
    // if some letters need to be replaced, replace them
    if result.contains('_') {
        let matcher = result.split('_').collect::<Vec<&str>>();
        let mut output: String = matcher[0].to_string();
        let letters = matcher[1];

        let parts: Vec<&str> = letters.split('$').filter(|x| !x.is_empty()).collect();
        parts.chunks(2).map(|a| (a[1], a[0])).rev().for_each(|(by, to)| {
            output = output.replace(by, to);
        });
        result = output;
    }

    let mut colors: Vec<String> = result.split('%').map(ToString::to_string).collect();
    let width: u32 = colors.remove(0).parse().unwrap();
    let height: u32 = colors.remove(0).parse().unwrap();
    let bg_color = colors.remove(0);
    let mut output = RgbaImage::from_pixel(
        width,
        height,
        image::Rgba(<[u8; 4]>::from(
            hex_color::HexColor::parse(&bg_color).unwrap().split_rgba(),
        )),
    );

    let mut colors: Vec<String> = colors.remove(0).split('#').map(ToString::to_string).filter(|x| !x.is_empty()).collect();
    for x in tqdm!(colors.iter_mut()) {
        let mut is_y = false;
        if x.ends_with('y') {
            is_y = true;
            x.pop().unwrap();
        }
        let split: Vec<&str> = x.split('{').collect();
        // if pixel list is just a plain array of tuples (x,y)
        if split.len() == 1 {
            let split: Vec<&str> = x.split('[').collect();
            let color = format!("#{}", split[0]);
            let mut pixels: Vec<&str> = split[1].trim_end_matches(']').split(')').collect();
            pixels = pixels
                .iter()
                .map(|x| {
                    x.trim_start_matches('(')
                        .trim_start_matches(',')
                        .trim_end_matches(',')
                        .trim_end_matches(')')
                        .trim_start_matches('(')
                        .trim_start_matches(',')
                        .trim_end_matches(',')
                        .trim_end_matches(')')
                })
                .filter(|x| !x.is_empty())
                .collect();
            for pixel in pixels {
                let split: Vec<&str> = pixel.split(',').collect();
                let x = split[0].parse().unwrap();
                let y = split[1].parse().unwrap();
                output.put_pixel(
                    x,
                    y,
                    image::Rgba(<[u8; 4]>::from(
                        hex_color::HexColor::parse(&color.to_string())
                            .unwrap()
                            .split_rgba(),
                    )),
                );
            }
            continue;
        }
        let color = format!("#{}", split[0]);
        let mut pixels = format!("{{{}", split[1])
            .replace(':', "\":\"")
            .replace(',', "\",\"");
        pixels.insert(1, '"');
        pixels.insert(pixels.len() - 1, '"');

        let parsed = json::parse(&pixels).unwrap();
        for (x, val) in parsed.entries() {
            // de-group and expand each key
            let expanded = expand_math(x);
            let vecs = math_to_vec(&expanded);
            // expand the value
            let expanded = expand_math(val.as_str().unwrap());
            let vecs2 = math_to_vec(&expanded);
            for y in vecs {
                for z in &vecs2 {
                    let color = image::Rgba(<[u8; 4]>::from(
                        hex_color::HexColor::parse(&color.to_string())
                            .unwrap()
                            .split_rgba(),
                    ));
                    if is_y {
                        output.put_pixel(y.parse().unwrap(), z.parse().unwrap(), color);
                    } else {
                        output.put_pixel(z.parse().unwrap(), y.parse().unwrap(), color);
                    }
                }
            }
        }
    }

    output.save(&output_file).unwrap();
    println!("\nSaved to {}.", output_file.blue());
}
