use super::{
    DAMAGE_ANIMATION_LENGTH, Game,
    draw::{CANVAS_H, CANVAS_W},
    sprite::{Asteroid, Explosion},
};
use crate::{flashcards::Flashcard, log::LogItem};

const ASTEROID_SIZE: f32 = 80.0;
pub const EXPLOSION_LIFETIME: f32 = 1.0; //1 second
pub const MAX_LOG_LEN: usize = 16;

fn calculate_asteroid_speed(level: u32) -> f32 {
    CANVAS_H / (25.0 - (level - 1) as f32).max(5.0)
}

pub fn is_red(rand_value: u32, level: u32) -> bool {
    match level {
        1 => false,
        2 => rand_value % 7 == 0,
        3..=5 => rand_value % 10 == 0,
        6..=8 => rand_value % 8 == 0,
        9..=15 => rand_value % 6 == 0,
        _ => rand_value % 5 == 0,
    }
}

fn intersects_another_asteroid(asteroid: &Asteroid, other: &[Asteroid]) -> bool {
    for asteroid2 in other {
        let dx = asteroid2.sprite.x - asteroid.sprite.x;
        let dy = asteroid2.sprite.y - asteroid.sprite.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < (asteroid.sprite.width + asteroid2.sprite.width) * 0.5 {
            return true;
        }
    }

    false
}

impl Game {
    pub fn get_random_card(&self) -> Flashcard {
        if self.flashcards.is_empty() {
            Flashcard::none()
        } else {
            self.flashcards[rand::random_range(0..self.flashcards.len())].clone()
        }
    }

    pub fn spawn_asteroid(&mut self, dt: f32) {
        //Do not spawn any extra asteroids if we are advancing to the next level
        if self.asteroids_until_next_level == 0 {
            return;
        }

        self.asteroid_spawn_timer -= dt;
        if self.asteroid_spawn_timer > 0.0 {
            return;
        }
        if self.asteroids_until_next_level == 0 {
            return;
        }
        self.asteroid_spawn_timer = self.spawn_interval;

        let range = CANVAS_W - ASTEROID_SIZE * 2.0;
        //Spawn an extra asteroid
        if rand::random() {
            let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
            let y = CANVAS_H + ASTEROID_SIZE + rand::random::<f32>() * 320.0;
            let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            let flashcard = self.get_random_card();
            let red = is_red(rand::random(), self.level);
            let new_asteroid = Asteroid::new(x, y, ASTEROID_SIZE, rotation, flashcard, red);
            if !intersects_another_asteroid(&new_asteroid, &self.asteroids) {
                self.asteroids.push(new_asteroid);
            }
        }

        //In later levels spawn a third asteroid
        if rand::random::<u32>() % 4 == 0 && self.level >= 6 {
            let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
            let y = CANVAS_H + ASTEROID_SIZE + rand::random::<f32>() * 320.0;
            let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            let flashcard = self.get_random_card();
            let red = is_red(rand::random(), self.level);
            let new_asteroid = Asteroid::new(x, y, ASTEROID_SIZE, rotation, flashcard, red);
            if !intersects_another_asteroid(&new_asteroid, &self.asteroids) {
                self.asteroids.push(new_asteroid);
            }
        }

        let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
        let y = CANVAS_H / 2.0 + ASTEROID_SIZE / 2.0;
        let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let flashcard = self.get_random_card();
        let red = is_red(rand::random(), self.level);
        let new_asteroid = Asteroid::new(x, y, ASTEROID_SIZE, rotation, flashcard, red);
        if !intersects_another_asteroid(&new_asteroid, &self.asteroids) {
            self.asteroids.push(new_asteroid);
        }
    }

    //Returns if its game over
    pub fn game_over(&self) -> bool {
        self.health == 0
    }

    //Update log
    pub fn update_log(&mut self, dt: f32) {
        if self.log.is_empty() {
            return;
        }

        let can_pop = if let Some(log_item) = self.log.get_mut(0) {
            log_item.update(dt);
            log_item.can_delete() || self.log.len() > MAX_LOG_LEN
        } else {
            self.log.len() > MAX_LOG_LEN
        };

        if can_pop {
            self.log.pop_front();
            while self.log.len() > MAX_LOG_LEN {
                self.log.pop_front();
            }
            if let Some(log_item) = self.log.get_mut(0) {
                log_item.reset_timer();
            }
        }
    }

    pub fn delete_asteroids(&mut self) {
        //Delete asteroids
        let mut keep = vec![];
        for asteroid in &self.asteroids {
            if !asteroid.deleted {
                keep.push(asteroid.clone());
                continue;
            }

            //If the asteroid hits the bottom of the screen, lose health
            if asteroid.at_bottom() && self.health > 0 {
                self.log.push_back(LogItem::new(asteroid.flashcard.clone()));
                self.health -= 1;
                //If it's red, then lose instantly
                if asteroid.is_red {
                    self.health = 0;
                }

                if self.health > 0 {
                    self.damage_animation_timer = DAMAGE_ANIMATION_LENGTH;
                }
            }

            //Add explosion
            if asteroid.at_bottom() || asteroid.destroyed {
                let (x, y) = (asteroid.sprite.x, asteroid.sprite.y);
                self.explosions.push(Explosion::new(x, y));
                self.audio.play("explosion");
            }
        }
        self.asteroids = keep;
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        if self.levelup_animation_timer > 0.0 {
            self.levelup_animation_timer -= dt;
        }

        if self.damage_animation_timer > 0.0 {
            self.damage_animation_timer -= dt;
        }

        //Update explosions
        for explosion in &mut self.explosions {
            explosion.update(dt);
        }
        //Delete explosions
        self.explosions = self
            .explosions
            .iter()
            .filter(|e| e.time < EXPLOSION_LIFETIME)
            .cloned()
            .collect();

        //Delete asteroids
        self.delete_asteroids();

        //Stop updating if game over
        if self.game_over() {
            return;
        }

        //Spawn asteroids
        self.spawn_asteroid(dt);

        //Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update(dt, calculate_asteroid_speed(self.level));
        }

        self.advance_to_next_level();
        self.update_log(dt);
    }
}
