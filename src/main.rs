use rayon::iter::ParallelIterator;
mod decode;

use crate::decode::decode;
use ahash::HashMap;
use hex_color::HexColor;
use image::{open, Pixel};
use rayon::iter::IntoParallelIterator;

#[derive(Debug)]
pub struct RPixel {
    x: u32,
    y: u32,
    color: String,
}

fn optimize_math_str(input: String) -> String {
    let mut nums: Vec<&str> = input.split('+').filter(|s| !s.is_empty()).collect();
    nums.push("");
    let mut new_sequence = String::new();
    let mut current_num = "";
    let mut count = 0;
    for (index, num) in nums.iter().enumerate() {
        if index == 0 {
            current_num = num;
            count = 1;
        } else if current_num == *num {
                count += 1;
            } else {
                if count > 1 {
                    new_sequence.push_str(&format!("+{current_num}*{count}"));
                } else {
                    new_sequence.push_str(&format!("+{current_num}"));
                }
                current_num = num;
                count = 1;
            }
    }
    new_sequence.strip_prefix("+").unwrap().to_string()
}

fn vec_to_math(input: HashMap<String, Vec<u32>>) -> HashMap<String, String> {
    let mut export_hash: HashMap<String, String> = Default::default();

    for (name, coord_pixels) in input {
        let mut math_sequence: String = format!("{}", coord_pixels[0]);
        let mut value = coord_pixels[0];
        for pixel in coord_pixels.iter().skip(1) {
            let diff = pixel - value;
            value += diff;
            math_sequence.push_str(&format!("+{diff}"))
        }
        export_hash.insert(name, optimize_math_str(math_sequence.to_string()));
    }
    export_hash
}

fn group_by_key(input: HashMap<String, String>) -> (HashMap<String, String>, bool) {
    let mut new: HashMap<String, Vec<u32>> = Default::default();
    let mut is_y = false;
    for x in input.keys() {
        if !new.contains_key(&input[x]) {
            if x.contains("y") {
                is_y = true;
                new.insert(
                    input[x].clone(),
                    vec![x.clone().replace("y", "").parse().unwrap()],
                );
            } else {
                new.insert(input[x].clone(), vec![x.clone().parse().unwrap()]);
            }
        } else if x.contains("y") {
            is_y = true;
            new.get_mut(&input[x])
                .unwrap()
                .push(x.replace("y", "").parse().unwrap());
        } else {
            new.get_mut(&input[x]).unwrap().push(x.parse().unwrap());
        }
    }
    for x in new.values_mut() {
        x.sort();
    }
    (vec_to_math(new), is_y)
}

fn convert(path: String) -> String {
    let image = open(path).unwrap().into_rgba8();
    let width: u32 = image.width();
    let height: u32 = image.height();
    let mut x = 0;
    let mut y = 0;
    let mut temp_pixels: Vec<RPixel> = Vec::with_capacity((width * height) as usize);
    for w in image.pixels() {
        let colors = w.channels().to_vec();
        let mut color = HexColor::rgba(colors[0], colors[1], colors[2], colors[3])
            .display_rgba()
            .to_string();
        let indexable: Vec<(usize, char)> = color.chars().enumerate().collect();
        if indexable[0] == indexable[1]
            && indexable[2] == indexable[3]
            && indexable[4] == indexable[5]
        {
            color.remove(1);
            color.remove(3);
            color.remove(5);
        }
        if color.ends_with("FF") {
            color = color.strip_suffix("FF").unwrap().to_string();
        }
        temp_pixels.push(RPixel { x, y, color });
        // if at EOL, go to start of next line
        if x == width - 1 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }

    // put the pixels in a hashmap
    let mut px_colors: HashMap<String, Vec<(u32, u32)>> = Default::default();
    for x in temp_pixels {
        if !px_colors.contains_key(&x.color) {
            px_colors.insert(x.color, vec![(x.x, x.y)]);
        } else {
            px_colors.get_mut(&x.color).unwrap().push((x.x, x.y));
        }
    }
    // remove dominant color
    let mut bg_color: String = String::new();
    let mut count: usize = 0;
    for x in px_colors.keys() {
        if px_colors[x].len() > count {
            count = px_colors[x].len();
            bg_color = x.to_string();
        }
    }
    px_colors.remove(&bg_color);

    let mut outputf: String = format!("{width}%{height}%{bg_color}%");

    for (color, pixels) in px_colors {
        let mut grouped_coords: HashMap<String, Vec<u32>> = Default::default();
        let mut y_coords: HashMap<String, Vec<u32>> = Default::default();
        for pixel in &pixels {
            // group by abscissa
            if !grouped_coords.contains_key(&format!("{}", pixel.0)) {
                grouped_coords.insert(format!("{}", pixel.0), vec![pixel.1]);
            } else {
                grouped_coords
                    .get_mut(&format!("{}", pixel.0))
                    .unwrap()
                    .push(pixel.1);
            }
            // group by ordinate (add "y" to be able to differentiate it)
            if !y_coords.contains_key(&format!("y{}", pixel.1)) {
                y_coords.insert(format!("y{}", pixel.1), vec![pixel.0]);
            } else {
                y_coords
                    .get_mut(&format!("y{}", pixel.1))
                    .unwrap()
                    .push(pixel.0);
            }
        }
        if format!("{grouped_coords:?}").len() > format!("{y_coords:?}").len() {
            grouped_coords = y_coords;
        }

        let export_hash: HashMap<String, String> = vec_to_math(grouped_coords);
        let (output, is_y) = group_by_key(export_hash);
        let mut sequenced = format!("{color}{output:?}").replace(" ", "");

        if format!("{output:?}").len() > format!("{pixels:?}").len() {
            outputf.push_str(&format!("{color}{pixels:?}").replace(" ", ""));
        } else {
            if is_y {
                sequenced = sequenced.replace("y", "");
                sequenced.push('y');
            }
            sequenced = sequenced.replace("\"", "").replace("\\", "");
            outputf.push_str(&sequenced);
        }
    }
    outputf
}

