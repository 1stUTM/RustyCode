use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
use std::sync::mpsc::{channel, Receiver};

use super::runner::Runner;
use super::highlighter::Highlighter;
use crate::core::i18n::{t, Language};

pub struct EditorTab {
    pub path: Option<PathBuf>,
    pub code: String,
    pub saved_code: String,
    pub highlighter: Highlighter,
}

impl EditorTab {
    pub fn new(path: Option<PathBuf>, code: String) -> Self {
        Self {
            path,
            code: code.clone(),
            saved_code: code,
            highlighter: Highlighter::new(),
        }
    }
    pub fn is_dirty(&self) -> bool {
        self.code != self.saved_code
    }
}

pub struct Workspace {
    pub opened_files: Vec<EditorTab>,
    pub active_tab: usize,
    pub last_directory: Option<PathBuf>,
    pub output_text: String,
    pub error_lines: HashSet<usize>,
    pub run_receiver: Option<Receiver<(String, HashSet<usize>)>>,
    pub is_running: bool,
}

impl Workspace {
    pub fn new(lang: Language) -> Self {
        Self {
            opened_files: vec![EditorTab::new(None, t(lang, "default_code"))],
            active_tab: 0,
            last_directory: None,
            output_text: t(lang, "system_ready"),
            error_lines: HashSet::new(),
            run_receiver: None,
            is_running: false,
        }
    }

    pub fn tick_runner(&mut self) {
        if let Some(rx) = &self.run_receiver {
            if let Ok((out, errs)) = rx.try_recv() {
                self.output_text = out;
                self.error_lines = errs;
                self.is_running = false;
                self.run_receiver = None;
            }
        }
    }

    pub fn handle_hotkeys(&mut self, ctx: &egui::Context, lang: Language) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
            self.save_current_file();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::R)) {
            self.run_current_code(lang);
        }
    }

    pub fn open_folder(&mut self) {
        if let Some(folder) = FileDialog::new().pick_folder() {
            self.last_directory = Some(folder);
        }
    }

    pub fn open_file(&mut self, path: PathBuf) {
        if let Some(index) = self.opened_files.iter().position(|t| t.path == Some(path.clone())) {
            self.active_tab = index;
            return;
        }
        if let Ok(code) = fs::read_to_string(&path) {
            self.opened_files.push(EditorTab::new(Some(path), code));
            self.active_tab = self.opened_files.len() - 1;
        }
    }

    pub fn save_current_file(&mut self) {
        if self.opened_files.is_empty() { return; }
        let active = &mut self.opened_files[self.active_tab];
        if let Some(path) = &active.path {
            if fs::write(path, &active.code).is_ok() {
                active.saved_code = active.code.clone();
            }
        } else if let Some(path) = FileDialog::new().save_file() {
            if fs::write(&path, &active.code).is_ok() {
                active.path = Some(path);
                active.saved_code = active.code.clone();
            }
        }
    }

    pub fn run_current_code(&mut self, lang: Language) {
        self.save_current_file();
        if self.opened_files.is_empty() { return; }
        
        let active = &self.opened_files[self.active_tab];
        if let Some(path) = &active.path {
            if self.is_running { return; }

            self.output_text = t(lang, "compiling");
            self.error_lines.clear();
            self.is_running = true;

            let path_clone = path.clone();
            let (tx, rx) = channel();
            self.run_receiver = Some(rx);

            std::thread::spawn(move || {
                let result = Runner::execute(&path_clone, lang);
                let _ = tx.send(result);
            });
        } else {
            self.output_text = t(lang, "save_first");
        }
    }
}
