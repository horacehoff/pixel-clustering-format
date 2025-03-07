use std::env;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

mod data;
mod decode;
mod encode;

use crate::data::compare;
use crate::decode::decode;
use crate::encode::convert;
use colored::{ColoredString, Colorize};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use egui::{IconData, Vec2};
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

    if *mode == 0 && left {
        if *sel > 0 {
            *sel -= 1;
        } else {
            *sel = 0;
        }
    } else if *mode == 0 && right {
        if *sel < 2 {
            *sel += 1;
        } else {
            *sel = 2;
        }
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
        } else if *sel == 0 {
            *mode = 1;
            *sel = 0;
        } else if *sel == 2 {
            disable_raw_mode().unwrap();
            exit(0);
        }
    } else if *mode == 1 && enter && *sel == 1 {
        *selected_lossy = !*selected_lossy;
    } else if (*mode == 1 || *mode == 2) && enter && *sel == 0 {
        *selected_file_path = FileDialog::new()
            .add_filter(
                "image",
                &[
                    "avif", "bmp", "ff", "gif", "hdr", "ico", "jpeg", "jpg", "exr", "png", "pnm",
                    "qoi", "tga", "tif", "webp",
                ],
            )
            .pick_file()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    } else if *mode == 1 && enter && *sel == 3 {
        disable_raw_mode().unwrap();
        let mut output_file = FileDialog::new()
            .set_file_name("output.pcf")
            .save_file()
            .unwrap();
        convert(
            selected_file_path,
            output_file.to_str().unwrap(),
            *selected_lossy,
            *base_radius,
            *diagonal_pixels,
            *extra_radius,
            *extra_extra_radius,
        );
        output_file.pop();
        open_file_explorer(output_file.to_str().unwrap().to_string());
        exit(0);
    } else if *mode == 2 && enter && *sel == 1 {
        disable_raw_mode().unwrap();
        let mut output_file = FileDialog::new()
            .set_file_name("output.png")
            .save_file()
            .unwrap();
        clearscreen::clear().unwrap();
        decode(
            (*selected_file_path).to_string(),
            output_file.to_str().unwrap(),
        );
        output_file.pop();
        open_file_explorer(output_file.to_str().unwrap().to_string());
        exit(0);
    } else if *mode == 2 && enter && *sel == 2 {
        *mode = 0;
        *sel = 0;
    }
    if *mode == 3 {
        println!(
            "Pixel Clustering Format 3000 -- Lossy Settings\nby Horace Hoff\nNOTE: These settings control the range of the palette used by the dithering algorithm on a per-pixel basis.\n      The more options enabled, the better the image will look, but the bigger it will be.\n      Try different settings combinations, there usually isn't a one-size-fits-all solution.\n\n         Base -- Diagonal -- Extra -- Extra-Extra\n{}  {}   {}       {}    {}",
            if *sel == 0 {
                Colorize::underline("GO BACK").bright_blue()
            } else {
                Colorize::white("GO BACK")
            },
            if *sel == 1 {
                format!("{}", base_radius.to_string().bright_blue().underline())
            } else {
                base_radius.to_string()
            },
            if *sel == 2 {
                format!("{}", diagonal_pixels.to_string().bright_blue().underline())
            } else {
                diagonal_pixels.to_string()
            },
            if *sel == 3 {
                format!("{}", extra_radius.to_string().bright_blue().underline())
            } else {
                extra_radius.to_string()
            },
            if *sel == 4 {
                format!("{}", extra_extra_radius.to_string().bright_blue().underline())
            } else {
                extra_extra_radius.to_string()
            },
        );
    } else if *mode == 2 {
        println!(
            "Pixel Clustering Format 3000\nby Horace Hoff\n\n{}    {}    {}",
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
            "Pixel Clustering Format 3000\nby Horace Hoff\n\n{}    {}    {} {}    {}",
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
                    format!("{}   ", Colorize::underline("Settings").bright_blue())
                } else {
                    "Settings   ".parse().unwrap()
                }
            } else {
                ColoredString::from("").parse().unwrap()
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
            "Pixel Clustering Format 3000\nby Horace Hoff\n\n{}       {}       {}",
            if *sel == 0 {
                Colorize::underline("Encode").bright_blue()
            } else {
                Colorize::white("Encode")
            },
            if *sel == 1 {
                Colorize::underline("Decode").bright_blue()
            } else {
                Colorize::white("Decode")
            },
            if *sel == 2 {
                Colorize::underline("QUIT").bright_blue()
            } else {
                Colorize::white("QUIT")
            }
        );
    }

    crossterm::terminal::enable_raw_mode().unwrap();
}

fn tui() {
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
        &mut extra_extra_radius,
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
                            &mut extra_extra_radius,
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
                            &mut extra_extra_radius,
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
                            &mut extra_extra_radius,
                        );
                    } else if event.code == KeyCode::Esc
                        || event.code == KeyCode::Char('q')
                        || (event.modifiers == KeyModifiers::CONTROL
                        && (event.code == KeyCode::Char('c')
                        || event.code == KeyCode::Char('z')))
                    {
                        disable_raw_mode().unwrap();
                        exit(0);
                    }
                }
            }
        }
    }
}

