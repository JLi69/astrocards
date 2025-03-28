mod assets;
mod game;
mod gfx;
mod impfile;

use game::Game;
use glfw::{Context, WindowMode};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to init glfw!");
    //Init window
    let (mut window, events) = glfw
        .create_window(960, 540, "Astrocards", WindowMode::Windowed)
        .expect("Failed to init window!");
    window.make_current();
    window.set_size_polling(true);
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

    let mut dt = 0.0f32;
    gfx::set_default_gl_state();
    while !window.should_close() {
        let start = std::time::Instant::now();
        //Clear screen
        gfx::clear();

        gamestate.draw();
        gamestate.update(dt);

        //Print OpenGL errors
        gfx::get_gl_errors();
        //Poll events and swap buffers
        glfw.poll_events();
        window.swap_buffers();
        //Handle events
        gamestate.process_events(&events);
        dt = start.elapsed().as_secs_f32();
    }
}
