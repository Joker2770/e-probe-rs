#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod probe_rs_invoke;

use probe_rs::{flashing, probe::DebugProbeInfo};
use probe_rs_invoke::probe_rs_integration;

use eframe::egui;

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
    probes_list_update_cnt: u16,
    probe_selected: u16,
    target_chip_name: String,
    file_format_selected: flashing::Format,
    picked_path: Option<String>,
    dowmload_rst_info: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    self.probes_list_update_cnt+=1;
                    if 60 <= self.probes_list_update_cnt {
                        self.probes_list_update_cnt = 0;
                        self.probes_list = probe_rs_integration::get_probes_list();
                    }
                    egui::ComboBox::from_label("probe")
                        .selected_text(format!("{}", self.probe_selected))
                        .show_ui(ui, |ui| {
                            for (i, p) in self.probes_list.iter().enumerate() {
                                ui.selectable_value(
                                    &mut self.probe_selected,
                                    i as u16,
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
                egui::ComboBox::from_label("target")
                    .selected_text(format!("{}", self.target_chip_name))
                    .show_ui(ui, |ui| {
                        for t in probe_rs_integration::get_availabe_chips().iter() {
                            ui.selectable_value(&mut self.target_chip_name, t.to_string(), t);
                        }
                    });
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
                let mut path = &"".to_owned();
                if let Some(picked_path) = &self.picked_path {
                    path = picked_path;
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
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
                    if 0 < self.probes_list.len() {
                        let rst = probe_rs_integration::try_to_download(
                            &self.probes_list[self.probe_selected as usize],
                            &self.target_chip_name,
                            path.into(),
                        );
                        match rst {
                            Ok(_) => self.dowmload_rst_info = "Download complete!".to_owned(),
                            Err(e) => {
                                let tmp = format!("{:?}", e).clone();
                                self.dowmload_rst_info = tmp
                            }
                        };
                    }
                }
                ui.label(&self.dowmload_rst_info);
            });
        });
    }
}
