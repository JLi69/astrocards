use cgmath::Matrix4;
use super::{Game, assets::models::draw_elements};

const CANVAS_W: f32 = 960.0;
const CANVAS_H: f32 = 540.0;
const ASPECT: f32 = CANVAS_W / CANVAS_H;

fn calculate_screen_mat(w: i32, h: i32) -> Matrix4<f32> {
    let (w, h) = (w as f32, h as f32);
    if w < h * ASPECT {
        Matrix4::from_nonuniform_scale((h / CANVAS_H) / w, (h / CANVAS_H) / h, 1.0)
    } else {
        Matrix4::from_nonuniform_scale((w / CANVAS_W) / w, (w / CANVAS_W) / h, 1.0)
    }
}

fn draw_background(gamestate: &Game, screen_mat: &Matrix4<f32>) {
    let transform = Matrix4::from_nonuniform_scale(CANVAS_W, CANVAS_H, 1.0);
    gamestate.textures.bind("background");
    let shader = gamestate.shaders.use_program("quadshader");
    shader.uniform_matrix4f("screen", &screen_mat);
    shader.uniform_matrix4f("transform", &transform);
    let quad = gamestate.models.bind("quad2d");
    draw_elements(quad);
}

impl Game {
    pub fn draw(&self) {
        let screen_mat = calculate_screen_mat(self.window_w, self.window_h);
        //Draw the background
        draw_background(self, &screen_mat);
    }
}
