use ahash::{HashMap, HashMapExt};
use colored::Colorize;
use const_currying::const_currying;
use crossterm::style::Stylize;
use hex_color::HexColor;
use image::{open, Pixel, Rgba, RgbaImage};
use kdam::tqdm;
use mashi_core::Encoder;
use rayon::prelude::ParallelSliceMut;
use std::fs;
use std::fs::File;
use std::io::Write;

fn optimize_math_str(input: &str) -> String {
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
    let mut export_hash: HashMap<String, String> = HashMap::default();

    for (name, coord_pixels) in input {
        let mut math_sequence: String = coord_pixels[0].to_string();
        let mut value = coord_pixels[0];
        for pixel in coord_pixels.iter().skip(1) {
            let diff = pixel - value;
            value += diff;
            math_sequence.push_str(&format!("+{diff}"));
        }
        export_hash.insert(name, optimize_math_str(&math_sequence));
    }
    export_hash
}

fn group_by_key(input: &HashMap<String, String>) -> HashMap<String, String> {
    let mut new: HashMap<String, Vec<u32>> = HashMap::default();
    for x in input.keys() {
        if new.contains_key(&input[x]) {
            new.get_mut(&input[x]).unwrap().push(x.parse().unwrap());
        } else {
            new.insert(input[x].clone(), vec![x.clone().parse().unwrap()]);
        }
    }
    for x in new.values_mut() {
        x.sort_unstable();
    }
    vec_to_math(new)
}

fn optimize_hex_color(input: String) -> String {
    let mut output_color: String = input;
    if output_color.ends_with("FF") {
        output_color.truncate(output_color.len() - 2);
    }
    let indexable = output_color.as_bytes();
    if indexable[0] == indexable[1] && indexable[2] == indexable[3] && indexable[4] == indexable[5]
    {
        output_color = format!(
            "#{}{}{}",
            &output_color[1..2],
            &output_color[3..4],
            &output_color[5..6]
        );
    }
    output_color
}

#[inline]
fn find_closest_palette_color(
    pixel: Rgba<u8>,
    image: &RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    base_radius: bool,
    diagonal_pixels: bool,
    extra_radius: bool,
    extra_extra_radius: bool,
) -> Rgba<u8> {
    let mut palette = Vec::new();

    macro_rules! add {
        ($x: expr, $y: expr) => {
            palette.push(image.get_pixel($x, $y));
        };
    }

    if x > 0 && y > 0 && diagonal_pixels {
        add!(x - 1, y - 1);
    }
    if x > 1 && y > 1 && extra_radius && diagonal_pixels {
        add!(x - 2, y - 2);
    }
    if x > 0 && base_radius {
        add!(x - 1, y);
    }
    if x > 1 && extra_radius {
        add!(x - 2, y);
    }
    if x > 2 && extra_extra_radius {
        add!(x - 3, y);
    }
    if x < width - 1 && y < height - 1 && diagonal_pixels {
        add!(x + 1, y + 1);
    }
    if x < width - 2 && y < height - 2 && extra_radius && diagonal_pixels {
        add!(x + 2, y + 2);
    }
    if y > 0 && base_radius {
        add!(x, y - 1);
    }
    if y > 1 && extra_radius {
        add!(x, y - 2);
    }
    if y > 2 && extra_extra_radius {
        add!(x, y - 3);
    }
    if x < width - 1 && base_radius {
        add!(x + 1, y);
    }
    if x < width - 2 && extra_radius {
        add!(x + 2, y);
    }
    if x < width - 3 && extra_extra_radius {
        add!(x + 3, y);
    }
    if y < height - 1 && base_radius {
        add!(x, y + 1);
    }
    if y < height - 2 && extra_radius {
        add!(x, y + 2);
    }
    if y < height - 3 && extra_extra_radius {
        add!(x, y + 3);
    }
    if x < width - 1 && y > 0 && diagonal_pixels {
        add!(x + 1, y - 1);
    }
    if x < width - 2 && y > 1 && extra_radius && diagonal_pixels {
        add!(x + 2, y - 2);
    }
    if x > 0 && y < height - 1 && diagonal_pixels {
        add!(x - 1, y + 1);
    }
    if x > 1 && y < height - 2 && extra_radius && diagonal_pixels {
        add!(x - 2, y + 2);
    }
    **palette
        .iter()
        .min_by_key(|p| {
            let dr = pixel[0] as i32 - p[0] as i32;
            let dg = pixel[1] as i32 - p[1] as i32;
            let db = pixel[2] as i32 - p[2] as i32;
            let da = pixel[3] as i32 - p[3] as i32;
            dr * dr + dg * dg + db * db + da * da
        })
        .unwrap_or(&&pixel)
}

