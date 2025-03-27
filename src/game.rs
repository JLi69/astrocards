pub mod assets;

use glfw::{WindowEvent, GlfwReceiver};
use assets::textures::TextureManager;
use assets::shaders::ShaderManager;
use assets::models::ModelManager;
use egui_gl_glfw::egui::FontDefinitions;
use crate::impfile;

//Application config values, these are not meant to be changed by normal users
#[derive(Default)]
struct Config {
    font_path: String,
}

pub struct Game {
    pub textures: TextureManager,
    pub shaders: ShaderManager,
    pub models: ModelManager,
    pub fonts: FontDefinitions,
    cfg: Config,
}

type EventHandler = GlfwReceiver<(f64, WindowEvent)>;

impl Game {
    pub fn new() -> Self {
        Self { 
            textures: TextureManager::new(), 
            shaders: ShaderManager::new(), 
            models: ModelManager::new(), 
            fonts: FontDefinitions::default(), 
            cfg: Config::default(),
        }
    }

    pub fn load_config(&mut self, path: &str) {
        let entries = impfile::parse_file(path);
        if entries.is_empty() {
            eprintln!("Error: empty config file");
            return;
        }
        let e = &entries[0];
        self.cfg.font_path = e.get_var("font_path");
    }
}

pub fn process_events(events: &EventHandler) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            _ => {}
        }
    }
}
