pub mod m_flash_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use eframe::egui;
    use egui_file::FileDialog;
    use probe_rs::flashing;
    use std::{borrow::Borrow, path::PathBuf};

    #[derive(Default)]
    pub struct FlashProgram {
        probe_selected_idx: usize,
        probe_rs_handler: ProbeRsHandler,
        target_chip_name: String,
        file_format_selected: flashing::Format,
        dowmload_rst_info: Option<String>,
        file_dialog: Option<FileDialog>,
        selected_file: Option<PathBuf>,
    }

    impl FlashProgram {
        pub fn ui(&mut self, ctx: &eframe::egui::Context, ui: &mut egui::Ui) {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if 0 >= self.probe_rs_handler.probes_list.len() {
                        self.probe_rs_handler.get_probes_list();
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
                        self.probe_rs_handler.get_probes_list();
                    }
                });
                if 0 >= self.probe_rs_handler.chips_list.len() {
                    self.probe_rs_handler.get_availabe_chips();
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
                        match self
                            .probe_rs_handler
                            .attach_target(self.probe_selected_idx, &self.target_chip_name)
                        {
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
                        match self.probe_rs_handler.attach_target_under_reset(
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
                        match self.probe_rs_handler.reset_all_cores() {
                            Ok(_) => {
                                self.target_chip_name = "".to_owned();
                                self.dowmload_rst_info.take();
                            }
                            Err(e) => {
                                let tmp = format!("{:#?}", e).clone();
                                self.dowmload_rst_info = Some(tmp)
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
                            let rst = self.probe_rs_handler.try_to_download(
                                &self.selected_file.clone().unwrap_or_default(),
                                self.file_format_selected.clone(),
                            );
                            match rst {
                                Ok(_) => {
                                    self.dowmload_rst_info = Some("Download complete!".to_owned());
                                    let _ = self.probe_rs_handler.reset_all_cores();
                                }
                                Err(e) => {
                                    let tmp = format!("{:?}", e).clone();
                                    self.dowmload_rst_info = Some(tmp);
                                }
                            };
                        } else {
                            match self.probe_rs_handler.attach_target_under_reset(
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