fn get_quant_error_mul(mul: u8, quant_error: (i16, i16, i16, i16)) -> Rgba<u8> {
    let factor = (mul / 16) as i16;
    Rgba([
        (quant_error.0 * factor) as u8,
        (quant_error.1 * factor) as u8,
        (quant_error.2 * factor) as u8,
        (quant_error.3 * factor) as u8,
    ])
}

fn add_colors(x1: Rgba<u8>, x2: Rgba<u8>) -> Rgba<u8> {
    let old_channels = x1.channels();
    let new_channels = x2.channels();
    Rgba([
        old_channels[0] + new_channels[0],
        old_channels[1] + new_channels[1],
        old_channels[2] + new_channels[2],
        old_channels[3] + new_channels[3],
    ])
}

pub fn floyd_steinberg_dither(
    image: &mut RgbaImage,
    base_radius: bool,
    diagonal_pixels: bool,
    extra_radius: bool,
    extra_extra_radius: bool,
) {
    let (width, height) = image.dimensions();

    for y in tqdm!(0..height, desc = "Dithering") {
        for x in 0..width {
            let old_pixel = *image.get_pixel(x, y);
            let new_pixel = find_closest_palette_color(
                old_pixel,
                image,
                x,
                y,
                width,
                height,
                base_radius,
                diagonal_pixels,
                extra_radius,
                extra_extra_radius,
            );
            image.put_pixel(x, y, new_pixel);
            let old_channels = old_pixel.channels();
            let new_channels = new_pixel.channels();
            let quant_error = (
                old_channels[0] as i16 - new_channels[0] as i16,
                old_channels[1] as i16 - new_channels[1] as i16,
                old_channels[2] as i16 - new_channels[2] as i16,
                old_channels[3] as i16 - new_channels[3] as i16,
            );

            if x < width - 1 {
                let value = add_colors(
                    *image.get_pixel(x + 1, y),
                    get_quant_error_mul(7, quant_error),
                );
                image.put_pixel(x + 1, y, value);
            }

            if x > 0 && y < height - 1 {
                let value = add_colors(
                    *image.get_pixel(x - 1, y + 1),
                    get_quant_error_mul(3, quant_error),
                );
                image.put_pixel(x - 1, y + 1, value);
            }

            if y < height - 1 {
                let value = add_colors(
                    *image.get_pixel(x, y + 1),
                    get_quant_error_mul(5, quant_error),
                );
                image.put_pixel(x, y + 1, value);
            }

            if x < width - 1 && y < height - 1 {
                let value = add_colors(
                    *image.get_pixel(x + 1, y + 1),
                    get_quant_error_mul(1, quant_error),
                );
                image.put_pixel(x + 1, y + 1, value);
            }
        }
    }
}

