#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::{DateTime, Local};
use eframe::egui;
use egui::{ScrollArea, TextStyle, Ui, Vec2};
use std::fs::{self, DirEntry};
use std::path::Path;
use image;

const BYTE_TO_KB: u64 = 1024;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Folder Fuser 5000!",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Clone)]
struct File {
    name: String,
    size_a: u64,
    date_a: String,
    size_b: u64,
    date_b: String,
}

#[derive(Default)]
struct MyApp {
    matching_files: Vec<File>,
    has_run: bool,
    picked_path_a: Option<String>,
    picked_path_b: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open folder A").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.picked_path_a = Some(path.display().to_string());
                    }
                }

                if let Some(picked_path) = &self.picked_path_a {
                    ui.horizontal(|ui| {
                        ui.label("Selected:");
                        ui.monospace(picked_path);
                    });
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Open folder B").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.picked_path_b = Some(path.display().to_string());
                    }
                }

                if let Some(picked_path) = &self.picked_path_b {
                    ui.horizontal(|ui| {
                        ui.label("Selected:");
                        ui.monospace(picked_path);
                    });
                }
            });

            ui.add_space(10_f32);

            if let Some(picked_path_a) = &self.picked_path_a {
                if let Some(picked_path_b) = &self.picked_path_b {
                    if picked_path_a == picked_path_b {
                        ui.label("WARNING:  Folders are identical");
                    } else {
                        ui.horizontal(|ui| {
                            if ui.button("Find matching files").clicked() {
                                self.has_run = true;
                                find_matching(picked_path_a, picked_path_b, &mut self.matching_files);
                            }
                            if self.has_run {
                                ui.label(format!("{} matches", self.matching_files.len()));
                            }
                        });
                    }
                }
            }

            if !&self.matching_files.is_empty() {
                ui.group(|ui| {
                    let text_style = TextStyle::Body;
                    let row_height = ui.text_style_height(&text_style);
                    let num_rows = self.matching_files.len();
                    ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                        ui,
                        row_height,
                        num_rows,
                        |ui, row_range| {
                            for i in row_range {
                                if i >= self.matching_files.len() {
                                    break;
                                }
                                let file = self.matching_files[i].to_owned();
                                ui.horizontal(|ui| {
                                    let path_a = String::from(format!(
                                            "{}/{}",
                                            self.picked_path_a.to_owned().unwrap(),
                                            file.name
                                            ));
                                    let path_a = Path::new(&path_a);
                                    let path_b = String::from(format!(
                                            "{}/{}",
                                            self.picked_path_a.to_owned().unwrap(),
                                            file.name
                                            ));
                                    let path_b = Path::new(&path_b);
                                    let tooltip_ui = |ui: &mut Ui| {
                                        ui.horizontal(|ui| {
                                            match load_image_from_path(path_a) {
                                                Ok(image) => {
                                                    let texture: egui::TextureHandle = ui.ctx()
                                                        .load_texture(
                                                            "Image A",
                                                            image,
                                                            Default::default()
                                                            );
                                                    ui.image(&texture, Vec2{
                                                        x: 400_f32,
                                                        y: (400 * texture.size()[1] / texture.size()[0]) as f32,
                                                    });
                                                },
                                                Err(_) => {
                                                    ui.label("Unknown type");
                                                }
                                            }
                                            ui.separator();
                                            match load_image_from_path(path_b) {
                                                Ok(image) => {
                                                    let texture: egui::TextureHandle = ui.ctx()
                                                        .load_texture(
                                                            "Image A",
                                                            image,
                                                            Default::default()
                                                            );
                                                    ui.image(&texture, Vec2{
                                                        x: 400_f32,
                                                        y: (400 * texture.size()[1] / texture.size()[0]) as f32,
                                                    });
                                                },
                                                Err(_) => {
                                                    ui.label("Unknown type");
                                                }
                                            }
                                        });
                                    };

                                    ui.label(file.name.to_owned())
                                        .on_hover_ui(tooltip_ui);
                                    ui.add_space(10_f32);
                                    ui.vertical(|ui| {
                                        // File A
                                        ui.horizontal(|ui| {
                                            ui.label("A:");
                                            ui.label(format!("{} Kb", file.size_a));
                                            ui.label(file.date_a.to_owned());
                                            ui.add_space(75_f32);
                                            if ui.button("Remove from A").clicked() {
                                                // println!("Removing {file}");
                                                fs::remove_file(path_a).expect("Failed to delete file");
                                                self.matching_files.remove(i);
                                            }
                                        });
                                        ui.separator();
                                        // File B
                                        ui.horizontal(|ui| {
                                            ui.label("B:");
                                            ui.label(format!("{} Kb", file.size_b));
                                            ui.label(file.date_b.to_owned());
                                            ui.add_space(75_f32);
                                            if ui.button("Remove from B").clicked() {
                                                fs::remove_file(path_b).expect("Failed to delete file");
                                                self.matching_files.remove(i);
                                            }
                                        });
                                    });
                                });
                                ui.separator();
                            }
                        },
                    );
                });
            }
        });
    }
}

fn find_matching(picked_path_a: &String, picked_path_b: &String, matching_files: &mut Vec<File>) {
    matching_files.clear();

    let path_a = Path::new(picked_path_a);
    let path_b = Path::new(picked_path_b);

    if !path_a.is_dir() {
        panic!("Path A isn't a directory")
    }
    if !path_b.is_dir() {
        panic!("Path B isn't a directory")
    }

    let path_a_entries: Vec<DirEntry> = fs::read_dir(path_a)
        .unwrap()
        .into_iter()
        .map(|s| s.unwrap())
        .filter(|s| !s.file_type().unwrap().is_dir())
        .collect();

    let path_a_entry_names: Vec<String> = path_a_entries
        .iter()
        .map(|s| s.file_name().to_str().unwrap().to_owned())
        .collect();

    let path_b_entries: Vec<(&DirEntry, DirEntry)> = fs::read_dir(path_b)
        .unwrap()
        .into_iter()
        .map(|s| s.unwrap())
        .filter(|s| {
            !s.file_type().unwrap().is_dir()
                && path_a_entry_names.contains(&s.file_name().to_str().unwrap().to_owned())
        })
        .map(|s| {
            return (
                &path_a_entries[path_a_entry_names
                    .iter()
                    .position(|r| r == &s.file_name().to_str().unwrap().to_owned())
                    .unwrap()],
                s,
            );
        })
        .collect();

    for entry in path_b_entries {
        let date_a: DateTime<Local> = entry.0.metadata().unwrap().modified().unwrap().into();
        let date_a = date_a.format("%T %m/%d/%Y").to_string();
        let date_b: DateTime<Local> = entry.1.metadata().unwrap().modified().unwrap().into();
        let date_b = date_b.format("%T %m/%d/%Y").to_string();
        matching_files.push(File {
            name: entry.0.file_name().to_str().unwrap().to_owned(),
            size_a: entry.0.metadata().unwrap().len() / BYTE_TO_KB,
            date_a,
            size_b: entry.1.metadata().unwrap().len() / BYTE_TO_KB,
            date_b,
        })
    }
    matching_files.sort_unstable_by_key(|s| s.name.to_owned());
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