fn gui() {
    #[derive(PartialEq)]
    enum Modes {
        Encode,
        Decode,
    }
    let mut mode = Modes::Encode;
    let mut selected_file_path = String::new();
    let mut selected_decode_file_path = String::new();
    let mut is_lossy = false;
    let mut base_pixels = true;
    let mut diagonal_pixels = false;
    let mut extra_pixels = false;
    let mut extra_extra_pixels = false;

    let mut options = eframe::NativeOptions::default();
    options.viewport.icon = Some(Arc::from(IconData::default()));
    options.viewport.inner_size = Option::from(Vec2::new(310.0, 458.0));
    options.viewport.resizable = Option::from(false);
    eframe::run_simple_native("Pixel Clustering Format - GUI 3000", options, move |ctx, _frame| {
        ctx.set_zoom_factor(1.1);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Pixel Clustering Format");
            ui.label("by Horace Hoff");
            ui.horizontal(|ui| {
                ui.hyperlink_to("GitHub", "https://github.com/horacehoff/pixel-clustering-format");
                ui.hyperlink_to("Website", "https://horacehoff.github.io");
            });
            ui.add_space(5.0);
            ui.horizontal_top(|ui| {
                ui.selectable_value(&mut mode, Modes::Encode, "Encode");
                ui.selectable_value(&mut mode, Modes::Decode, "Decode");
            });
            if mode == Modes::Encode {
                ui.group(|ui| {
                    if ui.button("Pick File").clicked() {
                        selected_file_path = FileDialog::new()
                            .add_filter(
                                "image",
                                &[
                                    "avif", "bmp", "ff", "gif", "hdr", "ico", "jpeg", "jpg", "exr", "png", "pnm",
                                    "qoi", "tga", "tif", "webp",
                                ],
                            )
                            .pick_file()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                    }
                    ui.checkbox(&mut is_lossy, "Lossy Compression");
                    if is_lossy {
                        ui.collapsing("Lossy Compression Settings", |ui| {
                            ui.label("These settings control the range of the palette used by the dithering algorithm on a per-pixel basis.");
                            ui.label("The more options enabled, the better the image will look, but the bigger it will be.");
                            ui.label("Try different settings combinations, there usually isn't a one-size-fits-all solution.");
                            ui.add_enabled(diagonal_pixels || extra_pixels || extra_extra_pixels,egui::Checkbox::new(&mut base_pixels, "Base pixels"));
                            ui.add_enabled(base_pixels || extra_pixels || extra_extra_pixels,egui::Checkbox::new(&mut diagonal_pixels, "Diagonal pixels"));
                            ui.add_enabled(base_pixels || diagonal_pixels || extra_extra_pixels,egui::Checkbox::new(&mut extra_pixels, "Extra pixels"));
                            ui.add_enabled(base_pixels || extra_pixels || diagonal_pixels,egui::Checkbox::new(&mut extra_extra_pixels, "Extra-Extra pixels"));
                        });
                    }
                    if ui.add_enabled(!selected_file_path.is_empty(),egui::Button::new("Start")).clicked() {
                        let mut output_file = FileDialog::new()
                            .set_file_name("output.pcf")
                            .save_file()
                            .unwrap();
                        convert(
                            &selected_file_path,
                            output_file.to_str().unwrap(),
                            is_lossy,
                            base_pixels,
                            diagonal_pixels,
                            extra_pixels,
                            extra_extra_pixels,
                        );
                        output_file.pop();
                        open_file_explorer(output_file.to_str()
                            .unwrap()
                            .to_string());
                    };
                })
            } else {
                ui.group(|ui| {
                    if ui.button("Pick File").clicked() {
                        selected_decode_file_path = FileDialog::new()
                            .add_filter(
                                "image",
                                &[
                                    "pcf",
                                ],
                            )
                            .pick_file()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                    }
                    if ui.add_enabled(!selected_decode_file_path.is_empty(),egui::Button::new("Start")).clicked() {
                        let mut output_file = FileDialog::new()
                            .set_file_name("output.png")
                            .save_file()
                            .unwrap();
                        clearscreen::clear().unwrap();
                        decode(selected_decode_file_path.to_string(), output_file.to_str()
                            .unwrap());
                        output_file.pop();
                        open_file_explorer(output_file.to_str()
                            .unwrap()
                            .to_string());
                    };
                })
            };
            ui.vertical_centered(|ui| {
                ui.label("Ad Astra Per Aspera");
            })
        });
    }).unwrap();
}

fn open_file_explorer(path: String) {
    if cfg!(windows) {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // tui();
        gui();
    } else if args.contains(&"--decode".to_string()) {
        decode(args[1].clone(), "output.png");
    } else {
        let file_path = args[1].to_string();
        let name = std::path::Path::new(&file_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .collect::<Vec<&str>>()[0]
            .to_string();
        convert(
            &args[1],
            &(name + ".pcf"),
            args.contains(&"--lossy".to_string()),
            true,
            false,
            false,
            false,
        );
    }
    // disable_raw_mode().unwrap();
}
