mod assets;
mod flashcards;
mod game;
mod gfx;
mod gui;
mod impfile;
mod log;

use game::Game;
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
    load_icon(&mut window);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    //Load OpenGL
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    //Initialize game
    let mut gamestate = Game::new();
    //Load config
    gamestate.load_config("cfg.impfile");
    //Load assets
    gamestate.load_assets();
    gamestate.init_window_dimensions(window.get_size());
    //Load flashcards
    gamestate.flashcards = flashcards::load_flashcards(&args[1..]);
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

        //Display gui
        gamestate.draw();
        gamestate.update(dt);
        gui::set_ui_gl_state();
        let gui_action = gui_controller.display_game_gui(&mut gamestate, w, h);

        if let Some(action) = gui_action {
            gui::handle_gui_action(&mut gamestate, action);
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
