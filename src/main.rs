#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_extras::RetainedImage;
use std::io::Read;
use rust_clock::RustClock;

use eframe::egui;
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem}, TrayIconBuilder,
};
#[cfg(not(target_os = "linux"))]
use std::{cell::RefCell, rc::Rc};
use ini::Ini;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    let mut dir = std::env::current_exe().unwrap();
    dir.pop();
    dir.push("conf.ini");
    let ini_path = dir.as_path();
    let result = fs::read_to_string(ini_path);
    if let Err(_) = result {
        let mut ini_default = "[Config]\ntime=:30:,:00:\n# sound=assets/sound.ogg\n# countdown=:20:,::20\n# pos=left,5%\n# round=0\n# show_time=1000\n# bg=assets/bg.png\n# init_show=0\n# timezone=+8\n# time_font=\n# tips=by the grave and thee\n# font_path=".to_string();
        #[cfg(target_os = "windows")]
        {
            ini_default = ini_default.to_owned() + &"C:/Windows/Fonts/msyh.ttc";
        }
        #[cfg(target_os = "macos")]
        {
            ini_default = ini_default.to_owned() + &"/System/Library/Fonts/STHeiti Light.ttc";
        }

        fs::write(ini_path, ini_default).unwrap();
    }

    let i = Ini::load_from_file(ini_path).unwrap();
    dir.pop();
    let mut sound_path = "".to_string();
    let mut time_str = "".to_string();
    let mut countdown = "".to_string();
    let mut pos_dir = "left".to_string();
    let mut pos_pc = -1;
    let mut custom_bg_color = "".to_string();
    let mut custom_border_color = "".to_string();
    let mut custom_number_bg_color = "".to_string();
    let mut custom_number_color = "".to_string();
    let mut custom_clock_bg_color = "".to_string();
    let mut tips_store = "".to_string();
    let mut font_path = "".to_string();
    let mut init_show = 1;
    let mut timezone = 0;
    let mut custom_timezone = false;
    let mut round = true;
    let mut time_font = "".to_string();
    let mut show_time = 0.0;
    let mut time_countdown = false;
    let mut image = Err("".to_string());
    for (sec, prop) in i.iter() {
        if let Some(s) = sec {
            if s == "Config" {
                for (k, v) in prop.iter() {
                    if k == "time" {
                        time_str = v.to_string();
                    } else if k == "sound" {
                        sound_path = String::from(dir.as_path().to_string_lossy()) + &"/*" + &v.to_string();
                    } else if k == "countdown" {
                        countdown = v.to_string();
                    } else if k == "pos" {
                        let v_str = v.to_string();
                        let pos_arr: Vec<&str> = v_str.split(',').collect();
                        pos_dir = pos_arr[0].to_string();
                        if pos_arr.len() == 2 {
                            pos_pc = pos_arr[1].replace("%", "").to_string().parse::<i32>().unwrap();
                        }
                    } else if k == "bg_color" {
                        custom_bg_color = v.to_string();
                    } else if k == "border_color" {
                        custom_border_color = v.to_string();
                    } else if k == "number_bg_color" {
                        custom_number_bg_color = v.to_string();
                    } else if k == "number_color" {
                        custom_number_color = v.to_string();
                    } else if k == "clock_bg_color" {
                        custom_clock_bg_color = v.to_string();
                    } else if k == "tips" {
                        tips_store = v.to_string();
                    } else if k == "font_path" {
                        font_path = v.to_string();
                    } else if k == "show_time" {
                        show_time = v.to_string().parse::<f32>().unwrap();
                    } else if k == "init_show" {
                        init_show = v.to_string().parse::<i32>().unwrap();
                    } else if k == "timezone" {
                        timezone = v.to_string().parse::<i32>().unwrap();
                        custom_timezone = true;
                    } else if k == "time_font" {
                        time_font = v.to_string();
                    } else if k == "round" {
                        round = v != "0";
                    } else if k == "time_countdown" {
                        time_countdown = v != "0";
                    } else if k == "bg" {
                        let bg_path = std::path::PathBuf::from(v);
                        let result = fs::File::open(&bg_path);
                        if let Ok(mut bg_file) = result {
                            let mut buffer = vec![];
                            bg_file.read_to_end(&mut buffer).unwrap();
                            image = RetainedImage::from_image_bytes("clock", &buffer[..]);
                        }
                    }
                }
            }
        }
    }

    dir.push("assets");
    dir.push("icon.png");
    let mut icon_exist = false;
    let result = fs::read(dir.as_path());
    if let Ok(_) = result {
        icon_exist = true;
    }

    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("Quit", true, None);
    let countdown_i = MenuItem::new("Countdown", true, None);
    tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("Rust clock".to_string()),
                copyright: Some("Copyright Hoothin @ 2023".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &countdown_i,
        &PredefinedMenuItem::separator(),
        &quit_i,
    ]);

    #[cfg(not(target_os = "linux"))]
    let mut _tray_icon = Rc::new(RefCell::new(None));
    #[cfg(not(target_os = "linux"))]
    let tray_c = _tray_icon.clone();

    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        always_on_top: true,
        run_and_return: true,
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        initial_window_size: Some(egui::vec2(320.0, 100.0)),
        default_theme: eframe::Theme::Dark,
        multisampling: 2,
        ..Default::default()
    };
    eframe::run_native(
        "Rust clock", // unused title
        options,
        Box::new(move |_cc| {
            #[cfg(not(target_os = "linux"))]
            {
                tray_c
                    .borrow_mut()
                    .replace(
                        if icon_exist == true {
                            let icon = load_icon(dir.as_path());
                            TrayIconBuilder::new()
                                .with_menu(Box::new(tray_menu))
                                .with_tooltip("Rust clock")
                                .with_icon(icon)
                                .build()
                                .unwrap()
                        } else {
                            TrayIconBuilder::new()
                                .with_menu(Box::new(tray_menu))
                                .with_tooltip("Rust clock")
                                .with_title("⏰")
                                .build()
                                .unwrap()
                        }
                    );
            }
            Box::new(RustClock::new(
                quit_i.id(),
                time_str,
                sound_path,
                countdown,
                countdown_i.id(),
                pos_dir,
                pos_pc,
                custom_bg_color,
                custom_border_color,
                custom_number_bg_color,
                custom_number_color,
                custom_clock_bg_color,
                tips_store,
                font_path,
                show_time,
                image,
                init_show,
                timezone,
                custom_timezone,
                time_font,
                round,
                time_countdown
            ).unwrap())
        }),
    )
}

fn load_icon(path: &std::path::Path) -> tray_icon::icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon")
}
