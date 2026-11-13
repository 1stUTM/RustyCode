use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::core::workspace::Workspace;
use crate::ui::{editor, explorer, menu, terminal};
use crate::core::i18n::Language;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub is_dark_mode: bool,
    pub zoom_factor: f32,
    pub show_terminal: bool,
    pub show_explorer: bool,
    pub word_wrap: bool,
    pub language: Language,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            is_dark_mode: true,
            zoom_factor: 1.2,
            show_terminal: true,
            show_explorer: true,
            word_wrap: false,
            language: Language::En,
        }
    }
}

pub struct EditorApp {
    pub settings: AppSettings,
    pub workspace: Workspace,
}

impl EditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings: AppSettings = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppSettings::default()
        };

        let lang = settings.language;
        Self {
            settings,
            workspace: Workspace::new(lang),
        }
    }
}

impl eframe::App for EditorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ФИКС МАСШТАБА: жестко ограничиваем зум, чтобы не сделать окно бесконечно огромным или микроскопическим
        self.settings.zoom_factor = self.settings.zoom_factor.clamp(0.5, 3.0);
        ctx.set_pixels_per_point(self.settings.zoom_factor);
        
        ctx.set_visuals(if self.settings.is_dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() });
        
        self.workspace.handle_hotkeys(ctx, self.settings.language);
        self.workspace.tick_runner();
        
        if self.workspace.is_running {
            ctx.request_repaint();
        }

        menu::render_top_menu(ctx, self);
        terminal::render_bottom_panel(ctx, self);
        explorer::render_side_panel(ctx, self);
        editor::render_central_panel(ctx, self);
    }
}
