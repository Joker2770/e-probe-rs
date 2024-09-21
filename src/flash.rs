pub mod m_flash_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use eframe::egui;
    use egui_file_dialog::FileDialog;
    use probe_rs::{flashing, probe::DebugProbeInfo};
    use std::
        path::PathBuf;

    #[derive(Default)]
    pub struct FlashProgram {
        probes_list: Vec<DebugProbeInfo>,
        cnt_4_update_probes_list: u16,
        probe_selected_idx: usize,
        probe_rs_handler: ProbeRsHandler,
        cnt_4_update_chips_list: u16,
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
                    self.cnt_4_update_probes_list += 1;
                    if 60 <= self.cnt_4_update_probes_list {
                        self.cnt_4_update_probes_list = 0;
                        self.probes_list = ProbeRsHandler::get_probes_list();
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
                        self.probes_list = ProbeRsHandler::get_probes_list();
                    }
                });
                self.cnt_4_update_chips_list += 1;
                if 100 <= self.cnt_4_update_chips_list {
                    self.cnt_4_update_chips_list = 0;
                    ProbeRsHandler::get_availabe_chips(
                        &mut self.probe_rs_handler,
                    );
                }
                egui::ComboBox::from_label("target")
                    .selected_text(format!("{}", self.target_chip_name))
                    .show_ui(ui, |ui| {
                        for t in self.probe_rs_handler.chips_list.iter() {
                            ui.selectable_value(&mut self.target_chip_name, t.to_string(), t);
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
                    if self.probe_selected_idx < self.probes_list.len() {
                        match ProbeRsHandler::attach(
                            &self.probes_list[self.probe_selected_idx],
                            &self.target_chip_name,
                        ) {
                            Ok(s) => {
                                let mut session = s;
                                let rst = ProbeRsHandler::try_to_download(
                                    &mut session,
                                    &self.selected_file.clone().unwrap_or_default(),
                                    self.file_format_selected.clone(),
                                );
                                match rst {
                                    Ok(_) => {
                                        self.dowmload_rst_info =
                                            Some("Download complete!".to_owned())
                                    }
                                    Err(e) => {
                                        let tmp = format!("{:?}", e).clone();
                                        self.dowmload_rst_info = Some(tmp);
                                    }
                                };
                            }
                            Err(e) => {
                                let tmp = format!("{:?}", e).clone();
                                self.dowmload_rst_info = Some(tmp);
                            }
                        }
                    }
                }
                ui.label(self.dowmload_rst_info.clone().unwrap_or_default());
            });
        }
    }
}
