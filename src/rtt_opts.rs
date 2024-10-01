pub mod m_rtt_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use chrono::Local;
    use eframe::egui;
    use egui_file::FileDialog;
    use probe_rs::rtt::ScanRegion;
    use std::{borrow::Borrow, path::PathBuf, time::Duration};

    #[derive(Default)]
    pub struct RTTIO {
        probe_selected_idx: usize,
        probe_rs_handler: ProbeRsHandler,
        target_chip_name: String,
        b_try_to_read: bool,
        cur_target_core_idx: usize,
        cur_target_channel_idx: usize,
        b_get_scan_region: bool,
        file_dialog: Option<FileDialog>,
        selected_file: Option<PathBuf>,
        n_items: usize,
    }

    impl RTTIO {
        pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
                    let _ = self
                        .probe_rs_handler
                        .attach_target(self.probe_selected_idx, &self.target_chip_name);
                    self.probe_rs_handler.get_core_num();
                }
                if ui.button("attach under reset").clicked() {
                    let _ = self
                        .probe_rs_handler
                        .attach_target_under_reset(self.probe_selected_idx, &self.target_chip_name);
                    self.probe_rs_handler.get_core_num();
                }
                if ui.button("reset all").clicked() {
                    match self.probe_rs_handler.reset_all_cores() {
                        Ok(_) => {
                            self.cur_target_core_idx = 0;
                            self.cur_target_channel_idx = 0;
                            self.b_try_to_read = false;
                            self.target_chip_name = "".to_owned();
                        }
                        Err(e) => {}
                    }
                }
            });

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("core")
                    .selected_text(format!("{}", self.cur_target_core_idx))
                    .show_ui(ui, |ui| {
                        for c in 0..self.probe_rs_handler.target_cores_num {
                            ui.selectable_value(&mut self.cur_target_core_idx, c, format!("{}", c));
                        }
                    });

                if ui.button("attach rtt").clicked() {
                    match self.probe_rs_handler.attach_rtt(self.cur_target_core_idx) {
                        Ok(_) => {}
                        Err(e) => {}
                    }
                    self.probe_rs_handler.get_up_channels_size();
                }
            });

            ui.separator();
            ui.add_space(4.0);

            ui.vertical(|ui| {
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

                if ui.button("get scan region from elf").clicked() {
                    match self
                        .probe_rs_handler
                        .get_scan_region(self.selected_file.borrow(), None)
                    {
                        Ok(_) => {
                            if let Some(sr) = self.probe_rs_handler.scan_region.borrow() {
                                match sr {
                                    ScanRegion::Exact(_) => {
                                        self.b_get_scan_region = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }

                if self.b_get_scan_region {
                    ui.horizontal(|ui| {
                        if ui.button("attach rtt region").clicked() {
                            match self
                                .probe_rs_handler
                                .attach_rtt_region(self.cur_target_core_idx)
                            {
                                Ok(_) => {}
                                Err(e) => {}
                            }
                            self.probe_rs_handler.get_up_channels_size();
                        }
                        let mut time_out = 1000;
                        ui.add(
                            egui::Slider::new(&mut time_out, 0..=10000).text("time out duration (ms)"),
                        );
                        if ui.button("try to attach rtt rigion with timeout").clicked() {
                            let time_out_duration = Duration::from_millis(time_out);
                            match self
                                .probe_rs_handler
                                .attach_retry_loop(self.cur_target_core_idx, time_out_duration)
                            {
                                Ok(_) => {}
                                Err(_) => {}
                            }
                            self.probe_rs_handler.get_up_channels_size();
                        }
                    });
                }
            });
            ui.add_space(4.0);
            ui.separator();

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("channel")
                    .selected_text(format!("{}", self.cur_target_channel_idx))
                    .show_ui(ui, |ui| {
                        for c in 0..self.probe_rs_handler.up_chs_size {
                            ui.selectable_value(
                                &mut self.cur_target_channel_idx,
                                c,
                                format!("{}", c),
                            );
                        }
                    });

                if ui.button("take channel").clicked() {
                    self.probe_rs_handler
                        .get_one_up_ch(self.cur_target_channel_idx);
                }
            });

            ui.add_space(4.0);

            ui.checkbox(&mut self.b_try_to_read, "try to read");
            let mut buf = [0u8; 64];
            let mut read_size = 0;
            if self.b_try_to_read {
                match self.probe_rs_handler.rtt_read_from_channel(&mut buf, 0) {
                    Ok(s) => {
                        read_size = s;
                    }
                    Err(e) => {
                        ui.label(format!("{:#?}", e));
                    }
                }
            }
            ui.add_space(4.0);
            ui.separator();
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                    if read_size > 0 {
                        for row in row_range {
                            let local_date_time = Local::now();
                            let ymdhms = local_date_time.format("%Y-%m-%d %H:%M:%S%.3f");
                            let text = format!(
                                "{}: {} {}",
                                row + 1,
                                ymdhms,
                                String::from_utf8_lossy(&buf)
                            );
                            ui.label(text);
                        }
                    }
                });

            self.n_items += 1;
            ui.ctx().request_repaint();
        }
    }
}
