mod utils;

use crate::utils::sys_font::add_sys_ui_fonts;
use eframe::egui;

fn main() -> eframe::Result {
    let width = 320.0;
    let height = 240.0;
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_inner_size([width, height]);
    native_options.centered = true;

    eframe::run_native(
        "amazing app",
        native_options,
        Box::new(|cc| {
            add_sys_ui_fonts(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    )
}

struct App {
    name: String,
    age: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
