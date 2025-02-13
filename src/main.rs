use std::fs::File;
use std::io::Write;
use ahash::HashMap;
use image::{open, Pixel, Rgba, RgbaImage};
use hex_color::HexColor;

#[derive(Debug)]
pub struct RPixel {
    x: u32,
    y: u32,
    color: String
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
        } else {
            if current_num == *num {
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
    }
    new_sequence = new_sequence.strip_prefix("+").unwrap().to_string();
    new_sequence
}

fn main() {
    let image = open("cat_pixel_art.png").unwrap().into_rgba8();
    let WIDTH:u32 = image.width();
    let HEIGHT:u32 = image.height();
    let mut x = 0;
    let mut y = 0;
    let mut temp_pixels: Vec<RPixel> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for w in image.pixels() {
        let colors = w.channels().to_vec();
        let mut color = HexColor::rgba(colors[0], colors[1], colors[2], colors[3]).display_rgba().to_string();
        let indexable: Vec<(usize, char)> = color.chars().enumerate().collect();
        if indexable[0] == indexable[1] && indexable[2] == indexable[3] && indexable[4] == indexable[5] {
            color.remove(1);
            color.remove(3);
            color.remove(5);
        }
        if color.ends_with("FF") {
            color = color.strip_suffix("FF").unwrap().to_string();
        }
        temp_pixels.push(RPixel {x, y, color });
        // if at EOL, go to start of next line
        if x == WIDTH - 1 {
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
    let mut BG_COLOR: String = String::new();
    let mut count:usize = 0;
    for x in px_colors.keys() {
        if px_colors[x].len() > count {
            count = px_colors[x].len();
            BG_COLOR = x.to_string();
        }
    }
    px_colors.remove(&BG_COLOR);

    let mut outputf: String = format!("{WIDTH}x{HEIGHT}%{BG_COLOR}%");

    for (color, pixels) in px_colors {
            let mut grouped_coords:HashMap<String, Vec<u32>> = Default::default();
            let mut y_coords:HashMap<String, Vec<u32>> = Default::default();
            for pixel in &pixels {
                // group by abscissa
                if !grouped_coords.contains_key(&format!("{}", pixel.0)) {
                    grouped_coords.insert(format!("{}", pixel.0), vec![pixel.1]);
                } else {
                    grouped_coords.get_mut(&format!("{}", pixel.0)).unwrap().push(pixel.1);
                }
                // group by ordinate (add "y" to be able to differentiate it)
                if !y_coords.contains_key(&format!("y{}", pixel.1)) {
                    y_coords.insert(format!("y{}", pixel.1), vec![pixel.0]);
                } else {
                    y_coords.get_mut(&format!("y{}", pixel.1)).unwrap().push(pixel.0);
                }
            }
            if format!("{grouped_coords:?}").len() > format!("{y_coords:?}").len() {
                grouped_coords = y_coords;
            }

            let mut export_hash:HashMap<String,String> = Default::default();

            for (name, coord_pixels) in grouped_coords {
                let mut math_sequence:String = format!("{}", coord_pixels[0]);
                let mut value = coord_pixels[0];
                for pixel in coord_pixels.iter().skip(1) {
                    let diff = pixel - value;
                    value += diff;
                    math_sequence.push_str(&format!("+{diff}"))
                }
                export_hash.insert(name, optimize_math_str(math_sequence.to_string()));
            }

            if format!("{export_hash:?}").len() > format!("{pixels:?}").len() {
                outputf.push_str(&format!("{color}{pixels:?}").replace(" ",""));
            } else {
                let mut sequenced = format!("{color}{export_hash:?}").replace(" ", "");
                if sequenced.contains("y") {
                    sequenced = sequenced.replace("y", "");
                    sequenced.push('y');
                }
                sequenced = sequenced.replace("\"", "");
                outputf.push_str(&sequenced);
            }
    }
    let mut file = File::create("output.txt").unwrap();
    let mut compressed = lzma::compress(outputf.as_bytes(), 9).unwrap();
    file.write_all(outputf.as_bytes()).unwrap();
    // file.write_all(&compressed).unwrap();
}
