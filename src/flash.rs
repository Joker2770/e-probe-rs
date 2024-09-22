pub mod m_flash_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use eframe::egui;
    use egui_file_dialog::FileDialog;
    use probe_rs::flashing;
    use std::{borrow::Borrow, path::PathBuf};

    #[derive(Default)]
    pub struct FlashProgram {
        probe_selected_idx: usize,
        probe_rs_handler: ProbeRsHandler,
        target_chip_name: String,
        file_format_selected: flashing::Format,
        dowmload_rst_info: Option<String>,
        file_dialog: FileDialog,
        selected_file: Option<PathBuf>,
    }

    impl FlashProgram {
        pub fn ui(&mut self, ui: &mut egui::Ui) {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if 0 >= self.probe_rs_handler.probes_list.len() {
                        ProbeRsHandler::get_probes_list(&mut self.probe_rs_handler);
                    }
                    egui::ComboBox::from_label("probe")
                        .selected_text(format!("{}", self.probe_selected_idx))
                        .show_ui(ui, |ui| {
                            for (i, p) in self.probe_rs_handler.probes_list.iter().enumerate() {
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
                        ProbeRsHandler::get_probes_list(&mut self.probe_rs_handler);
                    }
                });
                if 0 >= self.probe_rs_handler.chips_list.len() {
                    ProbeRsHandler::get_availabe_chips(&mut self.probe_rs_handler);
                }

                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("target")
                        .selected_text(format!("{}", self.target_chip_name))
                        .show_ui(ui, |ui| {
                            for t in self.probe_rs_handler.chips_list.iter() {
                                ui.selectable_value(&mut self.target_chip_name, t.to_string(), t);
                            }
                        });

                    if ui.button("attach").clicked() {
                        match ProbeRsHandler::attach_target(
                            &mut self.probe_rs_handler,
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
                    if ui.button("attach under reset").clicked() {
                        match ProbeRsHandler::attach_target_under_reset(
                            &mut self.probe_rs_handler,
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
                });

                if ui.button("Select file").clicked() {
                    // Open the file dialog to select a file.
                    self.file_dialog.select_file();
                }

                ui.label(format!("Selected file: {:?}", self.selected_file));

                // Update the dialog
                self.file_dialog.update(ui.ctx());

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
                    if self.probe_selected_idx < self.probe_rs_handler.probes_list.len() {
                        if let Some(_) = self.probe_rs_handler.session.borrow() {
                            let rst = ProbeRsHandler::try_to_download(
                                &mut self.probe_rs_handler,
                                &self.selected_file.clone().unwrap_or_default(),
                                self.file_format_selected.clone(),
                            );
                            match rst {
                                Ok(_) => {
                                    self.dowmload_rst_info = Some("Download complete!".to_owned());
                                    let _ =
                                        ProbeRsHandler::reset_all_cores(&mut self.probe_rs_handler);
                                }
                                Err(e) => {
                                    let tmp = format!("{:?}", e).clone();
                                    self.dowmload_rst_info = Some(tmp);
                                }
                            };
                        } else {
                            match ProbeRsHandler::attach_target_under_reset(
                                &mut self.probe_rs_handler,
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
                ui.separator();
                ui.label(self.dowmload_rst_info.clone().unwrap_or_default());
            });
        }
    }
}
