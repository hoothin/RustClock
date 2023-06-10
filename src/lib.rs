use crate::egui::Color32;
use crate::egui::Pos2;
use egui_extras::image::RetainedImage;
use eframe::egui;
use tray_icon::{
    menu::{MenuEvent},
    TrayEvent,
};

use chrono::{DateTime, Timelike, Local};

use std::fs;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub struct RustClock {
    quit_index: u32,
    time: f32,
    time2show: String,
    tikpop: bool,
    visible: bool,
    last_pos_x: f32,
    last_pos_y: f32,
    last_visible: bool,
    sound_path: String,
    countdown: String,
    countdown_index: u32,
    inited: bool,
    countdown_start: bool,
    countdown_start_time: i64,
    in_time_popup: bool,
    pos_pc: i32,
    pos_dir: String,
    init_x: f32,
    init_y: f32,
    custom_bg_color: String,
    custom_border_color: String,
    custom_number_bg_color: String,
    custom_number_color: String,
    custom_clock_bg_color: String,
    tips_store: String,
    show_tips: String,
    font_path: String,
    show_time: f32,
    now: DateTime<Local>,
    image: Result<RetainedImage, String>
}

impl RustClock {
    pub fn new(
        quit_index: u32,
        time2show: String,
        sound_path: String,
        countdown: String,
        countdown_index: u32,
        pos_dir: String,
        pos_pc: i32,
        custom_bg_color: String,
        custom_border_color: String,
        custom_number_bg_color: String,
        custom_number_color: String,
        custom_clock_bg_color: String,
        tips_store: String,
        font_path: String,
        show_time: f32,
        image: Result<RetainedImage, String>
    ) -> Result<RustClock, &'static str> {
        Ok(RustClock {
            quit_index,
            time: 0.0,
            time2show,
            tikpop: false,
            visible: true,
            last_pos_x: 0.0,
            last_pos_y: 0.0,
            last_visible: false,
            sound_path,
            countdown,
            countdown_index,
            inited: false,
            countdown_start: false,
            countdown_start_time: 0,
            in_time_popup: false,
            pos_pc,
            pos_dir,
            init_x: 0.0,
            init_y: 0.0,
            custom_bg_color,
            custom_border_color,
            custom_number_bg_color,
            custom_number_color,
            custom_clock_bg_color,
            tips_store,
            show_tips: "".to_string(),
            font_path,
            show_time,
            now: Local::now(),
            image
        })
    }
}

impl eframe::App for RustClock {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.inited == false {
            self.inited = true;
            let mut fonts = egui::FontDefinitions::default();
            if self.font_path == "" {
                #[cfg(target_os = "windows")]
                {
                    self.font_path = "C:/Windows/Fonts/msyh.ttc".to_string();
                }
                #[cfg(target_os = "macos")]
                {
                    self.font_path = "/System/Library/Fonts/STHeiti Light.ttc".to_string();
                }
            }
            let result = std::fs::read(&self.font_path);
            if let Ok(font) = result {
                fonts.font_data.insert(
                    "other_font".to_owned(),
                    egui::FontData::from_owned(font)
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "other_font".to_owned());
            }
            fonts.font_data.insert(
                "my_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "my_font".to_owned());
            ctx.set_fonts(fonts);
            self.init_y = 50.0;
            self.init_x = -320.0;
            if self.pos_pc != 0 {
                if let Some(egui::Vec2 { x: _, y }) = frame.info().window_info.monitor_size {
                    let pos = self.pos_pc as f32 / 100.0 * y;
                    self.init_y = pos;
                }
            }
            if self.pos_dir == "right" {
                if let Some(egui::Vec2 { x, y: _ }) = frame.info().window_info.monitor_size {
                    self.init_x = x as f32;
                }
            }
            if self.show_time == 0.0 {
                self.show_time = 100.0;
            } else {
                self.show_time = self.show_time / 16.0;
            }
        }
        
