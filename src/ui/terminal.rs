use eframe::egui;
use crate::app::EditorApp;
use crate::core::i18n::t;

pub fn render_bottom_panel(ctx: &egui::Context, app: &mut EditorApp) {
    let lang = app.settings.language;
    if !app.settings.show_terminal { return; }

    egui::TopBottomPanel::bottom("terminal")
        .resizable(true)
        .default_height(350.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(t(lang, "terminal_title"));
                if app.workspace.is_running {
                    ui.spinner();
                    ui.label(t(lang, "running"));
                }
                if ui.button(t(lang, "clear")).clicked() { app.workspace.output_text.clear(); }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✖").clicked() { app.settings.show_terminal = false; }
                });
            });
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                let color = if app.settings.is_dark_mode { egui::Color32::LIGHT_GRAY } else { egui::Color32::DARK_GRAY };
                ui.add(
                    egui::TextEdit::multiline(&mut app.workspace.output_text.as_str())
                        .font(egui::TextStyle::Monospace)
                        .text_color(color)
                        .desired_width(f32::INFINITY)
                        .interactive(false)
                        .frame(false)
                );
            });
        });
}
