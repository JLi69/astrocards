/*
 * This file contains code for various sprites in the game
 * */

use super::draw::CANVAS_H;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Asteroid {
    pub sprite: Sprite2D,
    pub rotation: f32,
    //Flag for if the asteroid is flagged to be deleted
    pub deleted: bool,
}

impl Asteroid {
    //x, y, size, rotation
    pub fn new(x: f32, y: f32, sz: f32, r: f32) -> Self {
        Self {
            sprite: Sprite2D::new(x, y, sz, sz),
            rotation: r,
            deleted: false,
        }
    }

    //Returns if the asteroid is at the bottom of the screen
    pub fn at_bottom(&self) -> bool {
        self.sprite.y < -CANVAS_H / 2.0
    }

    pub fn update(&mut self, dt: f32, speed: f32) {
        self.sprite.y -= speed * dt;
        self.rotation += dt * std::f32::consts::PI / 4.0;

        if self.at_bottom() {
            self.deleted = true;
        }
    }
}

#[derive(Clone)]
pub struct Explosion {
    pub x: f32,
    pub y: f32,
    pub time: f32,
}

impl Explosion {
    pub fn new(posx: f32, posy: f32) -> Self {
        Self {
            x: posx,
            y: posy,
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }
}
