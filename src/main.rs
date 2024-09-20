#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod probe_rs_invoke;
mod flash;
mod probe_opts;

use eframe::egui;
use flash::m_flash_opts::FlashProgram;
use probe_opts::m_probe_opts::ProbeOperations;

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
    flash_opt: FlashProgram,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.stack_window, ProbeOperations::FlashProgram, "flash");
                ui.selectable_value(&mut self.stack_window, ProbeOperations::RTTIO, "rtt");
            });
            ui.separator();
            match self.stack_window {
                ProbeOperations::FlashProgram => {
                    self.flash_opt.ui(ui);
                }
                ProbeOperations::RTTIO => {

                }
            }
        });
    }
}