pub fn convert(
    path: &str,
    output_file: &str,
    lossy: bool,
    base_radius: bool,
    diagonal_pixels: bool,
    extra_radius: bool,
    extra_extra_radius: bool,
) {
    println!("PCF -- Converting {}", Colorize::blue(path));
    let mut image = open(path).unwrap().into_rgba8();
    if lossy {
        floyd_steinberg_dither(
            &mut image,
            base_radius,
            diagonal_pixels,
            extra_radius,
            extra_extra_radius,
        );
        if cfg!(debug_assertions) {
            image.save("dither-output.png").unwrap();
        }
    }
    let width: u32 = image.width();
    let height: u32 = image.height();
    let mut x = 0;
    let mut y = 0;
    let pixels = image.pixels();

    // put the pixels in a hashmap
    let mut px_colors: HashMap<String, Vec<(u32, u32)>> = HashMap::with_capacity(pixels.len());
    for w in tqdm!(pixels, desc = "Registering pixels") {
        let colors = w.channels();
        let color = optimize_hex_color(
            HexColor::rgba(colors[0], colors[1], colors[2], colors[3])
                .display_rgba()
                .to_string(),
        );
        px_colors.entry(color).or_default().push((x, y));
        // if at EOL, go to start of next line
        x += 1;
        if x == width {
            x = 0;
            y += 1;
        }
    }

    // remove dominant color
    let bg_color = px_colors
        .iter()
        .max_by_key(|(_, y)| y.len())
        .unwrap()
        .0
        .to_string();
    px_colors.remove(&bg_color);

    let mut outputf: String = format!("{width}%{height}%{bg_color}%");

    for (color, pixels) in tqdm!(px_colors.iter(), desc = "Compressing") {
        let mut grouped_coords: HashMap<String, Vec<u32>> = HashMap::with_capacity(pixels.len());
        let mut y_coords: HashMap<String, Vec<u32>> = HashMap::with_capacity(pixels.len());
        let mut is_y = false;
        for pixel in pixels {
            // group by abscissa
            let key = format!("{}", pixel.0);
            grouped_coords.entry(key).or_default().push(pixel.1);


            // group by ordinate
            let key = format!("{}", pixel.1);
            y_coords.entry(key).or_default().push(pixel.0);

        }
        if size_of_val(&grouped_coords) > size_of_val(&y_coords) {
            grouped_coords = y_coords;
            is_y = true;
        }

        let export_hash: HashMap<String, String> = vec_to_math(grouped_coords);
        let output = group_by_key(&export_hash);
        let mut sequenced = format!("{color}{output:?}")
            .replace(' ', "")
            .replace('"', "");

        if format!("{output:?}").replace('"', "").len() > format!("{pixels:?}").len() {
            outputf.push_str(&format!("{color}{pixels:?}").replace(' ', ""));
        } else {
            if is_y {
                sequenced.push('y');
            }
            sequenced = sequenced.replace(['"', '\\'], "");
            outputf.push_str(&sequenced);
        }
    }
    let mut chars = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'z', '&', '-',
    ];
    let mut compressed = remove_dup_patterns(outputf, 2, 4, &mut chars);
    for _ in 0..7 {
        compressed = remove_dup_patterns(compressed, 2, 4, &mut chars);
    }
    // compressed = remove_dup_patterns(compressed, 2, 4, &mut chars);
    // compressed = compressed
    //     .replace("00", "^")
    //     .replace("01", "~")
    //     .replace("11", "è")
    //     .replace("42", "/")
    //     .replace("42", "/")
    //     .replace("17", "!")
    //     .replace("23", "§")
    //     .replace("12", ".")
    //     .replace("27", "&")
    //     .replace("22", "`")
    //     .replace("21", "=")
    //     .replace("14", "\\")
    //     .replace("24", "£")
    //     .replace("16", "¤")
    //     .replace("10", "|")
    //     .replace("37", "µ")
    //     .replace("19", ";")
    //     .replace("18", "ç")
    //     .replace("28", "@")
    //     .replace("69", "à")
    //     .replace("13", "é");


    let mut file = File::create(output_file).unwrap();
    let mut encoder = Encoder::new();
    let output = encoder.encode(compressed.as_bytes());

    println!("{output:?}");

    if size_of_val(&output) < size_of_val(compressed.as_bytes()) {
        file.write_all(&output).unwrap();
    } else {
        file.write_all(compressed.as_bytes()).unwrap();
    }
    println!(
        "\nSaved to {} - {}% of original size.",
        Colorize::blue(output_file),
        ((fs::metadata(output_file).unwrap().len() as f64) * 100.0 / fs::metadata(path).unwrap().len() as f64)
            .to_string()
            .blue()
    );
}

#[const_currying]
fn find_pattern(
    target: String,
    #[maybe_const(dispatch = step, consts = [2,3,4,5,6,7])] step: usize,
) -> Vec<(String, usize)> {
    let mut patterns = HashMap::with_capacity(target.len());
    let iter = 0..=target.len().saturating_sub(step);
    for i in iter {
        let slice = &target[i..i + step];
        *patterns.entry(slice).or_insert(0) += 1;
    }

    let mut new: Vec<_> = patterns.into_iter().collect();
    new.par_sort_by(|a, b| b.1.cmp(&a.1));
    new.into_iter()
        .take(1)
        .map(|(s, c)| (s.to_string(), c))
        .collect()
}

#[const_currying]
fn remove_dup_patterns(
    compressed: String,
    #[maybe_const(dispatch = min_pattern_size, consts = [2,3,4,5,6,7])] min_pattern_size: usize,
    #[maybe_const(dispatch = max_pattern_size, consts = [2,3,4,5,6,7])] max_pattern_size: usize,
    chars: &mut Vec<char>
) -> String {
    let mut worthy_patterns: Vec<(String, isize)> = Vec::new();
    (min_pattern_size..=max_pattern_size).for_each(|step| {
        for (pattern, count) in find_pattern(compressed.to_string(), step) {
            if count != 1 {
                let savings: isize = (count as isize) * (pattern.len() as isize - 2) - 1;
                worthy_patterns.push((pattern, savings));
            }
        }
    });
    worthy_patterns.par_sort_by(|a, b| b.1.cmp(&a.1));
    let mut use_letter = 0;

    let mut output = compressed;
    for (pattern, savings) in &worthy_patterns[0..1] {
        if output.matches(pattern).count() > 2 && savings > &0 {
            let letter = chars.remove(use_letter);
            output = output.replace(pattern, &letter.to_string());
            if use_letter == 0 && !output.contains("_") {
                output.push('_');
            }
            output.push_str(&format!("${pattern}${letter}"));
            use_letter += 1;
        }
    }
    output
}
