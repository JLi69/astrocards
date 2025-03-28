use super::{
    Game,
    draw::{CANVAS_H, CANVAS_W},
    sprite::{Asteroid, Explosion},
};

const ASTEROID_SIZE: f32 = 80.0;
const DEFAULT_ASTEROID_SPEED: f32 = CANVAS_H / 32.0;
pub const EXPLOSION_LIFETIME: f32 = 1.0; //1 second

impl Game {
    pub fn spawn_asteroid(&mut self, dt: f32) {
        self.asteroid_spawn_timer -= dt;
        if self.asteroid_spawn_timer > 0.0 {
            return;
        }
        self.asteroid_spawn_timer = self.spawn_interval;

        let range = CANVAS_W - ASTEROID_SIZE * 2.0;
        //Spawn an extra asteroid
        if rand::random() {
            let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
            let y = CANVAS_H + ASTEROID_SIZE + rand::random::<f32>() * 320.0;
            let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            self.asteroids
                .push(Asteroid::new(x, y, ASTEROID_SIZE, rotation));
        }

        let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W / 2.0;
        let y = CANVAS_H / 2.0 + ASTEROID_SIZE / 2.0;
        let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        self.asteroids
            .push(Asteroid::new(x, y, ASTEROID_SIZE, rotation));
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        //Spawn asteroids
        self.spawn_asteroid(dt);

        //Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update(dt, DEFAULT_ASTEROID_SPEED);
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
        let mut keep = vec![];
        for asteroid in &self.asteroids {
            if !asteroid.deleted {
                keep.push(asteroid.clone());
                continue;
            }

            //Add explosion
            if asteroid.at_bottom() {
                let (x, y) = (asteroid.sprite.x, asteroid.sprite.y);
                self.explosions.push(Explosion::new(x, y));
            }
        }
        self.asteroids = keep;
    }
}
