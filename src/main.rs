#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod flash;
mod probe_opts;
mod probe_rs_invoke;
mod rtt;

use eframe::egui;
use flash::m_flash_opts::FlashProgram;
use probe_opts::m_probe_opts::ProbeOperations;
use rtt::m_rtt_opts::RTTIO;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([650.0, 480.0]),
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
        egui::CentralPanel::default().show(ctx, |ui| {
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
