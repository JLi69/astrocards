pub mod audio;
pub mod models;
pub mod shaders;
pub mod textures;

use super::Game;
use egui_backend::egui::{FontData, FontDefinitions, FontFamily};
use egui_gl_glfw as egui_backend;
use std::{fs::File, io::Read};

pub fn open_file(path: &str) -> Result<File, String> {
    //Search in the current directory first
    let file = File::open(path);
    if file.is_ok() {
        return file.map_err(|e| e.to_string());
    }

    //If on a unix system, search in /usr/share/games/astrocards/
    #[cfg(unix)]
    {
        let usr_share = format!("/usr/share/games/astrocards/{path}");
        let file = File::open(usr_share);
        if file.is_ok() {
            return file.map_err(|e| e.to_string());
        }
    }

    Err(format!("File does not exist: {path}"))
}

pub fn load_font(path: &str, fonts: &mut FontDefinitions) {
    let font_file = open_file(path);
    match font_file {
        Ok(mut font_file) => {
            let mut bytes = vec![];
            let res = font_file.read_to_end(&mut bytes);
            match res {
                Ok(sz) => eprintln!("read {sz} bytes from {path}"),
                Err(msg) => eprintln!("{msg}"),
            }
            fonts
                .font_data
                .insert("font".to_string(), FontData::from_owned(bytes).into());
        }
        Err(msg) => {
            eprintln!("Failed to open: {path}");
            eprintln!("{msg}");
        }
    }

    if let Some(prop) = fonts.families.get_mut(&FontFamily::Proportional) {
        prop.insert(0, "font".to_string());
    }

    if let Some(mono) = fonts.families.get_mut(&FontFamily::Monospace) {
        mono.insert(0, "font".to_string());
    }
}

impl Game {
    pub fn get_font(&self) -> FontDefinitions {
        self.fonts.clone()
    }

    pub fn load_assets(&mut self) {
        load_font(&self.cfg.font_path, &mut self.fonts);
        self.models.add_default_models();
        self.shaders.load_shaders("assets/shaders.impfile");
        self.textures.load_textures("assets/textures.impfile");
        self.audio.load_audio("assets/audio.impfile");
    }
}
