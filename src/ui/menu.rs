use eframe::egui;
use crate::app::EditorApp;
use crate::core::i18n::{t, Language};

pub fn render_top_menu(ctx: &egui::Context, app: &mut EditorApp) {
    let lang = app.settings.language;
    egui::TopBottomPanel::top("top_menu").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(t(lang, "menu_file"), |ui| {
                if ui.button(t(lang, "menu_open_folder")).clicked() {
                    app.workspace.open_folder();
                    ui.close_menu();
                }
                if ui.button(t(lang, "menu_save")).clicked() {
                    app.workspace.save_current_file();
                    ui.close_menu();
                }
            });
            ui.menu_button(t(lang, "menu_settings"), |ui| {
                if ui.button(t(lang, "zoom_in")).clicked() { app.settings.zoom_factor += 0.1; }
                if ui.button(t(lang, "zoom_out")).clicked() { app.settings.zoom_factor -= 0.1; }
                if ui.button(t(lang, "zoom_reset")).clicked() { app.settings.zoom_factor = 1.0; }
                ui.separator();
                ui.checkbox(&mut app.settings.show_explorer, t(lang, "show_explorer"));
                ui.checkbox(&mut app.settings.show_terminal, t(lang, "show_terminal"));
                ui.checkbox(&mut app.settings.word_wrap, t(lang, "word_wrap"));
                
                ui.separator();
                ui.menu_button(t(lang, "menu_language"), |ui| {
                    let langs = [
                        (Language::En, "English"),
                        (Language::Ru, "Русский"),
                        (Language::Zh, "中文"),
                        (Language::Es, "Español"),
                        (Language::De, "Deutsch"),
                        (Language::Fr, "Français"),
                    ];
                    for (l, name) in langs {
                        if ui.radio_value(&mut app.settings.language, l, name).clicked() {
                            app.workspace.output_text = t(l, "system_ready");
                        }
                    }
                });
            });

            ui.add_space(20.0);
            if ui.button(t(lang, "run_code")).clicked() { 
                app.workspace.run_current_code(lang);
                app.settings.show_terminal = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(if app.settings.is_dark_mode { t(lang, "theme_light") } else { t(lang, "theme_dark") }).clicked() {
                    app.settings.is_dark_mode = !app.settings.is_dark_mode;
                }
            });
        });
    });
}
