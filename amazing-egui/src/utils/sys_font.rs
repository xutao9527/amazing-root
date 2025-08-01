use eframe::egui;
use egui::{
    FontData,
    epaint::text::{FontInsert, InsertFontFamily},
};

pub fn add_sys_ui_fonts(cc: &egui::Context) {
    #[cfg(target_os = "macos")]
    let fonts = ["PingFang", "Songti"];

    #[cfg(target_os = "windows")]
    let fonts = ["MSYH"];

    #[cfg(target_os = "linux")]
    let fonts = ["wqy-microhei", "SourceHanSansCN"];

    for (i, font) in fonts.iter().enumerate() {
        if let Some(f) = findfont::find(font) {
            if let Ok(font_bytes) = std::fs::read(f) {
                let priority = match i {
                    0 => egui::epaint::text::FontPriority::Highest,
                    _ => egui::epaint::text::FontPriority::Lowest,
                };

                cc.add_font(FontInsert::new(
                    font,
                    FontData::from_owned(font_bytes),
                    vec![
                        InsertFontFamily {
                            family: egui::FontFamily::Proportional,
                            priority,
                        },
                        InsertFontFamily {
                            family: egui::FontFamily::Monospace,
                            priority: egui::epaint::text::FontPriority::Lowest,
                        },
                    ],
                ));
            }
        }
    }
}