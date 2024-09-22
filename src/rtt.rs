pub mod m_rtt_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use chrono::Local;
    use eframe::egui;

    #[derive(Default)]
    pub struct RTTIO {
        probe_selected_idx: usize,
        probe_rs_handler: ProbeRsHandler,
        target_chip_name: String,
        b_try_to_read: bool,
        cur_target_core_num: usize,
        cur_target_core_idx: usize,
        cur_target_channel_num: usize,
        cur_target_channel_idx: usize,
        n_items: usize,
    }

    impl RTTIO {
        pub fn ui(&mut self, ui: &mut egui::Ui) {
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
                    let _ = ProbeRsHandler::attach_target(
                        &mut self.probe_rs_handler,
                        self.probe_selected_idx,
                        &self.target_chip_name,
                    );
                    self.cur_target_core_num = ProbeRsHandler::get_core_num(&self.probe_rs_handler);
                }
                if ui.button("attach under reset").clicked() {
                    let _ = ProbeRsHandler::attach_target_under_reset(
                        &mut self.probe_rs_handler,
                        self.probe_selected_idx,
                        &self.target_chip_name,
                    );
                    self.cur_target_core_num = ProbeRsHandler::get_core_num(&self.probe_rs_handler);
                }
            });

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("core")
                    .selected_text(format!("{}", self.cur_target_core_idx))
                    .show_ui(ui, |ui| {
                        for c in 0..self.cur_target_core_num {
                            ui.selectable_value(&mut self.cur_target_core_idx, c, format!("{}", c));
                        }
                    });

                if ui.button("attach rtt").clicked() {
                    let _ = ProbeRsHandler::get_rtt(
                        &mut self.probe_rs_handler,
                        self.cur_target_core_idx,
                    );
                    self.cur_target_channel_num =
                        ProbeRsHandler::get_up_channels_size(&mut self.probe_rs_handler);
                }
            });

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("channel")
                    .selected_text(format!("{}", self.cur_target_channel_idx))
                    .show_ui(ui, |ui| {
                        for c in 0..self.cur_target_channel_num {
                            ui.selectable_value(
                                &mut self.cur_target_channel_idx,
                                c,
                                format!("{}", c),
                            );
                        }
                    });

                if ui.button("take channel").clicked() {
                    ProbeRsHandler::get_one_up_ch(
                        &mut self.probe_rs_handler,
                        self.cur_target_channel_idx,
                    );
                }
            });

            ui.add_space(4.0);

            ui.checkbox(&mut self.b_try_to_read, "try to read");
            let mut buf = [0u8; 64];
            let mut read_size = 0;
            if self.b_try_to_read {
                match ProbeRsHandler::rtt_read_from_channel(&mut self.probe_rs_handler, &mut buf, 0)
                {
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
