#![windows_subsystem = "windows"]

mod assets;
mod flashcards;
mod game;
mod gfx;
mod gui;
mod impfile;
mod log;

use game::{Game, GameScreen};
use glfw::{Context, WindowMode};
use gui::GuiController;

//Load `assets/icon.png` as the window icon
fn load_icon(window: &mut glfw::Window) {
    match assets::texture::load_image_pixels("assets/icon.png") {
        Ok((pixel_data, info)) => {
            window.set_icon_from_pixels(vec![glfw::PixelImage {
                pixels: pixel_data,
                width: info.width,
                height: info.height,
            }]);
        }
        Err(msg) => eprintln!("{msg}"),
    }
}

fn run_game(gamestate: &mut Game, gui_controller: &mut GuiController, dt: f32) {
    gamestate.draw();
    gamestate.update(dt);
    //Display gui
    gui::set_ui_gl_state();
    let gui_action = gui_controller.display_game_gui(gamestate);
    if let Some(action) = gui_action {
        gui::handle_gui_action(gamestate, action);
    }
}

fn run_main_menu(gamestate: &mut Game, gui_controller: &mut GuiController, dt: f32) {
    //Display background
    gamestate.draw_background_only();
    gamestate.update_time(dt);
    //Display gui
    gui::set_ui_gl_state();
    let gui_action = gui_controller.display_main_menu_gui(gamestate);
    if let Some(action) = gui_action {
        gui::handle_gui_action(gamestate, action);
    }
}

fn run_about_screen(gamestate: &mut Game, gui_controller: &mut GuiController, dt: f32) {
    //Display background
    gamestate.draw_background_only();
    gamestate.update_time(dt);
    //Display gui
    gui::set_ui_gl_state();
    let gui_action = gui_controller.display_about_screen(gamestate);
    if let Some(action) = gui_action {
        gui::handle_gui_action(gamestate, action);
    }
}

fn run_load_flashcards(gamestate: &mut Game, gui_controller: &mut GuiController, dt: f32) {
    //Display background
    gamestate.draw_background_only();
    gamestate.update_time(dt);
    //Display gui
    gui::set_ui_gl_state();
    let gui_action = gui_controller.display_load_screen(gamestate);
    if let Some(action) = gui_action {
        gui::handle_gui_action(gamestate, action);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to init glfw!");
    //Init window
    let (mut window, events) = glfw
        .create_window(1152, 648, "Astrocards", WindowMode::Windowed)
        .expect("Failed to init window!");
    window.make_current();
    window.set_size_polling(true);
    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    load_icon(&mut window);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    //Load OpenGL
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    //Initialize game
    let mut gamestate = Game::new();
    //Load config
    gamestate.load_config("cfg.impfile");
    //Load about text
    gamestate.load_about();
    //Load assets
    gamestate.load_assets();
    gamestate.init_window_dimensions(window.get_size());
    //Load flashcards
    gamestate.flashcards = flashcards::load_flashcards(&args[1..]);
    if gamestate.flashcards.is_empty() {
        gamestate.current_screen = GameScreen::MainMenu;
    }
    //gui controller
    let mut gui_controller = GuiController::init(&window);
    gui_controller.init_font(&gamestate);

    let mut dt = 0.0f32;
    while !window.should_close() {
        let start = std::time::Instant::now();

        //Clear screen
        gfx::set_default_gl_state();
        gfx::clear();

        //Update gui state
        let (w, h) = window.get_size();
        let pixels_per_point = window.get_content_scale().0;
        gui_controller.update_state(w, h, gamestate.time(), pixels_per_point);

        match gamestate.current_screen {
            GameScreen::Game => run_game(&mut gamestate, &mut gui_controller, dt),
            GameScreen::MainMenu => run_main_menu(&mut gamestate, &mut gui_controller, dt),
            GameScreen::About => run_about_screen(&mut gamestate, &mut gui_controller, dt),
            GameScreen::LoadFlashcards => {
                run_load_flashcards(&mut gamestate, &mut gui_controller, dt)
            }
        }

        //Print OpenGL errors
        gfx::get_gl_errors();
        //Poll events and swap buffers
        glfw.poll_events();
        window.swap_buffers();
        //Handle events
        gamestate.process_events(&events, &mut gui_controller);
        dt = start.elapsed().as_secs_f32();
    }
}
