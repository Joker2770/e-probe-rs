/*
 *  Simple GUI for probe-rs with egui framework.
 *  Copyright (C) 2024-2025 Joker2770
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod m_flash_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use egui_file::FileDialog;
    use probe_rs::flashing;
    use std::{
        borrow::{Borrow, BorrowMut},
        path::PathBuf,
    };

    #[derive(Default)]
    pub struct FlashProgram {
        probe_selected_idx: usize,
        probe_rs_handler: Option<ProbeRsHandler>,
        target_chip_name: String,
        file_format_selected: flashing::Format,
        dowmload_rst_info: Option<String>,
        file_dialog: Option<FileDialog>,
        selected_file: Option<PathBuf>,
        filter_s: String,
    }

    impl FlashProgram {
        pub fn ui(&mut self, ctx: &eframe::egui::Context, ui: &mut eframe::egui::Ui) {
            if self.probe_rs_handler.borrow_mut().is_none() {
                self.probe_rs_handler = Some(ProbeRsHandler::default());
            }

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if let Some(h) = self.probe_rs_handler.borrow_mut() {
                        if h.probes_list.is_empty() {
                            h.get_probes_list();
                        }
                        eframe::egui::ComboBox::from_label("probe")
                            .selected_text(format!("{}", self.probe_selected_idx))
                            .show_ui(ui, |ui| {
                                for (i, p) in h.probes_list.iter().enumerate() {
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
                            h.get_probes_list();
                        }
                    }
                });
                if let Some(h) = self.probe_rs_handler.borrow_mut() {
                    if h.chips_list.is_empty() {
                        h.get_availabe_chips();
                    }
                }

                ui.horizontal(|ui| {
                    if let Some(h) = self.probe_rs_handler.borrow_mut() {
                        eframe::egui::ComboBox::from_label("target")
                            .selected_text(self.target_chip_name.to_string())
                            .show_ui(ui, |ui| {
                                for t in h.chips_list.iter() {
                                    if !self.filter_s.is_empty() {
                                        if t.contains(&self.filter_s) {
                                            ui.selectable_value(
                                                &mut self.target_chip_name,
                                                t.to_string(),
                                                t,
                                            );
                                        }
                                    } else {
                                        ui.selectable_value(
                                            &mut self.target_chip_name,
                                            t.to_string(),
                                            t,
                                        );
                                    }
                                }
                            });

                        ui.add(
                            eframe::egui::TextEdit::singleline(&mut self.filter_s)
                                .hint_text("chips filter")
                                .desired_width(100.0),
                        );

                        if ui.button("attach").clicked() {
                            match h.attach_target(self.probe_selected_idx, &self.target_chip_name) {
                                Ok(_) => {
                                    self.dowmload_rst_info.take();
                                }
                                Err(e) => {
                                    let tmp = format!("{:#?}", e).clone();
                                    self.dowmload_rst_info = Some(tmp)
                                }
                            }
                        }
                        if ui.button("attach under reset").clicked() {
                            match h.attach_target_under_reset(
                                self.probe_selected_idx,
                                &self.target_chip_name,
                            ) {
                                Ok(_) => {
                                    self.dowmload_rst_info.take();
                                }
                                Err(e) => {
                                    let tmp = format!("{:#?}", e).clone();
                                    self.dowmload_rst_info = Some(tmp)
                                }
                            }
                        }
                        if ui.button("reset all").clicked() {
                            match h.reset_all_cores() {
                                Ok(_) => {
                                    self.target_chip_name = "".to_owned();
                                    self.dowmload_rst_info.take();
                                    self.probe_selected_idx = 0;
                                    self.probe_rs_handler = None;
                                    self.selected_file = None;
                                }
                                Err(e) => {
                                    let tmp = format!("{:#?}", e).clone();
                                    self.dowmload_rst_info = Some(tmp)
                                }
                            }
                        }
                    }
                });

                if ui.button("Select file").clicked() {
                    // Open the file dialog to select a file.
                    // let filter = Box::new({
                    //     let ext_0 = Some(OsStr::new("hex"));
                    //     let ext_1 = Some(OsStr::new("elf"));
                    //     let ext_2= Some(OsStr::new("bin"));
                    //     move |path: &Path| -> bool { path.extension() == ext_0 || path.extension() == ext_1 || path.extension() == ext_2 }
                    // });
                    let mut dialog = FileDialog::open_file(self.selected_file.clone());
                    dialog.open();
                    self.file_dialog = Some(dialog);
                }

                ui.label(format!("Selected file: {:?}", self.selected_file));
                if let Some(dialog) = &mut self.file_dialog {
                    if dialog.show(ctx).selected() {
                        if let Some(file) = dialog.path() {
                            self.selected_file = Some(file.to_path_buf());
                        }
                    }
                }

                eframe::egui::ComboBox::from_label("File Format")
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
                    if let Some(h) = self.probe_rs_handler.borrow_mut() {
                        if self.probe_selected_idx < h.probes_list.len() {
                            if h.session.borrow().is_some() {
                                let rst = h.try_to_download(
                                    &self.selected_file.clone().unwrap_or_default(),
                                    self.file_format_selected.clone(),
                                );
                                match rst {
                                    Ok(_) => {
                                        self.dowmload_rst_info =
                                            Some("Download complete!".to_owned());
                                        let _ = h.reset_all_cores();
                                    }
                                    Err(e) => {
                                        let tmp = format!("{:?}", e).clone();
                                        self.dowmload_rst_info = Some(tmp);
                                    }
                                };
                            } else {
                                match h.attach_target_under_reset(
                                    self.probe_selected_idx,
                                    &self.target_chip_name,
                                ) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        let tmp = format!("{:#?}", e).clone();
                                        self.dowmload_rst_info = Some(tmp)
                                    }
                                }
                            }
                        }
                    }
                }
                ui.separator();
                ui.label(self.dowmload_rst_info.clone().unwrap_or_default());
            });
        }
    }
}
