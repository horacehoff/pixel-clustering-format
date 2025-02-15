use rayon::iter::{ParallelBridge, ParallelIterator};
use std::fs;
use std::fs::File;
use std::io::Write;
mod decode;

use crate::decode::decode;
use ahash::HashMap;
use colored::Colorize;
use const_currying::const_currying;
use hex_color::HexColor;
use image::{open, Pixel};
use indicatif::ProgressBar;
use lzma_rust::{CountingWriter, LZMA2Options, LZMA2Writer};
use rayon::iter::IntoParallelIterator;
use rayon::slice::ParallelSliceMut;

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


#[const_currying]
fn convert(path: &str,
           output_file:&str,
           #[maybe_const(dispatch = compress, consts = [true, false])]compress:bool,
           #[maybe_const(dispatch = verbose, consts = [true, false])]verbose: bool) {
    println!("Converting {}", path.blue());
    let image = open(path).unwrap().into_rgba8();
    let width: u32 = image.width();
    let height: u32 = image.height();
    let mut x = 0;
    let mut y = 0;
    let mut temp_pixels: Vec<RPixel> = Vec::with_capacity((width * height) as usize);
    let bar = ProgressBar::new(image.pixels().len() as u64);
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
        if verbose {
            bar.inc(1);
        }
    }

    // put the pixels in a hashmap
    let mut px_colors: HashMap<String, Vec<(u32, u32)>> = Default::default();
    let bar = ProgressBar::new(temp_pixels.len() as u64);
    for x in temp_pixels {
        if !px_colors.contains_key(&x.color) {
            px_colors.insert(x.color, vec![(x.x, x.y)]);
        } else {
            px_colors.get_mut(&x.color).unwrap().push((x.x, x.y));
        }
        if verbose {
            bar.inc(1);
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

    let bar = ProgressBar::new(px_colors.len() as u64);
    for (color, pixels) in px_colors {
        let mut grouped_coords: HashMap<String, Vec<u32>> = Default::default();
        let mut y_coords: HashMap<String, Vec<u32>> = Default::default();
        let mut is_y = false;
        for pixel in &pixels {
            // group by abscissa
            let key = format!("{}", pixel.0);
            if !grouped_coords.contains_key(&key) {
                grouped_coords.insert(key, vec![pixel.1]);
            } else {
                grouped_coords
                    .get_mut(&key)
                    .unwrap()
                    .push(pixel.1);
            }
            // group by ordinate (add "y" to be able to differentiate it)
            let key = format!("{}", pixel.1);
            if !y_coords.contains_key(&key) {
                y_coords.insert(key, vec![pixel.0]);
            } else {
                y_coords
                    .get_mut(&key)
                    .unwrap()
                    .push(pixel.0);
            }
        }
        if format!("{grouped_coords:?}").len() > format!("{y_coords:?}").len() {
            grouped_coords = y_coords;
            is_y = true;
        }

        let export_hash: HashMap<String, String> = vec_to_math(grouped_coords);
        let (output, _) = group_by_key(export_hash);
        let mut sequenced = format!("{color}{output:?}").replace(" ", "");

        if format!("{output:?}").replace("y","").replace('"',"").len() > format!("{pixels:?}").len() {
            outputf.push_str(&format!("{color}{pixels:?}").replace(" ", ""));
        } else {
            if is_y {
                sequenced = sequenced.replace("y", "");
                sequenced.push('y');
            }
            sequenced = sequenced.replace("\"", "").replace("\\", "");
            outputf.push_str(&sequenced);
        }
        if verbose {
            bar.inc(1);
        }
    }
    let mut compressed = outputf;
    compressed = remove_dup_patterns(compressed, 2, 4, verbose);

    let mut file = File::create(output_file).unwrap();
    if compress {
        let mut out = Vec::new();
        let mut options = LZMA2Options::with_preset(9);
        options.dict_size = 8000000;
        options.lc = 4;
        {
            let mut w = LZMA2Writer::new(CountingWriter::new(&mut out), &options);
            w.write_all(compressed.as_bytes()).unwrap();
            w.write(&[]).unwrap();
        }
        file.write_all(&out).unwrap();
    } else {
        file.write_all(&compressed.as_bytes()).unwrap();
    }
    println!("Saved to {} - {}% of original size.", output_file.blue(), (fs::metadata(output_file).unwrap().len()*100/fs::metadata(path).unwrap().len()).to_string().blue())
}

#[const_currying]
fn find_pattern(target: String,
                #[maybe_const(dispatch = step, consts = [2,3,4,5,6,7])]step: usize,
                #[maybe_const(dispatch = verbose, consts = [true, false])]verbose:bool) -> Vec<(String, usize)> {
    let mut patterns: Vec<(String, usize)> = Vec::with_capacity(target.len());
    let mut i = 0;
    let mut seen = std::collections::HashSet::new();
    let bar = ProgressBar::new(target.len() as u64);
    while i + step <= target.len() {
        let slice = &target[i..i + step];
        if seen.insert(slice) {
            patterns.push((slice.to_string(), target.matches(slice).count()));
        }
        i += 1;
        if verbose {
            bar.inc(1);
        }
    }
    patterns.par_sort_by(|a,b| b.1.cmp(&a.1));
    patterns.first_chunk::<2>().unwrap().to_vec()
}


#[const_currying]
fn remove_dup_patterns(
    compressed: String,
    #[maybe_const(dispatch = min_pattern_size, consts = [2,3,4,5,6,7])]min_pattern_size: usize,
    #[maybe_const(dispatch = max_pattern_size, consts = [2,3,4,5,6,7])]max_pattern_size: usize,
    #[maybe_const(dispatch = verbose, consts = [true, false])]verbose: bool
) -> String {
    let mut worthy_patterns: Vec<(String, isize)> = Vec::new();
    (min_pattern_size..=max_pattern_size)
        .for_each(|step| {
            for (pattern,count) in find_pattern(compressed.to_string(), step,verbose) {
                if count != 1 {
                    let savings: isize = (count as isize) * (pattern.len() as isize - 2) - 1;
                    worthy_patterns.push((pattern, savings));
                }
            }
        });
    worthy_patterns.par_sort_by(|a, b| b.1.cmp(&a.1));
    let mut use_letter = 0;
    static CHARS: [char; 27] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'z', '&', '_'
    ];
    let mut output = compressed;
    for (pattern, _) in &worthy_patterns[0..1] {
        if output.matches(pattern).count() > 2 {
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
    //convert("fig1.png", "fig1.txt", false, true);
    decode("fig1.txt".parse().unwrap());
}