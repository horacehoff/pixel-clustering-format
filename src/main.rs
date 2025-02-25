use std::env;
use std::process::exit;
use std::time::Duration;

mod data;
mod decode;
mod encode;

use crate::decode::decode;
use crate::encode::convert;
use colored::{ColoredString, Colorize};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::style::Stylize;
use crossterm::terminal::disable_raw_mode;
use rfd::FileDialog;

fn display_menu(
    left: bool,
    right: bool,
    enter: bool,
    mode: &mut u8,
    sel: &mut u8,
    selected_lossy: &mut bool,
    selected_file_path: &mut String,
    base_radius: &mut bool,
    diagonal_pixels: &mut bool,
    extra_radius: &mut bool,
    extra_extra_radius: &mut bool,
) {
    clearscreen::clear().unwrap();
    disable_raw_mode().unwrap();

    if (*mode == 0) && left {
        *sel = 0;
    } else if (*mode == 0) && right {
        *sel = 1;
    } else if *mode == 2 && left {
        if *sel > 0 {
            *sel -= 1;
        } else {
            *sel = 0;
        }
    } else if *mode == 2 && right {
        if *sel < 2 {
            *sel += 1;
        } else {
            *sel = 2;
        }
    } else if *mode == 1 && left {
        if !*selected_lossy && *sel == 3 {
            *sel = 1;
        } else if *sel > 0 {
            *sel -= 1;
        } else {
            *sel = 0;
        }
    } else if *mode == 1 && right {
        if !*selected_lossy && *sel == 1 {
            *sel = 3;
        } else if *sel < 4 {
            *sel += 1;
        } else {
            *sel = 4;
        }
    } else if *mode == 3 && left {
        if *sel > 0 {
            *sel -= 1;
        } else {
            *sel = 0;
        }
    } else if *mode == 3 && right {
        if *sel < 4 {
            *sel += 1;
        } else {
            *sel = 4;
        }
    } else if *mode == 3 && enter {
        if *sel == 0 {
            *mode = 1;
        } else if *sel == 1 {
            *base_radius = !*base_radius;
        } else if *sel == 2 {
            *diagonal_pixels = !*diagonal_pixels;
        } else if *sel == 3 {
            *extra_radius = !*extra_radius;
        } else if *sel == 4 {
            *extra_extra_radius = !*extra_extra_radius;
        }
    } else if *mode == 1 && enter && *sel == 4 {
        *mode = 0;
        *sel = 0;
    } else if *mode == 1 && enter && *sel == 2 {
        *mode = 3;
        *sel = 1;
    } else if *mode == 0 && enter {
        if *sel == 1 {
            *mode = 2;
            *sel = 0;
        } else {
            *mode = 1;
            *sel = 0;
        }
    } else if *mode == 1 && enter && *sel == 1 {
        *selected_lossy = !*selected_lossy
    } else if (*mode == 1 || *mode == 2) && enter && *sel == 0 {
        *selected_file_path = FileDialog::new()
            .pick_file()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    } else if *mode == 1 && enter && if *selected_lossy {
        *sel == 3
    } else {
        *sel == 2
    } {
        disable_raw_mode().unwrap();
        let name = std::path::Path::new(selected_file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_string();
        clearscreen::clear().unwrap();
        convert(
            selected_file_path,
            &(name + ".pcf"),
            true,
            *selected_lossy,
            *base_radius,
            *diagonal_pixels,
            *extra_radius,
            *extra_extra_radius,
        );
        exit(0);
    } else if *mode == 2 && enter && *sel == 1 {
        let output_file = FileDialog::new()
            .set_file_name("output.png")
            .save_file()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        clearscreen::clear().unwrap();
        decode(selected_file_path.to_string(), output_file, true);
        exit(0);
    } else if *mode == 2 && enter && *sel == 2 {
        *mode = 0;
        *sel = 0;
    }
    if *mode == 3 {
        println!(
            "Pixel Clustering Format 3000 -- Lossy Settings\nNOTE: The more options enabled, the better the image will look, but the bigger it will be.\n      Try different settings combinations, there usually isn't a one-size-fits-all solution.\n\n         Base -- Diagonal -- Extra -- Extra²\n{}  {}   {}       {}    {}",
            if *sel == 0 {
                Colorize::underline("GO BACK").bright_blue()
            } else {
                Colorize::white("GO BACK")
            },
            if *sel == 1 {
                format!("{}", base_radius.to_string().bright_blue().underline())
            } else {
                format!("{}", base_radius.to_string())
            },
            if *sel == 2 {
                format!("{}", diagonal_pixels.to_string().bright_blue().underline())
            } else {
                format!("{}", diagonal_pixels.to_string())
            },
            if *sel == 3 {
                format!("{}", extra_radius.to_string().bright_blue().underline())
            } else {
                format!("{}", extra_radius.to_string())
            },
            if *sel == 4 {
                format!("{}", extra_extra_radius.to_string().bright_blue().underline())
            } else {
                format!("{}", extra_extra_radius.to_string())
            },
        );
    } else if *mode == 2 {
        println!(
            "Pixel Clustering Format 3000\n\n{}    {}    {}",
            if *sel == 0 {
                Colorize::underline("Choose file").bright_blue()
            } else {
                Colorize::white("Choose file")
            },
            if *sel == 1 {
                Colorize::underline("Go!").bright_blue()
            } else {
                Colorize::white("Go!")
            },
            if *sel == 2 {
                Colorize::underline("GO BACK").bright_blue()
            } else {
                Colorize::white("GO BACK")
            }
        );
    } else if *mode == 1 {
        println!(
            "Pixel Clustering Format 3000\n\n{}    {}    {}    {}    {}",
            if *sel == 0 {
                Colorize::underline("Choose file").bright_blue()
            } else {
                Colorize::white("Choose file")
            },
            if *sel == 1 {
                Colorize::underline({
                    if *selected_lossy {
                        "Lossy"
                    } else {
                        "Lossless"
                    }
                })
                    .bright_blue()
            } else {
                Colorize::white({
                    if *selected_lossy {
                        "Lossy"
                    } else {
                        "Lossless"
                    }
                })
            },
            if *selected_lossy {
                if *sel == 2 {
                    Colorize::underline("Settings").bright_blue()
                } else {
                    ColoredString::from("Settings")
                }
            } else {
                ColoredString::from("")
            },
            if *sel == 3 {
                Colorize::underline("Go!").bright_blue()
            } else {
                Colorize::white("Go!")
            },
            if *sel == 4 {
                Colorize::underline("GO BACK").bright_blue()
            } else {
                Colorize::white("GO BACK")
            },
        );
    } else if *mode == 0 {
        println!(
            "Pixel Clustering Format 3000\n\n{}           {}",
            if *sel == 0 {
                Colorize::underline("Encode").bright_blue()
            } else {
                Colorize::white("Encode")
            },
            if *sel == 1 {
                Colorize::underline("Decode").bright_blue()
            } else {
                Colorize::white("Decode")
            }
        );
    }

    crossterm::terminal::enable_raw_mode().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut mode: u8 = 0;
        let mut sel: u8 = 0;
        let mut selected_lossy = false;
        let mut selected_file_path = String::new();
        let mut base_radius = false;
        let mut diagonal_pixels = false;
        let mut extra_radius = false;
        let mut extra_extra_radius = false;
        display_menu(
            false,
            false,
            false,
            &mut mode,
            &mut sel,
            &mut selected_lossy,
            &mut selected_file_path,
            &mut base_radius,
            &mut diagonal_pixels,
            &mut extra_radius,
            &mut extra_extra_radius
        );
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            if poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    if event.kind == KeyEventKind::Press {
                        if event.code == KeyCode::Left {
                            display_menu(
                                true,
                                false,
                                false,
                                &mut mode,
                                &mut sel,
                                &mut selected_lossy,
                                &mut selected_file_path,
                                &mut base_radius,
                                &mut diagonal_pixels,
                                &mut extra_radius,
                                &mut extra_extra_radius
                            );
                        } else if event.code == KeyCode::Right {
                            display_menu(
                                false,
                                true,
                                false,
                                &mut mode,
                                &mut sel,
                                &mut selected_lossy,
                                &mut selected_file_path,
                                &mut base_radius,
                                &mut diagonal_pixels,
                                &mut extra_radius,
                                &mut extra_extra_radius
                            );
                        } else if event.code == KeyCode::Enter {
                            display_menu(
                                false,
                                false,
                                true,
                                &mut mode,
                                &mut sel,
                                &mut selected_lossy,
                                &mut selected_file_path,
                                &mut base_radius,
                                &mut diagonal_pixels,
                                &mut extra_radius,
                                &mut extra_extra_radius
                            );
                        } else if event.code == KeyCode::Esc || event.code == KeyCode::Char('q') {
                            disable_raw_mode().unwrap();
                            return;
                        }
                    }

                }
            }
        }
    } else if args.contains(&"--decode".to_string()) {
        decode(
            args[1].clone(),
            "output.png".to_string(),
            args.contains(&"--verbose".to_string()),
        );
    } else {
        let file_path = args[1].to_string();
        let name = std::path::Path::new(&file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_string();
        convert(
            &args[1],
            &(name + ".pcf"),
            args.contains(&"--verbose".to_string()),
            args.contains(&"--lossy".to_string()),
            true,
            false,
            false,
            false,
        );
    }
    disable_raw_mode().unwrap();
}
