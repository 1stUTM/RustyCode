use eframe::egui;
use crate::app::EditorApp;
use crate::core::workspace::EditorTab;
use crate::core::i18n::t;

pub fn render_central_panel(ctx: &egui::Context, app: &mut EditorApp) {
    let lang = app.settings.language;
    
    egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                let mut tab_to_close = None;
                for (i, tab) in app.workspace.opened_files.iter().enumerate() {
                    let mut title = tab.path.as_ref()
                        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
                        .unwrap_or_else(|| t(lang, "untitled").to_string());
                    
                    if tab.is_dirty() { title.push('*'); }
                    
                    if ui.selectable_label(i == app.workspace.active_tab, title).clicked() {
                        app.workspace.active_tab = i;
                    }
                    if ui.button("✖").clicked() { tab_to_close = Some(i); }
                    ui.separator();
                }
                
                if ui.button(" + ").clicked() {
                    app.workspace.opened_files.push(EditorTab::new(None, String::new()));
                    app.workspace.active_tab = app.workspace.opened_files.len() - 1;
                }

                if let Some(i) = tab_to_close {
                    app.workspace.opened_files.remove(i);
                    if app.workspace.active_tab >= app.workspace.opened_files.len() && app.workspace.active_tab > 0 {
                        app.workspace.active_tab -= 1;
                    }
                    if app.workspace.opened_files.is_empty() {
                        app.workspace.opened_files.push(EditorTab::new(None, String::new()));
                        app.workspace.active_tab = 0;
                    }
                }
            });
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        if app.workspace.opened_files.is_empty() { return; }
        
        let active_tab = &mut app.workspace.opened_files[app.workspace.active_tab];
        let old_code = active_tab.code.clone();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                let line_count = active_tab.code.lines().count().max(1);
                let line_numbers = (1..=line_count).map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
                ui.add(egui::TextEdit::multiline(&mut line_numbers.as_str())
                    .font(egui::TextStyle::Monospace)
                    .interactive(false)
                    .frame(false)
                    .text_color(if app.settings.is_dark_mode { egui::Color32::DARK_GRAY } else { egui::Color32::GRAY })
                    .desired_width(35.0));

                let is_dark = app.settings.is_dark_mode;
                let errs = app.workspace.error_lines.clone();
                let highlighter = &mut active_tab.highlighter;

                let mut layouter = |ui: &egui::Ui, text: &str, _wrap_width: f32| {
                    let job = highlighter.layout(text, is_dark, &errs);
                    ui.fonts(|f| f.layout_job(job))
                };

                let mut text_edit = egui::TextEdit::multiline(&mut active_tab.code)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .frame(false)
                    .layouter(&mut layouter);
                
                if !app.settings.word_wrap {
                    text_edit = text_edit.clip_text(false).desired_rows(40);
                }
                
                ui.add(text_edit);

                // Умная обработка удаления (стирание пустых пар кавычек/скобок)
                if active_tab.code.len() < old_code.len() {
                    let mut diff_byte_idx = None;
                    for (i, (a, b)) in active_tab.code.bytes().zip(old_code.bytes()).enumerate() {
                        if a != b { diff_byte_idx = Some(i); break; }
                    }
                    let byte_idx = diff_byte_idx.unwrap_or(active_tab.code.len());
                    
                    if old_code.is_char_boundary(byte_idx) && active_tab.code.is_char_boundary(byte_idx) {
                        let deleted_char = old_code[byte_idx..].chars().next();
                        let next_char = if byte_idx < active_tab.code.len() { active_tab.code[byte_idx..].chars().next() } else { None };
                        let prev_char = if byte_idx > 0 { active_tab.code[..byte_idx].chars().last() } else { None };
                        
                        if let Some(d) = deleted_char {
                            if let Some(n) = next_char {
                                // Случай 1: стерли левую половину "()" или "[]"
                                let should_delete = match (d, n) {
                                    ('(', ')') | ('[', ']') | ('{', '}') | ('"', '"') | ('\'', '\'') => true,
                                    _ => false,
                                };
                                if should_delete {
                                    active_tab.code.remove(byte_idx);
                                }
                            } else if let Some(p) = prev_char {
                                // Случай 2: стерли первую кавычку в "" (срабатывает как удаление в конце одинаковой пары)
                                let should_delete = match (p, d) {
                                    ('"', '"') | ('\'', '\'') => true,
                                    _ => false,
                                };
                                if should_delete {
                                    let p_len = p.len_utf8();
                                    if active_tab.code.is_char_boundary(byte_idx - p_len) {
                                        active_tab.code.remove(byte_idx - p_len);
                                    }
                                }
                            }
                        }
                    }
                } else if active_tab.code.len() > old_code.len() {
                    // Добавление парных символов и авто-табуляция при Enter
                    let mut diff_byte_idx = None;
                    for (i, (a, b)) in active_tab.code.bytes().zip(old_code.bytes()).enumerate() {
                        if a != b { diff_byte_idx = Some(i); break; }
                    }
                    let byte_idx = diff_byte_idx.unwrap_or(old_code.len());
                    let tail_len = old_code.len() - byte_idx;
                    
                    if active_tab.code.is_char_boundary(byte_idx) && active_tab.code.is_char_boundary(active_tab.code.len() - tail_len) {
                        let inserted_str = &active_tab.code[byte_idx..active_tab.code.len() - tail_len];
                        
                        match inserted_str {
                            "(" => { active_tab.code.insert_str(byte_idx + 1, ")"); },
                            "[" => { active_tab.code.insert_str(byte_idx + 1, "]"); },
                            "{" => { active_tab.code.insert_str(byte_idx + 1, "}"); },
                            "\"" => { active_tab.code.insert_str(byte_idx + 1, "\""); },
                            "\'" => { active_tab.code.insert_str(byte_idx + 1, "\'"); },
                            "\n" => {
                                // Берем прошлую строку и вытаскиваем с нее отступ
                                let before_newline = &old_code[..byte_idx];
                                if let Some(last_line) = before_newline.lines().last() {
                                    let mut spaces: String = last_line.chars().take_while(|c| *c == ' ' || *c == '\t').collect();
                                    
                                    // Если открыли новый блок (скобку), добавляем 4 пробела к табуляции
                                    if last_line.trim_end().ends_with('{') {
                                        spaces.push_str("    ");
                                    }
                                    if !spaces.is_empty() {
                                        active_tab.code.insert_str(byte_idx + 1, &spaces);
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            });
        });
    });
}
