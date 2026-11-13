use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Language { En, Ru, Zh, Es, De, Fr }

impl Default for Language { fn default() -> Self { Self::En } }

pub fn t(lang: Language, key: &str) -> String {
    let s = match key {
        "menu_file" => match lang { Language::Ru => "Файл", Language::Zh => "文件", Language::Es => "Archivo", Language::De => "Datei", Language::Fr => "Fichier", _ => "File" },
        "menu_open_folder" => match lang { Language::Ru => "📁 Открыть папку...", Language::Zh => "📁 打开文件夹...", Language::Es => "📁 Abrir carpeta...", Language::De => "📁 Ordner öffnen...", Language::Fr => "📁 Ouvrir dossier...", _ => "📁 Open Folder..." },
        "menu_save" => match lang { Language::Ru => "💾 Сохранить (Ctrl+S)", Language::Zh => "💾 保存 (Ctrl+S)", Language::Es => "💾 Guardar (Ctrl+S)", Language::De => "💾 Speichern (Ctrl+S)", Language::Fr => "💾 Enregistrer (Ctrl+S)", _ => "💾 Save (Ctrl+S)" },
        "menu_settings" => match lang { Language::Ru => "Настройки", Language::Zh => "设置", Language::Es => "Ajustes", Language::De => "Einstellungen", Language::Fr => "Paramètres", _ => "Settings" },
        "menu_language" => match lang { Language::Ru => "Язык (Language)", Language::Zh => "语言 (Language)", Language::Es => "Idioma (Language)", Language::De => "Sprache (Language)", Language::Fr => "Langue (Language)", _ => "Language" },
        "zoom_in" => match lang { Language::Ru => "Увеличить масштаб", Language::Zh => "放大", Language::Es => "Acercar", Language::De => "Vergrößern", Language::Fr => "Zoomer", _ => "Zoom In" },
        "zoom_out" => match lang { Language::Ru => "Уменьшить масштаб", Language::Zh => "缩小", Language::Es => "Alejar", Language::De => "Verkleinern", Language::Fr => "Dézoomer", _ => "Zoom Out" },
        "zoom_reset" => match lang { Language::Ru => "Сбросить масштаб", Language::Zh => "重置缩放", Language::Es => "Restablecer zoom", Language::De => "Zoom zurücksetzen", Language::Fr => "Réinitialiser le zoom", _ => "Reset Zoom" },
        "show_explorer" => match lang { Language::Ru => "Показать Проводник", Language::Zh => "显示资源管理器", Language::Es => "Mostrar explorador", Language::De => "Explorer anzeigen", Language::Fr => "Afficher l'explorateur", _ => "Show Explorer" },
        "show_terminal" => match lang { Language::Ru => "Показать Терминал", Language::Zh => "显示终端", Language::Es => "Mostrar terminal", Language::De => "Terminal anzeigen", Language::Fr => "Afficher le terminal", _ => "Show Terminal" },
        "word_wrap" => match lang { Language::Ru => "Перенос строк", Language::Zh => "自动换行", Language::Es => "Ajuste de línea", Language::De => "Zeilenumbruch", Language::Fr => "Retour à la ligne", _ => "Word Wrap" },
        "run_code" => match lang { Language::Ru => "▶ ЗАПУСТИТЬ", Language::Zh => "▶ 运行", Language::Es => "▶ EJECUTAR", Language::De => "▶ AUSFÜHREN", Language::Fr => "▶ EXÉCUTER", _ => "▶ RUN" },
        "theme_light" => match lang { Language::Ru => "☀ Светлая", Language::Zh => "☀ 浅色", Language::Es => "☀ Claro", Language::De => "☀ Hell", Language::Fr => "☀ Clair", _ => "☀ Light" },
        "theme_dark" => match lang { Language::Ru => "🌙 Темная", Language::Zh => "🌙 深色", Language::Es => "🌙 Oscuro", Language::De => "🌙 Dunkel", Language::Fr => "🌙 Sombre", _ => "🌙 Dark" },
        "explorer_title" => match lang { Language::Ru => "ПРОВОДНИК", Language::Zh => "资源管理器", Language::Es => "EXPLORADOR", Language::De => "EXPLORER", Language::Fr => "EXPLORATEUR", _ => "EXPLORER" },
        "open_project_folder" => match lang { Language::Ru => "📁 Открыть папку", Language::Zh => "📁 打开项目", Language::Es => "📁 Abrir carpeta", Language::De => "📁 Projekt öffnen", Language::Fr => "📁 Ouvrir le dossier", _ => "📁 Open Folder" },
        "no_folder" => match lang { Language::Ru => "Папка не выбрана", Language::Zh => "未选择文件夹", Language::Es => "Sin carpeta", Language::De => "Kein Ordner", Language::Fr => "Aucun dossier", _ => "No folder" },
        "terminal_title" => match lang { Language::Ru => "Терминал", Language::Zh => "终端", Language::Es => "Terminal", Language::De => "Terminal", Language::Fr => "Terminal", _ => "Terminal" },
        "running" => match lang { Language::Ru => "Выполнение...", Language::Zh => "运行中...", Language::Es => "Ejecutando...", Language::De => "Läuft...", Language::Fr => "Exécution...", _ => "Running..." },
        "clear" => match lang { Language::Ru => "Очистить", Language::Zh => "清除", Language::Es => "Limpiar", Language::De => "Leeren", Language::Fr => "Effacer", _ => "Clear" },
        "untitled" => match lang { Language::Ru => "Без имени", Language::Zh => "未命名", Language::Es => "Sin título", Language::De => "Unbenannt", Language::Fr => "Sans titre", _ => "Untitled" },
        "system_ready" => match lang { Language::Ru => "Система готова. Ожидание запуска...", Language::Zh => "系统已准备就绪。", Language::Es => "Sistema listo. Esperando...", Language::De => "System bereit. Warten...", Language::Fr => "Système prêt. En attente...", _ => "System ready. Waiting for launch..." },
        "compiling" => match lang { Language::Ru => "Компиляция и запуск...\n", Language::Zh => "编译并运行...\n", Language::Es => "Compilando y ejecutando...\n", Language::De => "Kompiliere und starte...\n", Language::Fr => "Compilation et exécution...\n", _ => "Compiling and running...\n" },
        "save_first" => match lang { Language::Ru => "Сначала сохраните файл!", Language::Zh => "请先保存文件！", Language::Es => "¡Guarde el archivo primero!", Language::De => "Bitte zuerst speichern!", Language::Fr => "Enregistrez d'abord le fichier !", _ => "Save the file first!" },
        "default_code" => match lang { 
            Language::Ru => r#"// RustyCode
fn main() {
    println!("Всё работает!");
}"#, 
            Language::Zh => r#"// RustyCode
fn main() {
    println!("一切正常!");
}"#, 
            Language::Es => r#"// RustyCode
fn main() {
    println!("¡Todo funciona!");
}"#, 
            _ => r#"// RustyCode
fn main() {
    println!("Everything works!");
}"# 
        },
        "run_unsupported" => match lang { Language::Ru => "Запуск не поддерживается:", Language::Zh => "不支持运行:", Language::Es => "Ejecución no soportada:", Language::De => "Ausführung nicht unterstützt:", Language::Fr => "Exécution non prise en charge:", _ => "Running is not supported:" },
        "crit_error" => match lang { Language::Ru => "Критическая ошибка:", Language::Zh => "严重错误:", Language::Es => "Error crítico:", Language::De => "Kritischer Fehler:", Language::Fr => "Erreur critique:", _ => "Critical error:" },
        "errors_warnings" => match lang { Language::Ru => "\n\n--- ОШИБКИ/ПРЕДУПРЕЖДЕНИЯ ---\n", Language::Zh => "\n\n--- 错误/警告 ---\n", Language::Es => "\n\n--- ERRORES/ADVERTENCIAS ---\n", Language::De => "\n\n--- FEHLER/WARNUNGEN ---\n", Language::Fr => "\n\n--- ERREURS/AVERTISSEMENTS ---\n", _ => "\n\n--- ERRORS/WARNINGS ---\n" },
        _ => key,
    };
    s.to_string()
}
