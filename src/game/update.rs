use super::{Game, sprite::Asteroid, draw::{CANVAS_H, CANVAS_W}};

const ASTEROID_SIZE: f32 = 80.0;
const DEFAULT_ASTEROID_SPEED: f32 = CANVAS_H / 32.0;

impl Game {
    pub fn spawn_asteroid(&mut self, dt: f32) {
        self.asteroid_spawn_timer -= dt;
        if self.asteroid_spawn_timer > 0.0 {
            return;
        }
        self.asteroid_spawn_timer = self.spawn_interval;

        let count = rand::random::<u32>() % 3;
        let range = CANVAS_W - ASTEROID_SIZE;
        for _ in 0..count {
            let x = rand::random::<f32>() * range + ASTEROID_SIZE - CANVAS_W;
            let y = CANVAS_H + ASTEROID_SIZE + rand::random::<f32>() * 320.0;
            let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
            self.asteroids.push(Asteroid::new(x, y, ASTEROID_SIZE, rotation));
        }

        let x = rand::random::<f32>() * range + ASTEROID_SIZE / 2.0 - CANVAS_W / 2.0;
        let y = CANVAS_H / 2.0 + ASTEROID_SIZE / 2.0;
        let rotation = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        self.asteroids.push(Asteroid::new(x, y, ASTEROID_SIZE, rotation));
    }

    pub fn update(&mut self, dt: f32) {
        //Spawn asteroids
        self.spawn_asteroid(dt);

        //Update asteroids
        for asteroid in &mut self.asteroids {
            asteroid.update(dt, DEFAULT_ASTEROID_SPEED);
        }
    }
}
