use std::fs::File;
use std::io::Write;
use std::{env, fs};

mod decode;
mod data;

use crate::decode::decode;
use ahash::{HashMap, HashMapExt};
use colored::Colorize;
use const_currying::const_currying;
use hex_color::HexColor;
use image::{open, Pixel, Rgba, RgbaImage};
use indicatif::ProgressBar;
use mashi_core::Encoder;
use rayon::slice::ParallelSliceMut;

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

fn group_by_key(input: HashMap<String, String>) -> HashMap<String, String> {
    let mut new: HashMap<String, Vec<u32>> = Default::default();
    for x in input.keys() {
        if !new.contains_key(&input[x]) {
            new.insert(input[x].clone(), vec![x.clone().parse().unwrap()]);
        } else {
            new.get_mut(&input[x]).unwrap().push(x.parse().unwrap());
        }
    }
    for x in new.values_mut() {
        x.sort();
    }
    vec_to_math(new)
}

fn optimize_hex_color(input: String) -> String {
    let mut output_color: String = input;
    let indexable = output_color.as_bytes();
    if indexable[0] == indexable[1]
        && indexable[2] == indexable[3]
        && indexable[4] == indexable[5]
    {
        output_color = format!("#{}{}{}", &output_color[1..2], &output_color[3..4], &output_color[5..6])
    }
    if output_color.ends_with("FF") {
        output_color.truncate(output_color.len() - 2)
    }
    output_color
}

#[inline]
fn find_closest_palette_color(pixel: Rgba<u8>, palette: Vec<Rgba<u8>>) -> Rgba<u8> {
    palette
        .into_iter()
        .min_by_key(|p| {
            let dr = pixel[0] as i32 - p[0] as i32;
            let dg = pixel[1] as i32 - p[1] as i32;
            let db = pixel[2] as i32 - p[2] as i32;
            let da = pixel[3] as i32 - p[3] as i32;
            dr * dr + dg * dg + db * db + da * da
        })
        .unwrap_or(pixel)
}
fn get_quant_error_mul(mul: u8, quant_error: (i16, i16, i16, i16)) -> Rgba<u8> {
    let factor = (mul / 16) as i16;
    Rgba([(quant_error.0 * factor) as u8, (quant_error.1 * factor) as u8, (quant_error.2 * factor) as u8, (quant_error.3 * factor) as u8])
}

fn add_colors(x1: Rgba<u8>, x2: Rgba<u8>) -> Rgba<u8> {
    let old_channels = x1.channels();
    let new_channels = x2.channels();
    Rgba([old_channels[0] + new_channels[0], old_channels[1] + new_channels[1], old_channels[2] + new_channels[2], old_channels[3] + new_channels[3]])
}

pub fn floyd_steinberg_dither(image: &mut RgbaImage, path: String) {
    let (width, height) = image.dimensions();
    let max = image_palette::load(&path).unwrap().len();
    let colors = image_palette::load_with_maxcolor(&path, 256).unwrap();
    let mut palette: Vec<Rgba<u8>> = Vec::new();
    for item in colors {
        palette.push(image::Rgba(<[u8; 4]>::from(hex_color::HexColor::parse(item.color()).unwrap().split_rgba())));
    }

    for y in 0..height {
        for x in 0..width {
            let old_pixel = image.get_pixel(x, y).clone();
            let new_pixel = find_closest_palette_color(old_pixel, palette.clone());
            image.put_pixel(x, y, new_pixel);
            let old_channels = old_pixel.channels();
            let new_channels = new_pixel.channels();
            let quant_error = (old_channels[0] as i16 - new_channels[0] as i16, old_channels[1] as i16 - new_channels[1] as i16, old_channels[2] as i16 - new_channels[2] as i16, old_channels[3] as i16 - new_channels[3] as i16);

            if x < width - 1 {
                let value = add_colors(image.get_pixel(x + 1, y).clone(), get_quant_error_mul(7, quant_error));
                image.put_pixel(x + 1, y, value);
            }

            if x > 0 && y < height - 1 {
                let value = add_colors(image.get_pixel(x - 1, y + 1).clone(), get_quant_error_mul(3, quant_error));
                image.put_pixel(x - 1, y + 1, value);
            }

            if y < height - 1 {
                let value = add_colors(image.get_pixel(x, y + 1).clone(), get_quant_error_mul(5, quant_error));
                image.put_pixel(x, y + 1, value);
            }

            if x < width - 1 && y < height - 1 {
                let value = add_colors(image.get_pixel(x + 1, y + 1).clone(), get_quant_error_mul(1, quant_error));
                image.put_pixel(x + 1, y + 1, value);
            }
        }
    }
}

