// icons.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static ICONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        // Shells
        ("fish", "🐠"), // Fish
        ("bash", "🐚"), // Bash
        ("sh", "🐚"), // Bourne Shell
        ("bat", "🦇"), // Bacth
        ("ps1", "💪"), // PowerShell
        // Programing Languages
        ("pl", "🐪"), // Perl
        ("rs", "🦀"), // Rust
        ("lsp", "🦖"), // Lisp and Common Lisp
        ("java", "☕"), // Java
        ("lua", "🌕"), ("luac", "🌕"), // Lua
        ("nim", "👑"), // Nim
        ("ml", "🐫"), // OCaml
        ("py", "🐍"), ("pyc", "🐍"), // Python
        // Documents
        ("docx", "📘"), // Word Document
        ("xlsx", "📗"), // Excel Spreadsheet
        ("pptx", "📙"), // PowerPoint Presentation
        ("pdf", "📃"), // PDF
        ("epub", "📃"), // EPUB
        ("txt", "📝"), // Plain Text
        ("csv", "📊"), // CSV
        // Files
        ("tmp", "⏳"),
        ("log", "📜"),
        // Executables
        ("o", "⚒️"),
        ("obj", "🧱"),
        ("so", "🔩"),
        ("lib", "🪛"),
        ("dll", "🔧"),
        ("pdb", "🔨"),
        ("exe", "🧱"),
        // Security
        ("lock", "🔒"),
    ])
});
