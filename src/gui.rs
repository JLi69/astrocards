use crate::flashcards;
use crate::game::draw::{CANVAS_H, CANVAS_W, caclulate_canv_offset, calculate_screen_scale};
use crate::game::{Game, GameScreen};
use cgmath::Vector4;
use egui_backend::egui::{self, RichText};
use egui_backend::egui::{Align2, Color32, FontId, Pos2, RawInput, Rect, Ui, vec2};
use egui_backend::{EguiInputState, Painter};
use egui_gl_glfw as egui_backend;
use glfw::{Window, WindowEvent};

//gui action
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GuiAction {
    Restart,
    GotoMainMenu,
    GotoAbout,
    GotoLoadFlashcards,
    Load,
    ToggleMute,
    Quit,
}

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
    pub input_state: EguiInputState,
}

pub fn world_to_eguipos(x: f32, y: f32, w: i32, h: i32) -> Pos2 {
    let pos = Vector4::new(x / CANVAS_W, y / CANVAS_H, 0.0, 1.0);
    let canvas_pos = pos + Vector4::new(0.5, 0.5, 0.0, 0.0);

    let screen_scale = calculate_screen_scale(w, h);
    //Calculate the text position on the screen
    let (dx, dy) = caclulate_canv_offset(w, h);
    let tx = CANVAS_W * canvas_pos.x + dx / screen_scale;
    let ty = CANVAS_H * (1.0 - canvas_pos.y) + dy / screen_scale;
    Pos2::new(tx, ty)
}

fn display_asteroid_text(gamestate: &Game, ui: &Ui) {
    let (w, h) = gamestate.get_window_size();
    let painter = ui.painter();
    let font_id = FontId::new(16.0, egui::FontFamily::Monospace);
    for asteroid in &gamestate.asteroids {
        let text_pos = world_to_eguipos(asteroid.sprite.x, asteroid.sprite.y, w, h);
        //Display the text
        painter.text(
            text_pos,
            Align2::CENTER_CENTER,
            &asteroid.flashcard.question,
            font_id.clone(),
            Color32::WHITE,
        );
    }
}

pub fn gui_pos(x: f32, y: f32, w: i32, h: i32) -> Pos2 {
    let screen_scale = calculate_screen_scale(w, h);
    let corner = vec2(-w as f32 / 2.0, -h as f32 / 2.0) / screen_scale;
    world_to_eguipos(x, y, w, h) + corner
}

fn display_hud(gamestate: &Game, ui: &Ui) {
    let (w, h) = gamestate.get_window_size();
    let painter = ui.painter();
    let font_id = FontId::new(16.0, egui::FontFamily::Monospace);

    //Display health
    painter.text(
        gui_pos(40.0, -16.0, w, h),
        Align2::LEFT_TOP,
        format!("{}", gamestate.health),
        font_id.clone(),
        Color32::WHITE,
    );
    //Display score
    painter.text(
        gui_pos(16.0, -40.0, w, h),
        Align2::LEFT_TOP,
        format!("SCORE: {}", gamestate.score),
        font_id.clone(),
        Color32::WHITE,
    );
    //Display level
    painter.text(
        gui_pos(16.0, -64.0, w, h),
        Align2::LEFT_TOP,
        format!("LEVEL: {}", gamestate.level),
        font_id.clone(),
        Color32::WHITE,
    );
}

fn smoothstep_up(x: f32) -> f32 {
    1.0 - (1.0 - x).powi(2)
}

