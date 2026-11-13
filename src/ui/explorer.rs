use eframe::egui;
use std::fs;
use crate::app::EditorApp;
use crate::core::i18n::t;

pub fn render_side_panel(ctx: &egui::Context, app: &mut EditorApp) {
    let lang = app.settings.language;
    if !app.settings.show_explorer { return; }

    egui::SidePanel::left("explorer")
        .resizable(true)
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.heading(t(lang, "explorer_title"));
            ui.separator();

            if ui.button(t(lang, "open_project_folder")).clicked() {
                app.workspace.open_folder();
            }
            ui.add_space(10.0);

            let current_dir = app.workspace.last_directory.clone();
            if let Some(dir) = current_dir {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Ok(entries) = fs::read_dir(&dir) {
                        let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
                        paths.sort();
                        for path in paths {
                            if path.is_file() {
                                let name = path.file_name().unwrap_or_default().to_string_lossy();
                                if ui.selectable_label(false, format!("📄 {}", name)).clicked() {
                                    app.workspace.open_file(path.clone());
                                }
                            }
                        }
                    }
                });
            } else {
                ui.label(t(lang, "no_folder"));
            }
        });
}
