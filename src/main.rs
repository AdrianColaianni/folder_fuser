#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::vec;
use std::fs;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Native file dialogs and drag-and-drop files",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    matching_files: Vec<String>,
    picked_path_a: Option<String>,
    picked_path_b: Option<String>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Folder Fuser 5000!");

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
                    } else if ui.button("Find matching files").clicked() {
                        find_matching(picked_path_a, picked_path_b, &mut self.matching_files);
                    };
                }
            }

            if !&self.matching_files.is_empty() {
                ui.group(|ui| {
                    for (i, file) in self.matching_files.to_owned().iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(file);
                            ui.add_space(100_f32);
                            ui.vertical(|ui| {
                                if ui.button("Remove from A").clicked() {
                                    let path = String::from(format!("{}/{}", self.picked_path_a.to_owned().unwrap(), file));
                                    // println!("Removing {file}");
                                    let path = Path::new(&path);
                                    fs::remove_file(path).expect("Failed to delete file");
                                    self.matching_files.remove(i);
                                }
                                if ui.button("Remove from B").clicked() {
                                    let path = String::from(format!("{}/{}", self.picked_path_b.to_owned().unwrap(), file));
                                    // println!("Removing {file}");
                                    let path = Path::new(&path);
                                    fs::remove_file(path).expect("Failed to delete file");
                                    self.matching_files.remove(i);
                                }
                            });
                        });
                    }
                });
            }
        });

    }
}

fn find_matching(picked_path_a: &String, picked_path_b: &String, matching_files: &mut Vec<String>) {
    for _ in 0..matching_files.len() {
        matching_files.pop();
    }

    let path_a = Path::new(picked_path_a);
    let path_b = Path::new(picked_path_b);

    if !path_a.is_dir() {
        panic!("Path A isn't a directory")
    }
    if !path_b.is_dir() {
        panic!("Path B isn't a directory")
    }

    let mut path_a_entries: Vec<String> = vec![String::new(); 0];

    for entry in fs::read_dir(path_a).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }
        let path = path.file_name().unwrap().to_str().unwrap().to_owned();
        path_a_entries.push(path);
    }

    for entry in fs::read_dir(path_b).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            continue;
        }
        let path = path.file_name().unwrap().to_str().unwrap().to_owned();
        if path_a_entries.contains(&path) {
            matching_files.push(path)
        }
    }
}
