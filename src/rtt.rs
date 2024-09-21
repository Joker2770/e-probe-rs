pub mod m_rtt_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use eframe::egui;
    use probe_rs::probe::DebugProbeInfo;

    #[derive(Default)]
    pub struct RTTIO {
        probes_list: Vec<DebugProbeInfo>,
        cnt_4_update_probes_list: u16,
        probe_selected_idx: usize,
        probe_rs_handler: ProbeRsHandler,
        cnt_4_update_chips_list: u16,
        target_chip_name: String,
    }

    impl RTTIO {
        pub fn ui(&mut self, ui: &mut egui::Ui) {
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
                ProbeRsHandler::get_availabe_chips(&mut self.probe_rs_handler);
            }
            egui::ComboBox::from_label("target")
                .selected_text(format!("{}", self.target_chip_name))
                .show_ui(ui, |ui| {
                    for t in self.probe_rs_handler.chips_list.iter() {
                        ui.selectable_value(&mut self.target_chip_name, t.to_string(), t);
                    }
                });

            ui.add_space(4.0);
        }
    }
}
