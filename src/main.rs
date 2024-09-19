#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod probe_rs_invoke;

use eframe::egui;
use egui_file_dialog::FileDialog;
use probe_rs::{flashing, probe::DebugProbeInfo};
use probe_rs_invoke::probe_rs_integration;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([650.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    probes_list: Vec<DebugProbeInfo>,
    cnt_4_update_probes_list: u16,
    probe_selected_idx: usize,
    chips_list: Vec<String>,
    cnt_4_update_chips_list: u16,
    target_chip_name: String,
    file_format_selected: flashing::Format,
    dowmload_rst_info: Option<String>,
    file_dialog: FileDialog,
    selected_file: Option<PathBuf>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    self.cnt_4_update_probes_list += 1;
                    if 60 <= self.cnt_4_update_probes_list {
                        self.cnt_4_update_probes_list = 0;
                        self.probes_list = probe_rs_integration::get_probes_list();
                    }
                    egui::ComboBox::from_label("probe")
                        .selected_text(format!("{}", self.probe_selected_idx))
                        .show_ui(ui, |ui| {
                            for (i, p) in self.probes_list.iter().enumerate() {
                                ui.selectable_value(
                                    &mut self.probe_selected_idx,
                                    i,
                                    format!(
                                        "{} (pid: {} vid: {})",
                                        p.identifier.as_str(),
                                        p.product_id,
                                        p.vendor_id
                                    ),
                                );
                            }
                        });
                    if ui.button("refresh").clicked() {
                        self.probes_list = probe_rs_integration::get_probes_list();
                    }
                });
                self.cnt_4_update_chips_list += 1;
                if 100 <= self.cnt_4_update_chips_list {
                    self.cnt_4_update_chips_list = 0;
                    self.chips_list = probe_rs_integration::get_availabe_chips();
                }
                egui::ComboBox::from_label("target")
                    .selected_text(format!("{}", self.target_chip_name))
                    .show_ui(ui, |ui| {
                        for t in self.chips_list.iter() {
                            ui.selectable_value(&mut self.target_chip_name, t.to_string(), t);
                        }
                    });
                if ui.button("Select file").clicked() {
                    // Open the file dialog to select a file.
                    self.file_dialog.select_file();
                }

                ui.label(format!("Selected file: {:?}", self.selected_file));

                // Update the dialog
                self.file_dialog.update(ctx);

                // Check if the user selected a file.
                if let Some(path) = self.file_dialog.take_selected() {
                    self.selected_file = Some(path);
                }

                egui::ComboBox::from_label("File Format")
                    .selected_text(format!("{:?}", self.file_format_selected))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.file_format_selected,
                            flashing::Format::Elf,
                            "elf",
                        );
                        ui.selectable_value(
                            &mut self.file_format_selected,
                            flashing::Format::Hex,
                            "hex",
                        );
                        ui.selectable_value(
                            &mut self.file_format_selected,
                            flashing::Format::Uf2,
                            "uf2",
                        );
                    });
                if ui.button("try to download").clicked() {
                    if self.probe_selected_idx < self.probes_list.len() {
                        let rst = probe_rs_integration::try_to_download(
                            &self.probes_list[self.probe_selected_idx as usize],
                            &self.target_chip_name,
                            &self.selected_file.clone().unwrap_or_default(),
                            self.file_format_selected.clone(),
                        );
                        match rst {
                            Ok(_) => self.dowmload_rst_info = Some("Download complete!".to_owned()),
                            Err(e) => {
                                let tmp = format!("{:?}", e).clone();
                                self.dowmload_rst_info = Some(tmp);
                            }
                        };
                    }
                }
                ui.label(self.dowmload_rst_info.clone().unwrap_or_default());
            });
        });
    }
}
