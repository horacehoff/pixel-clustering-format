use std::env;
use std::process::exit;
use std::time::Duration;

mod data;
mod decode;
mod encode;

use crate::decode::decode;
use crate::encode::convert;
use colored::{ColoredString, Colorize};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::style::Stylize;
use crossterm::terminal::disable_raw_mode;
use iced::alignment::Horizontal;
use iced::widget::button::Status;
use iced::widget::{button, column, container, row, text, toggler};
use iced::{Element, Fill, Size, Theme};
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
        *selected_lossy = !*selected_lossy
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
    iced::application("Pixel Clustering Format", update, view)
        // .theme(|_| Theme::Nightfly)
        .window_size(Size {
            width: 350.0,
            height: 400.0,
        })
        .run()
        .unwrap();
}

#[derive(Debug, Clone)]
enum Command {
    SwitchFirst,
    SwitchSecond,
    ToggleLossy,
    ToggleBase,
    ToggleDiagonal,
    ToggleExtra,
    ToggleExtraExtra,
    SetFile,
    SetDecodeFile,
    Convert,
    Decode,
}

#[derive(Default)]
struct Data {
    mode: u8,
    is_lossy: bool,
    open_settings: bool,
    base_radius: bool,
    diagonal_pixels: bool,
    extra_pixels: bool,
    extra_extra_pixels: bool,
    selected_file: String,
    selected_decode_file:String,
}

fn open_file_explorer(path: String) {
    if cfg!(windows) {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .unwrap();
    } else {
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .unwrap();
    }
}

fn update(data: &mut Data, command: Command) {
    match command {
        Command::SwitchFirst => {
            data.mode = 0;
        }
        Command::SwitchSecond => {
            data.mode = 1;
        }
        Command::ToggleLossy => {
            data.is_lossy = !data.is_lossy;
        }
        Command::ToggleBase => {
            data.base_radius = !data.base_radius;
        }
        Command::ToggleDiagonal => {
            data.diagonal_pixels = !data.diagonal_pixels;
        }
        Command::ToggleExtra => {
            data.extra_pixels = !data.extra_pixels;
        }
        Command::ToggleExtraExtra => {
            data.extra_extra_pixels = !data.extra_extra_pixels;
        }
        Command::SetFile => {
            data.selected_file = FileDialog::new()
                .add_filter(
                    "image",
                    &[
                        "avif", "bmp", "ff", "gif", "hdr", "ico", "jpeg", "jpg", "exr", "png",
                        "pnm", "qoi", "tga", "tif", "webp",
                    ],
                )
                .pick_file()
                .unwrap_or("".parse().unwrap())
                .to_str()
                .unwrap_or("")
                .to_string();
        },
        Command::SetDecodeFile => {
            data.selected_decode_file = FileDialog::new()
                .add_filter(
                    "image",
                    &[
                        "pcf",
                    ],
                )
                .pick_file()
                .unwrap_or("".parse().unwrap())
                .to_str()
                .unwrap_or("")
                .to_string();
        }
        Command::Convert => {
            let name = std::path::Path::new(&data.selected_file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>()[0]
                .to_string();
            let mut output_file = FileDialog::new()
                .set_file_name("output.pcf")
                .save_file()
                .unwrap();
            convert(
                &data.selected_file,
                output_file.to_str().unwrap(),
                false,
                data.is_lossy,
                true,
                false,
                false,
                false,
            );
            // std::fs::copy(std::path::Path::new(&(name.to_string() + ".pcf")), output_file.clone()).unwrap();
            // std::fs::remove_file(std::path::Path::new(&(name + ".pcf"))).unwrap();
            output_file.pop();
            open_file_explorer(output_file.to_str()
                .unwrap()
                .to_string());
        }
        Command::Decode => {
            let mut output_file = FileDialog::new()
                .set_file_name("output.png")
                .save_file()
                .unwrap();
            clearscreen::clear().unwrap();
            decode(data.selected_decode_file.to_string(), output_file.to_str()
                .unwrap()
                .to_string(), true);
            output_file.pop();
            open_file_explorer(output_file.to_str()
                                   .unwrap()
                                   .to_string());
        }
    }
}

fn view(data: &Data) -> Element<Command> {
    container(
        column![
            text("Pixel Clustering Format"),
        row![
        button("Convert").style(button::primary).on_press(Command::SwitchFirst),
        button("Decode").on_press(Command::SwitchSecond),
    ],
      if data.mode == 0 {
          column![
              button("Pick file").on_press(Command::SetFile),
                toggler(data.is_lossy).label("Lossy Compression")
                .on_toggle(|_| Command::ToggleLossy)
                    ,
                if data.is_lossy {
                    column![
                            text("These settings dictate which pixels are used to generate the palette of available colors, for each pixel of the image. Some combinations of these options will give better results depending on the chosen image. Please note that the more options are enabled, the better the image will look, but the file size will in turn potentially be higher."),
                        toggler(data.base_radius).label("Base pixels")
                            .on_toggle(|_| Command::ToggleBase),
                        toggler(data.diagonal_pixels).label("Diagonal pixels")
                            .on_toggle(|_| Command::ToggleDiagonal),
                        toggler(data.extra_pixels).label("Extra pixels")
                            .on_toggle(|_| Command::ToggleExtra),
                        toggler(data.extra_extra_pixels).label("Extra Extra pixels")
                            .on_toggle(|_| Command::ToggleExtraExtra)
                    ].align_x(Horizontal::Left)
                } else {
                     column![]
                },
                button("Convert!").on_press(Command::Convert).style(|theme: &Theme, status|{button::primary(theme, if data.selected_file.is_empty() {Status::Disabled} else {status})})
         ].align_x(Horizontal::Center)
      } else {
          column![
              button("Pick file").on_press(Command::SetDecodeFile),
              button("Decode").on_press(Command::Decode).style(|theme: &Theme, status|{button::primary(theme, if data.selected_decode_file.is_empty() {Status::Disabled} else {status})})
          ]
      }].align_x(Horizontal::Center)
    ).center_x(Fill).into()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // tui();
        gui();
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