        let mut begin_tik = |index, in_time_popup| {
            self.last_visible = self.visible;
            if self.last_visible == true {
                if let Some(pos) = frame.get_window_pos() {
                    self.last_pos_x = pos.x;
                    self.last_pos_y = pos.y;
                }
            }
            self.visible = true;
            frame.set_visible(self.visible);
            self.time = 0.0;
            frame.set_window_pos(Pos2::new(self.init_x, self.init_y));
            if self.sound_path != "" {
                let mut path = "".to_string();
                let sound_path_arr: Vec<&str> = self.sound_path.split('*').collect();
                let normal_sound:Vec<&str> = sound_path_arr[1].split('|').collect();
                if sound_path_arr.len() == 2 || in_time_popup == true {
                    if normal_sound.len() > index {
                        path = sound_path_arr[0].to_owned() + &normal_sound[index];
                    } else {
                        path = sound_path_arr[0].to_owned() + &normal_sound[0];
                    }
                } else if sound_path_arr.len() == 3 {
                    let countdown_sound:Vec<&str> = sound_path_arr[2].split('|').collect();
                    if countdown_sound.len() > index {
                        path = sound_path_arr[0].to_owned() + &countdown_sound[index];
                    } else {
                        path = sound_path_arr[0].to_owned() + &countdown_sound[0];
                    }
                }
                if path != "" {
                    let result = fs::File::open(&path);
                    if let Ok(file) = result {
                        let file = BufReader::new(file);
                        std::thread::spawn(move || {
                            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                            let source = Decoder::new(file).unwrap();
                            let sink = Sink::try_new(&stream_handle).unwrap();
                            sink.append(source);
                            sink.sleep_until_end();
                        });
                    }
                }
            }
            if self.tips_store != "" {
                let mut tips = "".to_string();
                let tips_arr: Vec<&str> = self.tips_store.split('*').collect();
                let normal_tips:Vec<&str> = tips_arr[0].split('|').collect();
                if tips_arr.len() == 1 || in_time_popup == true {
                    if normal_tips.len() > index {
                        tips = normal_tips[index].to_string();
                    } else {
                        tips = normal_tips[0].to_string();
                    }
                } else if tips_arr.len() == 2 {
                    let countdown_tips:Vec<&str> = tips_arr[1].split('|').collect();
                    if countdown_tips.len() > index {
                        tips = countdown_tips[index].to_string();
                    } else {
                        tips = countdown_tips[0].to_string();
                    }
                }
                if tips != "" {
                    self.show_tips = tips;
                }
            }
            ctx.request_repaint();
        };
        self.now = Local::now();
        let mut custom_clock = "".to_string();
        if self.countdown_start == true && self.in_time_popup == false {
            let timestamp = self.now.timestamp();
            let over_time = (timestamp - self.countdown_start_time) as i32;
            if self.countdown == "" {
                if over_time > 600 {
                    self.countdown_start_time = timestamp;
                    if self.tikpop == false {
                        begin_tik(0, self.in_time_popup);
                        self.tikpop = true;
                    }
                }
                let left_time = 600.0 - over_time as f32;
                let minute = (left_time / 60.0) as u32;
                let second = (left_time % 60.0) as u32;
                custom_clock = format!("00:{:02}:{:02}", minute, second);
            } else {
                let countdown_arr: Vec<&str> = self.countdown.split(',').collect();
                let mut total_time:i32 = 0;
                let mut first_time:i32 = 0;
                let mut index:i32 = 0;
                for x in &countdown_arr {
                    let single_time: Vec<&str> = x.split(':').collect();
                    let mut cur_time:i32 = 0;
                    if single_time[0] != "" {
                        cur_time = cur_time + single_time[0].to_string().parse::<i32>().unwrap() * 3600;
                    }
                    if single_time[1] != "" {
                        cur_time = cur_time + single_time[1].to_string().parse::<i32>().unwrap() * 60;
                    }
                    if single_time[2] != "" {
                        cur_time = cur_time + single_time[2].to_string().parse::<i32>().unwrap();
                    }
                    if first_time == 0 {
                        first_time = cur_time;
                    }
                    total_time = total_time + cur_time;
                    if self.tikpop == false && over_time == total_time.into() {
                        begin_tik(index.try_into().unwrap(), self.in_time_popup);
                        self.tikpop = true;
                    } else if over_time < total_time {
                        let left_time = (total_time - over_time) as f32;
                        let hour = (left_time / 60.0 / 60.0) as u32;
                        let minute = (left_time / 60.0) as u32;
                        let second = (left_time % 60.0) as u32;
                        custom_clock = format!("{:02}:{:02}:{:02}", hour, minute, second);
                        break;
                    }
                    index = index + 1;
                }
                if custom_clock == "" {
                    self.countdown_start_time = timestamp;
                    let left_time = first_time as f32;
                    let hour = (left_time / 60.0 / 60.0) as u32;
                    let minute = (left_time / 60.0) as u32;
                    let second = (left_time % 60.0) as u32;
                    custom_clock = format!("{:02}:{:02}:{:02}", hour, minute, second);
                    if self.tikpop == false {
                        begin_tik(0, self.in_time_popup);
                        self.tikpop = true;
                    }
                }
            }
        }
        if self.tikpop == true {
            self.time += 1.0;
            frame.set_mouse_passthrough(false);
            if self.time < 50.0 {
                let mut add_x = (self.time / 100.0 * std::f32::consts::PI).sin() * 320.0;
                if self.pos_dir == "right" {
                    add_x = -add_x;
                }
                frame.set_window_pos(Pos2::new(self.init_x + add_x, self.init_y));
            } else if self.time > self.show_time + 50.0 && self.time < self.show_time + 100.0 {
                let mut add_x = ((self.time - self.show_time - 50.0) / 100.0 * std::f32::consts::PI).sin() * 320.0;
                if self.pos_dir != "right" {
                    add_x = -add_x;
                } else {
                    add_x = self.init_x + add_x - 320.0;
                }
                frame.set_window_pos(Pos2::new(add_x, self.init_y));
            } else if self.time > self.show_time + 100.0 {
                self.tikpop = false;
                self.show_tips = "".to_string();
                self.in_time_popup = false;
                self.visible = self.last_visible;
                frame.set_visible(self.visible);
                if self.visible == true {
                    frame.set_window_pos(Pos2::new(self.last_pos_x, self.last_pos_y));
                }
                frame.set_mouse_passthrough(true);
            }
            if self.visible == false {
                self.tikpop = false;
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        } else {
            self.in_time_popup = false;
            let hour = self.now.hour().to_string();
            let minute = self.now.minute().to_string();
            let second = self.now.second().to_string();
            if self.time2show != "" {
                let time2show_arr: Vec<&str> = self.time2show.split(',').collect();
                let mut index:i32 = 0;
                for x in &time2show_arr {
                    let single_time: Vec<&str> = x.split(':').collect();
                    if (single_time[0] == "" || single_time[0] == hour || single_time[0] == "0".to_string() + &hour) &&
                    (single_time[1] == "" || single_time[1] == minute || single_time[1] == "0".to_string() + &minute) &&
                    ((single_time[2] == "" && second == "0") || single_time[2] == second || single_time[2] == "0".to_string() + &second) {
                        if self.tikpop == false {
                            self.in_time_popup = true;
                            begin_tik(index.try_into().unwrap(), self.in_time_popup);
                            self.tikpop = true;
                        }
                        break;
                    }
                    index = index + 1;
                }
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(300));
        }

        if self.visible == true {
            clock_window_frame(ctx, frame, self, custom_clock);
        }

        if let Ok(TrayEvent {
            event: tray_icon::ClickEvent::Left,
            ..
        }) = tray_icon::TrayEvent::receiver().try_recv()
        {
            self.visible = !self.visible;
            frame.set_visible(self.visible);
            self.tikpop = false;
            self.time = 0.0;
            if self.visible == true {
                frame.set_window_pos(Pos2::new(0.0, self.init_y));
                frame.set_mouse_passthrough(true);
            } else {
                if let Some(pos) = frame.get_window_pos() {
                    self.last_pos_x = pos.x;
                    self.last_pos_y = pos.y;
                }
            }
            ctx.request_repaint();
        }
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == self.quit_index {
                std::process::exit(0)
            } else if event.id == self.countdown_index {
                self.countdown_start = !self.countdown_start;
                if self.countdown_start == true {
                    self.visible = true;
                    frame.set_visible(self.visible);
                    self.countdown_start_time = self.now.timestamp();
                }
                ctx.request_repaint();
            }
        }
    }
}

