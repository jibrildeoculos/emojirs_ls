use colored::Colorize;
use std::{fs, io};
use std::path::Path;
use crate::icons::ICONS;

pub struct Dir {
    pub dotfiles: bool,
}

impl Dir {
    pub fn list<P: AsRef<Path>>(&self, dir: P) -> std::io::Result<()> { 

        let path = dir.as_ref();

        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid directory {}", path.display())
            ));
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().and_then(|e| e.to_str()).unwrap_or("<unknown>");

            if !self.dotfiles && name.starts_with('.') {
                continue;
            }

            match path.is_dir() {
                true => {
                    println!(" {} 📁 {}", "—".bright_blue(), name.bright_white())
                }
            
                false => {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                
                    let icon = ICONS.get(ext).unwrap_or(&"📄");

                    println!(" {} {} {}", "—".bright_green(), icon, name.bright_white());
                }
            }
        }
        Ok(())
    }
}
