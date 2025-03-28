/*
 * This file contains code for various sprites in the game
 * */

pub struct Sprite2D {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Sprite2D {
    pub fn new(spritex: f32, spritey: f32, w: f32, h: f32) -> Self {
        Self { 
            x: spritex, 
            y: spritey,
            width: w,
            height: h,
        }
    }
}

pub struct Asteroid {
    pub sprite: Sprite2D,
    pub rotation: f32,
}

impl Asteroid {
    //x, y, size, rotation
    pub fn new(x: f32, y: f32, sz: f32, r: f32) -> Self {
        Self { 
            sprite: Sprite2D::new(x, y, sz, sz),
            rotation: r,
        }
    }

    pub fn update(&mut self, dt: f32, speed: f32) {
        self.sprite.y -= speed * dt;
        self.rotation += dt * std::f32::consts::PI / 4.0;
    }
}
