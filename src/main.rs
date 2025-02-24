use std::env;
use std::io::stdout;
use std::process::exit;
use std::time::Duration;

mod data;
mod decode;
mod encode;

use crate::decode::decode;
use crate::encode::convert;
use colored::Colorize;
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, ClearType};
use rfd::FileDialog;

fn display_menu(
    left: bool,
    right: bool,
    enter: bool,
    mode: &mut u8,
    sel: &mut u8,
    selected_lossy: &mut bool,
    selected_file_path: &mut String,
) {
    execute!(
        stdout(),
        crossterm::terminal::SetSize(80, 20),
        crossterm::terminal::Clear(ClearType::All)
    )
        .unwrap();
    disable_raw_mode().unwrap();

    if (*mode == 0 || *mode == 2) && left {
        *sel = 0;
    }
    if (*mode == 0 || *mode == 2) && right {
        *sel = 1;
    }

    if *mode == 1 && left {
        if *sel > 0 {
            *sel -= 1;
        } else {
            *sel = 0;
        }
    }
    if *mode == 1 && right {
        if *sel < 2 {
            *sel += 1;
        } else {
            *sel = 2;
        }
    }

    if *mode == 0 && enter {
        if *sel == 1 {
            *mode = 2;
        } else {
            *mode = 1;
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
    } else if *mode == 1 && enter && *sel == 2 {
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
            true,
            false,
            false,
            false,
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
        decode(selected_file_path.to_string(), output_file, true);
        exit(0);
    }
    if *mode == 2 {
        println!(
            "Pixel Clustering Format 3000\n\n{}           {}",
            if *sel == 0 {
                Colorize::underline("Choose file").bright_blue()
            } else {
                Colorize::white("Choose file")
            },
            if *sel == 1 {
                Colorize::underline("Go!").bright_blue()
            } else {
                Colorize::white("Go!")
            }
        );
    } else if *mode == 1 {
        println!(
            "Pixel Clustering Format 3000\n\n{}    {}       {}",
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
            if *sel == 2 {
                Colorize::underline("Go!").bright_blue()
            } else {
                Colorize::white("Go!")
            }
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
        crossterm::terminal::enable_raw_mode().unwrap();
        display_menu(
            false,
            false,
            false,
            &mut mode,
            &mut sel,
            &mut selected_lossy,
            &mut selected_file_path,
        );
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
                            );
                        } else if event.code == KeyCode::Esc || event.code == KeyCode::Char('q') {
                            disable_raw_mode().unwrap();
                            return;
                        }
                    }

                }
            }
        }
        disable_raw_mode().unwrap();
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
