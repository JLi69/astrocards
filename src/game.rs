pub mod assets;
pub mod draw;
pub mod sprite;
pub mod update;

use crate::{flashcards::Flashcard, gui::GuiController, impfile};
use assets::models::ModelManager;
use assets::shaders::ShaderManager;
use assets::textures::TextureManager;
use egui_gl_glfw::egui::FontDefinitions;
use glfw::{GlfwReceiver, WindowEvent};
use sprite::{Asteroid, Explosion};

const DEFAULT_SPAWN_INTERVAL: f32 = 8.0;
const DEFAULT_HEALTH: u32 = 5;

//Application config values, these are not meant to be changed by normal users
#[derive(Default)]
struct Config {
    font_path: String,
}

//Calculates how many asteroids are needed to advance to the next level
//Pass in the current level
fn calculate_asteroids_until_next(level: u32) -> u32 {
    match level {
        0..=2 => 7,
        3..=5 => 10,
        6..=8 => 15,
        9..=12 => 20,
        13..=15 => 25,
        _ => 30,
    }
}

fn calculate_spawn_interval(level: u32) -> f32 {
    (DEFAULT_SPAWN_INTERVAL * 0.9f32.powi(level as i32)).max(2.0)
}

pub struct Game {
    pub textures: TextureManager,
    pub shaders: ShaderManager,
    pub models: ModelManager,
    pub fonts: FontDefinitions,
    cfg: Config,
    window_w: i32,
    window_h: i32,
    //Sprites
    asteroid_spawn_timer: f32,
    spawn_interval: f32,
    pub asteroids: Vec<Asteroid>,
    pub explosions: Vec<Explosion>,
    time: f32,
    pub answer: String,
    pub flashcards: Vec<Flashcard>,
    //Player info
    pub health: u32,
    pub score: u64,
    pub level: u32,
    //Every time the player destroys an asteroid, this decreases by 1
    //When this hits 0, advance to the next level
    asteroids_until_next_level: u32,
}

type EventHandler = GlfwReceiver<(f64, WindowEvent)>;

fn handle_window_resize(gamestate: &mut Game, w: i32, h: i32) {
    unsafe {
        gl::Viewport(0, 0, w, h);
        gamestate.window_w = w;
        gamestate.window_h = h;
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            textures: TextureManager::new(),
            shaders: ShaderManager::new(),
            models: ModelManager::new(),
            fonts: FontDefinitions::default(),
            cfg: Config::default(),
            window_w: 0,
            window_h: 0,
            asteroid_spawn_timer: 0.0,
            spawn_interval: calculate_spawn_interval(1),
            asteroids: vec![],
            explosions: vec![],
            time: 0.0,
            answer: String::new(),
            flashcards: vec![],
            health: DEFAULT_HEALTH,
            score: 0,
            level: 1,
            asteroids_until_next_level: calculate_asteroids_until_next(1),
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

    pub fn process_events(&mut self, events: &EventHandler, gui_controller: &mut GuiController) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                WindowEvent::Size(w, h) => handle_window_resize(self, w, h),
                WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, _) => {
                    //Clear answer
                    self.submit_answer();
                    continue;
                }
                _ => {}
            }
            gui_controller.handle_window_event(event);
        }
    }

    pub fn submit_answer(&mut self) {
        //Ignore if game over
        if self.game_over() {
            return;
        }

        //Destroy asteroids
        //Find the lowest asteroid
        let mut index = None;
        let mut lowest_y = 999.0;
        for (i, asteroid) in self.asteroids.iter().enumerate() {
            //ignore asteroids that are off-screen
            if asteroid.above_top() {
                continue;
            }

            if asteroid.flashcard.answer == self.answer && lowest_y > asteroid.sprite.y {
                lowest_y = asteroid.sprite.y;
                index = Some(i);
            }
        }

        if let Some(index) = index {
            self.asteroids[index].deleted = true;
            self.asteroids[index].destroyed = true;
            self.score += 100 * self.level as u64;
            self.asteroids_until_next_level -= 1;
        }

        self.answer.clear();

        //Check if we advanced to the next level
        if self.asteroids_until_next_level == 0 {
            self.level += 1;
            self.asteroids = self.asteroids.iter()
                .filter(|asteroid| asteroid.destroyed)
                .cloned()
                .collect();
            self.asteroids_until_next_level = calculate_asteroids_until_next(self.level);
            self.spawn_interval = calculate_spawn_interval(self.level);
        }
    }

    pub fn init_window_dimensions(&mut self, dimensions: (i32, i32)) {
        let (w, h) = dimensions;
        self.window_w = w;
        self.window_h = h;
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
