/*
 *  Simple GUI for probe-rs with egui framework.
 *  Copyright (C) 2024 Joker2770
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

pub mod m_rtt_opts {
    use crate::probe_rs_invoke::probe_rs_integration::ProbeRsHandler;
    use chrono::Local;
    use eframe::egui;
    use egui_file::FileDialog;
    use probe_rs::rtt::ScanRegion;
    use std::{
        borrow::{Borrow, BorrowMut},
        collections::VecDeque,
        path::PathBuf,
        time::Duration,
    };

    #[derive(Default)]
    pub struct RTTIO {
        probe_selected_idx: usize,
        probe_rs_handler: Option<ProbeRsHandler>,
        target_chip_name: String,
        b_try_to_read: bool,
        cur_target_core_idx: usize,
        cur_target_channel_idx: usize,
        b_get_scan_region: bool,
        file_dialog: Option<FileDialog>,
        selected_file: Option<PathBuf>,
        retry_rtt_attach_time_out: u64,
        log_buf: VecDeque<String>,
        n_display_row: usize,
        b_take: bool,
        n_items: usize,
    }

    impl RTTIO {
        pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
            if let None = self.probe_rs_handler.borrow_mut() {
                self.probe_rs_handler = Some(ProbeRsHandler::default());
            }
            if let Some(h) = self.probe_rs_handler.borrow_mut() {
                if 0 >= h.probes_list.len() {
                    h.get_probes_list();
                }
            }

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("probe")
                    .selected_text(format!("{}", self.probe_selected_idx))
                    .show_ui(ui, |ui| {
                        if let Some(h) = self.probe_rs_handler.borrow() {
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
                        }
                    });
                if ui.button("refresh").clicked() {
                    if let Some(h) = self.probe_rs_handler.borrow_mut() {
                        h.get_probes_list();
                    }
                }
            });
            if let Some(h) = self.probe_rs_handler.borrow_mut() {
                if 0 >= h.chips_list.len() {
                    h.get_availabe_chips();
                }
            }

            ui.horizontal(|ui| {
                if let Some(h) = self.probe_rs_handler.borrow_mut() {
                    egui::ComboBox::from_label("target")
                        .selected_text(format!("{}", self.target_chip_name))
                        .show_ui(ui, |ui| {
                            for t in h.chips_list.iter() {
                                ui.selectable_value(&mut self.target_chip_name, t.to_string(), t);
                            }
                        });
                    if ui.button("attach").clicked() {
                        let _ = h.attach_target(self.probe_selected_idx, &self.target_chip_name);
                        h.get_core_num();
                    }
                    if ui.button("attach under reset").clicked() {
                        let _ = h.attach_target_under_reset(
                            self.probe_selected_idx,
                            &self.target_chip_name,
                        );
                        h.get_core_num();
                    }
                    if ui.button("reset all").clicked() {
                        match h.reset_all_cores() {
                            Ok(_) => {
                                self.cur_target_core_idx = 0;
                                self.cur_target_channel_idx = 0;
                                self.b_try_to_read = false;
                                self.target_chip_name = "".to_owned();
                                self.b_get_scan_region = false;
                                self.selected_file = None;
                                self.probe_rs_handler = None;
                            }
                            Err(_) => {}
                        }
                    }
                }
            });

            ui.horizontal(|ui| {
                if let Some(h) = self.probe_rs_handler.borrow_mut() {
                    egui::ComboBox::from_label("core")
                        .selected_text(format!("{}", self.cur_target_core_idx))
                        .show_ui(ui, |ui| {
                            for c in 0..h.target_cores_num {
                                ui.selectable_value(
                                    &mut self.cur_target_core_idx,
                                    c,
                                    format!("{}", c),
                                );
                            }
                        });

                    if ui.button("attach rtt").clicked() {
                        match h.attach_rtt(self.cur_target_core_idx) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                        h.get_up_channels_size();
                    }
                }
            });

            ui.separator();
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("SEGGER RTT");
                ui.separator();
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

                    ui.label(format!("Selected elf file: {:?}", self.selected_file));
                    if let Some(dialog) = &mut self.file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.selected_file = Some(file.to_path_buf());
                            }
                        }
                    }

                    if ui.button("get scan region from elf").clicked() {
                        if let Some(h) = self.probe_rs_handler.borrow_mut() {
                            match h.get_scan_region(self.selected_file.borrow(), None) {
                                Ok(_) => {
                                    if let Some(sr) = h.scan_region.borrow() {
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
                    }

                    if self.b_get_scan_region {
                        ui.horizontal(|ui| {
                            if let Some(h) = self.probe_rs_handler.borrow_mut() {
                                if ui.button("attach rtt region").clicked() {
                                    match h.attach_rtt_region(self.cur_target_core_idx) {
                                        Ok(_) => {}
                                        Err(_) => {}
                                    }
                                    h.get_up_channels_size();
                                }
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.retry_rtt_attach_time_out,
                                        0..=10000,
                                    )
                                    .text("time out duration (ms)"),
                                );
                                if ui.button("try to attach rtt rigion with timeout").clicked() {
                                    let time_out_duration =
                                        Duration::from_millis(self.retry_rtt_attach_time_out);
                                    match h.attach_retry_loop(
                                        self.cur_target_core_idx,
                                        time_out_duration,
                                    ) {
                                        Ok(_) => {}
                                        Err(_) => {}
                                    }
                                    h.get_up_channels_size();
                                }
                            }
                        });
                    }
                });
            });
            ui.add_space(4.0);
            ui.separator();

            ui.horizontal(|ui| {
                if let Some(h) = self.probe_rs_handler.borrow_mut() {
                    if let Some(_rtt) = h.rtt.borrow() {
                        egui::ComboBox::from_label("channel")
                            .selected_text(format!("{}", self.cur_target_channel_idx))
                            .show_ui(ui, |ui| {
                                for c in 0..h.up_chs_size {
                                    ui.selectable_value(
                                        &mut self.cur_target_channel_idx,
                                        c,
                                        format!("{}", c),
                                    );
                                }
                            });

                        if ui.checkbox(&mut self.b_take, "take channel").clicked() {
                            h.get_one_up_ch(self.cur_target_channel_idx);
                            if let None = h.cur_ch {
                                self.b_take = false;
                            }
                        }
                        ui.add_space(4.0);
                        ui.checkbox(&mut self.b_try_to_read, "try to read");
                    }
                }
            });

            if self.log_buf.len() >= self.n_display_row {
                self.log_buf.pop_front();
            } else {
            }

            let mut buf = [0u8; 128];
            if self.b_try_to_read {
                if let Some(h) = self.probe_rs_handler.borrow_mut() {
                    match h.rtt_read_from_channel(&mut buf, self.cur_target_channel_idx) {
                        Ok(s) => {
                            let read_size = s;
                            if read_size > 0 {
                                let local_date_time = Local::now();
                                let ymdhms = local_date_time.format("%Y-%m-%d %H:%M:%S%.3f");
                                let text = format!(
                                    "{}: {} {}",
                                    self.n_items,
                                    ymdhms,
                                    String::from_utf8_lossy(&buf)
                                );
                                self.log_buf.push_back(text);
                                self.n_items = self.n_items.wrapping_add(1);
                            }
                        }
                        Err(e) => {
                            ui.label(format!("{:#?}", e));
                        }
                    }
                }
            }

            ui.add_space(4.0);
            ui.separator();
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink(false)
                .show_rows(ui, row_height, self.n_items, |ui, row_range| {
                    let row_start = row_range.start;
                    self.n_display_row = row_range.len();
                    for row in row_range {
                        if let Some(t) = self.log_buf.get(row - row_start) {
                            let label = egui::Label::new(t).extend();
                            ui.add(label);
                        }
                    }
                });
            ui.ctx().request_repaint();
        }
    }
}
