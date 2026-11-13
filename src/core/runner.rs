use std::path::PathBuf;
use std::process::Command;
use std::collections::HashSet;
use regex::Regex;
use crate::core::i18n::{t, Language};

pub struct Runner;

impl Runner {
    pub fn execute(path: &PathBuf, lang: Language) -> (String, HashSet<usize>) {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        let output = match ext {
            "py" => Command::new("python").arg(path).output(),
            "rs" => Command::new("cargo").arg("run").current_dir(path.parent().unwrap()).output().or_else(|_| {
                Command::new("rustc").arg(path).output()
            }),
            "cpp" => {
                let exe = path.with_extension("exe");
                let comp = Command::new("g++").arg(path).arg("-o").arg(&exe).output();
                if let Ok(out) = comp {
                    if out.status.success() { Command::new(&exe).output() } else { Ok(out) }
                } else {
                    comp
                }
            },
            _ => return (format!("{} .{}", t(lang, "run_unsupported"), ext), HashSet::new()),
        };

        match output {
            Ok(out) => {
                let mut stdout_text = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr_text = String::from_utf8_lossy(&out.stderr).to_string();
                let mut errors = HashSet::new();

                let re_rust = Regex::new(r":(\d+):\d+").unwrap();
                for cap in re_rust.captures_iter(&stderr_text) {
                    if let Ok(num) = cap[1].parse::<usize>() { errors.insert(num); }
                }
                
                let re_py = Regex::new(r"line (\d+)").unwrap();
                for cap in re_py.captures_iter(&stderr_text) {
                    if let Ok(num) = cap[1].parse::<usize>() { errors.insert(num); }
                }

                if !stderr_text.is_empty() {
                    if stdout_text.is_empty() {
                        stdout_text = stderr_text;
                    } else {
                        stdout_text.push_str(&t(lang, "errors_warnings"));
                        stdout_text.push_str(&stderr_text);
                    }
                }
                (stdout_text, errors)
            }
            Err(e) => (format!("{} {}", t(lang, "crit_error"), e), HashSet::new()),
        }
    }
}
