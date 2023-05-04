#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Pigeon post",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

struct App {
    url: String,
    method: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            method: "GET".to_owned()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { url, method } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("").selected_text(method.clone()).show_ui(ui, |ui| {
                    ui.selectable_value(method, "GET".to_owned(), "GET");
                    ui.selectable_value(method, "POST".to_owned(), "POST");
                });
                egui::TextEdit::singleline(url).hint_text("Enter URL").show(ui);
                if ui.button("Send").clicked() {
                    println!("{url}");
                }
            });
        });
    }
}
