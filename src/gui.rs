use crate::game::Game;
use crate::game::draw::{CANVAS_H, CANVAS_W, caclulate_canv_offset, calculate_screen_scale};
use cgmath::Vector4;
use egui_backend::egui;
use egui_backend::egui::{Align2, Color32, FontId, Pos2, RawInput, Rect, Ui, vec2};
use egui_backend::{EguiInputState, Painter};
use egui_gl_glfw as egui_backend;
use glfw::{Window, WindowEvent};

//Initialized the egui input state
pub fn init_egui_input_state(window: &Window) -> EguiInputState {
    let (w, h) = window.get_size();
    let native_pixels_per_point = window.get_content_scale().0;
    let dimensions = vec2(w as f32, h as f32) / native_pixels_per_point;
    let rect = Rect::from_min_size(Pos2::new(0.0, 0.0), dimensions);
    let raw_input = RawInput {
        screen_rect: Some(rect),
        ..Default::default()
    };
    EguiInputState::new(raw_input, native_pixels_per_point)
}

//Sets the OpenGL state for rendering gui components
pub fn set_ui_gl_state() {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
        gl::Disable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(0.4, 0.8, 1.0, 1.0);
    }
}

pub struct GuiController {
    painter: Painter,
    ctx: egui::Context,
    input_state: EguiInputState,
}

fn display_asteroid_text(gamestate: &Game, ui: &Ui, w: i32, h: i32) {
    let painter = ui.painter();
    let font_id = FontId::new(16.0, egui::FontFamily::Monospace);
    let screen_scale = calculate_screen_scale(w, h);
    for asteroid in &gamestate.asteroids {
        //Convert the position of the asteroid to screen position
        let pos = Vector4::new(
            asteroid.sprite.x / CANVAS_W,
            asteroid.sprite.y / CANVAS_H,
            0.0,
            1.0,
        );
        let canvas_pos = pos + Vector4::new(0.5, 0.5, 0.0, 0.0);

        //Calculate the text position on the screen
        let (dx, dy) = caclulate_canv_offset(w, h);
        let tx = CANVAS_W * canvas_pos.x + dx / screen_scale;
        let ty = CANVAS_H * (1.0 - canvas_pos.y) + dy / screen_scale;
        let text_pos = Pos2::new(tx, ty);

        //Display the text
        //TODO: replace test text
        painter.text(
            text_pos,
            Align2::CENTER_CENTER,
            "[TEST]áéíóúàèìòù",
            font_id.clone(),
            Color32::WHITE,
        );
    }
}

impl GuiController {
    pub fn init(window: &Window) -> Self {
        Self {
            painter: Painter::new(window),
            ctx: egui::Context::default(),
            input_state: init_egui_input_state(window),
        }
    }

    pub fn init_font(&self, gamestate: &Game) {
        self.ctx.set_fonts(gamestate.get_font());
    }

    pub fn handle_window_event(&mut self, event: WindowEvent) {
        egui_backend::handle_event(event, &mut self.input_state);
    }

    pub fn update_state(&mut self, w: i32, h: i32, time: f32, pixels_per_point: f32) {
        self.painter.set_size(w as u32, h as u32);
        self.input_state.input.time = Some(time as f64);
        let screen_scale = calculate_screen_scale(w, h);
        self.input_state.pixels_per_point = pixels_per_point * screen_scale;
    }

    //Display and update game gui
    pub fn display_game_gui(&mut self, gamestate: &mut Game, w: i32, h: i32) {
        let pixels_per_point = self.input_state.pixels_per_point;
        self.ctx.set_pixels_per_point(pixels_per_point);
        self.ctx.begin_pass(self.input_state.input.take());

        //Display asteroid textures
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(&self.ctx, |ui| {
                display_asteroid_text(gamestate, ui, w, h);
            });

        //Answer input box
        egui::Window::new("bottom_panel")
            .movable(false)
            .title_bar(false)
            .scroll(true)
            .fixed_size(vec2(w as f32 / pixels_per_point - 64.0, 64.0))
            .fixed_pos(Pos2::new(24.0, h as f32 / pixels_per_point - 50.0))
            .show(&self.ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Type your answer here (press enter to submit):");
                    ui.text_edit_singleline(&mut gamestate.answer);
                })
            });

        //End frame
        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point: _,
            viewport_output: _,
        } = self.ctx.end_pass();

        //Handle copy pasting
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut self.input_state, platform_output.copied_text);
        }

        //Display
        let clipped_shapes = self.ctx.tessellate(shapes, pixels_per_point);
        self.painter
            .paint_and_update_textures(pixels_per_point, &clipped_shapes, &textures_delta);
    }
}
