pub mod assets;
pub mod draw;
pub mod sprite;
pub mod update;

use std::{collections::VecDeque, fs::File, io::Read};
use crate::{flashcards::Flashcard, gui::GuiController, impfile, log::LogItem};
use assets::models::ModelManager;
use assets::shaders::ShaderManager;
use assets::textures::TextureManager;
use egui_gl_glfw::egui::{FontDefinitions, Event, MouseWheelUnit, emath};
use glfw::{GlfwReceiver, WindowEvent};
use sprite::{Asteroid, Explosion};

const DEFAULT_SPAWN_INTERVAL: f32 = 8.0;
const DEFAULT_HEALTH: u32 = 5;
pub const LEVELUP_ANIMATION_LENGTH: f32 = 2.0; //In seconds
pub const DAMAGE_ANIMATION_LENGTH: f32 = 1.0; //In seconds

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameScreen {
    MainMenu,
    About,
    LoadFlashcards,
    Game,
}

//Application config values, these are not meant to be changed by normal users
#[derive(Default)]
struct Config {
    font_path: String,
}

//Calculates how many asteroids are needed to advance to the next level
//Pass in the current level
fn calculate_asteroids_until_next(level: u32) -> u32 {
    match level {
        1 => 5,
        2 => 7,
        3..=5 => 10,
        6..=8 => 15,
        9..=12 => 20,
        13..=15 => 25,
        16..=18 => 30,
        _ => 40,
    }
}

fn calculate_spawn_interval(level: u32) -> f32 {
    (DEFAULT_SPAWN_INTERVAL * 0.85f32.powi(level as i32)).max(2.0)
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
    pub levelup_animation_timer: f32,
    pub damage_animation_timer: f32,
    pub log: VecDeque<LogItem>,
    pub current_screen: GameScreen,
    pub about_text: Vec<String>,
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
            levelup_animation_timer: 0.0,
            damage_animation_timer: 0.0,
            log: VecDeque::new(),
            current_screen: GameScreen::Game,
            about_text: vec![],
        }
    }

    pub fn restart(&mut self) {
        /*
         * Reset values:
         * -------------
         * asteroid_spawn_timer: 0.0
         * spawn_interval: calculate_spawn_interval(1)
         * asteroids: vec![]
         * explosions: vec![]
         * answer: String::new()
         * health: DEFAULT_HEALTH
         * score: 0
         * level: 1
         * asteroids_until_next_level: calculate_asteroids_until_next(1)
         * levelup_animation_timer: 0.0
         * damage_animation_timer: 0.0
         * log: VecDeque::new(),
         * */

        self.asteroid_spawn_timer = 0.0;
        self.asteroids.clear();
        self.explosions.clear();
        self.answer.clear();
        self.health = DEFAULT_HEALTH;
        self.score = 0;
        self.level = 1;
        self.spawn_interval = calculate_spawn_interval(self.level);
        self.asteroids_until_next_level = calculate_asteroids_until_next(self.level);
        self.levelup_animation_timer = 0.0;
        self.damage_animation_timer = 0.0;
        self.log.clear();
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
                WindowEvent::Key(glfw::Key::Enter, _, glfw::Action::Press, _)
                | WindowEvent::Key(glfw::Key::KpEnter, _, glfw::Action::Press, _) => {
                    //Clear answer
                    self.submit_answer();
                    continue;
                }
                WindowEvent::Scroll(x, y) => {
                    let mouse_wheel = Event::MouseWheel {
                        unit: MouseWheelUnit::Line,
                        delta: emath::vec2(x as f32, y as f32),
                        modifiers: gui_controller.input_state.modifiers,
                    };
                    gui_controller.input_state.input.events.push(mouse_wheel);
                }
                _ => {}
            }
            gui_controller.handle_window_event(event);
        }
    }

    pub fn submit_answer(&mut self) {
        //Ignore if game over
        if self.game_over() || self.answer.is_empty() {
            return;
        }

        //Destroy asteroids
        //Find the lowest asteroid
        let mut index = None;
        let mut lowest_y = 999.0;
        let mut found_red = false;
        for (i, asteroid) in self.asteroids.iter().enumerate() {
            //ignore asteroids that are off-screen
            if asteroid.above_top() {
                continue;
            }

            if asteroid.is_red {
                found_red = true;
            }

            if asteroid.flashcard.answer == self.answer && lowest_y > asteroid.sprite.y {
                lowest_y = asteroid.sprite.y;
                index = Some(i);
            }
        }

        if let Some(index) = index {
            self.asteroids[index].deleted = true;
            self.asteroids[index].destroyed = true;
            if self.asteroids[index].is_red {
                //2 times as many points if it is red
                self.score += 200 * self.level as u64;
            } else {
                self.score += 100 * self.level as u64;
            }
            if self.asteroids_until_next_level > 0 {
                self.asteroids_until_next_level -= 1;
            }
        }

        //lose helath if we enter something wrong and there is a red asteroid
        //on the screen and destroy all red asteroids on the screen
        if found_red && index.is_none() {
            if self.health > 0 {
                self.health -= 1;
            }
            if self.health > 0 {
                self.damage_animation_timer = DAMAGE_ANIMATION_LENGTH;
            }
        }

        self.answer.clear();
        self.advance_to_next_level();
    }

    pub fn advance_to_next_level(&mut self) {
        if self.game_over() {
            return;
        }

        //If we are to advance ont the next level, delete any asteroids that
        //are above the top of the screen
        if self.asteroids_until_next_level == 0 {
            self.asteroids = self
                .asteroids
                .iter()
                .filter(|asteroid| !asteroid.above_top())
                .cloned()
                .collect();
        }

        //Count any non-destroyed asteroids
        let mut count = 0;
        for asteroid in &self.asteroids {
            if asteroid.destroyed || asteroid.deleted {
                continue;
            }
            count += 1;
        }

        //Check if we advanced to the next level
        if self.asteroids_until_next_level == 0 && count == 0 {
            self.level += 1;
            self.asteroids_until_next_level = calculate_asteroids_until_next(self.level);
            self.spawn_interval = calculate_spawn_interval(self.level);
            self.levelup_animation_timer = LEVELUP_ANIMATION_LENGTH;
        }
    }

    pub fn levelup_animation_perc(&self) -> f32 {
        1.0 - self.levelup_animation_timer / LEVELUP_ANIMATION_LENGTH
    }

    pub fn damage_animation_perc(&self) -> f32 {
        self.damage_animation_timer / DAMAGE_ANIMATION_LENGTH
    }

    pub fn init_window_dimensions(&mut self, dimensions: (i32, i32)) {
        let (w, h) = dimensions;
        self.window_w = w;
        self.window_h = h;
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn update_time(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        (self.window_w, self.window_h)
    }

    //Opens assets/credits.txt and assets/about.txt
    pub fn load_about(&mut self) {
        //Load about
        if let Ok(mut file) = File::open("assets/about.txt") {
            let mut buf = String::new();
            let res = file.read_to_string(&mut buf);
            if let Err(msg) = res {
                eprintln!("{msg}");
            }
            self.about_text.extend(buf.lines().map(|s| s.to_string())); 
        }

        //Load credits
        if let Ok(mut file) = File::open("assets/credits.txt") {
            let mut buf = String::new();
            let res = file.read_to_string(&mut buf);
            if let Err(msg) = res {
                eprintln!("{msg}");
            }
            self.about_text.extend(buf.lines().map(|s| s.to_string())); 
        } 
    }
}
