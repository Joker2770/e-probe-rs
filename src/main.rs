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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod flash_opts;
mod probe_opts;
mod probe_rs_invoke;
mod rtt_opts;

use flash_opts::m_flash_opts::FlashProgram;
use probe_opts::m_probe_opts::ProbeOperations;
use rtt_opts::m_rtt_opts::RTTIO;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([650.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "e-probe-rs",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    stack_window: ProbeOperations,
    flash_opts: FlashProgram,
    rttio_opts: RTTIO,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.stack_window,
                    ProbeOperations::FlashProgram,
                    "flash",
                );
                ui.selectable_value(&mut self.stack_window, ProbeOperations::RTTIO, "rtt");
            });
            ui.separator();
            match self.stack_window {
                ProbeOperations::FlashProgram => {
                    self.flash_opts.ui(ctx, ui);
                }
                ProbeOperations::RTTIO => {
                    self.rttio_opts.ui(ctx, ui);
                }
            }
        });
    }
}