fn display_levelup(gamestate: &Game, ui: &Ui) {
    if gamestate.levelup_animation_perc() <= 0.0 {
        return;
    }

    let (w, h) = gamestate.get_window_size();
    let painter = ui.painter();
    let font_id = FontId::new(64.0, egui::FontFamily::Monospace);
    let perc = gamestate.levelup_animation_perc();
    let y = if perc < 0.25 {
        -CANVAS_H - 80.0 + (80.0 + CANVAS_H / 2.0) * smoothstep_up(perc / 0.25)
    } else if (0.25..=0.75).contains(&perc) {
        -CANVAS_H / 2.0
    } else {
        -CANVAS_H / 2.0 + (80.0 + CANVAS_H / 2.0) * ((perc - 0.75) / 0.25).powi(2)
    };
    painter.text(
        gui_pos(CANVAS_W / 2.0, y, w, h),
        Align2::CENTER_CENTER,
        "LEVEL UP",
        font_id.clone(),
        Color32::WHITE,
    );
}

fn display_log(gamestate: &Game, ui: &Ui, pixels_per_point: f32) {
    if gamestate.log.is_empty() {
        return;
    }

    let (w, h) = gamestate.get_window_size();
    let painter = ui.painter();
    let font_id = FontId::new(16.0, egui::FontFamily::Monospace);
    for (i, log_item) in gamestate.log.iter().enumerate() {
        //Calculate gui x position
        let gui_position = gui_pos(32.0, 0.0, w, h);
        //Calculate the y position (subtract size of window at bottom of screen)
        let y = h as f32 / pixels_per_point - 56.0 - i as f32 * 24.0;
        painter.text(
            Pos2::new(gui_position.x, y),
            Align2::LEFT_BOTTOM,
            log_item.message(),
            font_id.clone(),
            Color32::from_rgb(255, 64, 64),
        );
    }
}

fn new_button(ui: &mut Ui, text: &str, sz: f32, action: GuiAction) -> Option<GuiAction> {
    let button_text = RichText::new(text).size(sz).color(Color32::WHITE);
    let button = ui.button(button_text);
    if button.clicked() {
        //Restart session
        Some(action)
    } else {
        None
    }
}

fn update_action(action: Option<GuiAction>, new_action: Option<GuiAction>) -> Option<GuiAction> {
    if new_action.is_some() {
        new_action
    } else {
        action
    }
}