#[const_currying]
fn convert(path: &str,
           output_file: &str,
           #[maybe_const(dispatch = verbose, consts = [true, false])]verbose: bool,
           lossy: bool
) {
    println!("PCF -- Converting {}", path.blue());
    let mut image = open(path).unwrap().into_rgba8();
    if lossy {
        floyd_steinberg_dither(&mut image, path.to_string());
    }
    let width: u32 = image.width();
    let height: u32 = image.height();
    let mut x = 0;
    let mut y = 0;


    // put the pixels in a hashmap
    let bar = ProgressBar::new(image.pixels().len() as u64);
    let mut px_colors: HashMap<String, Vec<(u32, u32)>> = Default::default();
    let pixels = image.pixels();
    for w in pixels {
        let colors = w.channels();
        let color = optimize_hex_color(HexColor::rgba(colors[0], colors[1], colors[2], colors[3])
            .display_rgba().to_string());
        px_colors.entry(color).or_default().push((x, y));
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
    // remove dominant color
    let bg_color = px_colors.iter().max_by_key(|(x, y)| y.len()).unwrap().0.to_string();
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
            // group by ordinate
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
        // if format!("{grouped_coords:?}").len() > format!("{y_coords:?}").len() {
        if size_of_val(&grouped_coords) > size_of_val(&y_coords) {
            grouped_coords = y_coords;
            is_y = true;
        }

        let export_hash: HashMap<String, String> = vec_to_math(grouped_coords);
        let mut output = group_by_key(export_hash);
        let mut sequenced = format!("{color}{output:?}").replace(" ", "").replace('"', "");

        if format!("{output:?}").replace('"', "").len() > format!("{pixels:?}").len() {
            outputf.push_str(&format!("{color}{pixels:?}").replace(" ", ""));
        } else {
            if is_y {
                sequenced.push('y');
            }
            sequenced = sequenced.replace("\"", "").replace("\\", "");
            outputf.push_str(&sequenced);
        }
        if verbose {
            bar.inc(1);
        }
    }

    let compressed = remove_dup_patterns(outputf, 2, 4, verbose);;

    let mut file = File::create(output_file).unwrap();
    let mut encoder = Encoder::new();
    let output = encoder.encode(&compressed.as_bytes());

    if size_of_val(&output) < size_of_val(compressed.as_bytes()) {
        file.write_all(&output).unwrap();
    } else {
        file.write_all(&compressed.as_bytes()).unwrap();
    }
    println!("Saved to {} - {}% of original size.", output_file.blue(), (fs::metadata(output_file).unwrap().len()*100/fs::metadata(path).unwrap().len()).to_string().blue())
}

#[const_currying]
fn find_pattern(target: String,
                #[maybe_const(dispatch = step, consts = [2,3,4,5,6,7])]step: usize,
                #[maybe_const(dispatch = verbose, consts = [true, false])]verbose:bool) -> Vec<(String, usize)> {
    let mut patterns = HashMap::with_capacity(target.len());
    let bar = ProgressBar::new(target.len() as u64);
    for i in 0..=target.len().saturating_sub(step) {
        let slice = &target[i..i + step];
        *patterns.entry(slice).or_insert(0) += 1;
        if verbose {
            bar.inc(1);
        }
    }

    let mut new: Vec<_> = patterns.into_iter().collect();
    new.par_sort_by(|a, b| b.1.cmp(&a.1));
    new.into_iter().take(2).map(|(s, c)| (s.to_string(), c)).collect()
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
        's', 't', 'u', 'v', 'w', 'x', 'z', '&', '-'
    ];
    let mut output = compressed;
    for (pattern, _) in &worthy_patterns[0..1] {
        if output.matches(pattern).count() > 2 {
            let letter = CHARS[use_letter];
            output = output.replace(pattern, &letter.to_string());
            if use_letter == 0 {
                output.push('_');
            }
            output.push_str(&format!("${pattern}${letter}"));
            use_letter += 1;
        }
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--decode".to_string()) {
        decode(args[1].clone(), args.contains(&"--verbose".to_string()));
    } else {
        let file_path = args[1].to_string();
        let name = std::path::Path::new(&file_path).file_name().unwrap().to_str().unwrap().split(".").collect::<Vec<&str>>()[0].to_string();
        convert(&args[1], &(name + ".pcf"), args.contains(&"--verbose".to_string()), args.contains(&"--lossy".to_string()));
    }
}