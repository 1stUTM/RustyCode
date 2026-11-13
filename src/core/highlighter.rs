use eframe::egui;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;

pub struct Highlighter {
    last_hash: u64,
    cached_job: egui::text::LayoutJob,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            last_hash: 0,
            cached_job: egui::text::LayoutJob::default(),
        }
    }

    pub fn layout(&mut self, text: &str, is_dark: bool, error_lines: &HashSet<usize>) -> egui::text::LayoutJob {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        is_dark.hash(&mut hasher);
        error_lines.len().hash(&mut hasher); 
        let hash = hasher.finish();

        if hash == self.last_hash && !text.is_empty() {
            return self.cached_job.clone();
        }

        let mut job = egui::text::LayoutJob::default();
        let font_id = egui::FontId::monospace(14.0);
        
        let normal_color = if is_dark { egui::Color32::from_rgb(220, 220, 220) } else { egui::Color32::from_rgb(40, 40, 40) };
        let keyword_color = egui::Color32::from_rgb(198, 120, 221);
        let string_color = egui::Color32::from_rgb(152, 195, 121);
        let fn_color = egui::Color32::from_rgb(97, 175, 239);
        
        // Добавил Python keywords
        let keywords = ["fn", "let", "mut", "pub", "struct", "impl", "enum", "match", "return", "if", "else", "use", "def", "class", "import", "from", "for", "while", "in", "and", "or", "not", "True", "False", "None"];
        // Встроенные функции
        let builtins = ["println", "print", "len", "range", "type", "int", "str", "float", "list", "dict", "set", "tuple"];

        let mut line_idx = 1;
        let mut in_string = false;
        let mut string_char = ' ';

        // Простой токенизатор, который не бьет строки на части из-за запятых и пробелов
        let mut current_word = String::new();

        let flush_word = |job: &mut egui::text::LayoutJob, word: &mut String, in_str: bool, err: bool| {
            if word.is_empty() { return; }
            let mut color = normal_color;
            let trimmed = word.trim_matches(|c: char| c.is_whitespace() || c == '(' || c == ')' || c == ':' || c == '[' || c == ']' || c == '{' || c == '}');

            if in_str {
                color = string_color;
            } else if keywords.contains(&trimmed) {
                color = keyword_color;
            } else if builtins.contains(&trimmed) {
                color = fn_color;
            } else if word.ends_with('(') || word.contains('!') { // Очень базовая эвристика для функций и макросов
                 color = fn_color;
            }

            let mut fmt = egui::TextFormat::simple(font_id.clone(), color);
            if err {
                fmt.underline = egui::Stroke::new(2.0_f32, egui::Color32::RED);
                fmt.background = egui::Color32::from_rgba_unmultiplied(255, 0, 0, 40);
            }
            job.append(&word, 0.0, fmt);
            word.clear();
        };


        for c in text.chars() {
            let is_err = error_lines.contains(&line_idx);

            if in_string {
                current_word.push(c);
                if c == string_char {
                    in_string = false;
                    flush_word(&mut job, &mut current_word, true, is_err);
                }
            } else {
                if c == '"' || c == '\'' {
                    flush_word(&mut job, &mut current_word, false, is_err);
                    in_string = true;
                    string_char = c;
                    current_word.push(c);
                } else if c.is_whitespace() || "()[]{}.,;:".contains(c) {
                    flush_word(&mut job, &mut current_word, false, is_err);
                    // Добавляем сам разделитель как отдельное слово нормального цвета
                    let mut sep = String::from(c);
                    flush_word(&mut job, &mut sep, false, is_err);
                } else {
                    current_word.push(c);
                }
            }

            if c == '\n' { line_idx += 1; }
        }
        
        // Добиваем остаток
        let is_err = error_lines.contains(&line_idx);
        flush_word(&mut job, &mut current_word, in_string, is_err);

        self.last_hash = hash;
        self.cached_job = job.clone();
        job
    }
}
