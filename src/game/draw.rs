use crate::{assets::shader::ShaderProgram, gui::gui_pos};
use cgmath::{Matrix4, Rad, Vector3};
use egui_gl_glfw::egui::vec2;

use super::{
    Game,
    assets::models::{draw_elements, draw_elements_instanced},
    update::EXPLOSION_LIFETIME,
};

pub const CANVAS_W: f32 = 960.0;
pub const CANVAS_H: f32 = 540.0;
const ASPECT: f32 = CANVAS_W / CANVAS_H;

pub fn calculate_screen_mat(w: i32, h: i32) -> Matrix4<f32> {
    let (w, h) = (w as f32, h as f32);
    if w < h * ASPECT {
        Matrix4::from_nonuniform_scale((h / CANVAS_H) / w * 2.0, (h / CANVAS_H) / h * 2.0, 1.0)
    } else {
        Matrix4::from_nonuniform_scale((w / CANVAS_W) / w * 2.0, (w / CANVAS_W) / h * 2.0, 1.0)
    }
}

pub fn calculate_screen_scale(w: i32, h: i32) -> f32 {
    let (w, h) = (w as f32, h as f32);
    if w < h * ASPECT {
        h / CANVAS_H
    } else {
        w / CANVAS_W
    }
}

pub fn caclulate_canv_offset(w: i32, h: i32) -> (f32, f32) {
    let (w, h) = (w as f32, h as f32);
    if w < h * ASPECT {
        (-(h * ASPECT - w) / 2.0, 0.0)
    } else {
        (0.0, -(w / ASPECT - h) / 2.0)
    }
}

//For debugging the font texture
//Assumes the font texture has already been bound and uses that texture when
//displaying the debug quad
#[allow(dead_code)]
fn debug_quad(gamestate: &Game, shader: &ShaderProgram) {
    let transform = Matrix4::from_nonuniform_scale(CANVAS_W, 100.0, 1.0);
    shader.uniform_matrix4f("transform", &transform);
    let quad = gamestate.models.bind("quad2d");
    draw_elements(quad);
}

//Display background
fn draw_background(gamestate: &Game, shader: &ShaderProgram) {
    let transform = Matrix4::from_nonuniform_scale(CANVAS_W, CANVAS_H, 1.0);
    gamestate.textures.bind("background");
    shader.uniform_matrix4f("transform", &transform);
    let quad = gamestate.models.bind("quad2d");
    draw_elements(quad);
}

fn draw_asteroids(gamestate: &Game, shader: &ShaderProgram) {
    gamestate.textures.bind("asteroid");
    let quad = gamestate.models.bind("quad2d");
    for asteroid in &gamestate.asteroids {
        let w = asteroid.sprite.width;
        let h = asteroid.sprite.height;
        let translate = Vector3::new(asteroid.sprite.x, asteroid.sprite.y, 0.0);
        let transform = Matrix4::from_translation(translate)
            * Matrix4::from_angle_z(Rad(asteroid.rotation))
            * Matrix4::from_nonuniform_scale(w, h, 1.0);
        shader.uniform_matrix4f("transform", &transform);
        draw_elements(quad.clone());
    }
}

fn draw_asteroids_flame(gamestate: &Game, shader: &ShaderProgram) {
    gamestate.textures.bind("fire");
    let quad = gamestate.models.bind("quad2d");
    for asteroid in &gamestate.asteroids {
        let translate = Vector3::new(asteroid.sprite.x, asteroid.sprite.y, 0.0);
        let perc = (-(-CANVAS_H / 2.0 - asteroid.sprite.y) * 2.0 / CANVAS_H).clamp(0.0, 1.0);
        shader.uniform_float("alpha", perc);
        let transform =
            Matrix4::from_translation(translate) * Matrix4::from_nonuniform_scale(40.0, 40.0, 1.0);
        shader.uniform_matrix4f("transform", &transform);
        draw_elements_instanced(quad.clone(), 128);
    }
}

fn draw_explosions(gamestate: &Game, shader: &ShaderProgram) {
    gamestate.textures.bind("fire");
    let quad = gamestate.models.bind("quad2d");
    shader.uniform_float("lifetime", EXPLOSION_LIFETIME);
    for explosion in &gamestate.explosions {
        let translate = Vector3::new(explosion.x, explosion.y, 0.0);
        let transform =
            Matrix4::from_translation(translate) * Matrix4::from_nonuniform_scale(80.0, 80.0, 1.0);
        shader.uniform_matrix4f("transform", &transform);
        shader.uniform_float("time", explosion.time);
        draw_elements_instanced(quad.clone(), 32);
    }
}

fn display_icon(
    gamestate: &Game,
    shader: &ShaderProgram,
    icon_name: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32
) {
    let scale = calculate_screen_scale(gamestate.window_w, gamestate.window_h) * 2.0;
    let pos = gui_pos(x, y, gamestate.window_w, gamestate.window_h)
        - vec2(gamestate.window_w as f32, -gamestate.window_h as f32) / scale;
    let transform = Matrix4::from_translation(Vector3::new(pos.x, pos.y, 0.0))
        * Matrix4::from_nonuniform_scale(w, h, 1.0);
    gamestate.textures.bind(icon_name);
    shader.uniform_matrix4f("transform", &transform);
    let quad = gamestate.models.bind("quad2d");
    draw_elements(quad);
}

impl Game {
    pub fn draw(&self) {
        let screen_mat = calculate_screen_mat(self.window_w, self.window_h);
        let shader = self.shaders.use_program("quadshader");
        shader.uniform_matrix4f("screen", &screen_mat);

        //For debug purposes
        //debug_quad(self, &shader);

        //Draw the background
        draw_background(self, &shader);

        //Draw asteroid flames
        let flameshader = self.shaders.use_program("flameshader");
        flameshader.uniform_float("time", self.time());
        flameshader.uniform_matrix4f("screen", &screen_mat);
        draw_asteroids_flame(self, &flameshader);

        //Draw asteroids
        shader.use_program();
        draw_asteroids(self, &shader);

        //Draw explosions
        let explosionshader = self.shaders.use_program("explosionshader");
        explosionshader.uniform_matrix4f("screen", &screen_mat);
        draw_explosions(self, &explosionshader); 

        //Display heart icon (for gui)
        shader.use_program();
        display_icon(self, &shader, "hearticon", 24.0, 26.0, 24.0, 24.0);

        //Unbind textures
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