fn display_lines(ui: &mut Ui, lines: &[String]) {
    for line in lines {
        let text = RichText::new(line).size(16.0).color(Color32::WHITE);
        ui.label(text);
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
    pub fn display_game_gui(&mut self, gamestate: &mut Game) -> Option<GuiAction> {
        let (w, h) = gamestate.get_window_size();
        let mut action = None;

        let pixels_per_point = self.input_state.pixels_per_point;
        if self.ctx.pixels_per_point() != pixels_per_point {
            self.ctx.set_pixels_per_point(pixels_per_point);
        }
        self.ctx.begin_pass(self.input_state.input.take());

        //Display asteroid textures
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(&self.ctx, |ui| {
                display_asteroid_text(gamestate, ui);
                display_hud(gamestate, ui);
                display_levelup(gamestate, ui);
                display_log(gamestate, ui, pixels_per_point);
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

        //Display go to main menu button and mute button
        egui::Window::new("buttons")
            .frame(egui::Frame::none())
            .movable(false)
            .title_bar(false)
            .scroll(true)
            .fixed_size(vec2(90.0, 80.0))
            .fixed_pos(Pos2::new(w as f32 / pixels_per_point - 98.0, 10.0))
            .show(&self.ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let main_menu = new_button(ui, "Main Menu", 12.0, GuiAction::GotoMainMenu);
                    action = update_action(action, main_menu);
                    let mute = if gamestate.audio.muted() {
                        new_button(ui, "Unmute", 12.0, GuiAction::ToggleMute)
                    } else {
                        new_button(ui, "Mute", 12.0, GuiAction::ToggleMute)
                    };
                    action = update_action(action, mute);
                });
            });

        //Display game over screen
        if gamestate.game_over() {
            let width = w as f32 / pixels_per_point;
            let height = h as f32 / pixels_per_point;
            egui::Window::new("game_over_screen")
                .frame(egui::Frame::none().fill(Color32::from_rgba_unmultiplied(255, 0, 0, 128)))
                .movable(false)
                .title_bar(false)
                .scroll(true)
                .fixed_size(vec2(width, height))
                .fixed_pos(Pos2::new(0.0, 0.0))
                .show(&self.ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(height / 4.0);
                        ui.label(RichText::new("Game Over!").size(64.0).color(Color32::WHITE));
                        let final_score = format!("Final Score: {}", gamestate.score);
                        ui.label(RichText::new(final_score).size(16.0).color(Color32::WHITE));
                        let final_level = format!("Final Level: {}", gamestate.level);
                        ui.label(RichText::new(final_level).size(16.0).color(Color32::WHITE));
                        ui.add_space(height / 32.0);
                        //Restart button
                        let restart = new_button(ui, "  Restart  ", 20.0, GuiAction::Restart);
                        action = update_action(action, restart);
                        //Go to main menu
                        let main_menu =
                            new_button(ui, " Main Menu ", 20.0, GuiAction::GotoMainMenu);
                        action = update_action(action, main_menu);
                    })
                });
        }

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

        action
    }

    //Display gui for main menu
    pub fn display_main_menu_gui(&mut self, gamestate: &mut Game) -> Option<GuiAction> {
        let mut action = None;
        let (w, h) = gamestate.get_window_size();

        let pixels_per_point = self.input_state.pixels_per_point;
        if self.ctx.pixels_per_point() != pixels_per_point {
            self.ctx.set_pixels_per_point(pixels_per_point);
        }
        self.ctx.begin_pass(self.input_state.input.take());

        //Display asteroid textures
        let width = w as f32 / pixels_per_point;
        let height = h as f32 / pixels_per_point;
        const BOTTOM_HEIGHT: f32 = 32.0;
        egui::Window::new("main_menu")
            .frame(egui::Frame::none())
            .movable(false)
            .title_bar(false)
            .scroll(true)
            .fixed_size(vec2(width, height - BOTTOM_HEIGHT))
            .fixed_pos(Pos2::new(0.0, 0.0))
            .show(&self.ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(height / 8.0);
                    let title_text = RichText::new("Astrocards").size(64.0).color(Color32::WHITE);
                    ui.label(title_text);
                    ui.add_space(height / 48.0);
                    //Load flashcard set screen
                    let load_set =
                        new_button(ui, " Load Set ", 24.0, GuiAction::GotoLoadFlashcards);
                    action = update_action(action, load_set);
                    ui.add_space(height / 48.0);
                    //Go to about screen
                    let about = new_button(ui, "   About   ", 24.0, GuiAction::GotoAbout);
                    action = update_action(action, about);
                    ui.add_space(height / 48.0);
                    //Quit button
                    let quit = new_button(ui, "     Quit     ", 24.0, GuiAction::Quit);
                    action = update_action(action, quit);
                });
            });

        egui::Window::new("mute")
            .frame(egui::Frame::none())
            .movable(false)
            .title_bar(false)
            .scroll(true)
            .fixed_size(vec2(80.0, BOTTOM_HEIGHT))
            .fixed_pos(Pos2::new(8.0, h as f32 / pixels_per_point - 32.0))
            .show(&self.ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let mute = if gamestate.audio.muted() {
                        new_button(ui, "Unmute", 16.0, GuiAction::ToggleMute)
                    } else {
                        new_button(ui, "Mute", 16.0, GuiAction::ToggleMute)
                    };
                    action = update_action(action, mute);
                });
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

        action
    }

    //Display gui for about menu
    pub fn display_about_screen(&mut self, gamestate: &mut Game) -> Option<GuiAction> {
        let mut action = None;
        let (w, h) = gamestate.get_window_size();

        let pixels_per_point = self.input_state.pixels_per_point;
        if self.ctx.pixels_per_point() != pixels_per_point {
            self.ctx.set_pixels_per_point(pixels_per_point);
        }
        self.ctx.begin_pass(self.input_state.input.take());

        //Display asteroid textures
        let margin = 160.0;
        let width = w as f32 / pixels_per_point - margin * 2.0;
        let height = h as f32 / pixels_per_point - 32.0;
        egui::Window::new("about")
            .frame(egui::Frame::none())
            .movable(false)
            .title_bar(false)
            .scroll(true)
            .fixed_size(vec2(width, height))
            .fixed_pos(Pos2::new(margin, 16.0))
            .show(&self.ctx, |ui| {
                ui.vertical(|ui| {
                    let title_text = RichText::new("About").size(32.0).color(Color32::WHITE);
                    ui.label(title_text);
                    //Return to main menu
                    let main_menu = new_button(ui, "Main Menu", 16.0, GuiAction::GotoMainMenu);
                    action = update_action(action, main_menu);
                    ui.add_space(24.0);
                    egui::ScrollArea::vertical()
                        .max_width(width)
                        .show(ui, |ui| {
                            //Display text
                            display_lines(ui, &gamestate.about_text);
                        });
                });
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

        action
    }

    //Display gui for load set menu
    pub fn display_load_screen(&mut self, gamestate: &mut Game) -> Option<GuiAction> {
        let mut action = None;
        let (w, h) = gamestate.get_window_size();

        let pixels_per_point = self.input_state.pixels_per_point;
        if self.ctx.pixels_per_point() != pixels_per_point {
            self.ctx.set_pixels_per_point(pixels_per_point);
        }
        self.ctx.begin_pass(self.input_state.input.take());

        //Display asteroid textures
        let margin = 160.0;
        let width = w as f32 / pixels_per_point - margin * 2.0;
        let height = h as f32 / pixels_per_point - 32.0;
        egui::Window::new("load_sets")
            .frame(egui::Frame::none())
            .movable(false)
            .title_bar(false)
            .scroll(true)
            .fixed_size(vec2(width, height))
            .fixed_pos(Pos2::new(margin, 16.0))
            .show(&self.ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let title_text = RichText::new("Load Sets").size(32.0).color(Color32::WHITE);
                    ui.label(title_text);
                    ui.add_space(8.0);
                    egui::ScrollArea::vertical()
                        .max_width(width)
                        .max_height(height - 256.0)
                        .show(ui, |ui| {
                            for set in &gamestate.set_paths {
                                let text = RichText::new(set).size(16.0).color(Color32::WHITE);
                                ui.selectable_value(
                                    &mut gamestate.selected_set_path,
                                    set.clone(),
                                    text,
                                );
                            }
                        });
                    ui.add_space(8.0);
                    //Load set
                    let load = new_button(ui, "Load", 16.0, GuiAction::Load);
                    action = update_action(action, load);
                    //Return to main menu
                    let main_menu = new_button(ui, "Main Menu", 16.0, GuiAction::GotoMainMenu);
                    action = update_action(action, main_menu);
                });
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

        action
    }
}

pub fn handle_gui_action(gamestate: &mut Game, action: GuiAction) {
    match action {
        GuiAction::Restart => gamestate.restart(),
        GuiAction::GotoMainMenu => gamestate.current_screen = GameScreen::MainMenu,
        GuiAction::GotoAbout => gamestate.current_screen = GameScreen::About,
        GuiAction::GotoLoadFlashcards => {
            gamestate.current_screen = GameScreen::LoadFlashcards;
            gamestate.get_set_list();
            gamestate.selected_set_path.clear();
        }
        GuiAction::Load => {
            if gamestate.selected_set_path.is_empty() {
                return;
            }
            gamestate.restart();
            let path = vec![flashcards::get_set_path(&gamestate.selected_set_path)];
            gamestate.flashcards = flashcards::load_flashcards(&path);
            if gamestate.flashcards.is_empty() {
                return;
            }
            gamestate.current_screen = GameScreen::Game;
        }
        GuiAction::ToggleMute => gamestate.audio.toggle_mute(),
        GuiAction::Quit => std::process::exit(0),
    }
}