fn gene_color(color_str: String, default_color: Color32) -> Color32 {
    if color_str == "" {
        return default_color
    }
    let color_arr: Vec<&str> = color_str.split(',').collect();
    if color_arr.len() < 3 {
        return Color32::from_rgb(0, 0, 0)
    }
    if color_arr.len() == 3 {
        return Color32::from_rgb(color_arr[0].to_string().parse::<u8>().unwrap(), 
            color_arr[1].to_string().parse::<u8>().unwrap(), 
            color_arr[2].to_string().parse::<u8>().unwrap())
    }
    return Color32::from_rgba_unmultiplied(color_arr[0].to_string().parse::<u8>().unwrap(), 
        color_arr[1].to_string().parse::<u8>().unwrap(), 
        color_arr[2].to_string().parse::<u8>().unwrap(),  
        color_arr[3].to_string().parse::<u8>().unwrap())
}

fn clock_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    app: &mut RustClock,
    custom_clock: String
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(
                rect.shrink(1.0),
                10.0,
                gene_color(app.custom_bg_color.to_owned(), Color32::from_rgba_premultiplied(32, 33, 36, 200)),
                Stroke::new(1.0, gene_color(app.custom_border_color.to_owned(), text_color)),
            );

            let mut has_bg = false;
            if let Ok(image) = &app.image {
                has_bg = true;
                let mut size = image.size_vec2();
                if size.x / size.y > 1.5 {
                    size *= 100.0 / size.y;
                    image.show_size(ui, size);
                } else {
                    size *= 80.0 / size.y;
                    let mut img_ui = ui.child_ui(Rect::from_points(&[
                            Pos2::new(15.0, 10.0),
                            Pos2::new(95.0, 90.0)
                        ]), *ui.layout());
                    image.show_size(&mut img_ui, size);
                }
            }

            let painter = ui.painter();

            painter.rect_filled(
                Rect::from_points(&[
                    Pos2::new(105.0, 25.0),
                    Pos2::new(305.0, 75.0)
                ]),
                5.0,
                gene_color(app.custom_number_bg_color.to_owned(), Color32::from_rgb(0, 0, 0)),
            );

            // Paint the title:
            if custom_clock == "" {
                painter.text(
                    rect.center_top() + vec2(-41.0, 51.0),
                    Align2::LEFT_CENTER,
                    app.now.format("%H:%M:%S"),
                    FontId::proportional(50.0),
                    gene_color(app.custom_number_color.to_owned(), text_color),
                );
            } else {
                painter.text(
                    rect.center_top() + vec2(-41.0, 51.0),
                    Align2::LEFT_CENTER,
                    custom_clock,
                    FontId::proportional(50.0),
                    gene_color(app.custom_number_color.to_owned(), text_color),
                );
            }

            if app.show_tips != "" {
                painter.rect_filled(
                    Rect::from_points(&[
                        Pos2::new(105.0, 78.0),
                        Pos2::new(305.0, 95.0)
                    ]),
                    1.0,
                    gene_color(app.custom_number_bg_color.to_owned(), Color32::from_rgb(0, 0, 0)),
                );
                painter.text(
                    rect.center_top() + vec2(45.0, 85.0),
                    Align2::CENTER_CENTER,
                    &app.show_tips,
                    FontId::proportional(16.0),
                    gene_color(app.custom_number_color.to_owned(), text_color),
                );
            }

            if has_bg == false {
                painter.circle_filled(
                    Pos2::new(55.0, 50.0),
                    40.0,
                    gene_color(app.custom_clock_bg_color.to_owned(), text_color)
                );
            }

            let (_, hour) = app.now.hour12();
            let minute = app.now.minute() as f32;
            let second = app.now.second() as f32;
            let rad = (hour as f32 + minute / 60.0) / 12.0 * std::f32::consts::PI * 2.0;
            //hour
            painter.line_segment(
                [
                    Pos2::new(55.0, 50.0),
                    Pos2::new(55.0, 50.0) + vec2(rad.sin() * 25.0, rad.cos() * -25.0),
                ],
                Stroke::new(3.0, Color32::from_rgb(0, 0, 0)),
            );
            let rad = minute / 60.0 * std::f32::consts::PI * 2.0;
            //minute
            painter.line_segment(
                [
                    Pos2::new(55.0, 50.0),
                    Pos2::new(55.0, 50.0) + vec2(rad.sin() * 35.0, rad.cos() * -35.0),
                ],
                Stroke::new(2.0, Color32::from_rgb(0, 0, 0)),
            );
            let rad = second / 60.0 * std::f32::consts::PI * 2.0;
            //second
            painter.line_segment(
                [
                    Pos2::new(55.0, 50.0),
                    Pos2::new(55.0, 50.0) + vec2(rad.sin() * 38.0, rad.cos() * -38.0),
                ],
                Stroke::new(1.0, Color32::from_rgb(255, 0, 0)),
            );

            let title_bar_response =
                ui.interact(rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            if app.tikpop == false {
                let close_response = ui.put(
                    Rect::from_min_size(rect.left_top(), Vec2::splat(28.0)),
                    Button::new(RichText::new("•").size(26.0)).frame(false),
                );
                if close_response.clicked() {
                    frame.set_visible(false);
                    app.visible = false;
                }
            }
        });
}