fn find_pattern(target: String, step: usize) -> (String, usize) {
    let mut patterns: Vec<String> = Vec::with_capacity(target.len());
    let mut i = 0;
    let mut seen = std::collections::HashSet::new();
    while i + step <= target.len() {
        let slice = &target[i..i + step];
        if seen.insert(slice) {
            patterns.push(slice.to_string());
        }
        i += 1;
    }
    // return => (pattern, max_pattern_count)
    patterns
        .iter()
        .map(|x| (x.to_string(), target.matches(x).count()))
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

fn remove_dup_patterns(
    compressed: String,
    min_pattern_size: usize,
    max_pattern_size: usize,
) -> String {
    let mut worthy_patterns: Vec<(String, isize)> = (min_pattern_size..=max_pattern_size)
        .into_par_iter()
        .map(|step| {
            let (pattern, count) = find_pattern(compressed.to_string(), step);
            println!("PROCESSED N.{step}");
            if count != 1 {
                let savings: isize = (count as isize) * (pattern.len() as isize - 2) - 1;
                (pattern, savings)
            } else {
                (String::from(""), -99999)
            }
        })
        .collect();

    worthy_patterns.sort_by(|a, b| b.1.cmp(&a.1));

    println!("PATTERNS ARE {worthy_patterns:?}");
    let mut use_letter = 0;
    static CHARS: [char; 27] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'z', '&', '_'
    ];
    let mut output = compressed;
    for (pattern, _) in &worthy_patterns[0..1] {
        if output.matches(pattern).count() > 1 {
            let letter = CHARS[use_letter];
            output = output.replace(pattern, &letter.to_string());
            if use_letter == 0 {
                output.push('%');
            }
            output.push_str(&format!("${pattern}${letter}"));
            use_letter += 1;
        }
    }
    output
}

fn main() {
    // static COMPRESS: bool = true;
    //
    // let mut compressed = convert("cat_pixel_art.png".to_string());
    //
    // compressed = remove_dup_patterns(compressed, 2, 4);
    //
    // let mut file = File::create("output.txt").unwrap();
    // if COMPRESS {
    //     let mut out = Vec::new();
    //     let mut options = LZMA2Options::with_preset(9);
    //     options.dict_size = LZMA2Options::DICT_SIZE_DEFAULT;
    //     {
    //         let mut w = LZMA2Writer::new(CountingWriter::new(&mut out), &options);
    //         w.write_all(compressed.as_bytes()).unwrap();
    //         w.write(&[]).unwrap();
    //     }
    //     file.write_all(&out).unwrap();
    // } else {
    //     file.write_all(&compressed.as_bytes()).unwrap();
    // }

    decode("output.txt".parse().unwrap());

}
