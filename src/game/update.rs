use super::{
    Game,
    draw::{CANVAS_H, CANVAS_W},
    sprite::{Asteroid, Explosion},
};
use crate::flashcards::Flashcard;

const ASTEROID_SIZE: f32 = 80.0;
pub const EXPLOSION_LIFETIME: f32 = 1.0; //1 second

fn calculate_asteroid_speed(level: u32) -> f32 {
    CANVAS_H / (25.0 - (level - 1) as f32).max(5.0)
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
            let new_asteroid = Asteroid::new(x, y, ASTEROID_SIZE, rotation, flashcard);
            self.asteroids.push(new_asteroid);
        }

        //In later levels spawn a third asteroid
        if rand::random::<u32>() % 4 == 0 && self.level >= 6 {
            let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
            let y = CANVAS_H + ASTEROID_SIZE + rand::random::<f32>() * 320.0;
            let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            let flashcard = self.get_random_card();
            let new_asteroid = Asteroid::new(x, y, ASTEROID_SIZE, rotation, flashcard);
            self.asteroids.push(new_asteroid);
        }

        let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
        let y = CANVAS_H / 2.0 + ASTEROID_SIZE / 2.0;
        let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let flashcard = self.get_random_card();
        let new_asteroid = Asteroid::new(x, y, ASTEROID_SIZE, rotation, flashcard);
        self.asteroids.push(new_asteroid);
    }

    //Returns if its game over
    pub fn game_over(&self) -> bool {
        self.health == 0
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        if self.levelup_animation_timer > 0.0 {
            self.levelup_animation_timer -= dt;
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

        //Delete asteroids
        let mut keep = vec![];
        for asteroid in &self.asteroids {
            if !asteroid.deleted {
                keep.push(asteroid.clone());
                continue;
            }

            //If the asteroid hits the bottom of the screen, lose health
            if asteroid.at_bottom() && self.health > 0 {
                self.health -= 1;
            }

            //Add explosion
            if asteroid.at_bottom() || asteroid.destroyed {
                let (x, y) = (asteroid.sprite.x, asteroid.sprite.y);
                self.explosions.push(Explosion::new(x, y));
            }
        }
        self.asteroids = keep;
    }
}
